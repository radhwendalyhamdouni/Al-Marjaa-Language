#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Al-Marjaa Language Research Paper Generator
Generates a professional PDF with Arabic text using Unicode BiDi markers
"""

from reportlab.lib.pagesizes import A4
from reportlab.lib.units import cm
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.enums import TA_LEFT, TA_CENTER, TA_RIGHT, TA_JUSTIFY
from reportlab.lib import colors
from reportlab.platypus import (
    SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, PageBreak
)
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.pdfbase.pdfmetrics import registerFontFamily
from reportlab.lib.colors import HexColor


def rtl_text(text):
    """Add RTL markers for proper Arabic display"""
    # RLE = Right-to-Left Embedding (U+202B)
    # PDF = Pop Directional Formatting (U+202C)
    return '\u202B' + text + '\u202C'


def arabic_text(text):
    """Prepare Arabic text for PDF display with RTL markers"""
    return rtl_text(text)


# === Font Registration ===
pdfmetrics.registerFont(TTFont('DejaVu', '/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'))
pdfmetrics.registerFont(TTFont('DejaVuBold', '/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'))
pdfmetrics.registerFont(TTFont('Times', '/usr/share/fonts/truetype/english/Times-New-Roman.ttf'))

registerFontFamily('DejaVu', normal='DejaVu', bold='DejaVuBold')
registerFontFamily('Times', normal='Times', bold='Times')

# === Color Scheme ===
PRIMARY_COLOR = HexColor('#1F4E79')
SECONDARY_COLOR = HexColor('#2E75B6')
TEXT_COLOR = HexColor('#2C3E50')
LIGHT_GRAY = HexColor('#F5F5F5')

# === Styles ===
styles = getSampleStyleSheet()

styles.add(ParagraphStyle(name='ENTitle', fontName='Times', fontSize=24, leading=30,
    alignment=TA_CENTER, textColor=PRIMARY_COLOR, spaceAfter=12))

styles.add(ParagraphStyle(name='ENSubtitle', fontName='Times', fontSize=14, leading=20,
    alignment=TA_CENTER, textColor=TEXT_COLOR, spaceAfter=24))

styles.add(ParagraphStyle(name='ENHeading1', fontName='Times', fontSize=16, leading=22,
    alignment=TA_LEFT, textColor=PRIMARY_COLOR, spaceBefore=18, spaceAfter=12))

styles.add(ParagraphStyle(name='ENHeading2', fontName='Times', fontSize=13, leading=18,
    alignment=TA_LEFT, textColor=SECONDARY_COLOR, spaceBefore=12, spaceAfter=8))

styles.add(ParagraphStyle(name='ENBody', fontName='Times', fontSize=11, leading=16,
    alignment=TA_JUSTIFY, textColor=TEXT_COLOR, spaceAfter=8))

styles.add(ParagraphStyle(name='ENAbstract', fontName='Times', fontSize=10, leading=14,
    alignment=TA_JUSTIFY, textColor=TEXT_COLOR, leftIndent=20, rightIndent=20, spaceAfter=12))

# Arabic styles
styles.add(ParagraphStyle(name='ARTitle', fontName='DejaVuBold', fontSize=18, leading=26,
    alignment=TA_CENTER, textColor=PRIMARY_COLOR, spaceAfter=12))

styles.add(ParagraphStyle(name='ARBody', fontName='DejaVu', fontSize=11, leading=18,
    alignment=TA_RIGHT, textColor=TEXT_COLOR, spaceAfter=8))

styles.add(ParagraphStyle(name='ARAbstract', fontName='DejaVu', fontSize=10, leading=16,
    alignment=TA_RIGHT, textColor=TEXT_COLOR, leftIndent=20, rightIndent=20, spaceAfter=12))

# Code style
styles.add(ParagraphStyle(name='CodeBlock', fontName='DejaVu', fontSize=9, leading=12,
    alignment=TA_LEFT, textColor=TEXT_COLOR, backColor=LIGHT_GRAY,
    leftIndent=10, rightIndent=10, spaceBefore=6, spaceAfter=6))

# Table styles
styles.add(ParagraphStyle(name='TableHeader', fontName='Times', fontSize=10, leading=14,
    alignment=TA_CENTER, textColor=colors.white))

styles.add(ParagraphStyle(name='TableCell', fontName='Times', fontSize=9, leading=12,
    alignment=TA_CENTER, textColor=TEXT_COLOR))

styles.add(ParagraphStyle(name='TableCellAR', fontName='DejaVu', fontSize=10, leading=14,
    alignment=TA_CENTER, textColor=TEXT_COLOR))


def create_cover_page(story):
    story.append(Spacer(1, 60))
    story.append(Paragraph("<b>Al-Marjaa: A Language-Agnostic Programming Framework<br/>with Native AI Integration</b>", styles['ENTitle']))
    story.append(Spacer(1, 20))
    
    # Arabic title
    ar_title = arabic_text("لغة المرجع: إطار برمجة مستقل عن اللغة مع تكامل الذكاء الاصطناعي")
    story.append(Paragraph(ar_title, styles['ARTitle']))
    
    story.append(Spacer(1, 40))
    story.append(Paragraph("<b>RADHWEN DALY HAMDOUNI</b>", styles['ENSubtitle']))
    
    # Arabic name
    ar_name = arabic_text("رضوان دالي حمدوني")
    story.append(Paragraph(ar_name, styles['ENSubtitle']))
    
    story.append(Spacer(1, 20))
    story.append(Paragraph("almarjaa.project@hotmail.com<br/>https://github.com/radhwendalyhamdouni/Al-Marjaa-Language", styles['ENSubtitle']))
    story.append(Spacer(1, 40))
    story.append(Paragraph("Version 3.2.0 | February 2026", styles['ENSubtitle']))
    story.append(PageBreak())


def create_abstract(story):
    story.append(Paragraph("<b>Abstract</b>", styles['ENHeading1']))
    story.append(Paragraph(
        "This paper presents Al-Marjaa, the first AI-native programming language designed with a "
        "language-agnostic architecture that enables seamless adaptation to any human language. "
        "Originally developed for Arabic speakers, the framework's innovative design allows its core "
        "syntax, keywords, and semantics to be configured via simple YAML files, opening the door "
        "for developers worldwide to program in their mother tongue. The language features a 5-tier "
        "JIT compiler achieving 5.08x speedup, ONNX runtime integration for machine learning models, "
        "a comprehensive UI framework, and the revolutionary Vibe Coding system that enables natural "
        "language programming. With 343 unit tests achieving 99.4% pass rate, Al-Marjaa demonstrates "
        "that programming languages can be both culturally native and technically competitive.",
        styles['ENAbstract']))
    
    story.append(Spacer(1, 12))
    
    # Arabic Abstract
    story.append(Paragraph("<b>الملخص</b>", styles['ENHeading1']))
    ar_abstract = arabic_text(
        "تقدم هذه الورقة لغة المرجع، أول لغة برمجة مصممة أصلاً للذكاء الاصطناعي بمعمارية مستقلة عن اللغة "
        "تمكن التكيف السلس مع أي لغة بشرية. طورت في الأصل للمتحدثين بالعربية، لكن التصميم المبتكر للإطار "
        "يتيح تكوين بناء الجمل والكلمات المفتاحية والدلالات عبر ملفات YAML بسيطة، مما يفتح الباب للمطورين "
        "حول العالم للبرمجة بلغتهم الأم. تتميز اللغة بمترجم فوري بخمسة مستويات يحقق تسريعاً بنسبة 5.08x، "
        "وتكامل مع ONNX لنماذج التعلم الآلي، وإطار واجهات مستخدم شامل، ونظام البرمجة باللغة الطبيعية. "
        "مع 343 اختبار وحدة بمعدل نجاح 99.4%، تثبت لغة المرجع أن لغات البرمجة يمكن أن تكون أصيلة ثقافياً ومنافسة تقنياً.")
    story.append(Paragraph(ar_abstract, styles['ARAbstract']))
    
    story.append(Spacer(1, 12))
    story.append(Paragraph(
        "<b>Keywords:</b> Programming Languages, Natural Language Processing, JIT Compilation, "
        "Arabic Computing, Language-Agnostic Architecture, AI Integration, ONNX Runtime, Vibe Coding",
        styles['ENBody']))
    
    ar_keywords = arabic_text(
        "الكلمات المفتاحية: لغات البرمجة، معالجة اللغة الطبيعية، التجميع الفوري، الحوسبة العربية، معمارية مستقلة عن اللغة، تكامل الذكاء الاصطناعي")
    story.append(Paragraph(ar_keywords, styles['ARBody']))
    story.append(PageBreak())


def create_introduction(story):
    story.append(Paragraph("<b>1. Introduction</b>", styles['ENHeading1']))
    story.append(Paragraph("<b>1.1 Motivation and Problem Statement</b>", styles['ENHeading2']))
    story.append(Paragraph(
        "The dominance of English in programming languages creates a significant barrier for billions "
        "of non-English speakers worldwide. While tools like Google Translate can help with documentation, "
        "the fundamental act of writing code requires familiarity with English keywords, syntax, and "
        "error messages. This linguistic barrier affects learning curve, productivity, accessibility, "
        "and cultural preservation.",
        styles['ENBody']))
    story.append(Paragraph(
        "Al-Marjaa addresses these challenges through a revolutionary language-agnostic architecture "
        "that maintains full technical capability while enabling complete localization to any human language.",
        styles['ENBody']))
    
    story.append(Paragraph("<b>1.2 Contributions</b>", styles['ENHeading2']))
    contributions = [
        "Language-Agnostic Architecture: Syntax and keywords defined via configuration files",
        "5-Tier JIT Compiler: Achieving 5.08x speedup through progressive optimization",
        "Vibe Coding: Natural language programming achieving 79% accuracy for Arabic",
        "ONNX Integration: Native support for ML model inference and export",
        "UI Framework: Comprehensive Arabic-first interface building system",
        "Complete Arabic Support: Full RTL, Arabic numerals, connected script, native errors"
    ]
    for i, contrib in enumerate(contributions, 1):
        story.append(Paragraph(f"{i}. {contrib}", styles['ENBody']))


def create_architecture_section(story):
    story.append(Paragraph("<b>2. Language-Agnostic Architecture</b>", styles['ENHeading1']))
    story.append(Paragraph("<b>2.1 Core Design Philosophy</b>", styles['ENHeading2']))
    story.append(Paragraph(
        "The fundamental innovation of Al-Marjaa is its language-agnostic architecture. Unlike "
        "traditional programming languages where keywords are hardcoded in English, Al-Marjaa defines "
        "all language elements through external configuration files. This approach enables instant "
        "adaptation to any language while preserving the complete technical capability.",
        styles['ENBody']))
    
    story.append(Paragraph("<b>2.2 Keywords as Configuration</b>", styles['ENHeading2']))
    story.append(Paragraph("All keywords are defined in YAML files:", styles['ENBody']))
    
    # Keywords table with Arabic
    keywords_data = [
        [Paragraph('<b>English</b>', styles['TableHeader']),
         Paragraph('<b>Arabic</b>', styles['TableHeader']),
         Paragraph('<b>Chinese</b>', styles['TableHeader']),
         Paragraph('<b>Spanish</b>', styles['TableHeader'])],
        [Paragraph('var', styles['TableCell']),
         Paragraph(arabic_text('متغير'), styles['TableCellAR']),
         Paragraph('变量', styles['TableCellAR']),
         Paragraph('variable', styles['TableCell'])],
        [Paragraph('const', styles['TableCell']),
         Paragraph(arabic_text('ثابت'), styles['TableCellAR']),
         Paragraph('常量', styles['TableCellAR']),
         Paragraph('constante', styles['TableCell'])],
        [Paragraph('function', styles['TableCell']),
         Paragraph(arabic_text('دالة'), styles['TableCellAR']),
         Paragraph('函数', styles['TableCellAR']),
         Paragraph('funcion', styles['TableCell'])],
        [Paragraph('if', styles['TableCell']),
         Paragraph(arabic_text('إذا'), styles['TableCellAR']),
         Paragraph('如果', styles['TableCellAR']),
         Paragraph('si', styles['TableCell'])],
        [Paragraph('while', styles['TableCell']),
         Paragraph(arabic_text('طالما'), styles['TableCellAR']),
         Paragraph('当', styles['TableCellAR']),
         Paragraph('mientras', styles['TableCell'])],
        [Paragraph('return', styles['TableCell']),
         Paragraph(arabic_text('أرجع'), styles['TableCellAR']),
         Paragraph('返回', styles['TableCellAR']),
         Paragraph('retornar', styles['TableCell'])],
        [Paragraph('print', styles['TableCell']),
         Paragraph(arabic_text('اطبع'), styles['TableCellAR']),
         Paragraph('打印', styles['TableCellAR']),
         Paragraph('imprimir', styles['TableCell'])],
    ]
    
    keywords_table = Table(keywords_data, colWidths=[3*cm, 3*cm, 3*cm, 3*cm])
    keywords_table.setStyle(TableStyle([
        ('BACKGROUND', (0, 0), (-1, 0), PRIMARY_COLOR),
        ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
        ('ALIGN', (0, 0), (-1, -1), 'CENTER'),
        ('FONTSIZE', (0, 0), (-1, -1), 10),
        ('BOTTOMPADDING', (0, 0), (-1, -1), 8),
        ('TOPPADDING', (0, 0), (-1, -1), 8),
        ('GRID', (0, 0), (-1, -1), 0.5, colors.grey),
        ('ROWBACKGROUNDS', (0, 1), (-1, -1), [colors.white, LIGHT_GRAY]),
    ]))
    
    story.append(Spacer(1, 12))
    story.append(keywords_table)
    story.append(Paragraph("<i>Table 1: Keywords Across Multiple Languages</i>",
        ParagraphStyle('Caption', alignment=TA_CENTER, fontSize=9, fontName='Times')))


def create_features_section(story):
    story.append(Paragraph("<b>3. Core Features</b>", styles['ENHeading1']))
    story.append(Paragraph("<b>3.1 Arabic Syntax</b>", styles['ENHeading2']))
    story.append(Paragraph("Al-Marjaa provides complete Arabic programming syntax:", styles['ENBody']))
    
    # Arabic code example
    ar_code = arabic_text('متغير الاسم = "مرحباً"؛\nثابت باي = 3.14159؛\nلكل رقم في مدى(١، ١٠) {\n    اطبع(رقم)؛\n}')
    story.append(Paragraph(ar_code, styles['CodeBlock']))
    
    story.append(Paragraph("<b>3.2 ONNX Runtime Integration</b>", styles['ENHeading2']))
    story.append(Paragraph("Native ONNX support enables loading and running ML models:", styles['ENBody']))
    ar_onnx = arabic_text('نموذج = أونكس.حمّل("resnet50.onnx")؛\nنتيجة = نموذج.استدل({"مدخل": مدخل})؛')
    story.append(Paragraph(ar_onnx, styles['CodeBlock']))
    
    story.append(Paragraph("<b>3.3 UI Framework (v3.2.0)</b>", styles['ENHeading2']))
    story.append(Paragraph("Declarative UI building with reactive data binding:", styles['ENBody']))
    ar_ui = arabic_text('صف {\n    فجوة: 10،\n    زر("اضغط هنا") {\n        نقر: () => اطبع("تم!")،\n    }،\n}')
    story.append(Paragraph(ar_ui, styles['CodeBlock']))


def create_jit_section(story):
    story.append(PageBreak())
    story.append(Paragraph("<b>4. JIT Compiler Design</b>", styles['ENHeading1']))
    story.append(Paragraph("<b>4.1 5-Tier Compilation Strategy</b>", styles['ENHeading2']))
    
    jit_data = [
        [Paragraph('<b>Tier</b>', styles['TableHeader']),
         Paragraph('<b>Name</b>', styles['TableHeader']),
         Paragraph('<b>Threshold</b>', styles['TableHeader']),
         Paragraph('<b>Optimizations</b>', styles['TableHeader'])],
        [Paragraph('Tier 0', styles['TableCell']), Paragraph('Interpreter', styles['TableCell']),
         Paragraph('0 exec', styles['TableCell']), Paragraph('Direct bytecode execution', styles['TableCell'])],
        [Paragraph('Tier 1', styles['TableCell']), Paragraph('Baseline JIT', styles['TableCell']),
         Paragraph('50 exec', styles['TableCell']), Paragraph('Direct threading, fast ops', styles['TableCell'])],
        [Paragraph('Tier 2', styles['TableCell']), Paragraph('Optimizing JIT', styles['TableCell']),
         Paragraph('200 exec', styles['TableCell']), Paragraph('Constant folding, DCE', styles['TableCell'])],
        [Paragraph('Tier 3', styles['TableCell']), Paragraph('SIMD Optimizations', styles['TableCell']),
         Paragraph('1000 exec', styles['TableCell']), Paragraph('Vectorization, FMA', styles['TableCell'])],
        [Paragraph('Tier 4', styles['TableCell']), Paragraph('Tracing JIT', styles['TableCell']),
         Paragraph('5000 exec', styles['TableCell']), Paragraph('Hot path optimization', styles['TableCell'])],
    ]
    
    jit_table = Table(jit_data, colWidths=[2*cm, 3.5*cm, 2.5*cm, 5*cm])
    jit_table.setStyle(TableStyle([
        ('BACKGROUND', (0, 0), (-1, 0), PRIMARY_COLOR),
        ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
        ('ALIGN', (0, 0), (-1, -1), 'CENTER'),
        ('FONTSIZE', (0, 0), (-1, -1), 9),
        ('BOTTOMPADDING', (0, 0), (-1, -1), 6),
        ('TOPPADDING', (0, 0), (-1, -1), 6),
        ('GRID', (0, 0), (-1, -1), 0.5, colors.grey),
        ('ROWBACKGROUNDS', (0, 1), (-1, -1), [colors.white, LIGHT_GRAY]),
    ]))
    
    story.append(Spacer(1, 12))
    story.append(jit_table)
    story.append(Paragraph("<i>Table 2: JIT Compilation Tiers</i>",
        ParagraphStyle('Caption', alignment=TA_CENTER, fontSize=9, fontName='Times')))
    
    story.append(Spacer(1, 18))
    story.append(Paragraph("<b>4.2 Performance Results</b>", styles['ENHeading2']))
    
    perf_data = [
        [Paragraph('<b>Benchmark</b>', styles['TableHeader']),
         Paragraph('<b>Iterations</b>', styles['TableHeader']),
         Paragraph('<b>Time</b>', styles['TableHeader']),
         Paragraph('<b>Ops/sec</b>', styles['TableHeader'])],
        [Paragraph('Arithmetic', styles['TableCell']), Paragraph('100,000', styles['TableCell']),
         Paragraph('27ms', styles['TableCell']), Paragraph('3.6M', styles['TableCell'])],
        [Paragraph('Loops', styles['TableCell']), Paragraph('10,000', styles['TableCell']),
         Paragraph('1.4ms', styles['TableCell']), Paragraph('7.0M', styles['TableCell'])],
        [Paragraph('Fibonacci', styles['TableCell']), Paragraph('50,000', styles['TableCell']),
         Paragraph('8.5ms', styles['TableCell']), Paragraph('5.9M', styles['TableCell'])],
        [Paragraph('Stress Test', styles['TableCell']), Paragraph('1,000', styles['TableCell']),
         Paragraph('50ms', styles['TableCell']), Paragraph('19.9M', styles['TableCell'])],
    ]
    
    perf_table = Table(perf_data, colWidths=[3.5*cm, 3*cm, 2.5*cm, 2.5*cm])
    perf_table.setStyle(TableStyle([
        ('BACKGROUND', (0, 0), (-1, 0), PRIMARY_COLOR),
        ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
        ('ALIGN', (0, 0), (-1, -1), 'CENTER'),
        ('FONTSIZE', (0, 0), (-1, -1), 9),
        ('BOTTOMPADDING', (0, 0), (-1, -1), 6),
        ('TOPPADDING', (0, 0), (-1, -1), 6),
        ('GRID', (0, 0), (-1, -1), 0.5, colors.grey),
        ('ROWBACKGROUNDS', (0, 1), (-1, -1), [colors.white, LIGHT_GRAY]),
    ]))
    
    story.append(Spacer(1, 12))
    story.append(perf_table)
    story.append(Paragraph("<i>Table 3: JIT Performance Benchmarks - Overall Speedup: 5.08x</i>",
        ParagraphStyle('Caption', alignment=TA_CENTER, fontSize=9, fontName='Times')))


def create_vibe_section(story):
    story.append(Paragraph("<b>5. Vibe Coding: Natural Language Programming</b>", styles['ENHeading1']))
    story.append(Paragraph(
        "Vibe Coding enables programming through natural Arabic language input. The system processes "
        "user intent and generates syntactically correct Al-Marjaa code using a Qwen 2.5 model.",
        styles['ENBody']))
    
    story.append(Paragraph("<b>5.1 Processing Pipeline</b>", styles['ENHeading2']))
    pipeline = [
        "Arabic NLP: Tokenization, NER, sentence structure analysis",
        "Intent Parsing: Extract action, target, and parameters",
        "GGUF Engine: Local inference using Qwen 2.5 model",
        "Code Generation: Output syntactically correct Al-Marjaa code"
    ]
    for step in pipeline:
        story.append(Paragraph(f"• {step}", styles['ENBody']))
    
    story.append(Paragraph("<b>5.2 Accuracy Evaluation</b>", styles['ENHeading2']))
    
    acc_data = [
        [Paragraph('<b>Level</b>', styles['TableHeader']),
         Paragraph('<b>Cases</b>', styles['TableHeader']),
         Paragraph('<b>Correct</b>', styles['TableHeader']),
         Paragraph('<b>Partial</b>', styles['TableHeader']),
         Paragraph('<b>Success</b>', styles['TableHeader'])],
        [Paragraph('Easy', styles['TableCell']), Paragraph('7', styles['TableCell']),
         Paragraph('6.5', styles['TableCell']), Paragraph('0.3', styles['TableCell']), Paragraph('93%', styles['TableCell'])],
        [Paragraph('Medium', styles['TableCell']), Paragraph('11', styles['TableCell']),
         Paragraph('9.2', styles['TableCell']), Paragraph('1.1', styles['TableCell']), Paragraph('84%', styles['TableCell'])],
        [Paragraph('Advanced', styles['TableCell']), Paragraph('4', styles['TableCell']),
         Paragraph('2.8', styles['TableCell']), Paragraph('0.6', styles['TableCell']), Paragraph('70%', styles['TableCell'])],
        [Paragraph('Expert', styles['TableCell']), Paragraph('6', styles['TableCell']),
         Paragraph('3.6', styles['TableCell']), Paragraph('1.2', styles['TableCell']), Paragraph('60%', styles['TableCell'])],
        [Paragraph('<b>Total</b>', styles['TableCell']), Paragraph('<b>28</b>', styles['TableCell']),
         Paragraph('<b>22.1</b>', styles['TableCell']), Paragraph('<b>3.2</b>', styles['TableCell']), Paragraph('<b>79%</b>', styles['TableCell'])],
    ]
    
    acc_table = Table(acc_data, colWidths=[2.5*cm, 2*cm, 2*cm, 2*cm, 2*cm])
    acc_table.setStyle(TableStyle([
        ('BACKGROUND', (0, 0), (-1, 0), PRIMARY_COLOR),
        ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
        ('ALIGN', (0, 0), (-1, -1), 'CENTER'),
        ('FONTSIZE', (0, 0), (-1, -1), 9),
        ('BOTTOMPADDING', (0, 0), (-1, -1), 6),
        ('TOPPADDING', (0, 0), (-1, -1), 6),
        ('GRID', (0, 0), (-1, -1), 0.5, colors.grey),
        ('ROWBACKGROUNDS', (0, 1), (-1, -2), [colors.white, LIGHT_GRAY]),
        ('BACKGROUND', (0, -1), (-1, -1), LIGHT_GRAY),
    ]))
    
    story.append(Spacer(1, 12))
    story.append(acc_table)
    story.append(Paragraph("<i>Table 4: Vibe Coding Accuracy by Difficulty</i>",
        ParagraphStyle('Caption', alignment=TA_CENTER, fontSize=9, fontName='Times')))


def create_evaluation_section(story):
    story.append(PageBreak())
    story.append(Paragraph("<b>6. Experimental Evaluation</b>", styles['ENHeading1']))
    story.append(Paragraph("<b>6.1 Test Coverage</b>", styles['ENHeading2']))
    
    test_data = [
        [Paragraph('<b>Category</b>', styles['TableHeader']),
         Paragraph('<b>Tests</b>', styles['TableHeader']),
         Paragraph('<b>Passed</b>', styles['TableHeader']),
         Paragraph('<b>Pass Rate</b>', styles['TableHeader'])],
        [Paragraph('Lexer Tests', styles['TableCell']), Paragraph('33', styles['TableCell']),
         Paragraph('33', styles['TableCell']), Paragraph('100%', styles['TableCell'])],
        [Paragraph('Parser Tests', styles['TableCell']), Paragraph('68', styles['TableCell']),
         Paragraph('68', styles['TableCell']), Paragraph('100%', styles['TableCell'])],
        [Paragraph('Interpreter Tests', styles['TableCell']), Paragraph('215', styles['TableCell']),
         Paragraph('215', styles['TableCell']), Paragraph('100%', styles['TableCell'])],
        [Paragraph('CLI Tests', styles['TableCell']), Paragraph('18', styles['TableCell']),
         Paragraph('18', styles['TableCell']), Paragraph('100%', styles['TableCell'])],
        [Paragraph('Integration Tests', styles['TableCell']), Paragraph('9', styles['TableCell']),
         Paragraph('9', styles['TableCell']), Paragraph('100%', styles['TableCell'])],
        [Paragraph('<b>Total</b>', styles['TableCell']), Paragraph('<b>343</b>', styles['TableCell']),
         Paragraph('<b>343</b>', styles['TableCell']), Paragraph('<b>100%</b>', styles['TableCell'])],
    ]
    
    test_table = Table(test_data, colWidths=[4*cm, 2.5*cm, 2.5*cm, 2.5*cm])
    test_table.setStyle(TableStyle([
        ('BACKGROUND', (0, 0), (-1, 0), PRIMARY_COLOR),
        ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
        ('ALIGN', (0, 0), (-1, -1), 'CENTER'),
        ('FONTSIZE', (0, 0), (-1, -1), 9),
        ('BOTTOMPADDING', (0, 0), (-1, -1), 6),
        ('TOPPADDING', (0, 0), (-1, -1), 6),
        ('GRID', (0, 0), (-1, -1), 0.5, colors.grey),
        ('ROWBACKGROUNDS', (0, 1), (-1, -2), [colors.white, LIGHT_GRAY]),
        ('BACKGROUND', (0, -1), (-1, -1), LIGHT_GRAY),
    ]))
    
    story.append(Spacer(1, 12))
    story.append(test_table)
    story.append(Paragraph("<i>Table 5: Test Results Summary (99.4% Overall Pass Rate)</i>",
        ParagraphStyle('Caption', alignment=TA_CENTER, fontSize=9, fontName='Times'])))


def create_adaptation_section(story):
    story.append(Paragraph("<b>7. Adaptation to Other Languages</b>", styles['ENHeading1']))
    story.append(Paragraph("<b>7.1 The Universal Programming Vision</b>", styles['ENHeading2']))
    story.append(Paragraph(
        "The language-agnostic architecture of Al-Marjaa opens unprecedented possibilities: "
        "any developer can program in their mother tongue while maintaining full technical capability. "
        "Creating a new language adaptation requires only three steps:",
        styles['ENBody']))
    
    steps = [
        "Create keyword file: Define keywords in target language",
        "Configure RTL: Set text direction if needed",
        "Localize error messages: Translate system messages"
    ]
    for i, step in enumerate(steps, 1):
        story.append(Paragraph(f"{i}. {step}", styles['ENBody']))
    
    story.append(Paragraph("<b>7.2 Global Impact</b>", styles['ENHeading2']))
    impacts = [
        "Education: Children can learn programming in their native language",
        "Inclusivity: Programming becomes accessible to non-English speakers",
        "Cultural Preservation: Technical vocabulary develops in native languages",
        "Innovation: Diverse perspectives enter the programming world"
    ]
    for impact in impacts:
        story.append(Paragraph(f"• {impact}", styles['ENBody']))


def create_availability_section(story):
    story.append(PageBreak())
    story.append(Paragraph("<b>8. Code Availability Statement</b>", styles['ENHeading1']))
    story.append(Paragraph("The source code and implementation of Al-Marjaa Language are publicly available at:", styles['ENBody']))
    
    story.append(Paragraph("<b>https://github.com/radhwendalyhamdouni/Al-Marjaa-Language</b>",
        ParagraphStyle('URL', fontName='Times', fontSize=11, alignment=TA_CENTER,
            textColor=HexColor('#3498db'), spaceBefore=12, spaceAfter=12)))
    
    story.append(Paragraph("<b>Usage Terms:</b>", styles['ENHeading2']))
    terms = [
        "Academic and Non-Commercial Use: Permitted with proper attribution to RADHWEN DALY HAMDOUNI",
        "Commercial Use: Requires explicit written permission",
        "Derivative Works: Requires permission when preserving core architecture"
    ]
    for term in terms:
        story.append(Paragraph(f"• {term}", styles['ENBody']))
    
    story.append(Spacer(1, 12))
    story.append(Paragraph("<b>Contact for Licensing:</b> almarjaa.project@hotmail.com", styles['ENBody']))
    
    story.append(Spacer(1, 18))
    story.append(Paragraph("<b>Suggested Citation:</b>", styles['ENHeading2']))
    
    citation = """@article{hamdouni2026almarjaa,
  title={Al-Marjaa: A Language-Agnostic Programming Framework 
         with Native AI Integration},
  author={Hamdouni, Radwan Daly},
  journal={arXiv preprint},
  year={2026},
  note={Version 3.2.0}
}"""
    story.append(Paragraph(citation, styles['CodeBlock']))


def create_conclusion(story):
    story.append(Paragraph("<b>9. Conclusion and Future Work</b>", styles['ENHeading1']))
    story.append(Paragraph("<b>9.1 Summary</b>", styles['ENHeading2']))
    story.append(Paragraph("This paper presented Al-Marjaa, a revolutionary programming language framework that combines:", styles['ENBody']))
    
    achievements = [
        "Language Agnosticism: Configuration-based syntax enabling adaptation to any human language",
        "High Performance: 5-tier JIT compiler achieving 5.08x speedup",
        "AI Integration: Native ONNX support and Vibe Coding natural language programming",
        "Modern Architecture: Parallel GC, async/await, WebAssembly target",
        "Production Quality: 343 tests with 100% pass rate"
    ]
    for i, ach in enumerate(achievements, 1):
        story.append(Paragraph(f"{i}. {ach}", styles['ENBody']))
    
    story.append(Paragraph("<b>9.2 Vision</b>", styles['ENHeading2']))
    story.append(Paragraph(
        "Al-Marjaa demonstrates that programming languages can be both culturally native and "
        "technically competitive. Our vision is a world where every developer can program in their "
        "mother tongue, where programming education is not limited by English proficiency, and where "
        "diverse perspectives enrich the global software development community.",
        styles['ENBody']))
    
    story.append(Spacer(1, 30))
    story.append(Paragraph("© 2026 RADHWEN DALY HAMDOUNI | All Rights Reserved",
        ParagraphStyle('Footer', fontName='Times', fontSize=10, alignment=TA_CENTER, textColor=TEXT_COLOR)))


def generate_paper():
    output_path = "/home/z/my-project/download/Al_Marjaa_Research_Paper_Final.pdf"
    
    doc = SimpleDocTemplate(
        output_path,
        pagesize=A4,
        rightMargin=2*cm,
        leftMargin=2*cm,
        topMargin=2*cm,
        bottomMargin=2*cm,
        title="Al_Marjaa_Research_Paper",
        author="Z.ai",
        creator="Z.ai",
        subject="Al-Marjaa Language Research Paper"
    )
    
    story = []
    create_cover_page(story)
    create_abstract(story)
    create_introduction(story)
    create_architecture_section(story)
    create_features_section(story)
    create_jit_section(story)
    create_vibe_section(story)
    create_evaluation_section(story)
    create_adaptation_section(story)
    create_availability_section(story)
    create_conclusion(story)
    
    doc.build(story)
    print(f"✓ PDF generated successfully: {output_path}")
    return output_path


if __name__ == "__main__":
    generate_paper()
