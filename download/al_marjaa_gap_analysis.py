#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from reportlab.lib.pagesizes import A4
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, PageBreak
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER, TA_LEFT, TA_RIGHT
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.pdfbase.pdfmetrics import registerFontFamily
from reportlab.lib.units import cm
import os

# Register fonts
pdfmetrics.registerFont(TTFont('SimHei', '/usr/share/fonts/truetype/chinese/SimHei.ttf'))
pdfmetrics.registerFont(TTFont('Times New Roman', '/usr/share/fonts/truetype/english/Times-New-Roman.ttf'))
pdfmetrics.registerFont(TTFont('DejaVuSans', '/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf'))

registerFontFamily('SimHei', normal='SimHei', bold='SimHei')
registerFontFamily('Times New Roman', normal='Times New Roman', bold='Times New Roman')

# Create document
doc = SimpleDocTemplate(
    "/home/z/my-project/download/Al_Marjaa_Gap_Analysis.pdf",
    pagesize=A4,
    rightMargin=2*cm,
    leftMargin=2*cm,
    topMargin=2*cm,
    bottomMargin=2*cm,
    title='Al_Marjaa_Gap_Analysis',
    author='Z.ai',
    creator='Z.ai',
    subject='تحليل فجوات لغة المرجع للاعتماد عليها حصرياً'
)

# Styles
styles = getSampleStyleSheet()

# Arabic title style (RTL - right aligned)
ar_title_style = ParagraphStyle(
    name='ArabicTitle',
    fontName='SimHei',
    fontSize=24,
    leading=32,
    alignment=TA_CENTER,
    textColor=colors.HexColor('#1F4E79'),
    spaceAfter=24
)

# Arabic heading style
ar_heading_style = ParagraphStyle(
    name='ArabicHeading',
    fontName='SimHei',
    fontSize=16,
    leading=22,
    alignment=TA_RIGHT,
    textColor=colors.HexColor('#1F4E79'),
    spaceBefore=18,
    spaceAfter=12
)

# Arabic subheading style
ar_subheading_style = ParagraphStyle(
    name='ArabicSubheading',
    fontName='SimHei',
    fontSize=13,
    leading=18,
    alignment=TA_RIGHT,
    textColor=colors.HexColor('#2E75B6'),
    spaceBefore=12,
    spaceAfter=8
)

# Arabic body style (RTL)
ar_body_style = ParagraphStyle(
    name='ArabicBody',
    fontName='SimHei',
    fontSize=11,
    leading=18,
    alignment=TA_RIGHT,
    spaceBefore=6,
    spaceAfter=8,
    wordWrap='CJK'
)

# English code style
en_code_style = ParagraphStyle(
    name='EnCode',
    fontName='DejaVuSans',
    fontSize=10,
    leading=14,
    alignment=TA_LEFT,
    textColor=colors.HexColor('#333333'),
    backColor=colors.HexColor('#F5F5F5'),
    spaceBefore=6,
    spaceAfter=6
)

# Table styles
header_style = ParagraphStyle(
    name='TableHeader',
    fontName='SimHei',
    fontSize=11,
    textColor=colors.white,
    alignment=TA_CENTER
)

cell_style = ParagraphStyle(
    name='TableCell',
    fontName='SimHei',
    fontSize=10,
    textColor=colors.black,
    alignment=TA_CENTER,
    wordWrap='CJK'
)

cell_style_right = ParagraphStyle(
    name='TableCellRight',
    fontName='SimHei',
    fontSize=10,
    textColor=colors.black,
    alignment=TA_RIGHT,
    wordWrap='CJK'
)

# Build content
story = []

# Cover page
story.append(Spacer(1, 60))
story.append(Paragraph("تحليل فجوات لغة المرجع", ar_title_style))
story.append(Spacer(1, 12))
story.append(Paragraph("ما ينقص اللغة للاعتماد عليها حصرياً", ParagraphStyle(
    name='Subtitle',
    fontName='SimHei',
    fontSize=16,
    leading=22,
    alignment=TA_CENTER,
    textColor=colors.HexColor('#666666')
)))
story.append(Spacer(1, 30))
story.append(Paragraph("Al-Marjaa Language Gap Analysis", ParagraphStyle(
    name='EnglishTitle',
    fontName='Times New Roman',
    fontSize=14,
    leading=18,
    alignment=TA_CENTER,
    textColor=colors.HexColor('#888888')
)))
story.append(Spacer(1, 50))

# Project info
info_data = [
    [Paragraph('<b>القيمة</b>', header_style), Paragraph('<b>العنصر</b>', header_style)],
    [Paragraph('3.2.0', cell_style), Paragraph('الإصدار الحالي', cell_style_right)],
    [Paragraph('RADHWEN DALY HAMDOUNI', cell_style), Paragraph('المؤلف', cell_style_right)],
    [Paragraph('Prototype/MVP', cell_style), Paragraph('مرحلة التطوير', cell_style_right)],
    [Paragraph('343 اختبار / 99.4%', cell_style), Paragraph('حالة الاختبارات', cell_style_right)],
]

info_table = Table(info_data, colWidths=[8*cm, 6*cm])
info_table.setStyle(TableStyle([
    ('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#1F4E79')),
    ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
    ('BACKGROUND', (0, 1), (-1, -1), colors.HexColor('#F5F5F5')),
    ('GRID', (0, 0), (-1, -1), 0.5, colors.grey),
    ('VALIGN', (0, 0), (-1, -1), 'MIDDLE'),
    ('LEFTPADDING', (0, 0), (-1, -1), 10),
    ('RIGHTPADDING', (0, 0), (-1, -1), 10),
    ('TOPPADDING', (0, 0), (-1, -1), 8),
    ('BOTTOMPADDING', (0, 0), (-1, -1), 8),
]))
story.append(info_table)

story.append(PageBreak())

# Section 1: Executive Summary
story.append(Paragraph("الملخص التنفيذي", ar_heading_style))

story.append(Paragraph(
    "لغة المرجع هي لغة برمجة عربية متكاملة مبنية بـ Rust، تتميز بمعمارية فريدة قابلة للتحويل لأي لغة بشرية (Language-Agnostic Architecture). "
    "حالياً في مرحلة Prototype/MVP مع إمكانيات إنتاجية أساسية ممتازة. "
    "المشروع يحتاج إلى تعزيزات جوهرية للوصول إلى مستوى 'بديل شامل قابل للاستخدام' يمكن الاعتماد عليه حصرياً في المشاريع الحقيقية.",
    ar_body_style
))

story.append(Paragraph(
    "العائق الرئيسي ليس الميزات اللغوية - فهي متقدمة جداً - بل البنية التحتية والمنظومة البيئية (Ecosystem). "
    "التقرير التالي يوضح الفجوات الحرجة والأولويات المطلوبة لتحويل اللغة من مشروع بحثي إلى أداة إنتاجية.",
    ar_body_style
))

story.append(Spacer(1, 18))

# Section 2: Current Strengths
story.append(Paragraph("نقاط القوة الحالية", ar_heading_style))

story.append(Paragraph(
    "تمتلك لغة المرجع أساساً تقنياً متقدماً جداً يتفوق على العديد من اللغات الناشئة. "
    "هذه النقاط القوية تشكل ركيزة صلبة للبناء عليها:",
    ar_body_style
))

strengths_data = [
    [Paragraph('<b>الحالة</b>', header_style), Paragraph('<b>التفاصيل</b>', header_style), Paragraph('<b>الميزة</b>', header_style)],
    [Paragraph('مكتمل', cell_style), Paragraph('5 مستويات (Tier 0-4) مع تسريع 5.08x', cell_style), Paragraph('JIT Compiler', cell_style_right)],
    [Paragraph('مكتمل', cell_style), Paragraph('جامع قمامة متوازي أجيالي', cell_style), Paragraph('Parallel GC', cell_style_right)],
    [Paragraph('مكتمل', cell_style), Paragraph('تحميل/تشغيل نماذج ML', cell_style), Paragraph('ONNX Runtime', cell_style_right)],
    [Paragraph('مكتمل', cell_style), Paragraph('دقة 79% للعربية', cell_style), Paragraph('Vibe Coding', cell_style_right)],
    [Paragraph('مكتمل', cell_style), Paragraph('30+ مكون جاهز', cell_style), Paragraph('UI Framework', cell_style_right)],
    [Paragraph('مكتمل', cell_style), Paragraph('استهداف الويب', cell_style), Paragraph('WebAssembly', cell_style_right)],
    [Paragraph('مكتمل', cell_style), Paragraph('دعم Async/Await', cell_style), Paragraph('البرمجة غير المتزامنة', cell_style_right)],
    [Paragraph('مكتمل', cell_style), Paragraph('ترجمة مسبقة للكود', cell_style), Paragraph('AOT Compilation', cell_style_right)],
    [Paragraph('مكتمل', cell_style), Paragraph('تحسين موجه بالتنميط', cell_style), Paragraph('Profile-Guided Optimization', cell_style_right)],
    [Paragraph('مكتمل', cell_style), Paragraph('استنباط تلقائي للأنواع', cell_style), Paragraph('Type Inference', cell_style_right)],
    [Paragraph('baseline', cell_style), Paragraph('نواة مدير حزم', cell_style), Paragraph('Package Manager', cell_style_right)],
    [Paragraph('baseline', cell_style), Paragraph('CLI bridge للـ IDE', cell_style), Paragraph('LSP Server', cell_style_right)],
    [Paragraph('مكتمل', cell_style), Paragraph('أدوات تنسيق وفحص الكود', cell_style), Paragraph('Formatter + Linter', cell_style_right)],
]

strengths_table = Table(strengths_data, colWidths=[2.5*cm, 7*cm, 5*cm])
strengths_table.setStyle(TableStyle([
    ('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#1F4E79')),
    ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
    ('BACKGROUND', (0, 1), (-1, 1), colors.white),
    ('BACKGROUND', (0, 2), (-1, 2), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 3), (-1, 3), colors.white),
    ('BACKGROUND', (0, 4), (-1, 4), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 5), (-1, 5), colors.white),
    ('BACKGROUND', (0, 6), (-1, 6), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 7), (-1, 7), colors.white),
    ('BACKGROUND', (0, 8), (-1, 8), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 9), (-1, 9), colors.white),
    ('BACKGROUND', (0, 10), (-1, 10), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 11), (-1, 11), colors.white),
    ('BACKGROUND', (0, 12), (-1, 12), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 13), (-1, 13), colors.white),
    ('BACKGROUND', (0, 14), (-1, 14), colors.HexColor('#F5F5F5')),
    ('GRID', (0, 0), (-1, -1), 0.5, colors.grey),
    ('VALIGN', (0, 0), (-1, -1), 'MIDDLE'),
    ('LEFTPADDING', (0, 0), (-1, -1), 8),
    ('RIGHTPADDING', (0, 0), (-1, -1), 8),
    ('TOPPADDING', (0, 0), (-1, -1), 6),
    ('BOTTOMPADDING', (0, 0), (-1, -1), 6),
]))
story.append(strengths_table)
story.append(Spacer(1, 6))
story.append(Paragraph("جدول 1: نقاط القوة الحالية للغة المرجع", ParagraphStyle(
    name='Caption',
    fontName='SimHei',
    fontSize=10,
    alignment=TA_CENTER,
    textColor=colors.HexColor('#666666')
)))

story.append(Spacer(1, 24))

# Section 3: Critical Gaps
story.append(Paragraph("الفجوات الحرجة المطلوبة", ar_heading_style))

story.append(Paragraph(
    "هذه الفجوات تمثل العناصر الأساسية التي تنقص اللغة للاعتماد عليها حصرياً في المشاريع الحقيقية. "
    "تم ترتيبها حسب الأولوية والأهمية:",
    ar_body_style
))

# 3.1 Standard Library
story.append(Paragraph("3.1 المكتبة القياسية (Standard Library)", ar_subheading_style))

story.append(Paragraph(
    "المكتبة القياسية هي العمود الفقري لأي لغة برمجة. لغة المرجع تحتاج مكتبة قياسية شاملة تغطي جميع الاحتياجات الأساسية للمطورين. "
    "حالياً تتوفر دوال أساسية للرياضيات والنصوص والقوائم والقواميس والشبكات والوقت والعشوائية وJSON، لكنها محدودة جداً مقارنة باللغات الناضجة.",
    ar_body_style
))

stdlib_data = [
    [Paragraph('<b>الأولوية</b>', header_style), Paragraph('<b>المثال/التفاصيل</b>', header_style), Paragraph('<b>المكون المطلوب</b>', header_style)],
    [Paragraph('حرجة', cell_style), Paragraph('GET/POST/PUT/DELETE، Headers، Cookies', cell_style), Paragraph('HTTP Client/Server', cell_style_right)],
    [Paragraph('حرجة', cell_style), Paragraph('MySQL, PostgreSQL, SQLite, MongoDB', cell_style), Paragraph('قواعد البيانات', cell_style_right)],
    [Paragraph('عالية', cell_style), Paragraph('File.watch(), File.copy(), File.move()', cell_style), Paragraph('معالجة الملفات المتقدمة', cell_style_right)],
    [Paragraph('عالية', cell_style), Paragraph('pattern matching, capture groups', cell_style), Paragraph('التعابير النمطية (Regex)', cell_style_right)],
    [Paragraph('عالية', cell_style), Paragraph('zip, gzip, tar, 7z', cell_style), Paragraph('الضغط وفك الضغط', cell_style_right)],
    [Paragraph('عالية', cell_style), Paragraph('AES, RSA, SHA, MD5', cell_style), Paragraph('التشفير والأمان', cell_style_right)],
    [Paragraph('متوسطة', cell_style), Paragraph('ImageSharp-like API', cell_style), Paragraph('معالجة الصور', cell_style_right)],
    [Paragraph('متوسطة', cell_style), Paragraph('Date parsing, Time zones', cell_style), Paragraph('معالجة التاريخ والوقت', cell_style_right)],
    [Paragraph('متوسطة', cell_style), Paragraph('Logging, Levels, Rotating files', cell_style), Paragraph('نظام السجلات', cell_style_right)],
]

stdlib_table = Table(stdlib_data, colWidths=[2.5*cm, 7*cm, 5*cm])
stdlib_table.setStyle(TableStyle([
    ('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#1F4E79')),
    ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
    ('BACKGROUND', (0, 1), (-1, 1), colors.white),
    ('BACKGROUND', (0, 2), (-1, 2), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 3), (-1, 3), colors.white),
    ('BACKGROUND', (0, 4), (-1, 4), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 5), (-1, 5), colors.white),
    ('BACKGROUND', (0, 6), (-1, 6), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 7), (-1, 7), colors.white),
    ('BACKGROUND', (0, 8), (-1, 8), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 9), (-1, 9), colors.white),
    ('GRID', (0, 0), (-1, -1), 0.5, colors.grey),
    ('VALIGN', (0, 0), (-1, -1), 'MIDDLE'),
    ('LEFTPADDING', (0, 0), (-1, -1), 8),
    ('RIGHTPADDING', (0, 0), (-1, -1), 8),
    ('TOPPADDING', (0, 0), (-1, -1), 6),
    ('BOTTOMPADDING', (0, 0), (-1, -1), 6),
]))
story.append(stdlib_table)
story.append(Spacer(1, 6))
story.append(Paragraph("جدول 2: مكونات المكتبة القياسية المطلوبة", ParagraphStyle(
    name='Caption',
    fontName='SimHei',
    fontSize=10,
    alignment=TA_CENTER,
    textColor=colors.HexColor('#666666')
)))

story.append(Spacer(1, 18))

# 3.2 Package Manager
story.append(Paragraph("3.2 نظام الحزم (Package Manager)", ar_subheading_style))

story.append(Paragraph(
    "نظام الحزم هو البوابة لبناء منظومة بيئية حية. حالياً يتوفر baseline لمدير الحزم مع mrj.toml وأوامر CLI أساسية، "
    "لكن المنظومة الكاملة تحتاج إلى:",
    ar_body_style
))

pkg_features = [
    "Registry مركزي للحزم: موقع رسمي لمشاركة واكتشاف الحزم (مثل crates.io أو npm)",
    "نشر الحزم: أمر mrj publish لنشر الحزم على الـ Registry",
    "حل التبعيات المتقدم: معالجة التضاربات والإصدارات بطريقة ذكية",
    "Lock file: mrj.lock لضمان تكرار البناء",
    "التوزيع الثنائي: تحميل الحزم المترجمة مسبقاً",
    "توقيع الحزم: التحقق من صحة الحزم",
]

for feature in pkg_features:
    story.append(Paragraph("- " + feature, ar_body_style))

story.append(Spacer(1, 12))

# 3.3 IDE Support
story.append(Paragraph("3.3 بيئة التطوير المتكاملة (IDE Support)", ar_subheading_style))

story.append(Paragraph(
    "تجربة المطور تعتمد بشكل كبير على أدوات التطوير. لغة المرجع تحتاج:",
    ar_body_style
))

ide_data = [
    [Paragraph('<b>الحالة الحالية</b>', header_style), Paragraph('<b>المتطلبات</b>', header_style), Paragraph('<b>الأداة</b>', header_style)],
    [Paragraph('baseline (CLI bridge)', cell_style), Paragraph('stateful server كامل', cell_style), Paragraph('LSP Server', cell_style_right)],
    [Paragraph('موجود (أساسي)', cell_style), Paragraph('IntelliJ Plugin, Vim/Neovim', cell_style), Paragraph('VS Code Extension', cell_style_right)],
    [Paragraph('غير موجود', cell_style), Paragraph('Breakpoints, Variables, Call Stack', cell_style), Paragraph('Debugger مرئي', cell_style_right)],
    [Paragraph('محدود', cell_style), Paragraph('Project templates, Auto-import', cell_style), Paragraph('Project Scaffolding', cell_style_right)],
    [Paragraph('محدود', cell_style), Paragraph('Test Explorer, Coverage Reports', cell_style), Paragraph('Testing Tools', cell_style_right)],
]

ide_table = Table(ide_data, colWidths=[4*cm, 6*cm, 4.5*cm])
ide_table.setStyle(TableStyle([
    ('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#1F4E79')),
    ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
    ('BACKGROUND', (0, 1), (-1, 1), colors.white),
    ('BACKGROUND', (0, 2), (-1, 2), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 3), (-1, 3), colors.white),
    ('BACKGROUND', (0, 4), (-1, 4), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 5), (-1, 5), colors.white),
    ('GRID', (0, 0), (-1, -1), 0.5, colors.grey),
    ('VALIGN', (0, 0), (-1, -1), 'MIDDLE'),
    ('LEFTPADDING', (0, 0), (-1, -1), 8),
    ('RIGHTPADDING', (0, 0), (-1, -1), 8),
    ('TOPPADDING', (0, 0), (-1, -1), 6),
    ('BOTTOMPADDING', (0, 0), (-1, -1), 6),
]))
story.append(ide_table)
story.append(Spacer(1, 6))
story.append(Paragraph("جدول 3: متطلبات أدوات التطوير", ParagraphStyle(
    name='Caption',
    fontName='SimHei',
    fontSize=10,
    alignment=TA_CENTER,
    textColor=colors.HexColor('#666666')
)))

story.append(Spacer(1, 18))

# 3.4 Documentation
story.append(Paragraph("3.4 التوثيق والتعلم (Documentation & Learning)", ar_subheading_style))

story.append(Paragraph(
    "التوثيق الجيد هو مفتاح تبني اللغة. المتطلبات تشمل:",
    ar_body_style
))

doc_features = [
    "دليل المبتدئين: دورة تعليمية خطوة بخطوة للقادمين الجدد",
    "مرجع API كامل: توثيق كل دالة وفئة مع أمثلة",
    "كتاب الطبخ (Cookbook): أمثلة عملية لحلول مشاكل شائعة",
    "دليل أفضل الممارسات: كيفية كتابة كود نظيف وفعال",
    "فيديوهات تعليمية: قناة يوتيوب مع دروس مصورة",
    "موقع تفاعلي: playground لتجربة الكود أونلاين",
    "نظام توليد التوثيق: أداة تلقائية لاستخراج التوثيق من الكود",
]

for feature in doc_features:
    story.append(Paragraph("- " + feature, ar_body_style))

story.append(Spacer(1, 12))

# 3.5 Community
story.append(Paragraph("3.5 المجتمع والدعم (Community & Support)", ar_subheading_style))

story.append(Paragraph(
    "لغة البرمجة لا تنجح بمفردها. بناء مجتمع نشط ضروري للاستدامة:",
    ar_body_style
))

community_features = [
    "منتدى رسمي: مساحة للنقاش والأسئلة وتبادل الخبرات",
    "قناة Discord/Telegram: دعم فوري وتواصل مباشر",
    "نظام GitHub Issues منظم: قوالب للإبلاغ عن الأخطاء وطلب الميزات",
    "Contributing Guide واضح: كيفية المساهمة في المشروع",
    "Good First Issues: مهام مناسبة للمساهمين الجدد",
    "Hacktoberfest: مشاركة في الفعاليات المفتوحة المصدر",
    "شهادات المساهمة: تقدير للمساهمين النشطين",
]

for feature in community_features:
    story.append(Paragraph("- " + feature, ar_body_style))

story.append(Spacer(1, 12))

# 3.6 Production Readiness
story.append(Paragraph("3.6 الجاهزية للإنتاج (Production Readiness)", ar_subheading_style))

story.append(Paragraph(
    "للاعتماد على اللغة في مشاريع حقيقية، يجب توفير:",
    ar_body_style
))

prod_data = [
    [Paragraph('<b>الأولوية</b>', header_style), Paragraph('<b>التفاصيل</b>', header_style), Paragraph('<b>المتطلب</b>', header_style)],
    [Paragraph('حرجة', cell_style), Paragraph('.deb, .rpm, .msi, .dmg, Homebrew', cell_style), Paragraph('حزم تثبيت رسمية', cell_style_right)],
    [Paragraph('حرجة', cell_style), Paragraph('Cross-compilation من Linux لكل المنصات', cell_style), Paragraph('بناء متعدد المنصات', cell_style_right)],
    [Paragraph('عالية', cell_style), Paragraph('Docker image رسمي', cell_style), Paragraph('دعم Docker', cell_style_right)],
    [Paragraph('عالية', cell_style), Paragraph('مرونة عالية مع أحمال كبيرة', cell_style), Paragraph('اختبارات الأداء', cell_style_right)],
    [Paragraph('عالية', cell_style), Paragraph('Security audit', cell_style), Paragraph('مراجعة أمنية', cell_style_right)],
    [Paragraph('متوسطة', cell_style), Paragraph('GitHub Actions, GitLab CI', cell_style), Paragraph('CI/CD Templates', cell_style_right)],
    [Paragraph('متوسطة', cell_style), Paragraph('Jenkins, Prometheus, Grafana', cell_style), Paragraph('مراقبة الإنتاج', cell_style_right)],
]

prod_table = Table(prod_data, colWidths=[2.5*cm, 7*cm, 5*cm])
prod_table.setStyle(TableStyle([
    ('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#1F4E79')),
    ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
    ('BACKGROUND', (0, 1), (-1, 1), colors.white),
    ('BACKGROUND', (0, 2), (-1, 2), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 3), (-1, 3), colors.white),
    ('BACKGROUND', (0, 4), (-1, 4), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 5), (-1, 5), colors.white),
    ('BACKGROUND', (0, 6), (-1, 6), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 7), (-1, 7), colors.white),
    ('GRID', (0, 0), (-1, -1), 0.5, colors.grey),
    ('VALIGN', (0, 0), (-1, -1), 'MIDDLE'),
    ('LEFTPADDING', (0, 0), (-1, -1), 8),
    ('RIGHTPADDING', (0, 0), (-1, -1), 8),
    ('TOPPADDING', (0, 0), (-1, -1), 6),
    ('BOTTOMPADDING', (0, 0), (-1, -1), 6),
]))
story.append(prod_table)
story.append(Spacer(1, 6))
story.append(Paragraph("جدول 4: متطلبات الجاهزية للإنتاج", ParagraphStyle(
    name='Caption',
    fontName='SimHei',
    fontSize=10,
    alignment=TA_CENTER,
    textColor=colors.HexColor('#666666')
)))

story.append(Spacer(1, 18))

# 3.7 Interoperability
story.append(Paragraph("3.7 التكامل مع الأنظمة الأخرى (Interoperability)", ar_subheading_style))

story.append(Paragraph(
    "للاعتماد على اللغة حصرياً، يجب أن تتكامل مع الأنظمة الموجودة:",
    ar_body_style
))

interop_features = [
    "FFI (Foreign Function Interface): استدعاء دوال من C/C++ و Rust",
    "Python Interop: مكتبة Python للتكامل مع مشاريع Python موجودة",
    "JavaScript Interop: تكامل مع Node.js والمتصفحات",
    "C Binding: تصدير واجهة C للتكامل مع أي لغة",
    "JSON-RPC / gRPC: خدمات RPC للتكامل مع الخدمات المصغرة",
    "REST API Generator: توليد تلقائي لواجهات REST",
]

for feature in interop_features:
    story.append(Paragraph("- " + feature, ar_body_style))

story.append(Spacer(1, 12))

# 3.8 Mobile & Embedded
story.append(Paragraph("3.8 دعم المنصات الإضافية", ar_subheading_style))

story.append(Paragraph(
    "للوصول لشرائح أوسع من المطورين:",
    ar_body_style
))

platform_features = [
    "Android: SDK للتطبيقات الأندرويد مع تكامل Kotlin/Java",
    "iOS: إطار عمل للتكامل مع Swift/Objective-C",
    "Embedded Systems: دعم الأنظمة المدمجة (Arduino, ESP32, Raspberry Pi)",
    "Desktop Apps: إطار عمل للتطبيقات المكتبية عبر WebAssembly أو Native",
]

for feature in platform_features:
    story.append(Paragraph("- " + feature, ar_body_style))

story.append(PageBreak())

# Section 4: Priority Matrix
story.append(Paragraph("مصفوفة الأولويات", ar_heading_style))

story.append(Paragraph(
    "بناءً على التحليل السابق، إليك مصفوفة الأولويات المقترحة للتنفيذ:",
    ar_body_style
))

priority_data = [
    [Paragraph('<b>الإطار الزمني</b>', header_style), Paragraph('<b>المهام الرئيسية</b>', header_style), Paragraph('<b>الأولوية</b>', header_style)],
    [Paragraph('Q2 2026', cell_style), Paragraph('إكمال Standard Library + حزم التثبيت', cell_style), Paragraph('حرجة', cell_style_right)],
    [Paragraph('Q3 2026', cell_style), Paragraph('Package Manager كامل + LSP Server', cell_style), Paragraph('حرجة', cell_style_right)],
    [Paragraph('Q4 2026', cell_style), Paragraph('Documentation + Community Building', cell_style), Paragraph('عالية', cell_style_right)],
    [Paragraph('Q1 2027', cell_style), Paragraph('IDE Plugins + Debugger', cell_style), Paragraph('عالية', cell_style_right)],
    [Paragraph('Q2 2027', cell_style), Paragraph('Mobile Support (Android/iOS)', cell_style), Paragraph('متوسطة', cell_style_right)],
    [Paragraph('Q3-Q4 2027', cell_style), Paragraph('Enterprise Features + LTS', cell_style), Paragraph('متوسطة', cell_style_right)],
]

priority_table = Table(priority_data, colWidths=[3*cm, 8*cm, 3.5*cm])
priority_table.setStyle(TableStyle([
    ('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#1F4E79')),
    ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
    ('BACKGROUND', (0, 1), (-1, 1), colors.white),
    ('BACKGROUND', (0, 2), (-1, 2), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 3), (-1, 3), colors.white),
    ('BACKGROUND', (0, 4), (-1, 4), colors.HexColor('#F5F5F5')),
    ('BACKGROUND', (0, 5), (-1, 5), colors.white),
    ('BACKGROUND', (0, 6), (-1, 6), colors.HexColor('#F5F5F5')),
    ('GRID', (0, 0), (-1, -1), 0.5, colors.grey),
    ('VALIGN', (0, 0), (-1, -1), 'MIDDLE'),
    ('LEFTPADDING', (0, 0), (-1, -1), 8),
    ('RIGHTPADDING', (0, 0), (-1, -1), 8),
    ('TOPPADDING', (0, 0), (-1, -1), 6),
    ('BOTTOMPADDING', (0, 0), (-1, -1), 6),
]))
story.append(priority_table)
story.append(Spacer(1, 6))
story.append(Paragraph("جدول 5: مصفوفة الأولويات الزمنية", ParagraphStyle(
    name='Caption',
    fontName='SimHei',
    fontSize=10,
    alignment=TA_CENTER,
    textColor=colors.HexColor('#666666')
)))

story.append(Spacer(1, 24))

# Section 5: Conclusion
story.append(Paragraph("الخلاصة والتوصيات", ar_heading_style))

story.append(Paragraph(
    "لغة المرجع تمتلك أساساً تقنياً استثنائياً يضعها في وضع فريد بين لغات البرمجة الناشئة. "
    "المعمارية القابلة للتحويل لأي لغة بشرية (Language-Agnostic Architecture) والكلمات المفتاحية كملفات تكوين (Keywords as Configuration) "
    "تجعلها قابلة للتوسع والانتشار عالمياً.",
    ar_body_style
))

story.append(Paragraph(
    "الفجوات المحددة في هذا التقرير هي فجوات 'منظومة بيئية' وليست فجوات 'لغوية'. "
    "اللغة نفسها قوية ومبتكرة، لكن المنظومة المحيطة بها تحتاج إلى تطوير مكثف.",
    ar_body_style
))

story.append(Paragraph("التوصيات الرئيسية:", ar_subheading_style))

recommendations = [
    "التركيز على المكتبة القياسية أولاً: هذا هو العامل الأهم لتبني المطورين",
    "بناء Registry للحزم: لخلق منظومة بيئية حية ومستدامة",
    "توثيق شامل بالعربية والإنجليزية: لجذب مجتمع عالمي",
    "حزم تثبيت سهلة: تقليل عوائق الدخول للمستخدمين الجدد",
    "بناء مجتمع نشط: المجتمع هو ضمان استمرارية المشروع",
]

for i, rec in enumerate(recommendations, 1):
    story.append(Paragraph(f"{i}. " + rec, ar_body_style))

story.append(Spacer(1, 18))

story.append(Paragraph(
    "مع الالتزام بخارطة الطريق المحددة، يمكن للغة المرجع أن تصبح خياراً حقيقياً للمطورين والشركات "
    "في غضون 12-18 شهراً، وتصل إلى مرحلة النضج والإنتاجية الكاملة خلال 24-36 شهراً.",
    ar_body_style
))

# Build PDF
doc.build(story)
print("PDF created successfully!")
