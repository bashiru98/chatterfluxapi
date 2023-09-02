use openai_api_rust::chat::*;
use openai_api_rust::*;

pub async fn generate_code_or_text(prompt: String) -> Result<String, Error> {
    // Load API key from environment OPENAI_API_KEY.
    let auth = Auth::from_env().unwrap();
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-3.5-turbo-0613".to_string(),
        max_tokens: Some(1024),
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        n: Some(2),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![Message {
            role: Role::User,
            content: prompt,
        }],
    };
    let rs = openai.chat_completion_create(&body)?;
    let choice = rs.choices;
    let message = &choice[0].message.as_ref().unwrap();

    Ok(message.content.clone())
}
