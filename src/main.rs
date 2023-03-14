mod parse;
mod cli;

use cli::Cli;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let module = parse::get_ast_from_file(&args.path).unwrap();
    parse::get_text_from_module(&module);
    Ok(())
}
