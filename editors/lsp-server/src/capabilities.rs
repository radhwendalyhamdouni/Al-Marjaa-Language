//! ═══════════════════════════════════════════════════════════════════════════════
//! Server Capabilities - قدرات الخادم
//! ═══════════════════════════════════════════════════════════════════════════════
//! تعريف جميع القدرات المدعومة

use lsp_types::*;

/// إنشاء قدرات الخادم الكاملة
pub fn create_server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        // مزامنة المستندات
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::INCREMENTAL),
                will_save: Some(false),
                will_save_wait_until: Some(false),
                save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                    include_text: Some(true),
                })),
            }
        )),
        
        // الإكمال التلقائي
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec![
                ".".to_string(),
                " ".to_string(),
                "(".to_string(),
                "،".to_string(),
            ]),
            resolve_provider: Some(true),
            all_commit_characters: Some(vec![
                "؛".to_string(),
                ";".to_string(),
                ",".to_string(),
                "،".to_string(),
            ]),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
            completion_item: Some(CompletionOptionsCompletionItem {
                label_details_support: Some(true),
                insert_text_mode_support: Some(InsertTextModeSupport {
                    value_set: vec![
                        InsertTextMode::AS_IS,
                        InsertTextMode::ADJUST_INDENTATION,
                    ],
                }),
                ..Default::default()
            }),
        }),
        
        // Hover
        hover_provider: Some(HoverProviderCapability::Options(HoverOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        })),
        
        // Signature Help
        signature_help_provider: Some(SignatureHelpOptions {
            trigger_characters: Some(vec!["(".to_string(), "،".to_string(), ",".to_string()]),
            retrigger_characters: Some(vec![",".to_string(), "،".to_string()]),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        }),
        
        // Definition
        definition_provider: Some(OneOf::Right(DefinitionOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        })),
        
        // References
        references_provider: Some(OneOf::Right(ReferenceOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        })),
        
        // Document Highlight
        document_highlight_provider: Some(OneOf::Right(DocumentHighlightOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        })),
        
        // Document Symbol
        document_symbol_provider: Some(OneOf::Right(DocumentSymbolOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
            label: Some("المرجع".to_string()),
        })),
        
        // Workspace Symbol
        workspace_symbol_provider: Some(OneOf::Right(WorkspaceSymbolOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
            resolve_provider: Some(true),
        })),
        
        // Code Action
        code_action_provider: Some(CodeActionProviderCapability::Options(CodeActionOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
            code_action_kinds: Some(vec![
                CodeActionKind::QUICKFIX,
                CodeActionKind::REFACTOR,
                CodeActionKind::REFACTOR_EXTRACT,
                CodeActionKind::REFACTOR_INLINE,
                CodeActionKind::REFACTOR_REWRITE,
                CodeActionKind::SOURCE,
                CodeActionKind::SOURCE_ORGANIZE_IMPORTS,
            ]),
            resolve_provider: Some(true),
        })),
        
        // Code Lens
        code_lens_provider: Some(CodeLensOptions {
            resolve_provider: Some(true),
        }),
        
        // Document Formatting
        document_formatting_provider: Some(OneOf::Right(DocumentFormattingOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        })),
        
        // Document Range Formatting
        document_range_formatting_provider: Some(OneOf::Right(DocumentRangeFormattingOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        })),
        
        // Document On Type Formatting
        document_on_type_formatting_provider: Some(DocumentOnTypeFormattingOptions {
            first_trigger_character: "}".to_string(),
            more_trigger_character: Some(vec![
                "؛".to_string(),
                ";".to_string(),
                "\n".to_string(),
            ]),
        }),
        
        // Rename
        rename_provider: Some(OneOf::Right(RenameOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
            prepare_provider: Some(true),
        })),
        
        // Folding Range
        folding_range_provider: Some(FoldingRangeProviderCapability::Right(FoldingRangeOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        })),
        
        // Selection Range
        selection_range_provider: Some(SelectionRangeProviderCapability::Right(SelectionRangeOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        })),
        
        // Execute Command
        execute_command_provider: Some(ExecuteCommandOptions {
            commands: vec![
                "almarjaa.runFile".to_string(),
                "almarjaa.format".to_string(),
                "almarjaa.fixAll".to_string(),
                "almarjaa.organizeImports".to_string(),
                "almarjaa.extractFunction".to_string(),
                "almarjaa.extractVariable".to_string(),
                "almarjaa.inlineVariable".to_string(),
                "almarjaa.generateTests".to_string(),
                "almarjaa.explainCode".to_string(),
                "almarjaa.optimizeCode".to_string(),
                "almarjaa.convertToTypeScript".to_string(),
                "almarjaa.convertToPython".to_string(),
                "almarjaa.showType".to_string(),
                "almarjaa.findImplementations".to_string(),
            ],
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        }),
        
        // Call Hierarchy
        call_hierarchy_provider: Some(CallHierarchyServerCapability::Options(CallHierarchyOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        })),
        
        // Semantic Tokens
        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
            SemanticTokensOptions {
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: Some(true),
                },
                legend: SemanticTokensLegend {
                    token_types: get_semantic_token_types(),
                    token_modifiers: get_semantic_token_modifiers(),
                },
                range: Some(true),
                full: Some(SemanticTokensFullOptions::Delta{ delta: Some(true) }),
            }
        )),
        
        // Inlay Hint
        inlay_hint_provider: Some(OneOf::Right(InlayHintServerCapabilities::Options(InlayHintOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
            resolve_provider: Some(true),
        }))),
        
        // Inline Value
        inline_value_provider: Some(OneOf::Left(true)),
        
        // Linked Editing Range
        linked_editing_range_provider: Some(OneOf::Right(LinkedEditingRangeOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        })),
        
        // Type Definition
        type_definition_provider: Some(TypeDefinitionProviderCapability::Options(TypeDefinitionOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        })),
        
        // Implementation
        implementation_provider: Some(ImplementationProviderCapability::Options(ImplementationOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        })),
        
        // Diagnostic
        diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
            identifier: Some("almarjaa".to_string()),
            inter_file_dependencies: false,
            workspace_diagnostics: true,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
        })),
        
        // Workspace
        workspace: Some(ServerCapabilitiesWorkspace {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(OneOf::Left(true)),
            }),
            file_operations: Some(FileOperationOptions {
                did_create: Some(FileOperationRegistrationOptions {
                    filters: vec![
                        FileOperationFilter {
                            scheme: Some("file".to_string()),
                            pattern: FileOperationPattern {
                                glob: "**/*.mrj".to_string(),
                                matches: Some(FileOperationPatternKind::FILE),
                                options: Some(FileOperationPatternOptions {
                                    ignore_case: Some(true),
                                }),
                            },
                        },
                    ],
                }),
                did_rename: Some(FileOperationRegistrationOptions {
                    filters: vec![
                        FileOperationFilter {
                            scheme: Some("file".to_string()),
                            pattern: FileOperationPattern {
                                glob: "**/*.mrj".to_string(),
                                matches: Some(FileOperationPatternKind::FILE),
                                options: Some(FileOperationPatternOptions {
                                    ignore_case: Some(true),
                                }),
                            },
                        },
                    ],
                }),
                did_delete: Some(FileOperationRegistrationOptions {
                    filters: vec![
                        FileOperationFilter {
                            scheme: Some("file".to_string()),
                            pattern: FileOperationPattern {
                                glob: "**/*.mrj".to_string(),
                                matches: Some(FileOperationPatternKind::FILE),
                                options: Some(FileOperationPatternOptions {
                                    ignore_case: Some(true),
                                }),
                            },
                        },
                    ],
                }),
                will_create: None,
                will_rename: None,
                will_delete: None,
            }),
        }),
        
        // Experimental
        experimental: Some(serde_json::json!({
            "almarjaa": {
                "version": "3.3.0",
                "features": {
                    "vibeCoding": true,
                    "arabicSupport": true,
                    "smartCompletion": true,
                    "typeInference": true,
                    "codeGeneration": true,
                }
            }
        })),
        
        ..Default::default()
    }
}

/// أنواع الرموز الدلالية
fn get_semantic_token_types() -> Vec<SemanticTokenType> {
    vec![
        SemanticTokenType::NAMESPACE,      // 0
        SemanticTokenType::TYPE,           // 1
        SemanticTokenType::CLASS,          // 2
        SemanticTokenType::ENUM,           // 3
        SemanticTokenType::INTERFACE,      // 4
        SemanticTokenType::STRUCT,         // 5
        SemanticTokenType::TYPE_PARAMETER, // 6
        SemanticTokenType::PARAMETER,      // 7
        SemanticTokenType::VARIABLE,       // 8
        SemanticTokenType::PROPERTY,       // 9
        SemanticTokenType::ENUM_MEMBER,    // 10
        SemanticTokenType::EVENT,          // 11
        SemanticTokenType::FUNCTION,       // 12
        SemanticTokenType::METHOD,         // 13
        SemanticTokenType::MACRO,          // 14
        SemanticTokenType::KEYWORD,        // 15
        SemanticTokenType::MODIFIER,       // 16
        SemanticTokenType::COMMENT,        // 17
        SemanticTokenType::STRING,         // 18
        SemanticTokenType::NUMBER,         // 19
        SemanticTokenType::REGEXP,         // 20
        SemanticTokenType::OPERATOR,       // 21
        // Custom types
        SemanticTokenType::new("builtin"),     // 22
        SemanticTokenType::new("decorator"),   // 23
        SemanticTokenType::new("label"),       // 24
    ]
}

/// معدّلات الرموز الدلالية
fn get_semantic_token_modifiers() -> Vec<SemanticTokenModifier> {
    vec![
        SemanticTokenModifier::DECLARATION,     // 0
        SemanticTokenModifier::DEFINITION,      // 1
        SemanticTokenModifier::READONLY,        // 2
        SemanticTokenModifier::STATIC,          // 3
        SemanticTokenModifier::DEPRECATED,      // 4
        SemanticTokenModifier::ABSTRACT,        // 5
        SemanticTokenModifier::ASYNC,           // 6
        SemanticTokenModifier::MODIFICATION,    // 7
        SemanticTokenModifier::DOCUMENTATION,   // 8
        SemanticTokenModifier::DEFAULT_LIBRARY, // 9
    ]
}

/// معلومات الخادم
pub fn get_server_info() -> ServerInfo {
    ServerInfo {
        name: "Al-Marjaa Language Server".to_string(),
        version: Some("3.3.0".to_string()),
    }
}
