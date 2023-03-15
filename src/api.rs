use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs,
        Role,
        ChatCompletionRequestMessage,
        CreateChatCompletionResponse,
    },
    Client,
};

pub async fn request_chatgpt(code: &String) -> Result<CreateChatCompletionResponse , Box<dyn std::error::Error>> {
    let messages = make_messages(&code);
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(521u16)
        .model("gpt-3.5-turbo")
        .messages(messages.unwrap())
        .build()?;
    let response = client.chat().create(request).await?;
    Ok::<CreateChatCompletionResponse, Box<dyn std::error::Error>>(response)
}

fn make_messages(code: &String) -> Result<Vec<ChatCompletionRequestMessage>, Box<dyn std::error::Error>>{
    Ok(vec![
        ChatCompletionRequestMessageArgs::default()
            .role(Role::System)
            .content("You are a programmer's tool.")
            .build()?,
        ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content("
                Given a test code, please come up with code that meets the requirements.\n
                No need for anything other than a code block in your response.\n
                No explanation is necessary.\n
                ")
            .build()?,
        ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(code)
            .build()?,
    ])
}
