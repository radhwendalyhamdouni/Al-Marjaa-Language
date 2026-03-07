use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::parser::ast::{Expr, Program, Stmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirection {
    Ltr,
    Rtl,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiRuntimeConfig {
    pub direction: TextDirection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiTheme {
    pub direction: TextDirection,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            direction: TextDirection::Rtl,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UiRuntimeProgram {
    pub components: HashMap<String, UiNode>,
    pub state: HashMap<String, HashMap<String, String>>,
    pub routes: HashMap<String, String>,
    pub event_handlers: HashMap<String, UiEventHandler>,
    pub theme: UiTheme,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UiEventHandler {
    pub params: Vec<String>,
    pub commands: Vec<UiCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiEventPolicy {
    pub component: String,
    pub event: String,
    pub handler: String,
    pub requires_confirmation: bool,
    pub allowed_roles: Vec<Role>,
    pub dangerous_targets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiCommand {
    ModbusWrite { target: String, is_dangerous: bool },
    Unknown,
}

impl Default for UiRuntimeConfig {
    fn default() -> Self {
        Self {
            direction: TextDirection::Rtl,
        }
    }
}

impl UiRuntimeProgram {
    pub fn from_ast(program: &Program) -> Result<Self, String> {
        let mut runtime = UiRuntimeProgram {
            theme: UiTheme::default(),
            ..UiRuntimeProgram::default()
        };

        for stmt in &program.statements {
            match stmt {
                Stmt::UiComponentDecl { name, body, .. } => {
                    let mut node = UiNode::new("section")
                        .with_id(name.clone())
                        .with_prop("data-component", name.clone());

                    for (event_name, handler_name) in collect_component_events(body) {
                        node = node.with_event(event_name, handler_name);
                    }

                    runtime.components.insert(name.clone(), node);
                }
                Stmt::StateDecl { name, value } => {
                    runtime.state.insert(name.clone(), expr_to_map(value)?);
                }
                Stmt::ThemeDecl { value, .. } => {
                    runtime.theme = UiTheme {
                        direction: parse_theme_direction(value)?,
                    };
                }
                Stmt::RouteDecl { value, .. } => {
                    runtime.routes = expr_to_map(value)?;
                }
                Stmt::EventHandlerDecl { name, params, body } => {
                    runtime.event_handlers.insert(
                        name.clone(),
                        UiEventHandler {
                            params: params.clone(),
                            commands: collect_commands(body),
                        },
                    );
                }
                _ => {}
            }
        }

        runtime.validate_routes()?;
        runtime.validate_event_bindings()?;

        Ok(runtime)
    }

    pub fn render_initial_route_html(&self) -> Result<String, String> {
        let initial_route = self
            .resolve_initial_route()
            .ok_or_else(|| "لا يوجد أي مسار UI مهيأ للعرض".to_string())?;

        self.render_route_html(initial_route)
    }

    pub fn render_route_html(&self, path: &str) -> Result<String, String> {
        let component_name = self
            .routes
            .get(path)
            .ok_or_else(|| format!("المسار غير معرف: {path}"))?;

        let component = self
            .components
            .get(component_name)
            .ok_or_else(|| format!("المكون غير معرف للمسار {path}: {component_name}"))?;

        let mut root = component.clone();
        if root.text.is_none() {
            root.text = Some(format!("واجهة {component_name}"));
        }

        Ok(render_html(
            &root,
            &UiRuntimeConfig {
                direction: self.theme.direction,
            },
        ))
    }

    pub fn trigger_component_event(
        &self,
        component_name: &str,
        event_name: &str,
        role: Role,
        confirmed: bool,
        gate: &mut SecurityGate,
    ) -> Result<(), String> {
        let component = self
            .components
            .get(component_name)
            .ok_or_else(|| format!("المكون غير معرف: {component_name}"))?;

        let canonical_name = canonical_event_name(event_name).unwrap_or(event_name);

        let handler = component
            .events
            .get(canonical_name)
            .ok_or_else(|| format!("الحدث {event_name} غير مربوط بالمكون {component_name}"))?;

        self.execute_event(handler, role, confirmed, gate)
    }

    pub fn execute_event(
        &self,
        event_name: &str,
        role: Role,
        confirmed: bool,
        gate: &mut SecurityGate,
    ) -> Result<(), String> {
        let handler = self
            .event_handlers
            .get(event_name)
            .ok_or_else(|| format!("معالج الحدث غير معرف: {event_name}"))?;

        for command in &handler.commands {
            if let UiCommand::ModbusWrite {
                target,
                is_dangerous,
            } = command
            {
                let confirmation = if *is_dangerous { confirmed } else { true };
                gate.validate_modbus_write(role, target, confirmation)?;
            }
        }

        Ok(())
    }

    fn validate_routes(&self) -> Result<(), String> {
        for (route, component_name) in &self.routes {
            if is_default_route_key(route) {
                continue;
            }

            if !self.components.contains_key(component_name) {
                return Err(format!(
                    "المسار {route} يشير إلى مكون غير معرف: {component_name}"
                ));
            }
        }

        Ok(())
    }

    fn validate_event_bindings(&self) -> Result<(), String> {
        for (component_name, component) in &self.components {
            for (event_name, handler_name) in &component.events {
                if !self.event_handlers.contains_key(handler_name) {
                    return Err(format!(
                        "المكون {component_name} يربط الحدث {event_name} بمعالج غير معرف: {handler_name}"
                    ));
                }
            }
        }

        Ok(())
    }

    pub fn list_security_sensitive_events(&self) -> Vec<&str> {
        self.event_handlers
            .iter()
            .filter_map(|(name, handler)| {
                handler
                    .commands
                    .iter()
                    .any(|command| {
                        matches!(
                            command,
                            UiCommand::ModbusWrite {
                                is_dangerous: true,
                                ..
                            }
                        )
                    })
                    .then_some(name.as_str())
            })
            .collect()
    }

    pub fn list_security_sensitive_bindings(&self) -> Vec<UiEventPolicy> {
        let mut bindings = Vec::new();

        for (component_name, component) in &self.components {
            for (event_name, handler_name) in &component.events {
                if let Some(policy) =
                    self.build_event_policy(component_name, event_name, handler_name)
                {
                    bindings.push(policy);
                }
            }
        }

        bindings
    }

    pub fn event_policy_for_component(
        &self,
        component_name: &str,
        event_name: &str,
    ) -> Result<UiEventPolicy, String> {
        let component = self
            .components
            .get(component_name)
            .ok_or_else(|| format!("المكون غير معرف: {component_name}"))?;

        let canonical_name = canonical_event_name(event_name).unwrap_or(event_name);
        let handler_name = component
            .events
            .get(canonical_name)
            .ok_or_else(|| format!("الحدث {event_name} غير مربوط بالمكون {component_name}"))?;

        self.build_event_policy(component_name, canonical_name, handler_name)
            .ok_or_else(|| {
                format!("لا توجد متطلبات أمنية خاصة للحدث {event_name} في المكون {component_name}")
            })
    }

    fn build_event_policy(
        &self,
        component_name: &str,
        event_name: &str,
        handler_name: &str,
    ) -> Option<UiEventPolicy> {
        let handler = self.event_handlers.get(handler_name)?;
        let dangerous_targets = handler
            .commands
            .iter()
            .filter_map(|command| match command {
                UiCommand::ModbusWrite {
                    target,
                    is_dangerous: true,
                } => Some(target.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        (!dangerous_targets.is_empty()).then_some(UiEventPolicy {
            component: component_name.to_string(),
            event: event_name.to_string(),
            handler: handler_name.to_string(),
            requires_confirmation: true,
            allowed_roles: vec![Role::Engineer, Role::Admin],
            dangerous_targets,
        })
    }

    fn resolve_initial_route(&self) -> Option<&str> {
        if self.routes.contains_key("/") {
            return Some("/");
        }

        if let Some(default_route) = self
            .routes
            .get("default")
            .or_else(|| self.routes.get("افتراضي"))
            .map(String::as_str)
        {
            if self.routes.contains_key(default_route) {
                return Some(default_route);
            }
        }

        self.routes
            .keys()
            .find(|key| !is_default_route_key(key))
            .map(String::as_str)
    }
}

fn is_default_route_key(route: &str) -> bool {
    matches!(route, "default" | "افتراضي")
}

fn collect_component_events(stmt: &Stmt) -> HashMap<String, String> {
    let mut events = HashMap::new();
    collect_component_events_into(stmt, &mut events);
    events
}

fn collect_component_events_into(stmt: &Stmt, events: &mut HashMap<String, String>) {
    match stmt {
        Stmt::Block(stmts) => {
            for stmt in stmts {
                collect_component_events_into(stmt, events);
            }
        }
        Stmt::Expression(expr) => collect_component_events_from_expr(expr, events),
        Stmt::If {
            then_branch,
            else_if_branches,
            else_branch,
            ..
        } => {
            collect_component_events_into(then_branch, events);
            for (_, branch) in else_if_branches {
                collect_component_events_into(branch, events);
            }
            if let Some(branch) = else_branch {
                collect_component_events_into(branch, events);
            }
        }
        _ => {}
    }
}

fn collect_component_events_from_expr(expr: &Expr, events: &mut HashMap<String, String>) {
    match expr {
        Expr::Call { callee, args } => {
            if matches!(args.first(), Some(Expr::Dictionary(_))) {
                if let Some(Expr::Dictionary(entries)) = args.first() {
                    for (key, value) in entries {
                        let Some(key_name) = expr_to_key_string(key) else {
                            continue;
                        };

                        if let (Some(canonical_event), Some(handler_name)) =
                            (canonical_event_name(&key_name), expr_to_handler_name(value))
                        {
                            events.insert(canonical_event.to_string(), handler_name);
                        }
                    }
                }
            }

            collect_component_events_from_expr(callee, events);
            for arg in args {
                collect_component_events_from_expr(arg, events);
            }
        }
        Expr::Assignment { value, .. } => collect_component_events_from_expr(value, events),
        Expr::Binary { left, right, .. }
        | Expr::Logical { left, right, .. }
        | Expr::Comparison { left, right, .. } => {
            collect_component_events_from_expr(left, events);
            collect_component_events_from_expr(right, events);
        }
        Expr::Unary { expr, .. } | Expr::Await(expr) => {
            collect_component_events_from_expr(expr, events)
        }
        Expr::Property { object, .. } => collect_component_events_from_expr(object, events),
        Expr::Index { object, index } => {
            collect_component_events_from_expr(object, events);
            collect_component_events_from_expr(index, events);
        }
        Expr::List(items) => {
            for item in items {
                collect_component_events_from_expr(item, events);
            }
        }
        Expr::Dictionary(entries) => {
            for (key, value) in entries {
                collect_component_events_from_expr(key, events);
                collect_component_events_from_expr(value, events);
            }
        }
        _ => {}
    }
}

fn expr_to_key_string(expr: &Expr) -> Option<String> {
    match expr {
        Expr::String(v) | Expr::Identifier(v) => Some(v.clone()),
        _ => None,
    }
}

fn expr_to_handler_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::String(v) | Expr::Identifier(v) => Some(v.clone()),
        _ => None,
    }
}

fn canonical_event_name(event: &str) -> Option<&'static str> {
    match event {
        "onClick" | "عند_النقر" => Some("onClick"),
        "onChange" | "عند_التغيير" => Some("onChange"),
        "onSubmit" | "عند_الإرسال" => Some("onSubmit"),
        _ => None,
    }
}

fn collect_commands(stmt: &Stmt) -> Vec<UiCommand> {
    match stmt {
        Stmt::Block(stmts) => stmts.iter().flat_map(collect_commands).collect(),
        Stmt::Expression(expr) => collect_commands_from_expr(expr),
        Stmt::If {
            then_branch,
            else_if_branches,
            else_branch,
            ..
        } => {
            let mut commands = collect_commands(then_branch);
            for (_, branch) in else_if_branches {
                commands.extend(collect_commands(branch));
            }
            if let Some(branch) = else_branch {
                commands.extend(collect_commands(branch));
            }
            commands
        }
        _ => Vec::new(),
    }
}

fn collect_commands_from_expr(expr: &Expr) -> Vec<UiCommand> {
    match expr {
        Expr::Call { callee, args } => {
            if let Expr::Property { object, property } = callee.as_ref() {
                if matches!(object.as_ref(), Expr::Identifier(name) if name == "modbus")
                    && property == "write"
                {
                    return vec![UiCommand::ModbusWrite {
                        target: extract_modbus_target(args),
                        is_dangerous: true,
                    }];
                }
            }

            let mut commands = collect_commands_from_expr(callee);
            for arg in args {
                commands.extend(collect_commands_from_expr(arg));
            }
            commands
        }
        Expr::Assignment { value, .. } => collect_commands_from_expr(value),
        Expr::Binary { left, right, .. }
        | Expr::Logical { left, right, .. }
        | Expr::Comparison { left, right, .. } => {
            let mut commands = collect_commands_from_expr(left);
            commands.extend(collect_commands_from_expr(right));
            commands
        }
        Expr::Unary { expr, .. } | Expr::Await(expr) => collect_commands_from_expr(expr),
        Expr::Property { object: expr, .. } => collect_commands_from_expr(expr),
        Expr::Index { object, index } => {
            let mut commands = collect_commands_from_expr(object);
            commands.extend(collect_commands_from_expr(index));
            commands
        }
        Expr::List(items) => items.iter().flat_map(collect_commands_from_expr).collect(),
        Expr::Dictionary(entries) => entries
            .iter()
            .flat_map(|(k, v)| {
                let mut commands = collect_commands_from_expr(k);
                commands.extend(collect_commands_from_expr(v));
                commands
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn extract_modbus_target(args: &[Expr]) -> String {
    args.first()
        .and_then(|expr| match expr {
            Expr::String(value) => Some(value.clone()),
            Expr::Identifier(value) => Some(value.clone()),
            _ => None,
        })
        .unwrap_or_else(|| "unknown_target".to_string())
}

fn expr_to_map(expr: &Expr) -> Result<HashMap<String, String>, String> {
    match expr {
        Expr::Dictionary(entries) => {
            let mut map = HashMap::new();
            for (k, v) in entries {
                map.insert(expr_to_scalar(k)?, expr_to_scalar(v)?);
            }
            Ok(map)
        }
        _ => Err("القيمة يجب أن تكون كائناً بصيغة قاموس".to_string()),
    }
}

fn parse_theme_direction(expr: &Expr) -> Result<TextDirection, String> {
    let theme = expr_to_map(expr)?;
    let direction = theme
        .get("اتجاه")
        .or_else(|| theme.get("direction"))
        .map(String::as_str)
        .unwrap_or("RTL");

    match direction {
        "RTL" | "rtl" | "يمين-يسار" => Ok(TextDirection::Rtl),
        "LTR" | "ltr" | "يسار-يمين" => Ok(TextDirection::Ltr),
        _ => Err(format!("قيمة اتجاه الثيم غير مدعومة: {direction}")),
    }
}

fn expr_to_scalar(expr: &Expr) -> Result<String, String> {
    match expr {
        Expr::String(v) => Ok(v.clone()),
        Expr::Identifier(v) => Ok(v.clone()),
        Expr::Number(v) => Ok(v.to_string()),
        Expr::Boolean(v) => Ok(v.to_string()),
        _ => Err("نوع القيمة غير مدعوم في هذا الslice من runtime".to_string()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiNode {
    pub kind: String,
    pub id: Option<String>,
    pub text: Option<String>,
    pub props: HashMap<String, String>,
    pub events: HashMap<String, String>,
    pub children: Vec<UiNode>,
}

impl UiNode {
    pub fn new(kind: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            id: None,
            text: None,
            props: HashMap::new(),
            events: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_child(mut self, child: UiNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_prop(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.props.insert(key.into(), value.into());
        self
    }

    pub fn with_event(mut self, name: impl Into<String>, handler: impl Into<String>) -> Self {
        self.events.insert(name.into(), handler.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Patch {
    ReplaceText {
        id: String,
        text: String,
    },
    ReplaceNode {
        id: String,
        kind: String,
    },
    SetProp {
        id: String,
        key: String,
        value: String,
    },
    RemoveProp {
        id: String,
        key: String,
    },
    SetEvent {
        id: String,
        event: String,
        handler: String,
    },
    RemoveEvent {
        id: String,
        event: String,
    },
    AppendChild {
        id: String,
        kind: String,
    },
    RemoveChild {
        id: String,
        index: usize,
    },
}

pub fn diff_trees(previous: &UiNode, current: &UiNode) -> Vec<Patch> {
    let mut patches = Vec::new();
    diff_node(previous, current, &mut patches);
    patches
}

fn diff_node(previous: &UiNode, current: &UiNode, patches: &mut Vec<Patch>) {
    let current_id = current.id.clone().unwrap_or_else(|| "__root__".to_string());
    if previous.kind != current.kind {
        patches.push(Patch::ReplaceNode {
            id: current_id.clone(),
            kind: current.kind.clone(),
        });
        return;
    }

    if previous.text != current.text {
        patches.push(Patch::ReplaceText {
            id: current_id.clone(),
            text: current.text.clone().unwrap_or_default(),
        });
    }

    for (key, value) in &current.props {
        if previous.props.get(key) != Some(value) {
            patches.push(Patch::SetProp {
                id: current_id.clone(),
                key: key.clone(),
                value: value.clone(),
            });
        }
    }

    for key in previous.props.keys() {
        if !current.props.contains_key(key) {
            patches.push(Patch::RemoveProp {
                id: current_id.clone(),
                key: key.clone(),
            });
        }
    }

    for (event, handler) in &current.events {
        if previous.events.get(event) != Some(handler) {
            patches.push(Patch::SetEvent {
                id: current_id.clone(),
                event: event.clone(),
                handler: handler.clone(),
            });
        }
    }

    for event in previous.events.keys() {
        if !current.events.contains_key(event) {
            patches.push(Patch::RemoveEvent {
                id: current_id.clone(),
                event: event.clone(),
            });
        }
    }

    let count = previous.children.len().min(current.children.len());
    for idx in 0..count {
        diff_node(&previous.children[idx], &current.children[idx], patches);
    }

    if current.children.len() > previous.children.len() {
        for child in &current.children[previous.children.len()..] {
            patches.push(Patch::AppendChild {
                id: current_id.clone(),
                kind: child.kind.clone(),
            });
        }
    }

    if previous.children.len() > current.children.len() {
        for idx in (current.children.len()..previous.children.len()).rev() {
            patches.push(Patch::RemoveChild {
                id: current_id.clone(),
                index: idx,
            });
        }
    }
}

#[derive(Debug, Default)]
pub struct ReactiveStore {
    values: HashMap<String, String>,
}

impl ReactiveStore {
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Operator,
    Engineer,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEntry {
    pub action: String,
    pub role: Role,
    pub target: String,
    pub confirmed: bool,
    pub allowed: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Default)]
pub struct SecurityGate {
    audit_log: Vec<AuditEntry>,
}

impl SecurityGate {
    pub fn validate_modbus_write(
        &mut self,
        role: Role,
        target: impl Into<String>,
        confirmed: bool,
    ) -> Result<(), String> {
        let target = target.into();
        let mut denied = |reason: &str| {
            self.audit_log.push(AuditEntry {
                action: "modbus.write".to_string(),
                role,
                target: target.clone(),
                confirmed,
                allowed: false,
                reason: Some(reason.to_string()),
            });
            Err(reason.to_string())
        };

        if !matches!(role, Role::Engineer | Role::Admin) {
            return denied("منع modbus.write: الصلاحية الحالية لا تسمح بأوامر كتابة PLC");
        }

        if !confirmed {
            return denied("الأمر الخطر يحتاج تأكيداً صريحاً قبل التنفيذ");
        }

        self.audit_log.push(AuditEntry {
            action: "modbus.write".to_string(),
            role,
            target,
            confirmed,
            allowed: true,
            reason: None,
        });
        Ok(())
    }

    pub fn audit_log(&self) -> &[AuditEntry] {
        &self.audit_log
    }
}

pub fn render_html(root: &UiNode, config: &UiRuntimeConfig) -> String {
    let direction = match config.direction {
        TextDirection::Ltr => "ltr",
        TextDirection::Rtl => "rtl",
    };

    format!(
        "<main dir=\"{direction}\" lang=\"ar\">{}</main>",
        render_node(root)
    )
}

fn render_node(node: &UiNode) -> String {
    let text = node.text.clone().unwrap_or_default();
    let children = node.children.iter().map(render_node).collect::<String>();
    let attributes = node
        .props
        .iter()
        .map(|(k, v)| format!(" {k}=\"{v}\""))
        .collect::<String>();
    format!(
        "<{tag}{attributes}>{text}{children}</{tag}>",
        tag = node.kind
    )
}

pub fn measure_poll_to_paint<F>(render: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    render();
    start.elapsed()
}

pub fn benchmark_frame_update(widget_count: usize) -> Duration {
    let previous = UiNode::new("section").with_id("dashboard");
    let mut current = UiNode::new("section").with_id("dashboard");

    for idx in 0..widget_count {
        current = current.with_child(
            UiNode::new("span")
                .with_id(format!("w-{idx}"))
                .with_text(format!("{idx}"))
                .with_prop("data-widget", "kpi")
                .with_event("onClick", "open_details"),
        );
    }

    measure_poll_to_paint(|| {
        let _ = diff_trees(&previous, &current);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn runtime_renders_rtl_by_default() {
        let root = UiNode::new("section").with_text("لوحة تشغيل");
        let output = render_html(&root, &UiRuntimeConfig::default());
        assert!(output.contains("dir=\"rtl\""));
    }

    #[test]
    fn diff_engine_updates_changed_text_only() {
        let previous = UiNode::new("section")
            .with_id("root")
            .with_child(UiNode::new("span").with_id("speed").with_text("120"));
        let current = UiNode::new("section")
            .with_id("root")
            .with_child(UiNode::new("span").with_id("speed").with_text("125"));

        let patches = diff_trees(&previous, &current);
        assert_eq!(patches.len(), 1);
        assert_eq!(
            patches[0],
            Patch::ReplaceText {
                id: "speed".to_string(),
                text: "125".to_string()
            }
        );
    }

    #[test]
    fn security_gate_blocks_unauthorized_writes_and_logs_authorized() {
        let mut gate = SecurityGate::default();
        let denied = gate.validate_modbus_write(Role::Operator, "holding:40110", true);
        assert!(denied.is_err());

        let allowed = gate.validate_modbus_write(Role::Engineer, "holding:40110", true);
        assert!(allowed.is_ok());
        assert_eq!(gate.audit_log().len(), 2);
        assert!(!gate.audit_log()[0].allowed);
        assert!(gate.audit_log()[1].allowed);
    }

    #[test]
    fn poll_to_paint_budget_under_200ms() {
        let elapsed = measure_poll_to_paint(|| {
            let _ = render_html(
                &UiNode::new("section").with_text("ok"),
                &UiRuntimeConfig::default(),
            );
        });

        assert!(elapsed < Duration::from_millis(200));
    }

    #[test]
    fn frame_update_budget_under_16ms_for_50_widgets() {
        let elapsed = benchmark_frame_update(50);
        assert!(elapsed < Duration::from_millis(16));
    }

    #[test]
    fn diff_engine_captures_props_events_and_children_changes() {
        let previous = UiNode::new("button")
            .with_id("reset")
            .with_prop("variant", "secondary")
            .with_event("onClick", "old_handler")
            .with_child(UiNode::new("span").with_text("قديم"));
        let current = UiNode::new("button")
            .with_id("reset")
            .with_prop("variant", "danger")
            .with_event("onClick", "confirm_reset")
            .with_child(UiNode::new("span").with_text("جديد"))
            .with_child(UiNode::new("icon"));

        let patches = diff_trees(&previous, &current);
        assert!(patches.iter().any(|p| {
            matches!(
                p,
                Patch::SetProp { id, key, value }
                if id == "reset" && key == "variant" && value == "danger"
            )
        }));
        assert!(patches.iter().any(|p| {
            matches!(
                p,
                Patch::SetEvent { id, event, handler }
                if id == "reset" && event == "onClick" && handler == "confirm_reset"
            )
        }));
        assert!(patches.iter().any(|p| {
            matches!(
                p,
                Patch::AppendChild { id, kind }
                if id == "reset" && kind == "icon"
            )
        }));
    }

    #[test]
    fn runtime_program_builds_from_ui_ast_and_renders_route() {
        let source = r#"
            واجهة صفحة_الرئيسية() {
                اطبع("جاهز")؛
            }

            حالة المتجر = {"عداد": 1}؛
            ثيم المصنع = {"اتجاه": "RTL"}؛
            موجه المسارات = {"/": "صفحة_الرئيسية"}؛
        "#;

        let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
        let runtime_program = UiRuntimeProgram::from_ast(&program).expect("يجب بناء runtime");

        assert_eq!(runtime_program.theme.direction, TextDirection::Rtl);
        assert_eq!(runtime_program.state["المتجر"]["عداد"], "1");

        let html = runtime_program
            .render_route_html("/")
            .expect("يجب أن يتم render للمسار");
        assert!(html.contains("dir=\"rtl\""));
        assert!(html.contains("واجهة صفحة_الرئيسية"));
    }

    #[test]
    fn runtime_program_rejects_invalid_theme_direction() {
        let source = r#"
            ثيم المصنع = {"اتجاه": "TOP"}؛
        "#;

        let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
        let error = UiRuntimeProgram::from_ast(&program).expect_err("يجب رفض الاتجاه غير المدعوم");
        assert!(error.contains("غير مدعومة"));
    }

    #[test]
    fn runtime_program_validates_that_routes_reference_existing_components() {
        let source = r#"
            واجهة صفحة_الرئيسية() {
                اطبع("ok")؛
            }

            موجه المسارات = {
                "/": "صفحة_غير_موجودة"
            }؛
        "#;

        let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
        let error = UiRuntimeProgram::from_ast(&program).expect_err("يجب رفض route غير صالح");
        assert!(error.contains("مكون غير معرف"));
    }

    #[test]
    fn runtime_program_renders_initial_route_from_default_key() {
        let source = r#"
            واجهة شاشة_الإنذارات() {
                اطبع("alarms")؛
            }

            موجه المسارات = {
                "افتراضي": "/alarms",
                "/alarms": "شاشة_الإنذارات"
            }؛
        "#;

        let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
        let runtime_program = UiRuntimeProgram::from_ast(&program).expect("يجب بناء runtime");

        let html = runtime_program
            .render_initial_route_html()
            .expect("يجب render المسار الافتراضي");
        assert!(html.contains("واجهة شاشة_الإنذارات"));
        assert!(html.contains("dir=\"rtl\""));
    }

    #[test]
    fn runtime_program_falls_back_to_first_declared_route_when_root_and_default_missing() {
        let source = r#"
            واجهة شاشة_تشغيل() {
                اطبع("run")؛
            }

            موجه المسارات = {
                "/run": "شاشة_تشغيل"
            }؛
        "#;

        let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
        let runtime_program = UiRuntimeProgram::from_ast(&program).expect("يجب بناء runtime");

        let html = runtime_program
            .render_initial_route_html()
            .expect("يجب render أول route");
        assert!(html.contains("واجهة شاشة_تشغيل"));
    }

    #[test]
    fn runtime_program_triggers_component_bound_event() {
        let source = r#"
            واجهة صفحة_الرئيسية() {
                زر({"onClick": "كتابة_حرجة"})؛
            }

            حدث كتابة_حرجة() {
                modbus.write("holding:40110", 42)؛
            }
        "#;

        let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
        let runtime_program = UiRuntimeProgram::from_ast(&program).expect("يجب بناء runtime");

        let mut gate = SecurityGate::default();
        let denied = runtime_program.trigger_component_event(
            "صفحة_الرئيسية",
            "onClick",
            Role::Operator,
            true,
            &mut gate,
        );
        assert!(denied.is_err());

        let allowed = runtime_program.trigger_component_event(
            "صفحة_الرئيسية",
            "onClick",
            Role::Engineer,
            true,
            &mut gate,
        );
        assert!(allowed.is_ok());
        assert_eq!(gate.audit_log().len(), 2);
    }

    #[test]
    fn runtime_program_supports_onchange_alias_in_arabic_and_english() {
        let source = r#"
            واجهة نموذج_تشغيل() {
                حقل({"onChange": "تحديث_آمن"})؛
            }

            حدث تحديث_آمن() {
                modbus.write("holding:40112", 7)؛
            }
        "#;

        let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
        let runtime_program = UiRuntimeProgram::from_ast(&program).expect("يجب بناء runtime");

        let mut gate = SecurityGate::default();
        assert!(runtime_program
            .trigger_component_event("نموذج_تشغيل", "onChange", Role::Engineer, true, &mut gate)
            .is_ok());

        assert!(runtime_program
            .trigger_component_event(
                "نموذج_تشغيل",
                "عند_التغيير",
                Role::Engineer,
                true,
                &mut gate
            )
            .is_ok());
        assert_eq!(gate.audit_log().len(), 2);
    }

    #[test]
    fn runtime_program_supports_onsubmit_alias_in_arabic_and_english() {
        let source = r#"
            واجهة نموذج_أوامر() {
                نموذج({"عند_الإرسال": "إرسال_أمر"})؛
            }

            حدث إرسال_أمر() {
                modbus.write("holding:40114", 11)؛
            }
        "#;

        let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
        let runtime_program = UiRuntimeProgram::from_ast(&program).expect("يجب بناء runtime");

        let mut gate = SecurityGate::default();
        assert!(runtime_program
            .trigger_component_event("نموذج_أوامر", "onSubmit", Role::Engineer, true, &mut gate)
            .is_ok());

        assert!(runtime_program
            .trigger_component_event("نموذج_أوامر", "عند_الإرسال", Role::Engineer, true, &mut gate)
            .is_ok());
        assert_eq!(gate.audit_log().len(), 2);
    }

    #[test]
    fn runtime_program_collects_event_handlers_and_enforces_security_gate() {
        let source = r#"
            حدث كتابة_حرجة() {
                modbus.write("holding:40110", 42)؛
            }
        "#;

        let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
        let runtime_program = UiRuntimeProgram::from_ast(&program).expect("يجب بناء runtime");
        assert!(runtime_program.event_handlers.contains_key("كتابة_حرجة"));

        let mut gate = SecurityGate::default();
        let denied = runtime_program.execute_event("كتابة_حرجة", Role::Operator, true, &mut gate);
        assert!(denied.is_err());

        let denied_by_confirmation =
            runtime_program.execute_event("كتابة_حرجة", Role::Engineer, false, &mut gate);
        assert!(denied_by_confirmation.is_err());

        let allowed = runtime_program.execute_event("كتابة_حرجة", Role::Engineer, true, &mut gate);
        assert!(allowed.is_ok());
        assert_eq!(gate.audit_log().len(), 3);
    }

    #[test]
    fn runtime_program_lists_sensitive_component_bindings_with_policy() {
        let source = r#"
            واجهة لوحة_تشغيل() {
                زر({"onClick": "كتابة_حرجة"})؛
                نص({"onChange": "قراءة_آمنة"})؛
            }

            حدث قراءة_آمنة() {
                اطبع("noop")؛
            }

            حدث كتابة_حرجة() {
                modbus.write("holding:40115", 1)؛
            }
        "#;

        let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
        let runtime_program = UiRuntimeProgram::from_ast(&program).expect("يجب بناء runtime");
        let bindings = runtime_program.list_security_sensitive_bindings();

        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].component, "لوحة_تشغيل");
        assert_eq!(bindings[0].event, "onClick");
        assert_eq!(bindings[0].handler, "كتابة_حرجة");
        assert!(bindings[0].requires_confirmation);
        assert_eq!(bindings[0].allowed_roles, vec![Role::Engineer, Role::Admin]);
        assert_eq!(bindings[0].dangerous_targets, vec!["holding:40115"]);
    }

    #[test]
    fn runtime_program_returns_event_policy_for_sensitive_component_event() {
        let source = r#"
            واجهة نموذج_أوامر() {
                نموذج({"onSubmit": "إرسال_أمر"})؛
            }

            حدث إرسال_أمر() {
                modbus.write("holding:40116", 2)؛
            }
        "#;

        let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
        let runtime_program = UiRuntimeProgram::from_ast(&program).expect("يجب بناء runtime");

        let policy = runtime_program
            .event_policy_for_component("نموذج_أوامر", "عند_الإرسال")
            .expect("يجب أن تعاد سياسة أمنية");
        assert_eq!(policy.handler, "إرسال_أمر");
        assert_eq!(policy.event, "onSubmit");
        assert_eq!(policy.dangerous_targets, vec!["holding:40116"]);
    }
}
