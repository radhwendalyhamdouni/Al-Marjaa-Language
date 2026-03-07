use super::value::{Environment, SharedValue, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write as IoWrite};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::rc::Rc;

pub fn define_io(env: &mut Environment) {
    env.define(
        "اطبع",
        Value::NativeFunction {
            name: "اطبع".to_string(),
            func: |args| {
                let parts: Vec<String> = args.iter().map(|a| a.borrow().to_string()).collect();
                println!("{}", parts.join(" "));
                Ok(Rc::new(RefCell::new(Value::Null)))
            },
        },
        false,
    );

    env.define(
        "طبع",
        Value::NativeFunction {
            name: "طبع".to_string(),
            func: |args| {
                let parts: Vec<String> = args.iter().map(|a| a.borrow().to_string()).collect();
                print!("{}", parts.join(" "));
                io::stdout().flush().ok();
                Ok(Rc::new(RefCell::new(Value::Null)))
            },
        },
        false,
    );

    env.define(
        "اطبع_خطأ",
        Value::NativeFunction {
            name: "اطبع_خطأ".to_string(),
            func: |args| {
                let parts: Vec<String> = args.iter().map(|a| a.borrow().to_string()).collect();
                eprintln!("{}", parts.join(" "));
                Ok(Rc::new(RefCell::new(Value::Null)))
            },
        },
        false,
    );

    env.define(
        "ادخل",
        Value::NativeFunction {
            name: "ادخل".to_string(),
            func: |args| {
                if let Some(arg) = args.first() {
                    print!("{}", arg.borrow());
                    io::stdout().flush().ok();
                }
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap_or(0);
                Ok(Rc::new(RefCell::new(Value::String(
                    input
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string(),
                ))))
            },
        },
        false,
    );

    env.define(
        "ادخل_رقم",
        Value::NativeFunction {
            name: "ادخل_رقم".to_string(),
            func: |args| {
                if let Some(arg) = args.first() {
                    print!("{}", arg.borrow());
                    io::stdout().flush().ok();
                }
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap_or(0);
                let trimmed = input.trim();
                match trimmed.parse::<f64>() {
                    Ok(n) => Ok(Rc::new(RefCell::new(Value::Number(n)))),
                    Err(_) => Err(format!("'{}' ليس رقماً", trimmed)),
                }
            },
        },
        false,
    );

    env.define(
        "مسح_الشاشة",
        Value::NativeFunction {
            name: "مسح_الشاشة".to_string(),
            func: |_| {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().ok();
                Ok(Rc::new(RefCell::new(Value::Null)))
            },
        },
        false,
    );

    env.define(
        "سطر",
        Value::NativeFunction {
            name: "سطر".to_string(),
            func: |_| {
                println!();
                Ok(Rc::new(RefCell::new(Value::Null)))
            },
        },
        false,
    );
}

pub fn define_file_funcs(env: &mut Environment) {
    env.define(
        "اقرأ_ملف",
        Value::NativeFunction {
            name: "اقرأ_ملف".to_string(),
            func: |a| {
                let path = a[0].borrow().to_string_value();
                match std::fs::read_to_string(&path) {
                    Ok(content) => Ok(Rc::new(RefCell::new(Value::String(content)))),
                    Err(e) => Err(format!("خطأ في قراءة الملف '{}': {}", path, e)),
                }
            },
        },
        false,
    );

    env.define(
        "اكتب_ملف",
        Value::NativeFunction {
            name: "اكتب_ملف".to_string(),
            func: |a| {
                let path = a[0].borrow().to_string_value();
                let content = a[1].borrow().to_string_value();
                match std::fs::write(&path, content) {
                    Ok(_) => Ok(Rc::new(RefCell::new(Value::Boolean(true)))),
                    Err(e) => Err(format!("خطأ في كتابة الملف '{}': {}", path, e)),
                }
            },
        },
        false,
    );

    env.define(
        "ألحق_ملف",
        Value::NativeFunction {
            name: "ألحق_ملف".to_string(),
            func: |a| {
                let path = a[0].borrow().to_string_value();
                let content = a[1].borrow().to_string_value();
                match std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&path)
                {
                    Ok(mut f) => {
                        let _ = f.write_all(content.as_bytes());
                        Ok(Rc::new(RefCell::new(Value::Boolean(true))))
                    }
                    Err(e) => Err(format!("خطأ في الإلحاق بالملف '{}': {}", path, e)),
                }
            },
        },
        false,
    );

    env.define(
        "يوجد_ملف",
        Value::NativeFunction {
            name: "يوجد_ملف".to_string(),
            func: |a| {
                let path = a[0].borrow().to_string_value();
                Ok(Rc::new(RefCell::new(Value::Boolean(
                    std::path::Path::new(&path).exists(),
                ))))
            },
        },
        false,
    );

    env.define(
        "احذف_ملف",
        Value::NativeFunction {
            name: "احذف_ملف".to_string(),
            func: |a| {
                let path = a[0].borrow().to_string_value();
                match std::fs::remove_file(&path) {
                    Ok(_) => Ok(Rc::new(RefCell::new(Value::Boolean(true)))),
                    Err(e) => Err(format!("خطأ في حذف الملف '{}': {}", path, e)),
                }
            },
        },
        false,
    );

    env.define(
        "أنشئ_مجلد",
        Value::NativeFunction {
            name: "أنشئ_مجلد".to_string(),
            func: |a| {
                let path = a[0].borrow().to_string_value();
                match std::fs::create_dir_all(&path) {
                    Ok(_) => Ok(Rc::new(RefCell::new(Value::Boolean(true)))),
                    Err(e) => Err(format!("خطأ في إنشاء المجلد '{}': {}", path, e)),
                }
            },
        },
        false,
    );

    env.define(
        "محتويات_مجلد",
        Value::NativeFunction {
            name: "محتويات_مجلد".to_string(),
            func: |a| {
                let path = a[0].borrow().to_string_value();
                match std::fs::read_dir(&path) {
                    Ok(entries) => {
                        let items: Vec<SharedValue> = entries
                            .filter_map(|e| e.ok())
                            .map(|e| {
                                Rc::new(RefCell::new(Value::String(
                                    e.file_name().to_string_lossy().to_string(),
                                )))
                            })
                            .collect();
                        Ok(Rc::new(RefCell::new(Value::List(items))))
                    }
                    Err(e) => Err(format!("خطأ في قراءة المجلد '{}': {}", path, e)),
                }
            },
        },
        false,
    );

    env.define(
        "اقرأ_أسطر",
        Value::NativeFunction {
            name: "اقرأ_أسطر".to_string(),
            func: |a| {
                let path = a[0].borrow().to_string_value();
                match std::fs::read_to_string(&path) {
                    Ok(content) => {
                        let lines: Vec<SharedValue> = content
                            .lines()
                            .map(|l| Rc::new(RefCell::new(Value::String(l.to_string()))))
                            .collect();
                        Ok(Rc::new(RefCell::new(Value::List(lines))))
                    }
                    Err(e) => Err(format!("خطأ في قراءة الملف '{}': {}", path, e)),
                }
            },
        },
        false,
    );
}

pub fn define_system_funcs(env: &mut Environment) {
    env.define(
        "أوقف_البرنامج",
        Value::NativeFunction {
            name: "أوقف_البرنامج".to_string(),
            func: |a| {
                let code = if a.is_empty() {
                    0
                } else {
                    a[0].borrow().to_number().unwrap_or(0.0) as i32
                };
                std::process::exit(code);
            },
        },
        false,
    );

    env.define(
        "متغير_البيئة",
        Value::NativeFunction {
            name: "متغير_البيئة".to_string(),
            func: |a| {
                let key = a[0].borrow().to_string_value();
                match std::env::var(&key) {
                    Ok(val) => Ok(Rc::new(RefCell::new(Value::String(val)))),
                    Err(_) => Ok(Rc::new(RefCell::new(Value::Null))),
                }
            },
        },
        false,
    );

    env.define(
        "وسائط_البرنامج",
        Value::NativeFunction {
            name: "وسائط_البرنامج".to_string(),
            func: |_| {
                let args: Vec<SharedValue> = std::env::args()
                    .map(|a| Rc::new(RefCell::new(Value::String(a))))
                    .collect();
                Ok(Rc::new(RefCell::new(Value::List(args))))
            },
        },
        false,
    );

    env.define(
        "نظام",
        Value::NativeFunction {
            name: "نظام".to_string(),
            func: |a| {
                let cmd = a[0].borrow().to_string_value();
                #[cfg(target_os = "windows")]
                let output = std::process::Command::new("cmd")
                    .args(["/C", &cmd])
                    .output();
                #[cfg(not(target_os = "windows"))]
                let output = std::process::Command::new("sh").args(["-c", &cmd]).output();
                match output {
                    Ok(out) => {
                        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                        Ok(Rc::new(RefCell::new(Value::String(stdout))))
                    }
                    Err(e) => Err(format!("خطأ في تنفيذ الأمر: {}", e)),
                }
            },
        },
        false,
    );

    env.define(
        "مسار_حالي",
        Value::NativeFunction {
            name: "مسار_حالي".to_string(),
            func: |_| match std::env::current_dir() {
                Ok(p) => Ok(Rc::new(RefCell::new(Value::String(
                    p.to_string_lossy().to_string(),
                )))),
                Err(e) => Err(format!("خطأ: {}", e)),
            },
        },
        false,
    );
}

pub fn define_time_funcs(env: &mut Environment) {
    env.define(
        "الآن",
        Value::NativeFunction {
            name: "الآن".to_string(),
            func: |_| {
                use std::time::{SystemTime, UNIX_EPOCH};
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as f64;
                Ok(Rc::new(RefCell::new(Value::Number(now))))
            },
        },
        false,
    );

    env.define(
        "الآن_ثواني",
        Value::NativeFunction {
            name: "الآن_ثواني".to_string(),
            func: |_| {
                use std::time::{SystemTime, UNIX_EPOCH};
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64();
                Ok(Rc::new(RefCell::new(Value::Number(now))))
            },
        },
        false,
    );

    env.define(
        "انتظر_ثوان",
        Value::NativeFunction {
            name: "انتظر_ثوان".to_string(),
            func: |a| {
                let secs = a[0].borrow().to_number()?;
                std::thread::sleep(std::time::Duration::from_secs_f64(secs));
                Ok(Rc::new(RefCell::new(Value::Null)))
            },
        },
        false,
    );

    env.define(
        "انتظر_مللي",
        Value::NativeFunction {
            name: "انتظر_مللي".to_string(),
            func: |a| {
                let ms = a[0].borrow().to_number()? as u64;
                std::thread::sleep(std::time::Duration::from_millis(ms));
                Ok(Rc::new(RefCell::new(Value::Null)))
            },
        },
        false,
    );
}

pub fn define_network_funcs(env: &mut Environment) {
    #[derive(Debug, Clone)]
    struct HttpOptions {
        timeout_seconds: u64,
        headers: Vec<(String, String)>,
        proxy: Option<String>,
        stealth: bool,
        user_agent: Option<String>,
        allow_private_networks: bool,
    }

    impl Default for HttpOptions {
        fn default() -> Self {
            Self {
                timeout_seconds: 30,
                headers: vec![],
                proxy: None,
                stealth: false,
                user_agent: None,
                allow_private_networks: false,
            }
        }
    }

    fn parse_http_options(arg: Option<&SharedValue>) -> Result<HttpOptions, String> {
        let mut options = HttpOptions::default();
        let Some(raw) = arg else {
            return Ok(options);
        };

        let borrowed = raw.borrow();
        let Value::Dictionary(dict) = &*borrowed else {
            return Err("خيارات الشبكة يجب أن تكون قاموساً".to_string());
        };

        if let Some(timeout_value) = dict.get("مهلة") {
            let timeout = timeout_value.borrow().to_number()?;
            if !(1.0..=300.0).contains(&timeout) {
                return Err("قيمة 'مهلة' يجب أن تكون بين 1 و300 ثانية".to_string());
            }
            options.timeout_seconds = timeout as u64;
        }

        if let Some(proxy_value) = dict.get("وكيل") {
            let proxy = proxy_value.borrow().to_string_value();
            if !proxy.is_empty() {
                options.proxy = Some(proxy);
            }
        }

        if let Some(stealth_value) = dict.get("تخفي") {
            options.stealth = stealth_value.borrow().is_truthy();
        }

        if let Some(user_agent_value) = dict.get("اسم_عميل") {
            let ua = user_agent_value.borrow().to_string_value();
            if !ua.is_empty() {
                options.user_agent = Some(ua);
            }
        }

        if let Some(allow_private) = dict.get("سماح_شبكة_داخلية") {
            options.allow_private_networks = allow_private.borrow().is_truthy();
        }

        if let Some(headers_value) = dict.get("رؤوس") {
            let headers_borrowed = headers_value.borrow();
            let Value::Dictionary(headers_dict) = &*headers_borrowed else {
                return Err("قيمة 'رؤوس' يجب أن تكون قاموساً".to_string());
            };

            for (k, v) in headers_dict {
                options
                    .headers
                    .push((k.clone(), v.borrow().to_string_value()));
            }
        }

        if options.stealth && options.user_agent.is_none() {
            options.user_agent = Some(
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) AlMarjaa/2.0 Safari/537.36".to_string(),
            );
        }

        Ok(options)
    }

    fn parse_url_parts(url: &str) -> Result<(&str, &str), String> {
        let trimmed = url.trim();
        if trimmed.is_empty() {
            return Err("عنوان URL فارغ".to_string());
        }

        let Some((scheme, rest)) = trimmed.split_once("://") else {
            return Err("عنوان URL غير صالح: يجب أن يبدأ بـ http:// أو https://".to_string());
        };

        if scheme != "http" && scheme != "https" {
            return Err("البروتوكول المسموح فقط هو http/https".to_string());
        }

        Ok((scheme, rest))
    }

    fn is_blocked_host(host: &str) -> bool {
        let lowered = host.to_ascii_lowercase();

        if lowered == "localhost" || lowered == "::1" {
            return true;
        }

        if let Ok(ip) = lowered.parse::<IpAddr>() {
            return match ip {
                IpAddr::V4(ipv4) => {
                    ipv4.is_loopback()
                        || ipv4.is_private()
                        || ipv4.is_link_local()
                        || ipv4.is_multicast()
                        || ipv4 == Ipv4Addr::new(0, 0, 0, 0)
                }
                IpAddr::V6(ipv6) => {
                    ipv6.is_loopback()
                        || ipv6.is_multicast()
                        || ipv6.is_unicast_link_local()
                        || ipv6.is_unspecified()
                        || ipv6.segments()[0] & 0xfe00 == 0xfc00
                        || ipv6 == Ipv6Addr::LOCALHOST
                }
            };
        }

        lowered.ends_with(".local")
            || lowered.ends_with(".internal")
            || lowered.ends_with(".localhost")
    }

    fn validate_network_target(url: &str, options: &HttpOptions) -> Result<(), String> {
        let (_, rest) = parse_url_parts(url)?;
        let host_port = rest.split('/').next().unwrap_or_default();
        let host = host_port
            .trim_start_matches('[')
            .split(']')
            .next()
            .unwrap_or_default()
            .split(':')
            .next()
            .unwrap_or_default();

        if host.is_empty() {
            return Err("عنوان URL غير صالح: المضيف مفقود".to_string());
        }

        if !options.allow_private_networks && is_blocked_host(host) {
            return Err(format!(
                "تم حظر الوصول إلى المضيف الداخلي '{}' افتراضياً. فعّل 'سماح_شبكة_داخلية' إذا كنت متأكداً.",
                host
            ));
        }

        Ok(())
    }

    /// تنفيذ طلب HTTP آمن باستخدام reqwest (بدون حقن الأوامر)
    #[cfg(feature = "network")]
    fn execute_http_request(
        url: &str,
        method: &str,
        body: Option<&str>,
        options: &HttpOptions,
    ) -> Result<(String, u16), String> {
        use std::time::Duration;

        let mut client_builder = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(options.timeout_seconds))
            .redirect(reqwest::redirect::Policy::limited(10));

        // إضافة الوكيل إذا تم تحديده
        if let Some(proxy_url) = &options.proxy {
            let proxy = reqwest::Proxy::all(proxy_url)
                .map_err(|e| format!("خطأ في إعداد الوكيل: {}", e))?;
            client_builder = client_builder.proxy(proxy);
        }

        // إضافة User-Agent
        if let Some(ua) = &options.user_agent {
            client_builder = client_builder.user_agent(ua);
        } else {
            client_builder = client_builder.user_agent("AlMarjaa/2.0");
        }

        let client = client_builder
            .build()
            .map_err(|e| format!("خطأ في إنشاء العميل: {}", e))?;

        // بناء الطلب
        let mut request = match method.to_uppercase().as_str() {
            "GET" => client.get(url),
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            "PATCH" => client.patch(url),
            "HEAD" => client.head(url),
            _ => return Err(format!("طريقة HTTP غير مدعومة: {}", method)),
        };

        // إضافة الرؤوس
        for (key, value) in &options.headers {
            request = request.header(key, value);
        }

        // إضافة الجسم
        if let Some(content) = body {
            request = request.body(content.to_string());
        }

        // تنفيذ الطلب
        let response = request
            .send()
            .map_err(|e| format!("خطأ في الطلب: {}", e))?;

        let status = response.status().as_u16();
        let text = response
            .text()
            .map_err(|e| format!("خطأ في قراءة الرد: {}", e))?;

        Ok((text, status))
    }

    /// تنفيذ طلب HTTP باستخدام curl (احتياطي عندما لا يتوفر network feature)
    #[cfg(not(feature = "network"))]
    fn execute_http_request_fallback(
        url: &str,
        method: &str,
        body: Option<&str>,
        options: &HttpOptions,
    ) -> Result<(String, u16), String> {
        use std::process::Command;

        let mut cmd = Command::new("curl");
        cmd.arg("-s")
            .arg("-L")
            .arg("--max-time")
            .arg(options.timeout_seconds.to_string())
            .arg("-X")
            .arg(method)
            .arg("-w")
            .arg("\n%{http_code}");

        if let Some(proxy) = &options.proxy {
            cmd.arg("--proxy").arg(proxy);
        }

        if let Some(ua) = &options.user_agent {
            cmd.arg("-A").arg(ua);
        }

        for (k, v) in &options.headers {
            cmd.arg("-H").arg(format!("{}: {}", k, v));
        }

        if let Some(content) = body {
            cmd.arg("-d").arg(content);
        }

        cmd.arg(url);

        let output = cmd
            .output()
            .map_err(|e| format!("خطأ في تنفيذ curl: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();

        if lines.len() >= 2 {
            let status: u16 = lines.last()
                .unwrap_or(&"0")
                .parse()
                .unwrap_or(0);
            let text = lines[..lines.len() - 1].join("\n");
            Ok((text, status))
        } else {
            Ok((stdout.to_string(), if output.status.success() { 200 } else { 500 }))
        }
    }

    fn http_response_to_value(text: String, status: u16) -> SharedValue {
        let mut dict = HashMap::new();
        dict.insert(
            "نص".to_string(),
            Rc::new(RefCell::new(Value::String(text))),
        );
        dict.insert(
            "رمز".to_string(),
            Rc::new(RefCell::new(Value::Number(status as f64))),
        );
        Rc::new(RefCell::new(Value::Dictionary(dict)))
    }

    env.define(
        "http_أحضر",
        Value::NativeFunction {
            name: "http_أحضر".to_string(),
            func: |a| {
                let url = a[0].borrow().to_string_value();
                let options = HttpOptions::default();

                #[cfg(feature = "network")]
                {
                    match execute_http_request(&url, "GET", None, &options) {
                        Ok((text, status)) => Ok(http_response_to_value(text, status)),
                        Err(e) => Err(e),
                    }
                }

                #[cfg(not(feature = "network"))]
                {
                    match execute_http_request_fallback(&url, "GET", None, &options) {
                        Ok((text, status)) => Ok(http_response_to_value(text, status)),
                        Err(e) => Err(e),
                    }
                }
            },
        },
        false,
    );

    env.define(
        "http_أرسل",
        Value::NativeFunction {
            name: "http_أرسل".to_string(),
            func: |a| {
                let url = a[0].borrow().to_string_value();
                let data = if a.len() > 1 {
                    a[1].borrow().to_string_value()
                } else {
                    String::new()
                };
                let method = if a.len() > 2 {
                    a[2].borrow().to_string_value()
                } else {
                    "POST".to_string()
                };
                let options = HttpOptions::default();

                #[cfg(feature = "network")]
                {
                    match execute_http_request(&url, &method, Some(&data), &options) {
                        Ok((text, status)) => Ok(http_response_to_value(text, status)),
                        Err(e) => Err(e),
                    }
                }

                #[cfg(not(feature = "network"))]
                {
                    match execute_http_request_fallback(&url, &method, Some(&data), &options) {
                        Ok((text, status)) => Ok(http_response_to_value(text, status)),
                        Err(e) => Err(e),
                    }
                }
            },
        },
        false,
    );

    env.define(
        "http_أحضر_آمن",
        Value::NativeFunction {
            name: "http_أحضر_آمن".to_string(),
            func: |a| {
                let url = a[0].borrow().to_string_value();
                let options = parse_http_options(a.get(1))?;
                validate_network_target(&url, &options)?;

                #[cfg(feature = "network")]
                {
                    match execute_http_request(&url, "GET", None, &options) {
                        Ok((text, status)) => Ok(http_response_to_value(text, status)),
                        Err(e) => Err(e),
                    }
                }

                #[cfg(not(feature = "network"))]
                {
                    match execute_http_request_fallback(&url, "GET", None, &options) {
                        Ok((text, status)) => Ok(http_response_to_value(text, status)),
                        Err(e) => Err(e),
                    }
                }
            },
        },
        false,
    );

    env.define(
        "http_أرسل_آمن",
        Value::NativeFunction {
            name: "http_أرسل_آمن".to_string(),
            func: |a| {
                let url = a[0].borrow().to_string_value();
                let data = if a.len() > 1 {
                    a[1].borrow().to_string_value()
                } else {
                    String::new()
                };
                let method = if a.len() > 2 {
                    a[2].borrow().to_string_value()
                } else {
                    "POST".to_string()
                };
                let options = parse_http_options(a.get(3))?;
                validate_network_target(&url, &options)?;

                #[cfg(feature = "network")]
                {
                    match execute_http_request(&url, &method, Some(&data), &options) {
                        Ok((text, status)) => Ok(http_response_to_value(text, status)),
                        Err(e) => Err(e),
                    }
                }

                #[cfg(not(feature = "network"))]
                {
                    match execute_http_request_fallback(&url, &method, Some(&data), &options) {
                        Ok((text, status)) => Ok(http_response_to_value(text, status)),
                        Err(e) => Err(e),
                    }
                }
            },
        },
        false,
    );
}

pub fn define_hardware_funcs(env: &mut Environment) {
    fn ensure_min_args(args: &[SharedValue], min: usize, name: &str) -> Result<(), String> {
        if args.len() < min {
            return Err(format!("{} يتطلب {} معاملات على الأقل", name, min));
        }
        Ok(())
    }

    env.define(
        "منفذ_تسلسلي_فتح",
        Value::NativeFunction {
            name: "منفذ_تسلسلي_فتح".to_string(),
            func: |a| {
                ensure_min_args(a, 1, "منفذ_تسلسلي_فتح")?;
                let port = a[0].borrow().to_string_value();
                let baud = if a.len() > 1 {
                    a[1].borrow().to_number().unwrap_or(9600.0) as u32
                } else {
                    9600
                };
                let mut dict = HashMap::new();
                dict.insert(
                    "منفذ".to_string(),
                    Rc::new(RefCell::new(Value::String(port))),
                );
                dict.insert(
                    "سرعة".to_string(),
                    Rc::new(RefCell::new(Value::Number(baud as f64))),
                );
                dict.insert(
                    "مفتوح".to_string(),
                    Rc::new(RefCell::new(Value::Boolean(true))),
                );
                Ok(Rc::new(RefCell::new(Value::Dictionary(dict))))
            },
        },
        false,
    );

    env.define(
        "منفذ_تسلسلي_أرسل",
        Value::NativeFunction {
            name: "منفذ_تسلسلي_أرسل".to_string(),
            func: |a| {
                ensure_min_args(a, 2, "منفذ_تسلسلي_أرسل")?;
                let data = a[1].borrow().to_string_value();
                println!("[تسلسلي] إرسال: {}", data);
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );

    env.define(
        "منفذ_تسلسلي_أقرأ",
        Value::NativeFunction {
            name: "منفذ_تسلسلي_أقرأ".to_string(),
            func: |_| Ok(Rc::new(RefCell::new(Value::String(String::new())))),
        },
        false,
    );

    env.define(
        "gpio_اكتب",
        Value::NativeFunction {
            name: "gpio_اكتب".to_string(),
            func: |a| {
                ensure_min_args(a, 2, "gpio_اكتب")?;
                let pin = a[0].borrow().to_number()? as i32;
                let val = a[1].borrow().is_truthy();
                println!(
                    "[GPIO] رقم {} = {}",
                    pin,
                    if val { "عالي" } else { "منخفض" }
                );
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );

    env.define(
        "gpio_اقرأ",
        Value::NativeFunction {
            name: "gpio_اقرأ".to_string(),
            func: |a| {
                ensure_min_args(a, 1, "gpio_اقرأ")?;
                let pin = a[0].borrow().to_number()? as i32;
                println!("[GPIO] قراءة رقم {}", pin);
                Ok(Rc::new(RefCell::new(Value::Number(0.0))))
            },
        },
        false,
    );

    env.define(
        "gpio_ضبط_اتجاه",
        Value::NativeFunction {
            name: "gpio_ضبط_اتجاه".to_string(),
            func: |a| {
                ensure_min_args(a, 2, "gpio_ضبط_اتجاه")?;
                let pin = a[0].borrow().to_number()? as i32;
                let dir = a[1].borrow().to_string_value();
                println!("[GPIO] اتجاه رقم {} = {}", pin, dir);
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );

    env.define(
        "i2c_أرسل",
        Value::NativeFunction {
            name: "i2c_أرسل".to_string(),
            func: |a| {
                ensure_min_args(a, 2, "i2c_أرسل")?;
                let addr = a[0].borrow().to_number()? as u8;
                let data = a[1].borrow().to_string_value();
                println!("[I2C] إرسال إلى 0x{:02X}: {}", addr, data);
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );

    env.define(
        "i2c_أقرأ",
        Value::NativeFunction {
            name: "i2c_أقرأ".to_string(),
            func: |a| {
                ensure_min_args(a, 1, "i2c_أقرأ")?;
                let addr = a[0].borrow().to_number()? as u8;
                println!("[I2C] قراءة من 0x{:02X}", addr);
                Ok(Rc::new(RefCell::new(Value::List(vec![]))))
            },
        },
        false,
    );

    env.define(
        "spi_نقل",
        Value::NativeFunction {
            name: "spi_نقل".to_string(),
            func: |a| {
                ensure_min_args(a, 1, "spi_نقل")?;
                let data = a[0].borrow().to_string_value();
                println!("[SPI] نقل: {}", data);
                Ok(Rc::new(RefCell::new(Value::String(data))))
            },
        },
        false,
    );

    env.define(
        "pwm_اكتب",
        Value::NativeFunction {
            name: "pwm_اكتب".to_string(),
            func: |a| {
                ensure_min_args(a, 2, "pwm_اكتب")?;
                let pin = a[0].borrow().to_number()? as i32;
                let duty = a[1].borrow().to_number()?;
                println!("[PWM] رقم {} = {:.1}%", pin, duty);
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );

    env.define(
        "حساس_درجة",
        Value::NativeFunction {
            name: "حساس_درجة".to_string(),
            func: |a| {
                ensure_min_args(a, 1, "حساس_درجة")?;
                let sensor = a[0].borrow().to_string_value();
                println!("[حساس] قراءة درجة الحرارة من {}", sensor);
                Ok(Rc::new(RefCell::new(Value::Number(25.0))))
            },
        },
        false,
    );

    env.define(
        "حساس_مسافة",
        Value::NativeFunction {
            name: "حساس_مسافة".to_string(),
            func: |a| {
                ensure_min_args(a, 2, "حساس_مسافة")?;
                let trigger = a[0].borrow().to_number()? as i32;
                let echo = a[1].borrow().to_number()? as i32;
                println!("[حساس] قراءة المسافة trigger={} echo={}", trigger, echo);
                Ok(Rc::new(RefCell::new(Value::Number(100.0))))
            },
        },
        false,
    );

    env.define(
        "محرك_خطوة",
        Value::NativeFunction {
            name: "محرك_خطوة".to_string(),
            func: |a| {
                ensure_min_args(a, 1, "محرك_خطوة")?;
                let steps = a[0].borrow().to_number()? as i32;
                let speed = if a.len() > 1 {
                    a[1].borrow().to_number().unwrap_or(100.0)
                } else {
                    100.0
                };
                println!("[محرك] {} خطوة بسرعة {}", steps, speed);
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );

    env.define(
        "شاشة_lcd_اطبع",
        Value::NativeFunction {
            name: "شاشة_lcd_اطبع".to_string(),
            func: |a| {
                ensure_min_args(a, 2, "شاشة_lcd_اطبع")?;
                let row = a[0].borrow().to_number()? as i32;
                let text = a[1].borrow().to_string_value();
                println!("[LCD] صف {}: {}", row, text);
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );

    env.define(
        "شاشة_lcd_امسح",
        Value::NativeFunction {
            name: "شاشة_lcd_امسح".to_string(),
            func: |_| {
                println!("[LCD] مسح الشاشة");
                Ok(Rc::new(RefCell::new(Value::Null)))
            },
        },
        false,
    );

    env.define(
        "plc_اقرأ_دخل",
        Value::NativeFunction {
            name: "plc_اقرأ_دخل".to_string(),
            func: |a| {
                ensure_min_args(a, 1, "plc_اقرأ_دخل")?;
                let addr = a[0].borrow().to_string_value();
                println!("[PLC] قراءة دخل {}", addr);
                Ok(Rc::new(RefCell::new(Value::Boolean(false))))
            },
        },
        false,
    );

    env.define(
        "plc_اكتب_خرج",
        Value::NativeFunction {
            name: "plc_اكتب_خرج".to_string(),
            func: |a| {
                ensure_min_args(a, 2, "plc_اكتب_خرج")?;
                let addr = a[0].borrow().to_string_value();
                let val = a[1].borrow().is_truthy();
                println!("[PLC] كتابة خرج {} = {}", addr, val);
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );

    env.define(
        "modbus_أقرأ",
        Value::NativeFunction {
            name: "modbus_أقرأ".to_string(),
            func: |a| {
                ensure_min_args(a, 2, "modbus_أقرأ")?;
                let slave = a[0].borrow().to_number()? as u8;
                let reg = a[1].borrow().to_number()? as u16;
                println!("[Modbus] قراءة من عنوان {} سجل {}", slave, reg);
                Ok(Rc::new(RefCell::new(Value::Number(0.0))))
            },
        },
        false,
    );

    env.define(
        "modbus_أكتب",
        Value::NativeFunction {
            name: "modbus_أكتب".to_string(),
            func: |a| {
                ensure_min_args(a, 3, "modbus_أكتب")?;
                let slave = a[0].borrow().to_number()? as u8;
                let reg = a[1].borrow().to_number()? as u16;
                let val = a[2].borrow().to_number()? as u16;
                println!(
                    "[Modbus] كتابة إلى عنوان {} سجل {} قيمة {}",
                    slave, reg, val
                );
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );

    env.define(
        "cnc_نفذ",
        Value::NativeFunction {
            name: "cnc_نفذ".to_string(),
            func: |a| {
                ensure_min_args(a, 1, "cnc_نفذ")?;
                let gcode = a[0].borrow().to_string_value();
                println!("[CNC] تنفيذ: {}", gcode);
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );

    env.define(
        "cnc_حالة",
        Value::NativeFunction {
            name: "cnc_حالة".to_string(),
            func: |_| {
                let mut status = HashMap::new();
                status.insert(
                    "الحالة".to_string(),
                    Rc::new(RefCell::new(Value::String("جاهز".to_string()))),
                );
                status.insert(
                    "المحاور".to_string(),
                    Rc::new(RefCell::new(Value::Number(3.0))),
                );
                status.insert(
                    "الإنذار".to_string(),
                    Rc::new(RefCell::new(Value::Boolean(false))),
                );
                Ok(Rc::new(RefCell::new(Value::Dictionary(status))))
            },
        },
        false,
    );

    env.define(
        "hmi_اعرض",
        Value::NativeFunction {
            name: "hmi_اعرض".to_string(),
            func: |a| {
                ensure_min_args(a, 2, "hmi_اعرض")?;
                let screen = a[0].borrow().to_string_value();
                let message = a[1].borrow().to_string_value();
                println!("[HMI] شاشة {}: {}", screen, message);
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );

    env.define(
        "esp_اتصل_واي_فاي",
        Value::NativeFunction {
            name: "esp_اتصل_واي_فاي".to_string(),
            func: |a| {
                ensure_min_args(a, 1, "esp_اتصل_واي_فاي")?;
                let ssid = a[0].borrow().to_string_value();
                let mut info = HashMap::new();
                info.insert(
                    "ssid".to_string(),
                    Rc::new(RefCell::new(Value::String(ssid))),
                );
                info.insert(
                    "متصل".to_string(),
                    Rc::new(RefCell::new(Value::Boolean(true))),
                );
                info.insert(
                    "ip".to_string(),
                    Rc::new(RefCell::new(Value::String("192.168.1.77".to_string()))),
                );
                Ok(Rc::new(RefCell::new(Value::Dictionary(info))))
            },
        },
        false,
    );

    env.define(
        "اردوينو_اكتب",
        Value::NativeFunction {
            name: "اردوينو_اكتب".to_string(),
            func: |a| {
                ensure_min_args(a, 2, "اردوينو_اكتب")?;
                let pin = a[0].borrow().to_number()? as i32;
                let val = a[1].borrow().is_truthy();
                println!(
                    "[Arduino] pin {} = {}",
                    pin,
                    if val { "HIGH" } else { "LOW" }
                );
                Ok(Rc::new(RefCell::new(Value::Boolean(true))))
            },
        },
        false,
    );

    env.define(
        "متحكم_نوع",
        Value::NativeFunction {
            name: "متحكم_نوع".to_string(),
            func: |_| {
                Ok(Rc::new(RefCell::new(Value::String(
                    "محاكي_مرجع_v1".to_string(),
                ))))
            },
        },
        false,
    );
}
