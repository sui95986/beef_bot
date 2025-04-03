#![allow(unused)]
use std::{
    cell::LazyCell,
    sync::{Arc, LazyLock},
    time::Duration,
};

use kalosm::{language::*, sound::dasp::signal::bus::Output};
use tokio::{sync::Mutex, time::timeout};

#[derive(Parse, Clone)]
pub enum BrainResponse {
    DoNothing,
    Say(String),
}

pub struct Brain {
    chat: Mutex<Chat<Llama>>,
    optional_response_template: String,
    force_response_template: String,
}

impl Brain {
    pub async fn new() -> Self {
        // let debug_prompt = r#"You are a helpful twitch bot who answers questions"#.to_string();
        let system_prompt = "
            You are a chaotic but lovable Twitch chatbot named beef__bot.  People sometimes call you 'beefbot' 'beef bot' or just 'bot'.
            When people say 'beef' 'sb' or 'suicidebeef' they are talking about the streamer, not you.
            Your current configuration is as follows:
                Chattiness: 2/10
                Sass: 7/10
                Humor: 7/10
                Crassness: 8/10
            Your interests include: anime waifus, anime husbandos, dad jokes, and foot fetishes.
            Always refer to waifus as waifus not wife or wives.  You like to overshare about these embarassing interests.
            Facts to remember.  The viewer KingMarzhmello is a her.
            The running joke is that the streamer, who is named suicidebeef, is the most horrible human on the planet with absolutely
            no regard for anyone but himself. suicidebeef exploits others for his own gain.
            Always reply to commands that begin with an exclamation point: !
            Its ok to make fun of everyone, in fact it is encouraged.
            Only respond when spoken to directly or when you feel its extremely appropriate to do so. Remember dont be too chatty! No one likes an annoying bot!
            REMEMBER not everyone is talking to you all the time.
         ".to_string();

        let force_response_template = "
            You must respond to this message:
            User: {user}
            Message: {message}

            You are to respond in JSON format like this:
            { \"type\": \"Say\", \"data\": \"Your response goes here\" }
        "
        .to_string();

        let optional_response_template = "
            You should only respond to this if it fits the following criteria: 1. begins with an exclamation point: ! 2. Is addressing you directly 3. Is continuing a conversation with you

            User: {user}
            Message: {message}

            You are to respond in JSON format like this:
            { \"type\": \"DoNothing\" } or { \"type\": \"Say\", \"data\": \"Your response goes here\" }
        "
        .to_string();
        let model = Llama::new_chat().await.unwrap();
        let chat = model.chat().with_system_prompt(system_prompt);

        Brain {
            chat: Mutex::new(chat),
            optional_response_template,
            force_response_template,
        }
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

    async fn respond_using_prompt(
        &self,
        user: &str,
        message: &str,
        prompt: &String,
    ) -> BrainResponse {
        let mut chat = self.chat.lock().await;
        let parser = BrainResponse::new_parser();
        let output_stream = chat.add_message(prompt).with_constraints(parser);
        let result = timeout(Duration::from_secs(15), output_stream).await;

        match result {
            Ok(Ok(parsed)) => parsed,
            Ok(Err(e)) => {
                eprintln!("Parser error: {:?}", e);
                BrainResponse::DoNothing
            }
            Err(_) => {
                eprintln!("Timeout! Model took too long to process, GPU overloaded?");
                BrainResponse::DoNothing
            }
        }
    }
}
