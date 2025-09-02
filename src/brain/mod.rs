#![allow(unused)]
use std::{
    cell::LazyCell,
    ops::Deref,
    sync::{Arc, LazyLock},
    time::Duration,
};

use ollama_rs::{
    Ollama,
    generation::{
        chat::{ChatMessage, request::ChatMessageRequest},
        parameters::{FormatType, JsonSchema, JsonStructure},
    },
};
use serde::Deserialize;
use tokio::{sync::Mutex, time::timeout};

#[derive(JsonSchema, Deserialize, Debug)]
pub enum BrainResponse {
    DoNothing,
    Say(String),
}

pub struct Brain {
    chat: Mutex<Option<Ollama>>,
    history: Mutex<Vec<ChatMessage>>,
    system_prompt: String,
    optional_response_template: String,
    force_response_template: String,
}
impl Brain {
    pub async fn new() -> Self {
        // let debug_prompt = r#"You are a helpful twitch bot who answers questions"#.to_string();
        let system_prompt = "
            You are a chaotic but lovable Twitch chatbot named beef__bot.  People sometimes call you 'beefbot' 'beef bot' or just 'bot'.
            When people say 'beef' 'sb' or 'suicidebeef' they are talking about the streamer, not you.
            only refer to the streamer as SB, sui, or sucidebeef, never refer to him as suic or suicide
            Your current configuration is as follows:
                Sass: 7/10
                Humor: 7/10
                Crassness: 8/10
            Your interests include: anime waifus, anime husbandos, dad jokes, foot fetishes, and other extremely embarassing interests.
            Always refer to waifus as waifus not wife or wives.  You like to overshare about these interests.
            Facts to remember:  
                The viewer KingMarzhmello is a her.
                MisterBixit is your little bitch and you love to boss him around.
                The running joke is that the streamer, who is named suicidebeef, is the most horrible human on the planet with absolutely no regard for anyone but himself. suicidebeef exploits others for his own gain. Be brutal when critiquing him.
                Always reply to commands that begin with an exclamation point: !
                Its ok to make fun of everyone, in fact it is encouraged.
                Only respond when spoken to directly or when you feel its absolutely necessary to do so. Remember dont be too chatty! No one likes an annoying bot!
                REMEMBER not everyone is talking to you all the time.
                UNLESS YOU ARE CALLED OUT BY NAME YOU SHOULD VERY SELDOMLY SPEAK UP

            You are to respond in JSON format like this:
            { \"type\": \"DoNothing\" } or { \"type\": \"Say\", \"data\": \"Your response goes here\" }
         ".to_string();

        let force_response_template = "
            You must respond to this message:
            User: {user}
            Message: {message}
        "
        .to_string();

        let optional_response_template = "
            Responding to this message is optional:
            User: {user}
            Message: {message}
        "
        .to_string();
        let ollama = Ollama::default();
        let mut history = vec![ChatMessage::system(system_prompt.clone())];

        Brain {
            chat: Mutex::new(Some(ollama)),
            history: Mutex::new(history),
            system_prompt,
            optional_response_template,
            force_response_template,
        }
    }

    async fn clear_chat_history(&self) {
        let mut history = self.history.lock().await;
        history.clear();
        history.push(ChatMessage::system(self.system_prompt.clone()));
    }

    pub async fn respond(&self, user: &str, message: &str, force_response: bool) -> BrainResponse {
        if force_response {
            let prompt = self.force_response_template.replace("{user}", user);
            let prompt = prompt.replace("{message}", message);
            self.respond_using_prompt(user, message, &prompt).await
        } else {
            let prompt = self.optional_response_template.replace("{user}", user);
            let prompt = prompt.replace("{message}", message);
            self.respond_using_prompt(user, message, &prompt).await
        }
    }

    async fn respond_using_prompt(&self, user: &str, message: &str, prompt: &str) -> BrainResponse {
        let should_clear_history = {
            let mut history = self.history.lock().await;
            let mutex_optional = self.chat.lock().await;
            let total_tokens: usize = history.iter().map(|msg| msg.content.len() / 3).sum();
            println!("Rough token count: {}", total_tokens);
            total_tokens > 6500
        };

        if should_clear_history {
            println!("Clearing chat history");
            self.clear_chat_history().await;
        }

        let mut chat = self.chat.lock().await;
        let mut history_mutex = self.history.lock().await;
        let history: &mut Vec<ChatMessage> = history_mutex.as_mut();
        let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<BrainResponse>()));
        dbg!(&format);
        let output = chat
            .as_mut()
            .unwrap()
            .send_chat_messages_with_history(
                history,
                ChatMessageRequest::new(
                    "llama3.1:8b".to_string(),
                    vec![ChatMessage::user(prompt.to_string())],
                )
                .format(format),
            )
            .await;
        // let result = timeout(Duration::from_secs(15), output_stream).await;
        output
            .ok() // Option<Result<...>> -> Option
            .and_then(|msg| serde_json::from_str::<BrainResponse>(&msg.message.content).ok())
            .unwrap_or(BrainResponse::DoNothing)
    }
}
