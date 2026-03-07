/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * VS Code Extension للغة المرجع - الإصدار المتقدم
 * ═══════════════════════════════════════════════════════════════════════════════
 * الميزات:
 * - LSP Server Integration
 * - Vibe Coding مع AI
 * - الإكمال التلقائي الذكي
 * - تصحيح الأخطاء
 * - Code Actions
 * - Inline Hints
 * - Code Lens
 * ═══════════════════════════════════════════════════════════════════════════════
 */

import * as path from 'path';
import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
    NotificationType,
    RequestType,
    CodeAction
} from 'vscode-languageclient/node';

// ═══════════════════════════════════════════════════════════════════════════════
// الثوابت والأنواع
// ═══════════════════════════════════════════════════════════════════════════════

const VIBE_CODING_REQUEST = new RequestType<{
    code: string;
    context: string;
    action: 'complete' | 'explain' | 'fix' | 'optimize' | 'document' | 'test';
}, {
    result: string;
    suggestions: string[];
    confidence: number;
}, void>('almarjaa/vibeCoding');

interface VibeSuggestion {
    text: string;
    kind: 'completion' | 'refactor' | 'fix' | 'document';
    confidence: number;
    insertText?: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// الحالة العامة
// ═══════════════════════════════════════════════════════════════════════════════

let client: LanguageClient | undefined;
let vibeProvider: VibeCodingProvider | undefined;
let outputChannel: vscode.OutputChannel;
let diagnosticCollection: vscode.DiagnosticCollection;
let codeLensProvider: AlmarjaaCodeLensProvider | undefined;

// ═══════════════════════════════════════════════════════════════════════════════
// التفعيل الرئيسي
// ═══════════════════════════════════════════════════════════════════════════════

export function activate(context: vscode.ExtensionContext) {
    console.log('[Al-Marjaa] 🚀 Extension activated - Advanced Version');
    
    // إنشاء قناة الإخراج
    outputChannel = vscode.window.createOutputChannel('Al-Marjaa');
    outputChannel.appendLine('🚀 تفعيل إضافة المرجع المتقدمة');
    
    // إنشاء مجموعة التشخيص
    diagnosticCollection = vscode.languages.createDiagnosticCollection('almarjaa');
    
    // بدء LSP Client
    startLanguageClient(context);
    
    // تسجيل المزودين
    registerProviders(context);
    
    // تسجيل الأوامر
    registerCommands(context);
    
    // تسجيل حالة الشريط
    showStatusBar(context);
    
    // تهيئة Vibe Coding
    initVibeCoding(context);
    
    // إظهار رسالة الترحيب
    showWelcomeMessage(context);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Language Client
// ═══════════════════════════════════════════════════════════════════════════════

function startLanguageClient(context: vscode.ExtensionContext) {
    const config = vscode.workspace.getConfiguration('almarjaa');
    const serverPath = config.get<string>('serverPath', 'almarjaa-lsp');
    
    outputChannel.appendLine(`📡 بدء خادم اللغة: ${serverPath}`);
    
    const serverOptions: ServerOptions = {
        command: serverPath,
        transport: TransportKind.stdio,
        args: ['--stdio']
    };
    
    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'almarjaa' },
            { scheme: 'untitled', language: 'almarjaa' }
        ],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.mrj'),
            configurationSection: 'almarjaa'
        },
        initializationOptions: {
            clientName: 'almarjaa-vscode-advanced',
            clientVersion: '2.0.0',
            features: {
                codeActions: true,
                codeLens: true,
                inlineHints: true,
                semanticTokens: true
            }
        },
        middleware: {
            provideCompletionItem: async (document, position, context, token, next) => {
                const result = await next(document, position, context, token);
                if (result) {
                    // إضافة اقتراحات Vibe Coding
                    const vibeSuggestions = await getVibeSuggestions(document, position);
                    if (vibeSuggestions.length > 0 && 'items' in result) {
                        result.items.push(...vibeSuggestions.map(s => ({
                            label: `✨ ${s.text}`,
                            kind: vscode.CompletionItemKind.Snippet,
                            insertText: s.insertText || s.text,
                            documentation: `اقتراح Vibe Coding (ثقة: ${Math.round(s.confidence * 100)}%)`,
                            sortText: `0${s.text}` // أولوية عالية
                        })));
                    }
                }
                return result;
            },
            
            provideCodeActions: async (document, range, context, token, next) => {
                const result = await next(document, range, context, token);
                // إضافة Code Actions من Vibe
                const vibeActions = await getVibeCodeActions(document, range);
                if (result && 'length' in result) {
                    result.push(...vibeActions);
                }
                return result;
            },
            
            provideHover: async (document, position, token, next) => {
                const result = await next(document, position, token);
                // يمكن إضافة توثيق إضافي هنا
                return result;
            }
        }
    };
    
    client = new LanguageClient(
        'almarjaa',
        'Al-Marjaa Language Server',
        serverOptions,
        clientOptions
    );
    
    client.start().then(() => {
        outputChannel.appendLine('✅ خادم اللغة يعمل');
        vscode.window.showInformationMessage('🟢 خادم المرجع المتقدم يعمل');
    }).catch((error) => {
        outputChannel.appendLine(`❌ فشل بدء الخادم: ${error.message}`);
        vscode.window.showErrorMessage(`❌ فشل بدء خادم المرجع: ${error.message}`);
    });
}

// ═══════════════════════════════════════════════════════════════════════════════
// مزودي الخدمات
// ═══════════════════════════════════════════════════════════════════════════════

function registerProviders(context: vscode.ExtensionContext) {
    // مزود Inline Hints
    const hintsProvider = vscode.languages.registerInlineValuesProvider(
        { language: 'almarjaa' },
        {
            provideInlineValues(document, viewPort, context, token) {
                const values: vscode.InlineValue[] = [];
                // إضافة قيم inline لل debugging
                return values;
            }
        }
    );
    
    // مزود Code Lens
    codeLensProvider = new AlmarjaaCodeLensProvider();
    const codeLens = vscode.languages.registerCodeLensProvider(
        { language: 'almarjaa' },
        codeLensProvider
    );
    
    // مزود Document Highlight
    const highlightProvider = vscode.languages.registerDocumentHighlightProvider(
        { language: 'almarjaa' },
        {
            provideDocumentHighlights(document, position, token) {
                const highlights: vscode.DocumentHighlight[] = [];
                const word = document.getText(document.getWordRangeAtPosition(position));
                
                // البحث عن جميع تكرارات الكلمة
                const text = document.getText();
                const regex = new RegExp(`\\b${word}\\b`, 'g');
                let match;
                
                while ((match = regex.exec(text)) !== null) {
                    const startPos = document.positionAt(match.index);
                    const endPos = document.positionAt(match.index + match[0].length);
                    highlights.push(new vscode.DocumentHighlight(
                        new vscode.Range(startPos, endPos),
                        vscode.DocumentHighlightKind.Read
                    ));
                }
                
                return highlights;
            }
        }
    );
    
    // مزود Document Symbol
    const symbolProvider = vscode.languages.registerDocumentSymbolProvider(
        { language: 'almarjaa' },
        {
            provideDocumentSymbols(document, token) {
                const symbols: vscode.DocumentSymbol[] = [];
                const text = document.getText();
                
                // استخراج الدوال
                const funcRegex = /دالة\s+(\w+)\s*\(/g;
                let match;
                
                while ((match = funcRegex.exec(text)) !== null) {
                    const startPos = document.positionAt(match.index);
                    const symbol = new vscode.DocumentSymbol(
                        match[1],
                        'دالة',
                        vscode.SymbolKind.Function,
                        new vscode.Range(startPos, startPos),
                        new vscode.Range(startPos, startPos.translate(0, match[0].length))
                    );
                    symbols.push(symbol);
                }
                
                // استخراج المتغيرات
                const varRegex = /متغير\s+(\w+)\s*=/g;
                while ((match = varRegex.exec(text)) !== null) {
                    const startPos = document.positionAt(match.index);
                    const symbol = new vscode.DocumentSymbol(
                        match[1],
                        'متغير',
                        vscode.SymbolKind.Variable,
                        new vscode.Range(startPos, startPos),
                        new vscode.Range(startPos, startPos.translate(0, match[0].length))
                    );
                    symbols.push(symbol);
                }
                
                return symbols;
            }
        }
    );
    
    // مزود Definition
    const definitionProvider = vscode.languages.registerDefinitionProvider(
        { language: 'almarjaa' },
        {
            provideDefinition(document, position, token) {
                const word = document.getText(document.getWordRangeAtPosition(position));
                // البحث عن التعريف
                const text = document.getText();
                const regex = new RegExp(`(دالة|متغير|ثابت)\\s+${word}\\b`, 'g');
                const match = regex.exec(text);
                
                if (match) {
                    const startPos = document.positionAt(match.index);
                    return new vscode.Location(document.uri, startPos);
                }
                
                return undefined;
            }
        }
    );
    
    // مزود Reference
    const referenceProvider = vscode.languages.registerReferenceProvider(
        { language: 'almarjaa' },
        {
            provideReferences(document, position, context, token) {
                const references: vscode.Location[] = [];
                const word = document.getText(document.getWordRangeAtPosition(position));
                const text = document.getText();
                
                const regex = new RegExp(`\\b${word}\\b`, 'g');
                let match;
                
                while ((match = regex.exec(text)) !== null) {
                    const pos = document.positionAt(match.index);
                    references.push(new vscode.Location(document.uri, pos));
                }
                
                return references;
            }
        }
    );
    
    // مزود Rename
    const renameProvider = vscode.languages.registerRenameProvider(
        { language: 'almarjaa' },
        {
            provideRenameEdits(document, position, newName, token) {
                const word = document.getText(document.getWordRangeAtPosition(position));
                const text = document.getText();
                const edit = new vscode.WorkspaceEdit();
                
                const regex = new RegExp(`\\b${word}\\b`, 'g');
                let match;
                
                while ((match = regex.exec(text)) !== null) {
                    const pos = document.positionAt(match.index);
                    const range = new vscode.Range(pos, pos.translate(0, word.length));
                    edit.replace(document.uri, range, newName);
                }
                
                return edit;
            },
            prepareRename(document, position, token) {
                const range = document.getWordRangeAtPosition(position);
                if (!range) {
                    throw new Error('لا يمكن إعادة تسمية هذا العنصر');
                }
                return range;
            }
        }
    );
    
    // مزود Document Formatting
    const formattingProvider = vscode.languages.registerDocumentFormattingEditProvider(
        { language: 'almarjaa' },
        {
            provideDocumentFormattingEdits(document, options, token) {
                const edits: vscode.TextEdit[] = [];
                const text = document.getText();
                const formatted = formatCode(text, options);
                
                if (text !== formatted) {
                    edits.push(vscode.TextEdit.replace(
                        new vscode.Range(
                            document.positionAt(0),
                            document.positionAt(text.length)
                        ),
                        formatted
                    ));
                }
                
                return edits;
            }
        }
    );
    
    // مزود Color Provider
    const colorProvider = vscode.languages.registerColorProvider(
        { language: 'almarjaa' },
        {
            provideDocumentColors(document, token) {
                const colors: vscode.ColorInformation[] = [];
                // البحث عن الألوان في الكود
                return colors;
            },
            provideColorPresentations(color, context, token) {
                const presentations: vscode.ColorPresentation[] = [];
                presentations.push(new vscode.ColorPresentation(
                    `rgb(${Math.round(color.red * 255)}, ${Math.round(color.green * 255)}, ${Math.round(color.blue * 255)})`
                ));
                return presentations;
            }
        }
    );
    
    // مزود Folding
    const foldingProvider = vscode.languages.registerFoldingRangeProvider(
        { language: 'almarjaa' },
        {
            provideFoldingRanges(document, context, token) {
                const ranges: vscode.FoldingRange[] = [];
                const text = document.getText();
                const lines = text.split('\n');
                
                let startLine = -1;
                let braceCount = 0;
                
                for (let i = 0; i < lines.length; i++) {
                    const line = lines[i];
                    
                    if (line.includes('{')) {
                        if (braceCount === 0) {
                            startLine = i;
                        }
                        braceCount += (line.match(/{/g) || []).length;
                    }
                    
                    if (line.includes('}')) {
                        braceCount -= (line.match(/}/g) || []).length;
                        if (braceCount === 0 && startLine >= 0) {
                            ranges.push(new vscode.FoldingRange(startLine, i));
                            startLine = -1;
                        }
                    }
                }
                
                return ranges;
            }
        }
    );
    
    context.subscriptions.push(
        hintsProvider,
        codeLens,
        highlightProvider,
        symbolProvider,
        definitionProvider,
        referenceProvider,
        renameProvider,
        formattingProvider,
        colorProvider,
        foldingProvider
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Vibe Coding Integration
// ═══════════════════════════════════════════════════════════════════════════════

function initVibeCoding(context: vscode.ExtensionContext) {
    vibeProvider = new VibeCodingProvider();
    
    // تسجيل مزود Inline Chat
    const inlineChatProvider = vscode.languages.registerInlineCompletionItemProvider(
        { language: 'almarjaa' },
        {
            async provideInlineCompletionItems(document, position, context, token) {
                const items: vscode.InlineCompletionItem[] = [];
                
                // الحصول على السياق
                const line = document.lineAt(position.line);
                const prefix = line.text.substring(0, position.character);
                
                // طلب اقتراحات AI
                const suggestions = await vibeProvider?.getSuggestions(prefix, document.getText());
                
                if (suggestions) {
                    for (const suggestion of suggestions) {
                        items.push(new vscode.InlineCompletionItem(
                            suggestion.text,
                            new vscode.Range(position, position)
                        ));
                    }
                }
                
                return { items };
            }
        }
    );
    
    context.subscriptions.push(inlineChatProvider);
}

class VibeCodingProvider {
    private cache: Map<string, VibeSuggestion[]> = new Map();
    
    async getSuggestions(prefix: string, context: string): Promise<VibeSuggestion[]> {
        const cacheKey = `${prefix}:${context.substring(0, 100)}`;
        
        if (this.cache.has(cacheKey)) {
            return this.cache.get(cacheKey)!;
        }
        
        // محاكاة اقتراحات AI (في التطبيق الحقيقي سيتم استدعاء الخادم)
        const suggestions: VibeSuggestion[] = [];
        
        // اقتراحات بناءً على السياق
        if (prefix.trim().endsWith('اطبع')) {
            suggestions.push({
                text: 'اطبع("مرحبا بالعالم!")',
                kind: 'completion',
                confidence: 0.9,
                insertText: '("$1")'
            });
        }
        
        if (prefix.trim().endsWith('دالة')) {
            suggestions.push({
                text: 'دالة اسم_الدالة(المعاملات) {\n\t$1\n}',
                kind: 'completion',
                confidence: 0.85,
                insertText: ' ${1:اسم_الدالة}(${2:المعاملات}) {\n\t$3\n}'
            });
        }
        
        if (prefix.trim().endsWith('لكل')) {
            suggestions.push({
                text: 'لكل عنصر من مجموعة {\n\t$1\n}',
                kind: 'completion',
                confidence: 0.9,
                insertText: ' ${1:عنصر} من ${2:مجموعة} {\n\t$3\n}'
            });
        }
        
        if (prefix.trim().endsWith('إذا')) {
            suggestions.push({
                text: 'إذا شرط {\n\t$1\n} وإلا {\n\t$2\n}',
                kind: 'completion',
                confidence: 0.9,
                insertText: ' ${1:شرط} {\n\t$2\n} وإلا {\n\t$3\n}'
            });
        }
        
        this.cache.set(cacheKey, suggestions);
        return suggestions;
    }
    
    async explainCode(code: string): Promise<string> {
        // محاكاة الشرح
        return `📝 شرح الكود:\n\nهذا الكود يقوم بـ:\n- تعريف دالة\n- معالجة البيانات\n- إرجاع النتيجة`;
    }
    
    async optimizeCode(code: string): Promise<{ optimized: string; improvements: string[] }> {
        return {
            optimized: code,
            improvements: [
                'يمكن تحسين الأداء باستخدام خوارزمية أسرع',
                'يُنصح بتقليل عدد المتغيرات المؤقتة'
            ]
        };
    }
    
    async generateTests(code: string): Promise<string> {
        return `// اختبارات تلقائية
اختبار("الوظيفة الأساسية", دالة() {
    // ترتيب
    const نتيجة = الدالة_المختبرة();
    // تحقق
    تأكيد(نتيجة == المتوقع);
});`;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Code Lens Provider
// ═══════════════════════════════════════════════════════════════════════════════

class AlmarjaaCodeLensProvider implements vscode.CodeLensProvider {
    
    onDidChangeCodeLenses?: vscode.Event<void>;
    
    provideCodeLenses(document: vscode.TextDocument, token: vscode.CancellationToken): vscode.ProviderResult<vscode.CodeLens[]> {
        const lenses: vscode.CodeLens[] = [];
        const text = document.getText();
        
        // إضافة Code Lens للدوال
        const funcRegex = /دالة\s+(\w+)\s*\(/g;
        let match;
        
        while ((match = funcRegex.exec(text)) !== null) {
            const line = document.positionAt(match.index).line;
            const range = new vscode.Range(line, 0, line, 0);
            
            // تشغيل الدالة
            lenses.push(new vscode.CodeLens(range, {
                title: '$(play) تشغيل',
                command: 'almarjaa.runFunction',
                arguments: [match[1]]
            }));
            
            // اختبار الدالة
            lenses.push(new vscode.CodeLens(range, {
                title: '$(beaker) اختبار',
                command: 'almarjaa.testFunction',
                arguments: [match[1]]
            }));
            
            // شرح الدالة
            lenses.push(new vscode.CodeLens(range, {
                title: '$(info) شرح',
                command: 'almarjaa.explainFunction',
                arguments: [match[1], document.uri]
            }));
        }
        
        return lenses;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الأوامر
// ═══════════════════════════════════════════════════════════════════════════════

function registerCommands(context: vscode.ExtensionContext) {
    // إعادة تشغيل الخادم
    const restartCommand = vscode.commands.registerCommand(
        'almarjaa.restartServer',
        async () => {
            if (client) {
                await client.stop();
                startLanguageClient(context);
                vscode.window.showInformationMessage('🔄 تم إعادة تشغيل خادم المرجع');
            }
        }
    );
    
    // تشغيل الملف
    const runCommand = vscode.commands.registerCommand(
        'almarjaa.runFile',
        () => {
            const editor = vscode.window.activeTextEditor;
            if (editor && editor.document.languageId === 'almarjaa') {
                const filePath = editor.document.uri.fsPath;
                runAlmarjaaFile(filePath);
            } else {
                vscode.window.showWarningMessage('الملف الحالي ليس ملف مرجع (.mrj)');
            }
        }
    );
    
    // Vibe Coding: شرح الكود
    const explainCommand = vscode.commands.registerCommand(
        'almarjaa.explainCode',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor) {
                const selection = editor.selection;
                const code = editor.document.getText(selection);
                const explanation = await vibeProvider?.explainCode(code);
                
                if (explanation) {
                    // إظهار في لوحة جانبية
                    const panel = vscode.window.createWebviewPanel(
                        'almarjaaExplanation',
                        'شرح الكود',
                        vscode.ViewColumn.Beside,
                        {}
                    );
                    panel.webview.html = getExplanationHtml(explanation);
                }
            }
        }
    );
    
    // Vibe Coding: تحسين الكود
    const optimizeCommand = vscode.commands.registerCommand(
        'almarjaa.optimizeCode',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor) {
                const selection = editor.selection;
                const code = editor.document.getText(selection);
                const result = await vibeProvider?.optimizeCode(code);
                
                if (result) {
                    // إظهار التحسينات
                    const choice = await vscode.window.showInformationMessage(
                        `تم العثور على ${result.improvements.length} تحسين`,
                        'تطبيق',
                        'عرض التفاصيل'
                    );
                    
                    if (choice === 'تطبيق') {
                        editor.edit(editBuilder => {
                            editBuilder.replace(selection, result.optimized);
                        });
                    }
                }
            }
        }
    );
    
    // Vibe Coding: توليد اختبارات
    const generateTestsCommand = vscode.commands.registerCommand(
        'almarjaa.generateTests',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor) {
                const code = editor.document.getText();
                const tests = await vibeProvider?.generateTests(code);
                
                if (tests) {
                    // إنشاء ملف اختبار جديد
                    const doc = await vscode.workspace.openTextDocument({
                        language: 'almarjaa',
                        content: tests
                    });
                    await vscode.window.showTextDocument(doc);
                }
            }
        }
    );
    
    // شرح دالة
    const explainFunctionCommand = vscode.commands.registerCommand(
        'almarjaa.explainFunction',
        async (funcName: string, uri: vscode.Uri) => {
            vscode.window.showInformationMessage(`شرح الدالة: ${funcName}`);
        }
    );
    
    // تشغيل دالة
    const runFunctionCommand = vscode.commands.registerCommand(
        'almarjaa.runFunction',
        async (funcName: string) => {
            vscode.window.showInformationMessage(`تشغيل: ${funcName}`);
        }
    );
    
    // اختبار دالة
    const testFunctionCommand = vscode.commands.registerCommand(
        'almarjaa.testFunction',
        async (funcName: string) => {
            vscode.window.showInformationMessage(`اختبار: ${funcName}`);
        }
    );
    
    // فتح التوثيق
    const openDocsCommand = vscode.commands.registerCommand(
        'almarjaa.openDocs',
        () => {
            vscode.env.openExternal(vscode.Uri.parse('https://docs.almarjaa.io'));
        }
    );
    
    // إظهار الإحصائيات
    const showStatsCommand = vscode.commands.registerCommand(
        'almarjaa.showStats',
        () => {
            const editor = vscode.window.activeTextEditor;
            if (editor && editor.document.languageId === 'almarjaa') {
                const text = editor.document.getText();
                const stats = analyzeCode(text);
                
                vscode.window.showInformationMessage(
                    `📊 الإحصائيات: ${stats.lines} سطر | ${stats.functions} دالة | ${stats.variables} متغير`
                );
            }
        }
    );
    
    // تحويل إلى TypeScript
    const convertToTsCommand = vscode.commands.registerCommand(
        'almarjaa.convertToTypeScript',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor) {
                const code = editor.document.getText();
                const tsCode = convertToTypeScript(code);
                
                const doc = await vscode.workspace.openTextDocument({
                    language: 'typescript',
                    content: tsCode
                });
                await vscode.window.showTextDocument(doc);
            }
        }
    );
    
    // تحويل إلى Python
    const convertToPythonCommand = vscode.commands.registerCommand(
        'almarjaa.convertToPython',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor) {
                const code = editor.document.getText();
                const pyCode = convertToPython(code);
                
                const doc = await vscode.workspace.openTextDocument({
                    language: 'python',
                    content: pyCode
                });
                await vscode.window.showTextDocument(doc);
            }
        }
    );
    
    context.subscriptions.push(
        restartCommand,
        runCommand,
        explainCommand,
        optimizeCommand,
        generateTestsCommand,
        explainFunctionCommand,
        runFunctionCommand,
        testFunctionCommand,
        openDocsCommand,
        showStatsCommand,
        convertToTsCommand,
        convertToPythonCommand
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال مساعدة
// ═══════════════════════════════════════════════════════════════════════════════

function runAlmarjaaFile(filePath: string) {
    const terminal = vscode.window.createTerminal({
        name: '🚀 Al-Marjaa',
        iconPath: new vscode.ThemeIcon('play')
    });
    
    terminal.sendText(`almarjaa "${filePath}"`);
    terminal.show();
}

async function getVibeSuggestions(document: vscode.TextDocument, position: vscode.Position): Promise<VibeSuggestion[]> {
    const line = document.lineAt(position.line);
    const prefix = line.text.substring(0, position.character);
    
    return await vibeProvider?.getSuggestions(prefix, document.getText()) || [];
}

async function getVibeCodeActions(document: vscode.TextDocument, range: vscode.Range): Promise<vscode.CodeAction[]> {
    const actions: vscode.CodeAction[] = [];
    const code = document.getText(range);
    
    // إضافة شرح
    const explainAction = new vscode.CodeAction('✨ شرح الكود', vscode.CodeActionKind.QuickFix);
    explainAction.command = {
        command: 'almarjaa.explainCode',
        title: 'شرح الكود'
    };
    actions.push(explainAction);
    
    // تحسين الكود
    const optimizeAction = new vscode.CodeAction('⚡ تحسين الكود', vscode.CodeActionKind.Refactor);
    optimizeAction.command = {
        command: 'almarjaa.optimizeCode',
        title: 'تحسين الكود'
    };
    actions.push(optimizeAction);
    
    // توليد اختبارات
    const testAction = new vscode.CodeAction('🧪 توليد اختبارات', vscode.CodeActionKind.Refactor);
    testAction.command = {
        command: 'almarjaa.generateTests',
        title: 'توليد اختبارات'
    };
    actions.push(testAction);
    
    return actions;
}

function formatCode(text: string, options: vscode.FormattingOptions): string {
    const lines = text.split('\n');
    const indentStr = options.insertSpaces ? ' '.repeat(options.tabSize) : '\t';
    let indent = 0;
    const formatted: string[] = [];
    
    for (let line of lines) {
        line = line.trim();
        
        if (line.endsWith('}') || line.startsWith('}')) {
            indent = Math.max(0, indent - 1);
        }
        
        formatted.push(indentStr.repeat(indent) + line);
        
        if (line.endsWith('{')) {
            indent++;
        }
    }
    
    return formatted.join('\n');
}

function analyzeCode(text: string): { lines: number; functions: number; variables: number } {
    const lines = text.split('\n').length;
    const functions = (text.match(/دالة\s+\w+/g) || []).length;
    const variables = (text.match(/متغير\s+\w+/g) || []).length;
    
    return { lines, functions, variables };
}

function convertToTypeScript(code: string): string {
    return code
        .replace(/دالة/g, 'function')
        .replace(/متغير/g, 'let')
        .replace(/ثابت/g, 'const')
        .replace(/إذا/g, 'if')
        .replace(/وإلا/g, 'else')
        .replace(/بينما/g, 'while')
        .replace(/لكل/g, 'for')
        .replace(/من/g, 'of')
        .replace(/أرجع/g, 'return')
        .replace(/اطبع/g, 'console.log')
        .replace(/صحيح/g, 'true')
        .replace(/خطأ/g, 'false');
}

function convertToPython(code: string): string {
    return code
        .replace(/دالة/g, 'def')
        .replace(/متغير/g, '')
        .replace(/ثابت/g, '')
        .replace(/إذا/g, 'if')
        .replace(/وإلا/g, 'else')
        .replace(/بينما/g, 'while')
        .replace(/لكل/g, 'for')
        .replace(/من/g, 'in')
        .replace(/أرجع/g, 'return')
        .replace(/اطبع/g, 'print')
        .replace(/صحيح/g, 'True')
        .replace(/خطأ/g, 'False')
        .replace(/{/g, ':')
        .replace(/}/g, '');
}

function getExplanationHtml(explanation: string): string {
    return `<!DOCTYPE html>
<html dir="rtl">
<head>
    <meta charset="UTF-8">
    <title>شرح الكود</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, sans-serif; padding: 20px; direction: rtl; }
        pre { background: #1e1e1e; color: #d4d4d4; padding: 15px; border-radius: 8px; }
    </style>
</head>
<body>
    <h1>📝 شرح الكود</h1>
    <pre>${explanation}</pre>
</body>
</html>`;
}

function showStatusBar(context: vscode.ExtensionContext) {
    const statusBarItem = vscode.window.createStatusBarItem(
        vscode.StatusBarAlignment.Right,
        100
    );
    
    statusBarItem.text = '$(code) المرجع';
    statusBarItem.tooltip = 'لغة المرجع المتقدمة - اضغط للخيارات';
    statusBarItem.command = 'almarjaa.restartServer';
    
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor((editor) => {
            if (editor && editor.document.languageId === 'almarjaa') {
                statusBarItem.show();
            } else {
                statusBarItem.hide();
            }
        })
    );
    
    if (vscode.window.activeTextEditor?.document.languageId === 'almarjaa') {
        statusBarItem.show();
    }
    
    context.subscriptions.push(statusBarItem);
}

function showWelcomeMessage(context: vscode.ExtensionContext) {
    const showWelcome = vscode.workspace.getConfiguration('almarjaa').get('showWelcome', true);
    
    if (showWelcome) {
        vscode.window.showInformationMessage(
            '🌙 مرحباً بك في لغة المرجع المتقدمة!',
            'فتح التوثيق',
            'عدم الإظهار مجدداً'
        ).then((selection) => {
            if (selection === 'فتح التوثيق') {
                vscode.commands.executeCommand('almarjaa.openDocs');
            } else if (selection === 'عدم الإظهار مجدداً') {
                vscode.workspace.getConfiguration('almarjaa').update('showWelcome', false, true);
            }
        });
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// التعطيل
// ═══════════════════════════════════════════════════════════════════════════════

export function deactivate(): Thenable<void> | undefined {
    outputChannel?.appendLine('🔴 تعطيل إضافة المرجع');
    
    if (client) {
        return client.stop();
    }
    return undefined;
}
