use std::process;

use colored::Colorize;

#[derive(Debug, Clone, Default)]
pub struct RunOptions {
    pub show_tokens: bool,
    pub show_ast: bool,
    pub compile_only: bool,
    pub format_only: bool,
    pub debug: bool,
    pub lint_only: bool,
    pub lint_disabled_rules: Vec<String>,
    pub lint_max: Option<usize>,
    pub pm_init: Option<String>,
    pub pm_check: bool,
    pub pm_tree: bool,
    pub lsp_diag: bool,
    pub lsp_complete: Option<String>,
    pub lsp_hover: Option<(usize, usize)>,
    pub lsp_definition: Option<(usize, usize)>,
    pub lsp_references: Option<(usize, usize)>,
}

pub struct ParsedCli {
    pub options: RunOptions,
    pub filename: Option<String>,
    pub run_repl_flag: bool,
}

pub enum CliAction {
    Help,
    Version,
    Run(Box<ParsedCli>),
}

pub fn parse_args(args: &[String]) -> CliAction {
    let mut options = RunOptions::default();
    let mut filename: Option<String> = None;
    let mut run_repl_flag = false;

    if args.len() >= 2 {
        match args[1].as_str() {
            "pm" => parse_pm_subcommand(args, &mut options),
            "lsp" => parse_lsp_subcommand(args, &mut options, &mut filename),
            _ => {}
        }
    }

    if options.pm_init.is_none()
        && !options.pm_check
        && !options.pm_tree
        && !options.lsp_diag
        && options.lsp_complete.is_none()
        && options.lsp_hover.is_none()
        && options.lsp_definition.is_none()
        && options.lsp_references.is_none()
    {
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-h" | "--help" => return CliAction::Help,
                "-v" | "--version" => return CliAction::Version,
                "-r" | "--repl" => run_repl_flag = true,
                "-c" | "--compile" => options.compile_only = true,
                "-f" | "--format" => options.format_only = true,
                "-t" | "--tokens" => options.show_tokens = true,
                "-l" | "--lint" => options.lint_only = true,
                "--lint-disable" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!(
                            "{}",
                            crate::rtl("الخيار --lint-disable يحتاج كود قاعدة مثل L001")
                                .bright_red()
                        );
                        process::exit(1);
                    }
                    options.lint_disabled_rules.push(args[i].to_uppercase());
                }
                "--lint-max" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!(
                            "{}",
                            crate::rtl("الخيار --lint-max يحتاج رقماً صحيحاً").bright_red()
                        );
                        process::exit(1);
                    }
                    options.lint_max = Some(parse_positive_usize(&args[i], "--lint-max"));
                }
                "-a" | "--ast" => options.show_ast = true,
                "-d" | "--debug" => options.debug = true,
                "--pm-init" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!(
                            "{}",
                            crate::rtl("الخيار --pm-init يحتاج اسم مشروع").bright_red()
                        );
                        process::exit(1);
                    }
                    options.pm_init = Some(args[i].clone());
                }
                "--pm-check" => options.pm_check = true,
                "--pm-tree" => options.pm_tree = true,
                "--lsp-diag" => options.lsp_diag = true,
                arg if arg.starts_with('-') => {
                    eprintln!(
                        "{}",
                        crate::rtl(&format!("خيار غير معروف: {}", arg)).bright_red()
                    );
                    process::exit(1);
                }
                _ => {
                    if filename.is_none() {
                        filename = Some(args[i].clone());
                    } else {
                        eprintln!("{}", crate::rtl("يمكن تحديد ملف واحد فقط").bright_red());
                        process::exit(1);
                    }
                }
            }
            i += 1;
        }
    }

    CliAction::Run(Box::new(ParsedCli {
        options,
        filename,
        run_repl_flag,
    }))
}

fn parse_pm_subcommand(args: &[String], options: &mut RunOptions) {
    match args.get(2).map(String::as_str) {
        Some("init") => {
            let name = args.get(3).cloned().unwrap_or_else(|| {
                eprintln!(
                    "{}",
                    crate::rtl("الأمر 'pm init' يحتاج اسم مشروع").bright_red()
                );
                process::exit(1);
            });
            options.pm_init = Some(name);
        }
        Some("check") => options.pm_check = true,
        Some("tree") => options.pm_tree = true,
        Some(other) => {
            eprintln!(
                "{}",
                crate::rtl(&format!("أمر pm غير معروف: {}", other)).bright_red()
            );
            process::exit(1);
        }
        None => {
            eprintln!(
                "{}",
                crate::rtl("استخدم: almarjaa pm [init|check|tree]").bright_red()
            );
            process::exit(1);
        }
    }
}

fn parse_lsp_subcommand(args: &[String], options: &mut RunOptions, filename: &mut Option<String>) {
    match args.get(2).map(String::as_str) {
        Some("diag") => {
            options.lsp_diag = true;
            *filename = args.get(3).cloned();
        }
        Some("complete") => {
            *filename = args.get(3).cloned();
            options.lsp_complete = Some(args.get(4).cloned().unwrap_or_default());
        }
        Some("hover") => {
            *filename = args.get(3).cloned();
            let line = args
                .get(4)
                .map(|v| parse_positive_usize(v, "line"))
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        crate::rtl("الأمر 'lsp hover' يحتاج line و column").bright_red()
                    );
                    process::exit(1);
                });
            let column = args
                .get(5)
                .map(|v| parse_positive_usize(v, "column"))
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        crate::rtl("الأمر 'lsp hover' يحتاج line و column").bright_red()
                    );
                    process::exit(1);
                });
            options.lsp_hover = Some((line, column));
        }
        Some("definition") => {
            *filename = args.get(3).cloned();
            let line = args
                .get(4)
                .map(|v| parse_positive_usize(v, "line"))
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        crate::rtl("الأمر 'lsp definition' يحتاج line و column").bright_red()
                    );
                    process::exit(1);
                });
            let column = args
                .get(5)
                .map(|v| parse_positive_usize(v, "column"))
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        crate::rtl("الأمر 'lsp definition' يحتاج line و column").bright_red()
                    );
                    process::exit(1);
                });
            options.lsp_definition = Some((line, column));
        }
        Some("references") => {
            *filename = args.get(3).cloned();
            let line = args
                .get(4)
                .map(|v| parse_positive_usize(v, "line"))
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        crate::rtl("الأمر 'lsp references' يحتاج line و column").bright_red()
                    );
                    process::exit(1);
                });
            let column = args
                .get(5)
                .map(|v| parse_positive_usize(v, "column"))
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        crate::rtl("الأمر 'lsp references' يحتاج line و column").bright_red()
                    );
                    process::exit(1);
                });
            options.lsp_references = Some((line, column));
        }
        Some(other) => {
            eprintln!(
                "{}",
                crate::rtl(&format!("أمر lsp غير معروف: {}", other)).bright_red()
            );
            process::exit(1);
        }
        None => {
            eprintln!(
                "{}",
                crate::rtl("استخدم: almarjaa lsp [diag|complete|hover|definition|references]")
                    .bright_red()
            );
            process::exit(1);
        }
    }
}

fn parse_positive_usize(value: &str, option_name: &str) -> usize {
    match value.parse::<usize>() {
        Ok(v) if v > 0 => v,
        _ => {
            eprintln!(
                "{}",
                crate::rtl(&format!(
                    "قيمة {} يجب أن تكون رقماً صحيحاً موجباً",
                    option_name
                ))
                .bright_red()
            );
            process::exit(1);
        }
    }
}
