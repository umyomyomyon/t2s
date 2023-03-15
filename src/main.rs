mod parse;
mod cli;
mod output;

use cli::Cli;
use clap::Parser;

use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs,
        Role,
        ChatCompletionRequestMessage
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let module = parse::get_ast_from_file(&args.path).unwrap();
    let spans = parse::get_describe_spans(&module);
    
    let code = parse::get_text_from_span(&spans[0], &args.path).unwrap();
    let messages = make_messages(&code);
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(521u16)
        .model("gpt-3.5-turbo")
        .messages(messages.unwrap())
        .build()?;
    let response = client.chat().create(request).await?;
    for choise in response.choices {
        output::make_file(&choise.message.content).unwrap();
    }

    Ok(())
}

fn make_messages(code: &String) -> Result<Vec<ChatCompletionRequestMessage>, Box<dyn std::error::Error>>{
    Ok(vec![
        ChatCompletionRequestMessageArgs::default()
            .role(Role::System)
            .content("You are a programmer's tool.")
            .build()?,
        ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content("Given a test code, please come up with code that meets the requirements.No need for anything other than a code block in your response.")
            .build()?,
        ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(code)
            .build()?,
    ])
}
