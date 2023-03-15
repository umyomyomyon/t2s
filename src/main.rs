mod parse;
mod cli;
mod output;
mod api;

use cli::Cli;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let module = parse::get_ast_from_file(&args.path).unwrap();
    let imports = parse::get_import_spans(&module);
    let import_text = parse::make_import_decl_text(&imports, &args.path).unwrap();
    let spans = parse::get_describe_spans(&module);
    let code = import_text + parse::get_text_from_span(&spans[0], &args.path).unwrap().as_str();
    println!("{}", code);
    let response = api::request_chatgpt(&code).await.unwrap();
    for choise in response.choices {
        output::make_file(&choise.message.content).unwrap();
    }

    Ok(())
}
