use std::time::Duration;

use almarjaa::parser::Parser;
use almarjaa::runtime::{
    benchmark_frame_update, diff_trees, measure_poll_to_paint, render_html, Role, SecurityGate,
    TextDirection, UiNode, UiRuntimeConfig, UiRuntimeProgram,
};

#[test]
fn ui_html_supports_rtl_explicitly() {
    let root = UiNode::new("section").with_text("لوحة");
    let config = UiRuntimeConfig {
        direction: TextDirection::Rtl,
    };

    let html = render_html(&root, &config);
    assert!(html.contains("dir=\"rtl\""));
    assert!(html.contains("lang=\"ar\""));
}

#[test]
fn security_gate_requires_confirmation_and_audits_denied_attempt() {
    let mut gate = SecurityGate::default();

    let denied = gate.validate_modbus_write(Role::Engineer, "holding:40110", false);
    assert!(denied.is_err());

    assert_eq!(gate.audit_log().len(), 1);
    assert!(!gate.audit_log()[0].allowed);
}

#[test]
fn poll_to_paint_and_frame_update_stay_within_budget() {
    let poll_to_paint = measure_poll_to_paint(|| {
        let _ = render_html(
            &UiNode::new("section").with_text("ok"),
            &UiRuntimeConfig::default(),
        );
    });

    let frame_update = benchmark_frame_update(50);

    assert!(poll_to_paint < Duration::from_millis(200));
    assert!(frame_update < Duration::from_millis(16));
}

#[test]
fn diff_detects_new_widget_append() {
    let previous = UiNode::new("section").with_id("root");
    let current = UiNode::new("section")
        .with_id("root")
        .with_child(UiNode::new("card").with_id("kpi"));

    let patches = diff_trees(&previous, &current);
    assert!(patches
        .iter()
        .any(|patch| format!("{patch:?}").contains("AppendChild")));
}

#[test]
fn runtime_event_execution_requires_role_and_confirmation() {
    let source = r#"
        حدث كتابة_حرجة() {
            modbus.write("holding:40110", 42)؛
        }
    "#;

    let program = almarjaa::parser::Parser::parse(source).expect("يجب أن ينجح التحليل");
    let runtime = UiRuntimeProgram::from_ast(&program).expect("يجب أن ينجح بناء runtime");

    let mut gate = SecurityGate::default();
    assert!(runtime
        .execute_event("كتابة_حرجة", Role::Operator, true, &mut gate)
        .is_err());
    assert!(runtime
        .execute_event("كتابة_حرجة", Role::Engineer, false, &mut gate)
        .is_err());
    assert!(runtime
        .execute_event("كتابة_حرجة", Role::Engineer, true, &mut gate)
        .is_ok());
    assert_eq!(gate.audit_log().len(), 3);
}

#[test]
fn runtime_component_event_binding_executes_secured_handler() {
    let source = r#"
        واجهة صفحة_الرئيسية() {
            زر({"عند_النقر": "كتابة_حرجة"})؛
        }

        حدث كتابة_حرجة() {
            modbus.write("holding:40111", 1)؛
        }
    "#;

    let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
    let runtime = UiRuntimeProgram::from_ast(&program).expect("يجب أن ينجح بناء runtime");

    let mut gate = SecurityGate::default();
    assert!(runtime
        .trigger_component_event("صفحة_الرئيسية", "onClick", Role::Operator, true, &mut gate)
        .is_err());
    assert!(runtime
        .trigger_component_event("صفحة_الرئيسية", "onClick", Role::Engineer, true, &mut gate)
        .is_ok());
    assert_eq!(gate.audit_log().len(), 2);
}

#[test]
fn runtime_supports_onchange_event_aliases() {
    let source = r#"
        واجهة نموذج_تشغيل() {
            حقل({"عند_التغيير": "تحديث_آمن"})؛
        }

        حدث تحديث_آمن() {
            modbus.write("holding:40112", 7)؛
        }
    "#;

    let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
    let runtime = UiRuntimeProgram::from_ast(&program).expect("يجب أن ينجح بناء runtime");

    let mut gate = SecurityGate::default();
    assert!(runtime
        .trigger_component_event("نموذج_تشغيل", "onChange", Role::Engineer, true, &mut gate)
        .is_ok());
    assert!(runtime
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
fn runtime_supports_onsubmit_event_aliases() {
    let source = r#"
        واجهة نموذج_صيانة() {
            نموذج({"عند_الإرسال": "إرسال_أمر_صيانة"})؛
        }

        حدث إرسال_أمر_صيانة() {
            modbus.write("holding:40120", 5)؛
        }
    "#;

    let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
    let runtime = UiRuntimeProgram::from_ast(&program).expect("يجب أن ينجح بناء runtime");

    let mut gate = SecurityGate::default();
    assert!(runtime
        .trigger_component_event("نموذج_صيانة", "onSubmit", Role::Engineer, true, &mut gate)
        .is_ok());
    assert!(runtime
        .trigger_component_event("نموذج_صيانة", "عند_الإرسال", Role::Engineer, true, &mut gate)
        .is_ok());
    assert_eq!(gate.audit_log().len(), 2);
}

#[test]
fn runtime_initial_route_uses_default_alias_key() {
    let source = r#"
        واجهة شاشة_الرئيسية() {
            اطبع("ready")؛
        }

        موجه المسارات = {
            "default": "/home",
            "/home": "شاشة_الرئيسية"
        }؛
    "#;

    let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
    let runtime = UiRuntimeProgram::from_ast(&program).expect("يجب أن ينجح بناء runtime");

    let html = runtime
        .render_initial_route_html()
        .expect("يجب render المسار الافتراضي");
    assert!(html.contains("واجهة شاشة_الرئيسية"));
}

#[test]
fn runtime_rejects_route_to_unknown_component() {
    let source = r#"
        واجهة شاشة_الرئيسية() {
            اطبع("ready")؛
        }

        موجه المسارات = {
            "/home": "شاشة_مفقودة"
        }؛
    "#;

    let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
    let error = UiRuntimeProgram::from_ast(&program).expect_err("يجب رفض route غير صالح");
    assert!(error.contains("مكون غير معرف"));
}

#[test]
fn runtime_rejects_component_event_bound_to_missing_handler() {
    let source = r#"
        واجهة شاشة_الرئيسية() {
            زر({"عند_النقر": "معالج_غير_موجود"})؛
        }
    "#;

    let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
    let error = UiRuntimeProgram::from_ast(&program).expect_err("يجب رفض الربط غير الصالح");
    assert!(error.contains("معالج غير معرف"));
}

#[test]
fn runtime_lists_security_sensitive_events_for_plc_write() {
    let source = r#"
        حدث قراءة_آمنة() {
            اطبع("noop")؛
        }

        حدث كتابة_حرجة() {
            modbus.write("holding:40113", 9)؛
        }
    "#;

    let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
    let runtime = UiRuntimeProgram::from_ast(&program).expect("يجب أن ينجح بناء runtime");

    let mut sensitive_events = runtime.list_security_sensitive_events();
    sensitive_events.sort_unstable();
    assert_eq!(sensitive_events, vec!["كتابة_حرجة"]);
}

#[test]
fn runtime_lists_sensitive_bindings_with_security_policy() {
    let source = r#"
        واجهة لوحة_عمليات() {
            زر({"onClick": "كتابة_حرجة"})؛
            حقل({"onChange": "قراءة_آمنة"})؛
        }

        حدث قراءة_آمنة() {
            اطبع("noop")؛
        }

        حدث كتابة_حرجة() {
            modbus.write("holding:40121", 9)؛
        }
    "#;

    let program = Parser::parse(source).expect("يجب أن ينجح التحليل");
    let runtime = UiRuntimeProgram::from_ast(&program).expect("يجب أن ينجح بناء runtime");

    let policies = runtime.list_security_sensitive_bindings();
    assert_eq!(policies.len(), 1);
    assert_eq!(policies[0].component, "لوحة_عمليات");
    assert_eq!(policies[0].event, "onClick");
    assert_eq!(policies[0].handler, "كتابة_حرجة");
    assert!(policies[0].requires_confirmation);
    assert_eq!(policies[0].dangerous_targets, vec!["holding:40121"]);
}
