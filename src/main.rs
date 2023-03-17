mod parse;
mod cli;
mod output;
mod api;

use cli::Cli;
use clap::Parser;
use futures::future;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let module = parse::get_ast_from_file(&args.path).unwrap();
    let imports = parse::get_import_spans(&module);
    let import_text = parse::make_import_decl_text(&imports, &args.path).unwrap();
    let spans = parse::get_describe_spans(&module);


    let requests = spans.iter().map(|span| {
        let code = import_text.clone() + parse::get_text_from_span(&span, &args.path).unwrap().as_str();
        async move {
            api::request_chatgpt(&code).await.unwrap()
        }
    });

    let results = future::join_all(requests).await;
    results.iter().enumerate().for_each(|(i, response)| {
        response.choices.iter().for_each(|choise| {
            output::make_file(&choise.message.content, i.to_string()).unwrap();
        });
    });

    Ok(())
}
