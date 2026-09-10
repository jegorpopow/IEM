use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, about)]
#[command(
    bin_name = "cargo run --bin interpret --",
    arg_required_else_help = true,
    help_expected = true
)]
#[derive(Debug)]
pub(crate) struct Cli {
    /// File to run
    pub path: PathBuf,
}

impl Cli {
    #[must_use]
    #[expect(clippy::same_name_method, reason = "hiding clap under the rug")]
    pub(crate) fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
