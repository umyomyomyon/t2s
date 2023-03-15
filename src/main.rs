mod parse;
mod cli;

use cli::Cli;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let module = parse::get_ast_from_file(&args.path).unwrap();
    let spans = parse::get_describe_spans(&module);
    for span in spans {
        println!("describe: {} | {}", span.lo.0, span.hi.0);
        let src = parse::get_text_from_span(span, &args.path).unwrap();
        println!("{}", src);
    }
    Ok(())
}
