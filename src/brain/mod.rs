#![allow(unused)]
use std::{
    cell::LazyCell,
    sync::{Arc, LazyLock},
};

use kalosm::{language::*, sound::dasp::signal::bus::Output};
use tokio::sync::Mutex;

#[derive(Parse, Clone)]
pub enum BrainResponse {
    DoNothing,
    Say(String),
}

pub struct Brain {
    chat: Mutex<Chat<Llama>>,
    prompt_template: String,
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

            You are to respond in JSON format like this:
            { \"type\": \"DoNothing\" } or { \"type\": \"Say\", \"data\": \"Your response goes here\" }
         ".to_string();

        let prompt_template = "
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
            prompt_template,
        }
    }

    pub async fn respond(&self, user: &str, message: &str) -> BrainResponse {
        let parser = BrainResponse::new_parser();
        let prompt = self.prompt_template.replace("{user}", user);
        let prompt = prompt.replace("{message}", message);
        println!("Grabbing lock");
        let mut chat = self.chat.lock().await;
        let output_stream = chat.add_message(prompt).with_constraints(parser);
        println!("unwrapping output");
        let result = output_stream.await.unwrap();
        println!("Releasing lock");
        result
    }
}
