use std::env;

pub mod args;
pub mod commands;
pub mod repl;

pub fn run_from_env(version: &str) {
    let args: Vec<String> = env::args().collect();
    run(&args, version);
}

pub fn run(args: &[String], version: &str) {
    match args::parse_args(args) {
        args::CliAction::Help => commands::print_help(),
        args::CliAction::Version => commands::print_version(version),
        args::CliAction::Run(parsed) => {
            let parsed = *parsed;
            if commands::handle_package_commands(&parsed.options) {
                return;
            }

            if commands::handle_lsp_commands(&parsed.options, parsed.filename.as_deref()) {
                return;
            }

            if let Some(fname) = parsed.filename {
                commands::run_file(&fname, &parsed.options);
            } else if parsed.run_repl_flag || args.len() == 1 {
                repl::run_repl();
            }
        }
    }
}
