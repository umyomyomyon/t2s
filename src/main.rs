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
    let spans = parse::get_describe_spans(&module);
    let code = parse::get_text_from_span(&spans[0], &args.path).unwrap();
    let response = api::request_chatgpt(&code).await.unwrap();
    for choise in response.choices {
        output::make_file(&choise.message.content).unwrap();
    }

    Ok(())
}
