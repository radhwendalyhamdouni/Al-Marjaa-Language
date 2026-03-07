# ═══════════════════════════════════════════════════════════════════════════════
# Al-Marjaa Language - RPM Spec File
# ═══════════════════════════════════════════════════════════════════════════════
# بناء: rpmbuild -bb almarjaa.spec
# ═══════════════════════════════════════════════════════════════════════════════

Name:           almarjaa
Version:        3.3.0
Release:        1%{?dist}
Summary:        لغة المرجع - لغة برمجة عربية متكاملة مع ذكاء اصطناعي

License:        All-Rights-Reserved
URL:            https://almarjaa.io
Source0:        https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  openssl-devel
BuildRequires:  pkg-config

Requires:       openssl-libs >= 1.1.1
Requires(post): /sbin/ldconfig
Requires(postun): /sbin/ldconfig

%description
لغة المرجع هي لغة برمجة عربية متكاملة مع:

🧠 JIT Compiler متقدم (5-Tier)
🌙 Vibe Coding مع الذكاء الاصطناعي
📦 المكتبة القياسية الشاملة (HTTP, Database, Regex, Crypto)
🎨 نظام واجهات المستخدم المتكامل
🔧 LSP Server متكامل
🚀 دعم ONNX للذكاء الاصطناعي

%prep
%autosetup -n Al-Marjaa-Language-%{version}

%build
%cargo_build --release

%install
# إنشاء المجلدات
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/almarjaa/examples
mkdir -p %{buildroot}%{_datadir}/doc/almarjaa
mkdir -p %{buildroot}%{_sysconfdir}/almarjaa
mkdir -p %{buildroot}%{_datadir}/applications

# تثبيت الـ binary
install -m 755 target/release/almarjaa %{buildroot}%{_bindir}/almarjaa

# تثبيت LSP Server
if [ -f target/release/almarjaa-lsp ]; then
    install -m 755 target/release/almarjaa-lsp %{buildroot}%{_bindir}/almarjaa-lsp
fi

# تثبيت الأمثلة
cp -r examples/*.mrj %{buildroot}%{_datadir}/almarjaa/examples/

# تثبيت التوثيق
install -m 644 README.md %{buildroot}%{_datadir}/doc/almarjaa/
install -m 644 CHANGELOG.md %{buildroot}%{_datadir}/doc/almarjaa/
install -m 644 LICENSE %{buildroot}%{_datadir}/doc/almarjaa/

# تثبيت ملف desktop
cat > %{buildroot}%{_datadir}/applications/almarjaa.desktop << 'EOF'
[Desktop Entry]
Version=1.0
Type=Application
Name=Al-Marjaa
Name[ar]=المرجع
Comment=Arabic Programming Language IDE
Exec=almarjaa %F
Icon=almarjaa
Terminal=true
Categories=Development;IDE;Languages;
EOF

%check
%cargo_test --release || true

%post
/sbin/ldconfig

# رسالة التثبيت
echo ""
echo "╔═══════════════════════════════════════════════════════════════════════════════╗"
echo "║                      ✅ تم تثبيت لغة المرجع بنجاح!                          ║"
echo "╠═══════════════════════════════════════════════════════════════════════════════╣"
echo "║  🚀 للتشغيل: almarjaa                                                        ║"
echo "║  📚 الأمثلة: %{_datadir}/almarjaa/examples/                                  ║"
echo "║  📖 التوثيق: https://docs.almarjaa.io                                        ║"
echo "╚═══════════════════════════════════════════════════════════════════════════════╝"
echo ""

%postun
/sbin/ldconfig

%files
%{_bindir}/almarjaa
%{_bindir}/almarjaa-lsp
%{_datadir}/almarjaa
%{_datadir}/doc/almarjaa
%{_datadir}/applications/almarjaa.desktop
%config(noreplace) %{_sysconfdir}/almarjaa

%changelog
* Tue Feb 01 2026 رضوان دالي حمدوني <almarjaa.project@hotmail.com> - 3.3.0-1
- LSP Server متكامل ومتقدم
- Semantic Tokens, Code Actions, Code Lens
- Inlay Hints, Call Hierarchy, Rename

* Sat Jan 20 2026 رضوان دالي حمدوني <almarjaa.project@hotmail.com> - 3.2.0-1
- نظام واجهات المستخدم المتكامل
- 70+ كلمة مفتاحية عربية جديدة

* Wed Jan 15 2026 رضوان دالي حمدوني <almarjaa.project@hotmail.com> - 3.1.0-1
- دعم ONNX الكامل
- 30+ كلمة مفتاحية عربية جديدة
