// ═══════════════════════════════════════════════════════════════════════════════
// قوالب React Native - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use crate::mobile::MobileExportConfig;

/// توليد package.json
pub fn generate_package_json(config: &MobileExportConfig) -> String {
    format!(
r#"{{
  "name": "{}",
  "version": "{}",
  "description": "{}",
  "main": "node_modules/expo/AppEntry.js",
  "scripts": {{
    "start": "expo start",
    "android": "expo run:android",
    "ios": "expo run:ios",
    "web": "expo start --web",
    "test": "jest",
    "lint": "eslint ."
  }},
  "dependencies": {{
    "expo": "~50.0.0",
    "expo-status-bar": "~1.11.0",
    "react": "18.2.0",
    "react-native": "0.73.0",
    "react-native-svg": "14.1.0",
    "@react-navigation/native": "^6.1.9",
    "@react-navigation/native-stack": "^6.9.17",
    "react-native-screens": "~3.29.0",
    "react-native-safe-area-context": "4.8.2",
    "react-native-gesture-handler": "~2.14.0",
    "react-native-reanimated": "~3.6.0",
    "i18next": "^23.7.0",
    "react-i18next": "^14.0.0",
    "expo-localization": "~14.8.0",
    "@react-native-async-storage/async-storage": "1.21.0"
  }},
  "devDependencies": {{
    "@babel/core": "^7.20.0",
    "@types/react": "~18.2.45",
    "typescript": "^5.1.3",
    "@react-native/babel-preset": "0.73.0",
    "@react-native/eslint-config": "0.73.0",
    "@react-native/metro-config": "0.73.0"
  }},
  "private": true
}}
"#,
        config.project_name,
        config.version,
        config.description
    )
}

/// توليد App.tsx
pub fn generate_app(config: &MobileExportConfig) -> String {
    format!(
r#"import React from 'react';
import {{ StatusBar }} from 'expo-status-bar';
import {{ NavigationContainer }} from '@react-navigation/native';
import {{ createNativeStackNavigator }} from '@react-navigation/native-stack';
import {{ I18nextProvider }} from 'react-i18next';
import {{ SafeAreaProvider }} from 'react-native-safe-area-context';
import {{ GestureHandlerRootView }} from 'react-native-gesture-handler';
import HomeScreen from './src/screens/HomeScreen';
import i18n from './src/i18n/rtl';

const Stack = createNativeStackNavigator();

export default function App() {{
  return (
    <GestureHandlerRootView style={{{{ flex: 1 }}}}>
      <SafeAreaProvider>
        <I18nextProvider i18n={{i18n}}>
          <NavigationContainer>
            <StatusBar style="auto" />
            <Stack.Navigator
              initialRouteName="Home"
              screenOptions={{{{
                headerStyle: {{
                  backgroundColor: '#667eea',
                }},
                headerTintColor: '#fff',
                headerTitleStyle: {{
                  fontWeight: 'bold',
                }},
                headerTitleAlign: 'center',
              }}}}
            >
              <Stack.Screen 
                name="Home" 
                component={{HomeScreen}}
                options={{{{ title: '{}' }}}}
              />
            </Stack.Navigator>
          </NavigationContainer>
        </I18nextProvider>
      </SafeAreaProvider>
    </GestureHandlerRootView>
  );
}}
"#,
        config.project_name
    )
}

/// توليد HomeScreen.tsx
pub fn generate_home_screen(config: &MobileExportConfig) -> String {
    format!(
r#"import React, {{ useState, useEffect }} from 'react';
import {{
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  ScrollView,
  ActivityIndicator,
}} from 'react-native';
import {{ useTranslation }} from 'react-i18next';
import {{ SafeAreaView }} from 'react-native-safe-area-context';
import WasmRuntime from '../runtime/wasmRuntime';

export default function HomeScreen() {{
  const {{ t, i18n }} = useTranslation();
  const [output, setOutput] = useState<string>('');
  const [isLoading, setIsLoading] = useState(false);
  const [runtime] = useState(() => new WasmRuntime());

  useEffect(() => {{
    initRuntime();
  }}, []);

  const initRuntime = async () => {{
    setIsLoading(true);
    await runtime.initialize();
    setIsLoading(false);
  }};

  const runCode = async () => {{
    setIsLoading(true);
    const result = await runtime.execute();
    setOutput(result);
    setIsLoading(false);
  }};

  const toggleLanguage = () => {{
    i18n.changeLanguage(i18n.language === 'ar' ? 'en' : 'ar');
  }};

  return (
    <SafeAreaView style={{styles.container}}>
      <ScrollView 
        style={{styles.scrollView}}
        contentContainerStyle={{styles.contentContainer}}
      >
        {/* بطاقة الترحيب */}
        <View style={{styles.welcomeCard}}>
          <Text style={{styles.icon}}>🚀</Text>
          <Text style={{styles.welcomeTitle}}>{{t('welcome')}}</Text>
          <Text style={{styles.welcomeMessage}}>
            {{t('welcomeMessage')}}
          </Text>
        </View>

        {/* أزرار التحكم */}
        <View style={{styles.buttonsContainer}}>
          <TouchableOpacity 
            style={{[styles.button, styles.primaryButton]}}
            onPress={{runCode}}
            disabled={{isLoading}}
          >
            <Text style={{styles.buttonText}}>
              {{isLoading ? t('loading') : t('run')}}
            </Text>
          </TouchableOpacity>

          <TouchableOpacity 
            style={{[styles.button, styles.secondaryButton]}}
            onPress={{toggleLanguage}}
          >
            <Text style={{styles.secondaryButtonText}}>
              {{i18n.language === 'ar' ? 'English' : 'العربية'}}
            </Text>
          </TouchableOpacity>
        </View>

        {/* النتيجة */}
        {{output ? (
          <View style={{styles.outputCard}}>
            <Text style={{styles.outputTitle}}>{{t('output')}}</Text>
            <View style={{styles.outputContent}}>
              <Text style={{styles.outputText}}>{{output}}</Text>
            </View>
          </View>
        ) : null}}

        {{isLoading && (
          <View style={{styles.loadingContainer}}>
            <ActivityIndicator size="large" color="#667eea" />
          </View>
        )}}
      </ScrollView>
    </SafeAreaView>
  );
}}

const styles = StyleSheet.create({{
  container: {{
    flex: 1,
    backgroundColor: '#f8f9fa',
  }},
  scrollView: {{
    flex: 1,
  }},
  contentContainer: {{
    padding: 16,
  }},
  welcomeCard: {{
    backgroundColor: '#fff',
    borderRadius: 20,
    padding: 24,
    alignItems: 'center',
    shadowColor: '#000',
    shadowOffset: {{ width: 0, height: 4 }},
    shadowOpacity: 0.1,
    shadowRadius: 12,
    elevation: 5,
  }},
  icon: {{
    fontSize: 64,
    marginBottom: 16,
  }},
  welcomeTitle: {{
    fontSize: 24,
    fontWeight: 'bold',
    color: '#1a1a2e',
    marginBottom: 8,
  }},
  welcomeMessage: {{
    fontSize: 14,
    color: '#666',
    textAlign: 'center',
  }},
  buttonsContainer: {{
    marginTop: 24,
    gap: 12,
  }},
  button: {{
    borderRadius: 12,
    paddingVertical: 16,
    alignItems: 'center',
  }},
  primaryButton: {{
    backgroundColor: '#667eea',
  }},
  secondaryButton: {{
    backgroundColor: '#fff',
    borderWidth: 2,
    borderColor: '#667eea',
  }},
  buttonText: {{
    color: '#fff',
    fontSize: 16,
    fontWeight: 'bold',
  }},
  secondaryButtonText: {{
    color: '#667eea',
    fontSize: 16,
    fontWeight: 'bold',
  }},
  outputCard: {{
    marginTop: 24,
    backgroundColor: '#fff',
    borderRadius: 16,
    padding: 16,
  }},
  outputTitle: {{
    fontSize: 16,
    fontWeight: 'bold',
    color: '#1a1a2e',
    marginBottom: 12,
  }},
  outputContent: {{
    backgroundColor: '#f0f0f0',
    borderRadius: 8,
    padding: 12,
  }},
  outputText: {{
    fontFamily: 'monospace',
    fontSize: 14,
    color: '#333',
  }},
  loadingContainer: {{
    marginTop: 24,
    alignItems: 'center',
  }},
}});
"#,
        project_name = config.project_name
    )
}

/// توليد wasmRuntime.ts
pub fn generate_wasm_runtime(_config: &MobileExportConfig) -> String {
    String::from(
r#"class WasmRuntime {
  private initialized: boolean = false;
  private output: string = '';

  async initialize(): Promise<void> {
    if (this.initialized) return;

    try {
      // تحميل ملف WASM
      // في React Native، نستخدم react-native-wasm أو حلول مشابهة
      const wasmResponse = await fetch('assets/wasm/app.wasm');
      const wasmBytes = await wasmResponse.arrayBuffer();
      
      // محاكاة التشغيل
      this.output = 'تم تحميل WASM بنجاح';
      this.initialized = true;
    } catch (e) {
      this.output = `خطأ في تحميل WASM: ${e}`;
    }
  }

  async execute(): Promise<string> {
    if (!this.initialized) {
      await this.initialize();
    }
    return this.output;
  }

  async callFunction(name: string, args: any[]): Promise<string> {
    return `تنفيذ الدالة: ${name} مع المعاملات: ${JSON.stringify(args)}`;
  }
}

export default WasmRuntime;
"""
    )
}

/// توليد RTL utilities
pub fn generate_rtl_utils(_config: &MobileExportConfig) -> String {
    String::from(
r#"import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import * as Localization from 'expo-localization';

const resources = {
  ar: {
    translation: {
      welcome: 'مرحباً بك',
      welcomeMessage: 'تطبيق مولد بلغة المرجع - لغة برمجة عربية متكاملة',
      run: 'تشغيل',
      stop: 'إيقاف',
      loading: 'جاري التحميل...',
      output: 'النتيجة',
      settings: 'الإعدادات',
      language: 'اللغة',
    },
  },
  en: {
    translation: {
      welcome: 'Welcome',
      welcomeMessage: 'Al-Marjaa Generated App - Arabic Programming Language',
      run: 'Run',
      stop: 'Stop',
      loading: 'Loading...',
      output: 'Output',
      settings: 'Settings',
      language: 'Language',
    },
  },
};

i18n
  .use(initReactI18next)
  .init({
    resources,
    lng: Localization.locale.startsWith('ar') ? 'ar' : 'en',
    fallbackLng: 'ar',
    interpolation: {
      escapeValue: false,
    },
  });

export const isRTL = i18n.language === 'ar';

export default i18n;
"""
    )
}
