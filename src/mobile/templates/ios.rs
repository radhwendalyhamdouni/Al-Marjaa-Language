// ═══════════════════════════════════════════════════════════════════════════════
// قوالب iOS Native - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use crate::mobile::MobileExportConfig;

/// توليد AppDelegate.swift
pub fn generate_app_delegate(config: &MobileExportConfig) -> String {
    format!(
r#"import UIKit

@main
class AppDelegate: UIResponder, UIApplicationDelegate {{

    var window: UIWindow?

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
    ) -> Bool {{
        
        // إعداد النافذة
        window = UIWindow(frame: UIScreen.main.bounds)
        
        // إنشاء ViewController الرئيسي
        let mainViewController = ViewController()
        let navigationController = UINavigationController(rootViewController: mainViewController)
        
        // تخصيص شريط التنقل
        let appearance = UINavigationBarAppearance()
        appearance.configureWithOpaqueBackground()
        appearance.backgroundColor = UIColor(hex: "#667eea")
        appearance.titleTextAttributes = [
            .foregroundColor: UIColor.white,
            .font: UIFont.boldSystemFont(ofSize: 18)
        ]
        
        UINavigationBar.appearance().standardAppearance = appearance
        UINavigationBar.appearance().scrollEdgeAppearance = appearance
        
        window?.rootViewController = navigationController
        window?.makeKeyAndVisible()
        
        return true
    }}

    // MARK: - UISceneSession Lifecycle
    func application(
        _ application: UIApplication,
        configurationForConnecting connectingSceneSession: UISceneSession,
        options: UIScene.ConnectionOptions
    ) -> UISceneConfiguration {{
        return UISceneConfiguration(name: "Default Configuration", sessionRole: connectingSceneSession.role)
    }}
}}

// MARK: - UIColor Extension
extension UIColor {{
    convenience init?(hex: String) {{
        let r, g, b: CGFloat
        
        var hexSanitized = hex.trimmingCharacters(in: .whitespacesAndNewlines)
        hexSanitized = hexSanitized.replacingOccurrences(of: "#", with: "")
        
        var rgb: UInt64 = 0
        guard Scanner(string: hexSanitized).scanHexInt64(&rgb) else {{ return nil }}
        
        r = CGFloat((rgb & 0xFF0000) >> 16) / 255.0
        g = CGFloat((rgb & 0x00FF00) >> 8) / 255.0
        b = CGFloat(rgb & 0x0000FF) / 255.0
        
        self.init(red: r, green: g, blue: b, alpha: 1.0)
    }}
}}
"#,
        project_name = config.project_name
    )
}

/// توليد ViewController.swift
pub fn generate_view_controller(config: &MobileExportConfig) -> String {
    format!(
r#"import UIKit

class ViewController: UIViewController {{
    
    // MARK: - UI Elements
    
    private let scrollView = UIScrollView()
    private let contentView = UIView()
    
    private let iconLabel: UILabel = {{
        let label = UILabel()
        label.text = "🚀"
        label.font = UIFont.systemFont(ofSize: 64)
        label.textAlignment = .center
        label.translatesAutoresizingMaskIntoConstraints = false
        return label
    }}()
    
    private let titleLabel: UILabel = {{
        let label = UILabel()
        label.text = "{}"
        label.font = UIFont.boldSystemFont(ofSize: 24)
        label.textColor = .label
        label.textAlignment = .center
        label.translatesAutoresizingMaskIntoConstraints = false
        return label
    }}()
    
    private let welcomeLabel: UILabel = {{
        let label = UILabel()
        label.text = "مرحباً بك في تطبيق المرجع"
        label.font = UIFont.systemFont(ofSize: 14)
        label.textColor = .secondaryLabel
        label.textAlignment = .center
        label.numberOfLines = 0
        label.translatesAutoresizingMaskIntoConstraints = false
        return label
    }}()
    
    private let runButton: UIButton = {{
        let button = UIButton(type: .system)
        button.setTitle("تشغيل", for: .normal)
        button.titleLabel?.font = UIFont.boldSystemFont(ofSize: 16)
        button.backgroundColor = UIColor(hex: "#667eea")
        button.setTitleColor(.white, for: .normal)
        button.layer.cornerRadius = 12
        button.translatesAutoresizingMaskIntoConstraints = false
        return button
    }}()
    
    private let activityIndicator: UIActivityIndicatorView = {{
        let indicator = UIActivityIndicatorView(style: .large)
        indicator.hidesWhenStopped = true
        indicator.translatesAutoresizingMaskIntoConstraints = false
        return indicator
    }}()
    
    private let outputLabel: UILabel = {{
        let label = UILabel()
        label.text = "النتيجة"
        label.font = UIFont.boldSystemFont(ofSize: 16)
        label.textColor = .label
        label.translatesAutoresizingMaskIntoConstraints = false
        return label
    }}()
    
    private let outputTextView: UITextView = {{
        let textView = UITextView()
        textView.font = UIFont.monospacedSystemFont(ofSize: 14, weight: .regular)
        textView.backgroundColor = .secondarySystemBackground
        textView.layer.cornerRadius = 8
        textView.isEditable = false
        textView.textContainerInset = UIEdgeInsets(top: 12, left: 12, bottom: 12, right: 12)
        textView.translatesAutoresizingMaskIntoConstraints = false
        return textView
    }}()
    
    // MARK: - Properties
    
    private let wasmRuntime = WasmRuntime()
    
    // MARK: - Lifecycle
    
    override func viewDidLoad() {{
        super.viewDidLoad()
        setupUI()
        setupConstraints()
        setupActions()
    }}
    
    override func viewDidLayoutSubviews() {{
        super.viewDidLayoutSubviews()
        updateScrollViewContentSize()
    }}
    
    // MARK: - Setup
    
    private func setupUI() {{
        view.backgroundColor = .systemBackground
        title = "{}"
        
        // إعداد ScrollView
        scrollView.translatesAutoresizingMaskIntoConstraints = false
        contentView.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(scrollView)
        scrollView.addSubview(contentView)
        
        // إضافة العناصر
        contentView.addSubview(iconLabel)
        contentView.addSubview(titleLabel)
        contentView.addSubview(welcomeLabel)
        contentView.addSubview(runButton)
        contentView.addSubview(activityIndicator)
        contentView.addSubview(outputLabel)
        contentView.addSubview(outputTextView)
    }}
    
    private func setupConstraints() {{
        NSLayoutConstraint.activate([
            // ScrollView
            scrollView.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor),
            scrollView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            scrollView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            scrollView.bottomAnchor.constraint(equalTo: view.bottomAnchor),
            
            // ContentView
            contentView.topAnchor.constraint(equalTo: scrollView.topAnchor),
            contentView.leadingAnchor.constraint(equalTo: scrollView.leadingAnchor),
            contentView.trailingAnchor.constraint(equalTo: scrollView.trailingAnchor),
            contentView.bottomAnchor.constraint(equalTo: scrollView.bottomAnchor),
            contentView.widthAnchor.constraint(equalTo: scrollView.widthAnchor),
            
            // Icon
            iconLabel.topAnchor.constraint(equalTo: contentView.topAnchor, constant: 32),
            iconLabel.centerXAnchor.constraint(equalTo: contentView.centerXAnchor),
            
            // Title
            titleLabel.topAnchor.constraint(equalTo: iconLabel.bottomAnchor, constant: 16),
            titleLabel.leadingAnchor.constraint(equalTo: contentView.leadingAnchor, constant: 16),
            titleLabel.trailingAnchor.constraint(equalTo: contentView.trailingAnchor, constant: -16),
            
            // Welcome
            welcomeLabel.topAnchor.constraint(equalTo: titleLabel.bottomAnchor, constant: 8),
            welcomeLabel.leadingAnchor.constraint(equalTo: contentView.leadingAnchor, constant: 32),
            welcomeLabel.trailingAnchor.constraint(equalTo: contentView.trailingAnchor, constant: -32),
            
            // Run Button
            runButton.topAnchor.constraint(equalTo: welcomeLabel.bottomAnchor, constant: 32),
            runButton.leadingAnchor.constraint(equalTo: contentView.leadingAnchor, constant: 16),
            runButton.trailingAnchor.constraint(equalTo: contentView.trailingAnchor, constant: -16),
            runButton.heightAnchor.constraint(equalToConstant: 50),
            
            // Activity Indicator
            activityIndicator.topAnchor.constraint(equalTo: runButton.bottomAnchor, constant: 16),
            activityIndicator.centerXAnchor.constraint(equalTo: contentView.centerXAnchor),
            
            // Output Label
            outputLabel.topAnchor.constraint(equalTo: activityIndicator.bottomAnchor, constant: 16),
            outputLabel.leadingAnchor.constraint(equalTo: contentView.leadingAnchor, constant: 16),
            outputLabel.trailingAnchor.constraint(equalTo: contentView.trailingAnchor, constant: -16),
            
            // Output TextView
            outputTextView.topAnchor.constraint(equalTo: outputLabel.bottomAnchor, constant: 12),
            outputTextView.leadingAnchor.constraint(equalTo: contentView.leadingAnchor, constant: 16),
            outputTextView.trailingAnchor.constraint(equalTo: contentView.trailingAnchor, constant: -16),
            outputTextView.heightAnchor.constraint(greaterThanOrEqualToConstant: 100),
            outputTextView.bottomAnchor.constraint(equalTo: contentView.bottomAnchor, constant: -32),
        ])
    }}
    
    private func setupActions() {{
        runButton.addTarget(self, action: #selector(runButtonTapped), for: .touchUpInside)
    }}
    
    private func updateScrollViewContentSize() {{
        scrollView.contentSize = CGSize(width: view.frame.width, height: contentView.frame.height)
    }}
    
    // MARK: - Actions
    
    @objc private func runButtonTapped() {{
        runButton.isEnabled = false
        activityIndicator.startAnimating()
        
        Task {{ [weak self] in
            guard let self = self else {{ return }}
            
            let output = await self.wasmRuntime.execute()
            
            await MainActor.run {{
                self.outputTextView.text = output
                self.runButton.isEnabled = true
                self.activityIndicator.stopAnimating()
            }}
        }}
    }}
}}
"#,
        config.project_name,
        config.project_name
    )
}

/// توليد WasmRuntime.swift
pub fn generate_wasm_runtime(_config: &MobileExportConfig) -> String {
    String::from(
r#"import Foundation

actor WasmRuntime {
    
    private var initialized = false
    private var output = ""
    
    func initialize() async {
        if initialized { return }
        
        do {
            // تحميل ملف WASM من Bundle
            guard let wasmPath = Bundle.main.path(forResource: "app", ofType: "wasm") else {
                output = "لم يتم العثور على ملف WASM"
                return
            }
            
            let wasmData = try Data(contentsOf: URL(fileURLWithPath: wasmPath))
            
            // في الإنتاج، سنستخدم مكتبة WASM حقيقية
            // للآن، نحاكي التشغيل
            output = "تم تحميل WASM بنجاح (\(wasmData.count) bytes)"
            initialized = true
        } catch {
            output = "خطأ في تحميل WASM: \(error.localizedDescription)"
        }
    }
    
    func execute() async -> String {
        if !initialized {
            await initialize()
        }
        return output
    }
    
    func callFunction(_ name: String, args: [Any]) async -> String {
        return "تنفيذ الدالة: \(name) مع المعاملات: \(args)"
    }
}
"""
    )
}

/// توليد Info.plist
pub fn generate_info_plist(config: &MobileExportConfig) -> String {
    format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>ar</string>
    <key>CFBundleDisplayName</key>
    <string>{}</string>
    <key>CFBundleExecutable</key>
    <string>$(EXECUTABLE_NAME)</string>
    <key>CFBundleIdentifier</key>
    <string>{}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>{}</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSRequiresIPhoneOS</key>
    <true/>
    <key>UIApplicationSceneManifest</key>
    <dict>
        <key>UIApplicationSupportsMultipleScenes</key>
        <false/>
    </dict>
    <key>UILaunchStoryboardName</key>
    <string>LaunchScreen</string>
    <key>UIMainStoryboardFile</key>
    <string>Main</string>
    <key>UIRequiredDeviceCapabilities</key>
    <array>
        <string>armv7</string>
    </array>
    <key>UISupportedInterfaceOrientations</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
    <key>UISupportedInterfaceOrientations~ipad</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationPortraitUpsideDown</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
</dict>
</plist>
"#,
        config.project_name,
        config.package_name,
        config.project_name,
        config.version
    )
}

/// توليد Main.storyboard
pub fn generate_main_storyboard(config: &MobileExportConfig) -> String {
    format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<document type="com.apple.InterfaceBuilder3.CocoaTouch.Storyboard.XIB" version="3.0" toolsVersion="21701" targetRuntime="iOS.CocoaTouch" propertyAccessControl="none" useAutolayout="YES" useTraitCollections="YES" useSafeAreas="YES" colorMatched="YES" initialViewController="BYZ-38-t0r">
    <device id="retina6_12" orientation="portrait" appearance="light"/>
    <dependencies>
        <plugIn identifier="com.apple.InterfaceBuilder.IBCocoaTouchPlugin" version="21678"/>
        <capability name="Safe area layout guides" minToolsVersion="9.0"/>
        <capability name="System colors in document resources" minToolsVersion="11.0"/>
        <capability name="documents saved in the Xcode 8 format" minToolsVersion="8.0"/>
    </dependencies>
    <scenes>
        <!--View Controller-->
        <scene sceneID="tne-QT-ifu">
            <objects>
                <viewController id="BYZ-38-t0r" customClass="ViewController" customModuleProvider="target" sceneMemberID="viewController">
                    <view key="view" contentMode="scaleToFill" id="8bC-Xf-vdC">
                        <rect key="frame" x="0.0" y="0.0" width="393" height="852"/>
                        <autoresizingMask key="autoresizingMask" widthSizable="YES" heightSizable="YES"/>
                        <viewLayoutGuide key="safeArea" id="6Tk-OE-BBY"/>
                        <color key="backgroundColor" systemColor="systemBackgroundColor"/>
                    </view>
                    <navigationItem key="navigationItem" title="{}" id="hWk-YQ-D1g"/>
                </viewController>
                <placeholder placeholderIdentifier="IBFirstResponder" id="dkx-z0-nzr" sceneMemberID="firstResponder"/>
            </objects>
            <point key="canvasLocation" x="1056.4885496183206" y="3.5211267605633805"/>
        </scene>
    </scenes>
    <resources>
        <systemColor name="systemBackgroundColor">
            <color white="1" alpha="1" colorSpace="custom" customColorSpace="genericGamma22GrayColorSpace"/>
        </systemColor>
    </resources>
</document>
"#,
        config.project_name
    )
}

/// توليد Localizable.strings
pub fn generate_localizable(config: &MobileExportConfig) -> String {
    format!(
r#"/*
  Localizable.strings (Arabic)
  {}
*/

"app_name" = "{}";
"welcome" = "مرحباً بك";
"welcome_message" = "تطبيق مولد بلغة المرجع - لغة برمجة عربية متكاملة";
"run" = "تشغيل";
"output" = "النتيجة";
"settings" = "الإعدادات";
"loading" = "جاري التحميل...";
"#,
        config.project_name,
        config.project_name
    )
}
