// ═══════════════════════════════════════════════════════════════════════════════
// قوالب Capacitor - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use crate::mobile::MobileExportConfig;

/// توليد package.json
pub fn generate_package_json(config: &MobileExportConfig) -> String {
    format!(
r#"{{
  "name": "{}",
  "version": "{}",
  "description": "{}",
  "main": "src/main.js",
  "scripts": {{
    "start": "npx serve public",
    "build": "echo 'Building...'",
    "cap:init": "npx cap init {} {}",
    "cap:add:android": "npx cap add android",
    "cap:add:ios": "npx cap add ios",
    "cap:sync": "npx cap sync",
    "cap:open:android": "npx cap open android",
    "cap:open:ios": "npx cap open ios"
  }},
  "dependencies": {{
    "@capacitor/core": "^5.6.0",
    "@capacitor/android": "^5.6.0",
    "@capacitor/ios": "^5.6.0",
    "@capacitor/preferences": "^5.0.6",
    "@capacitor/browser": "^5.1.0",
    "@capacitor/haptics": "^5.0.6",
    "@capacitor/keyboard": "^5.0.6",
    "@capacitor/status-bar": "^5.0.6"
  }},
  "devDependencies": {{
    "@capacitor/cli": "^5.6.0",
    "serve": "^14.2.0"
  }},
  "private": true
}}
"#,
        config.project_name,
        config.version,
        config.description,
        config.project_name,
        config.package_name
    )
}

/// توليد capacitor.config.json
pub fn generate_config(config: &MobileExportConfig) -> String {
    format!(
r#"{{
  "appId": "{}",
  "appName": "{}",
  "webDir": "public",
  "server": {{
    "androidScheme": "https"
  }},
  "plugins": {{
    "StatusBar": {{
      "style": "dark",
      "backgroundColor": "#667eea"
    }},
    "Keyboard": {{
      "resize": "body",
      "resizeOnFullScreen": true
    }}
  }},
  "android": {{
    "backgroundColor": "#667eea"
  }},
  "ios": {{
    "contentInset": "automatic"
  }}
}}
"#,
        config.package_name,
        config.project_name
    )
}

/// توليد index.html
pub fn generate_index_html(config: &MobileExportConfig) -> String {
    let primary_color = &config.primary_color;
    format!(
r#"<!DOCTYPE html>
<html dir="rtl" lang="ar">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover">
    <meta name="theme-color" content="{}">
    <meta name="apple-mobile-web-app-capable" content="yes">
    <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent">
    <title>{}</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div id="app">
        <!-- شريط التنقل -->
        <header class="app-bar">
            <h1 class="app-title">{}</h1>
            <button class="icon-button" id="settings-btn" aria-label="الإعدادات">
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="3"></circle>
                    <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
                </svg>
            </button>
        </header>

        <!-- المحتوى الرئيسي -->
        <main class="content">
            <!-- بطاقة الترحيب -->
            <div class="welcome-card">
                <div class="icon">🚀</div>
                <h2 class="title">مرحباً بك</h2>
                <p class="message">تطبيق مولد بلغة المرجع - لغة برمجة عربية متكاملة</p>
            </div>

            <!-- أزرار التحكم -->
            <div class="buttons-container">
                <button class="primary-button" id="run-btn">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                        <polygon points="5 3 19 12 5 21 5 3"></polygon>
                    </svg>
                    <span>تشغيل</span>
                </button>
            </div>

            <!-- مؤشر التحميل -->
            <div class="loading-indicator" id="loading" style="display: none;">
                <div class="spinner"></div>
                <span>جاري التحميل...</span>
            </div>

            <!-- النتيجة -->
            <div class="output-card" id="output-card" style="display: none;">
                <div class="output-header">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <polyline points="4 17 10 11 4 5"></polyline>
                        <line x1="12" y1="19" x2="20" y2="19"></line>
                    </svg>
                    <span>النتيجة</span>
                </div>
                <pre class="output-content" id="output-content"></pre>
            </div>
        </main>
    </div>

    <script src="main.js"></script>
</body>
</html>
"#,
        primary_color,
        config.project_name,
        config.project_name
    )
}

/// توليد main.js
pub fn generate_main_js(_config: &MobileExportConfig) -> String {
    String::from(
r#"// تهيئة Capacitor
const { StatusBar, Style } = Capacitor.Plugins;
const { Haptics, ImpactStyle } = Capacitor.Plugins;
const { Keyboard } = Capacitor.Plugins;

// حالة التطبيق
const state = {
    isLoading: false,
    output: '',
    wasmModule: null
};

// تهيئة التطبيق
async function initApp() {
    // إعداد شريط الحالة
    try {
        await StatusBar.setStyle({ style: Style.Dark });
        await StatusBar.setBackgroundColor({ color: '#667eea' });
    } catch (e) {
        console.log('StatusBar not available');
    }

    // إعداد RTL
    document.documentElement.dir = 'rtl';
    document.documentElement.lang = 'ar';

    // تحميل WASM
    await loadWasm();

    // إعداد الأحداث
    setupEventListeners();
}

// تحميل WASM
async function loadWasm() {
    try {
        // تحميل ملف WASM
        const response = await fetch('app.wasm');
        const wasmBytes = await response.arrayBuffer();
        
        // في الإنتاج، سنستخدم WebAssembly.instantiate
        // للآن، نحاكي التحميل
        state.wasmModule = {
            loaded: true,
            size: wasmBytes.byteLength
        };
        
        console.log('WASM loaded:', state.wasmModule.size, 'bytes');
    } catch (e) {
        console.error('Failed to load WASM:', e);
    }
}

// إعداد أحداث المستمعين
function setupEventListeners() {
    // زر التشغيل
    const runBtn = document.getElementById('run-btn');
    runBtn.addEventListener('click', runCode);

    // زر الإعدادات
    const settingsBtn = document.getElementById('settings-btn');
    settingsBtn.addEventListener('click', openSettings);

    // لوحة المفاتيح
    Keyboard.addListener('keyboardWillShow', (info) => {
        document.body.style.paddingBottom = info.keyboardHeight + 'px';
    });

    Keyboard.addListener('keyboardWillHide', () => {
        document.body.style.paddingBottom = '0';
    });
}

// تشغيل الكود
async function runCode() {
    if (state.isLoading) return;

    // اهتزاز خفيف
    try {
        await Haptics.impact({ style: ImpactStyle.Light });
    } catch (e) {}

    state.isLoading = true;
    showLoading(true);

    // محاكاة التنفيذ
    await new Promise(resolve => setTimeout(resolve, 1000));

    state.output = state.wasmModule
        ? `تم تحميل WASM بنجاح (${state.wasmModule.size} bytes)\nتطبيق المرجع يعمل بشكل صحيح!`
        : 'WASM غير محمل';

    showOutput(state.output);
    state.isLoading = false;
    showLoading(false);

    // اهتزاز للنجاح
    try {
        await Haptics.impact({ style: ImpactStyle.Medium });
    } catch (e) {}
}

// فتح الإعدادات
function openSettings() {
    alert('الإعدادات قيد التطوير');
}

// عرض مؤشر التحميل
function showLoading(show) {
    const loading = document.getElementById('loading');
    loading.style.display = show ? 'flex' : 'none';
    
    const runBtn = document.getElementById('run-btn');
    runBtn.disabled = show;
}

// عرض النتيجة
function showOutput(output) {
    const outputCard = document.getElementById('output-card');
    const outputContent = document.getElementById('output-content');
    
    outputCard.style.display = 'block';
    outputContent.textContent = output;
}

// بدء التطبيق
document.addEventListener('DOMContentLoaded', initApp);
"""
    )
}

/// توليد styles.css
pub fn generate_styles(config: &MobileExportConfig) -> String {
    let primary_color = &config.primary_color;
    format!(
r#"/* المتغيرات */
:root {{
    --primary: {};
    --primary-dark: #5a6fd6;
    --secondary: #764BA2;
    --background: #f8f9fa;
    --surface: #ffffff;
    --text-primary: #1a1a2e;
    --text-secondary: #666666;
    --border-radius: 16px;
    --shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}}

/* Reset */
* {{
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}}

body {{
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    background: var(--background);
    color: var(--text-primary);
    line-height: 1.6;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    safe-area-inset-top: env(safe-area-inset-top);
    safe-area-inset-bottom: env(safe-area-inset-bottom);
}}

#app {{
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}}

/* شريط التنقل */
.app-bar {{
    background: var(--primary);
    color: white;
    padding: 16px 20px;
    padding-top: calc(16px + env(safe-area-inset-top));
    display: flex;
    justify-content: space-between;
    align-items: center;
    position: sticky;
    top: 0;
    z-index: 100;
}}

.app-title {{
    font-size: 20px;
    font-weight: 700;
}}

.icon-button {{
    background: transparent;
    border: none;
    color: white;
    padding: 8px;
    cursor: pointer;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
}}

.icon-button:active {{
    background: rgba(255, 255, 255, 0.2);
}}

/* المحتوى */
.content {{
    flex: 1;
    padding: 20px;
    padding-bottom: calc(20px + env(safe-area-inset-bottom));
}}

/* بطاقة الترحيب */
.welcome-card {{
    background: var(--surface);
    border-radius: var(--border-radius);
    padding: 32px 24px;
    text-align: center;
    box-shadow: var(--shadow);
}}

.icon {{
    font-size: 64px;
    margin-bottom: 16px;
}}

.title {{
    font-size: 24px;
    font-weight: 700;
    margin-bottom: 8px;
}}

.message {{
    font-size: 14px;
    color: var(--text-secondary);
}}

/* الأزرار */
.buttons-container {{
    margin-top: 24px;
    display: flex;
    flex-direction: column;
    gap: 12px;
}}

.primary-button {{
    background: var(--primary);
    color: white;
    border: none;
    border-radius: 12px;
    padding: 16px 24px;
    font-size: 16px;
    font-weight: 700;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    transition: transform 0.2s, box-shadow 0.2s;
}}

.primary-button:active {{
    transform: scale(0.98);
}}

.primary-button:disabled {{
    opacity: 0.6;
    cursor: not-allowed;
}}

/* مؤشر التحميل */
.loading-indicator {{
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    margin-top: 24px;
    color: var(--text-secondary);
}}

.spinner {{
    width: 24px;
    height: 24px;
    border: 3px solid var(--background);
    border-top-color: var(--primary);
    border-radius: 50%;
    animation: spin 1s linear infinite;
}}

@keyframes spin {{
    to {{ transform: rotate(360deg); }}
}}

/* بطاقة النتيجة */
.output-card {{
    background: var(--surface);
    border-radius: 12px;
    margin-top: 24px;
    overflow: hidden;
    box-shadow: var(--shadow);
}}

.output-header {{
    background: var(--background);
    padding: 12px 16px;
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 600;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}}

.output-content {{
    padding: 16px;
    font-family: 'SF Mono', Monaco, 'Courier New', monospace;
    font-size: 14px;
    white-space: pre-wrap;
    word-break: break-word;
    background: var(--background);
    margin: 0;
}}

/* Dark Mode */
@media (prefers-color-scheme: dark) {{
    :root {{
        --background: #1a1a2e;
        --surface: #252542;
        --text-primary: #ffffff;
        --text-secondary: #aaaaaa;
    }}
}}
"#,
        primary_color
    )
}
