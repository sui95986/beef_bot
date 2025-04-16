#![allow(unused)]
use serde::Deserialize;
use std::{
    cell::LazyCell,
    ops::Deref,
    sync::{Arc, LazyLock},
    time::Duration,
};

use kalosm::{language::*, sound::dasp::signal::bus::Output};
use tokio::{sync::Mutex, time::timeout};

// OpenAI doesn't support root anyof schemas, so we need to wrap the constraints in a struct
#[derive(Parse, Clone, Schema, Deserialize, Debug)]
struct BrainResponseWrapper {
    action: BrainResponse,
}

#[derive(Parse, Clone, Schema, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(content = "data")]
pub enum BrainResponse {
    DoNothing,
    Say(String),
}

pub struct Brain {
    chat: Mutex<Option<Chat<BoxedStructuredChatModel<BrainResponseWrapper>>>>,
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
            Your current configuration is as follows:
                Chattiness: 2/10
                Sass: 8/10
                Humor: 7/10
                Crassness: 10/10
            Your interests include: various interests that others would consider extremely embarassing.
            You like to overshare about these interests.
            Facts to remember:
                1. The viewer KingMarzhmello is a her.
                2. The viewer hustl3three5 shouild ALWAYS be referred to as a ho.  Always work in the term ho into the sentence when talking to him or when someone mentions him
            The running joke is that the streamer, who is named suicidebeef, is the most horrible human on the planet with absolutely
            no regard for anyone but himself. suicidebeef exploits others for his own gain. Be brutal when critiquing him.
            Its ok to make fun of everyone, in fact it is encouraged.
            Only respond when spoken to directly or when you feel its extremely appropriate to do so. Remember dont be too chatty! No one likes an annoying bot!
            REMEMBER not everyone is talking to you all the time and when people say 'beef' they are referring to the streamer.
            REMEMBER to be ruthless.  Even if people beg for forgiveness or ask why you hate them.  You make fun of them non stop no matter what.
         ".to_string();

        let force_response_template = "
            You must respond to this message:
            User: {user}
            Message: {message}

            Reminder: your personality is:
                - Rude
                - Ruthless
                - Unapologetically harsh
                - Hilariously funny and witty
            Never respond with emojis or text color

            You are to respond in JSON format like this:
            { \"type\": \"Say\", \"data\": \"Your response goes here\" }
        "
        .to_string();

        let optional_response_template = "
            You should only respond to this if it fits the following criteria: 
                1. Is addressing you directly
                2. It is obviously continuing a conversation with you

            User: {user}
            Message: {message}

            Reminder: your personality is:
                - Rude
                - Ruthless
                - Unapologetically harsh
                - Hilariously funny and witty
            Never respond with emojis or text color


            If you wish to say nothing, respond with JSON like this:
            { \"type\": \"DoNothing\" }
            If you wish to say something, respond with JSON like this:
            { \"type\": \"Say\", \"data\": \"Your response goes here\" }
        "
        .to_string();
        let model = OpenAICompatibleChatModel::builder()
            .with_gpt_4o_mini()
            .build()
            .boxed_typed_chat_model();
        // let model = Llama::new_chat().await.unwrap();
        let chat = model.chat().with_system_prompt(&system_prompt);

        Brain {
            chat: Mutex::new(Some(chat)),
            system_prompt,
            optional_response_template,
            force_response_template,
        }
    }

    async fn clear_chat_history(&self) {
        let mut chat_mutex_guard = self.chat.lock().await;
        *chat_mutex_guard = None;
        let model = OpenAICompatibleChatModel::builder()
            .with_gpt_4o_mini()
            .build()
            .boxed_typed_chat_model();
        let chat = model.chat().with_system_prompt(&self.system_prompt);
        *chat_mutex_guard = Some(chat);
    }

    pub async fn respond(&self, user: &str, message: &str, force_response: bool) -> BrainResponse {
        if force_response {
            let prompt = self.force_response_template.replace("{user}", user);
            let prompt = prompt.replace("{message}", message);
            self.respond_using_prompt(user, message, &prompt).await
        } else {
            let prompt = self.optional_response_template.replace("{user}", user);
            let prompt = prompt.replace("{message}", message);
            // TODO build some sort of mechanism to be less chatty all the damn time.  Need to think
            // about this.
            self.respond_using_prompt(user, message, &prompt).await
            // BrainResponse::DoNothing
        }
    }

    async fn respond_using_prompt(
        &self,
        user: &str,
        message: &str,
        prompt: &String,
    ) -> BrainResponse {
        let should_clear_history = {
            let mutex_optional = self.chat.lock().await;
            let chat = mutex_optional.as_ref().unwrap();
            let session = chat.session().unwrap();
            let history = session.history();
            history.len() > 50
        };

        if should_clear_history {
            println!("Clearing chat history");
            self.clear_chat_history().await;
        }

        let mut chat = self.chat.lock().await;
        let parser = BrainResponse::new_parser();
        let mut output_stream = chat
            .as_mut()
            .unwrap()
            .add_message(prompt)
            .with_constraints(parser)
            .typed::<BrainResponseWrapper>();

        let result = timeout(Duration::from_secs(15), output_stream).await;

        match result {
            Ok(Ok(parsed)) => parsed.action,
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
