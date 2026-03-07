// ═══════════════════════════════════════════════════════════════════════════════
// قوالب Android Native - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use crate::mobile::MobileExportConfig;

/// توليد app/build.gradle
pub fn generate_build_gradle(config: &MobileExportConfig) -> String {
    format!(
r#"plugins {{
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}}

android {{
    namespace = "{}"
    compileSdk = 34

    defaultConfig {{
        applicationId = "{}"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "{}"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }}

    buildTypes {{
        release {{
            isMinifyEnabled = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }}
    }}

    compileOptions {{
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }}

    kotlinOptions {{
        jvmTarget = "17"
    }}

    buildFeatures {{
        viewBinding = true
    }}
}}

dependencies {{
    implementation("androidx.core:core-ktx:1.12.0")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("com.google.android.material:material:1.11.0")
    implementation("androidx.constraintlayout:constraintlayout:2.1.4")
    implementation("androidx.lifecycle:lifecycle-viewmodel-ktx:2.7.0")
    implementation("androidx.lifecycle:lifecycle-livedata-ktx:2.7.0")
    
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
}}
"#,
        config.package_name,
        config.package_name,
        config.version
    )
}

/// توليد settings.gradle
pub fn generate_settings_gradle(_config: &MobileExportConfig) -> String {
    String::from(
r#"pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
    }
}

rootProject.name = "AlmarjaaApp"
include(":app")
"""
    )
}

/// توليد root build.gradle
pub fn generate_root_build_gradle(_config: &MobileExportConfig) -> String {
    String::from(
r#"plugins {
    id("com.android.application") version "8.2.0" apply false
    id("org.jetbrains.kotlin.android") version "1.9.20" apply false
}
"""
    )
}

/// توليد MainActivity.kt
pub fn generate_main_activity(config: &MobileExportConfig) -> String {
    format!(
r#"package com.almarjaa.app

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.lifecycleScope
import com.almarjaa.app.databinding.ActivityMainBinding
import kotlinx.coroutines.launch

class MainActivity : AppCompatActivity() {{

    private lateinit var binding: ActivityMainBinding
    private lateinit var viewModel: MainViewModel
    private lateinit var wasmRuntime: WasmRuntime

    override fun onCreate(savedInstanceState: Bundle?) {{
        super.onCreate(savedInstanceState)
        
        // إعداد ViewBinding
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        // إعداد ViewModel
        viewModel = ViewModelProvider(this)[MainViewModel::class.java]
        
        // إعداد WASM Runtime
        wasmRuntime = WasmRuntime(this)
        lifecycleScope.launch {{
            wasmRuntime.initialize()
        }}

        // إعداد الواجهة
        setupUI()
        observeViewModel()
    }}

    private fun setupUI() {{
        // عنوان التطبيق
        binding.titleText.text = "{}"
        binding.welcomeText.text = "مرحباً بك في تطبيق المرجع"
        
        // زر التشغيل
        binding.runButton.setOnClickListener {{
            runCode()
        }}
        
        // زر الإعدادات
        binding.settingsButton.setOnClickListener {{
            // فتح الإعدادات
        }}
    }}

    private fun observeViewModel() {{
        viewModel.output.observe(this) {{ output ->
            binding.outputText.text = output
        }}
        
        viewModel.isLoading.observe(this) {{ isLoading ->
            binding.runButton.isEnabled = !isLoading
            binding.progressBar.visibility = if (isLoading) 
                android.view.View.VISIBLE 
            else 
                android.view.View.GONE
        }}
    }}

    private fun runCode() {{
        lifecycleScope.launch {{
            viewModel.setLoading(true)
            val result = wasmRuntime.execute()
            viewModel.setOutput(result)
            viewModel.setLoading(false)
        }}
    }}
}}
"#,
        config.project_name
    )
}

/// توليد WasmRuntime.kt
pub fn generate_wasm_runtime(_config: &MobileExportConfig) -> String {
    String::from(
r#"package com.almarjaa.app

import android.content.Context
import java.io.InputStream

class WasmRuntime(private val context: Context) {
    
    private var initialized = false
    private var output = ""
    
    suspend fun initialize() {
        if (initialized) return
        
        try {
            // تحميل ملف WASM من assets
            val wasmStream: InputStream = context.assets.open("wasm/app.wasm")
            val wasmBytes = wasmStream.readBytes()
            wasmStream.close()
            
            // في الإنتاج، سنستخدم مكتبة WASM حقيقية
            // للآن، نحاكي التشغيل
            output = "تم تحميل WASM بنجاح (${wasmBytes.size} bytes)"
            initialized = true
        } catch (e: Exception) {
            output = "خطأ في تحميل WASM: ${e.message}"
        }
    }
    
    fun execute(): String {
        return output
    }
    
    fun callFunction(name: String, vararg args: Any): String {
        return "تنفيذ الدالة: $name مع المعاملات: ${args.toList()}"
    }
}
"""
    )
}

/// توليد AndroidManifest.xml
pub fn generate_manifest(config: &MobileExportConfig) -> String {
    format!(
r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    xmlns:tools="http://schemas.android.com/tools">

    <application
        android:allowBackup="true"
        android:icon="@mipmap/ic_launcher"
        android:label="@string/app_name"
        android:roundIcon="@mipmap/ic_launcher_round"
        android:supportsRtl="true"
        android:theme="@style/Theme.AlmarjaaApp"
        tools:targetApi="34">
        
        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:label="@string/app_name"
            android:theme="@style/Theme.AlmarjaaApp">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
        
    </application>

</manifest>
"#,
        package_name = config.package_name
    )
}

/// توليد activity_main.xml
pub fn generate_main_layout(config: &MobileExportConfig) -> String {
    format!(
r#"<?xml version="1.0" encoding="utf-8"?>
<androidx.constraintlayout.widget.ConstraintLayout 
    xmlns:android="http://schemas.android.com/apk/res/android"
    xmlns:app="http://schemas.android.com/apk/res-auto"
    xmlns:tools="http://schemas.android.com/tools"
    android:layout_width="match_parent"
    android:layout_height="match_parent"
    android:background="@color/background"
    tools:context=".MainActivity">

    <!-- شريط التطبيق -->
    <com.google.android.material.appbar.AppBarLayout
        android:id="@+id/appBarLayout"
        android:layout_width="match_parent"
        android:layout_height="wrap_content"
        app:layout_constraintTop_toTopOf="parent">

        <com.google.android.material.appbar.MaterialToolbar
            android:id="@+id/toolbar"
            android:layout_width="match_parent"
            android:layout_height="?attr/actionBarSize"
            android:background="@color/primary"
            app:title="@string/app_name"
            app:titleTextColor="@color/white" />

    </com.google.android.material.appbar.AppBarLayout>

    <!-- المحتوى الرئيسي -->
    <androidx.core.widget.NestedScrollView
        android:layout_width="match_parent"
        android:layout_height="0dp"
        android:fillViewport="true"
        app:layout_constraintBottom_toBottomOf="parent"
        app:layout_constraintTop_toBottomOf="@id/appBarLayout">

        <LinearLayout
            android:layout_width="match_parent"
            android:layout_height="wrap_content"
            android:orientation="vertical"
            android:padding="16dp">

            <!-- بطاقة الترحيب -->
            <com.google.android.material.card.MaterialCardView
                android:layout_width="match_parent"
                android:layout_height="wrap_content"
                app:cardCornerRadius="16dp"
                app:cardElevation="4dp">

                <LinearLayout
                    android:layout_width="match_parent"
                    android:layout_height="wrap_content"
                    android:gravity="center"
                    android:orientation="vertical"
                    android:padding="24dp">

                    <TextView
                        android:layout_width="wrap_content"
                        android:layout_height="wrap_content"
                        android:text="🚀"
                        android:textSize="64sp" />

                    <TextView
                        android:id="@+id/titleText"
                        android:layout_width="wrap_content"
                        android:layout_height="wrap_content"
                        android:layout_marginTop="16dp"
                        android:text="@string/app_name"
                        android:textColor="@color/text_primary"
                        android:textSize="24sp"
                        android:textStyle="bold" />

                    <TextView
                        android:id="@+id/welcomeText"
                        android:layout_width="wrap_content"
                        android:layout_height="wrap_content"
                        android:layout_marginTop="8dp"
                        android:text="@string/welcome_message"
                        android:textColor="@color/text_secondary"
                        android:textSize="14sp" />

                </LinearLayout>

            </com.google.android.material.card.MaterialCardView>

            <!-- أزرار التحكم -->
            <com.google.android.material.button.MaterialButton
                android:id="@+id/runButton"
                android:layout_width="match_parent"
                android:layout_height="56dp"
                android:layout_marginTop="24dp"
                android:text="@string/run"
                android:textSize="16sp"
                app:backgroundTint="@color/primary"
                app:cornerRadius="12dp"
                app:icon="@android:drawable/ic_media_play"
                app:iconGravity="textStart" />

            <!-- مؤشر التحميل -->
            <ProgressBar
                android:id="@+id/progressBar"
                android:layout_width="wrap_content"
                android:layout_height="wrap_content"
                android:layout_gravity="center"
                android:layout_marginTop="16dp"
                android:visibility="gone" />

            <!-- بطاقة النتيجة -->
            <com.google.android.material.card.MaterialCardView
                android:layout_width="match_parent"
                android:layout_height="wrap_content"
                android:layout_marginTop="24dp"
                app:cardCornerRadius="12dp"
                app:cardElevation="2dp">

                <LinearLayout
                    android:layout_width="match_parent"
                    android:layout_height="wrap_content"
                    android:orientation="vertical"
                    android:padding="16dp">

                    <TextView
                        android:layout_width="wrap_content"
                        android:layout_height="wrap_content"
                        android:text="@string/output"
                        android:textColor="@color/text_primary"
                        android:textSize="16sp"
                        android:textStyle="bold" />

                    <TextView
                        android:id="@+id/outputText"
                        android:layout_width="match_parent"
                        android:layout_height="wrap_content"
                        android:layout_marginTop="12dp"
                        android:background="@color/output_background"
                        android:fontFamily="monospace"
                        android:padding="12dp"
                        android:textColor="@color/text_primary"
                        android:textSize="14sp" />

                </LinearLayout>

            </com.google.android.material.card.MaterialCardView>

        </LinearLayout>

    </androidx.core.widget.NestedScrollView>

</androidx.constraintlayout.widget.ConstraintLayout>
"#,
        project_name = config.project_name
    )
}

/// توليد strings.xml
pub fn generate_strings(config: &MobileExportConfig) -> String {
    format!(
r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="app_name">{}</string>
    <string name="welcome">Welcome</string>
    <string name="welcome_message">Al-Marjaa Generated App - Arabic Programming Language</string>
    <string name="run">Run</string>
    <string name="output">Output</string>
    <string name="settings">Settings</string>
</resources>
"#,
        config.project_name
    )
}

/// توليد strings-ar.xml
pub fn generate_strings_ar(config: &MobileExportConfig) -> String {
    format!(
r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="app_name">{}</string>
    <string name="welcome">مرحباً بك</string>
    <string name="welcome_message">تطبيق مولد بلغة المرجع - لغة برمجة عربية متكاملة</string>
    <string name="run">تشغيل</string>
    <string name="output">النتيجة</string>
    <string name="settings">الإعدادات</string>
</resources>
"#,
        config.project_name
    )
}
