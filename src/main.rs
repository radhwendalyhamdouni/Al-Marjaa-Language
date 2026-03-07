mod cli;

pub(crate) fn rtl(text: &str) -> String {
    format!("\u{200F}{}", text)
}

fn main() {
    cli::run_from_env(almarjaa::VERSION);
}
