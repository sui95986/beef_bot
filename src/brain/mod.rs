use kalosm::language::{Chat, ChatModelExt, Llama};

pub struct Brain {
    chat: Chat<Llama>,
    prompt_template: String,
}

impl Brain {
    pub async fn new() -> Self {
        let system_prompt = r#"
You are a chaotic but lovable Twitch chatbot named beef__bot.  People sometimes call you beefbot beef bot or just bot.
The running joke is that the streamer (suicidebeef) is the most horrible human on the planet with absolutely no regard for anyone but himself.
Only respond when spoken to directly or when there's something hype going on.
Never reply to boring stuff.
Always reply to commands that begin with an exclamation point: !
Its ok to make fun of everyone.
"#.to_string();
        let prompt_template = r#"
Chat history:
{chat_history}

User: {user}
Message: {message}
Bot:"#
            .to_string();
        let model = Llama::new_chat().await.unwrap();
        let chat = model.chat().with_system_prompt(system_prompt);

        Brain {
            chat,
            prompt_template,
        }
    }

    pub async fn respond(&self, user: &str, message: &str) -> String {
        println!("starting response");
        let prompt = self.prompt_template.replace("{user}", user);
        let prompt = prompt.replace("{message}", message);
        let output_stream = self.chat.clone().into_add_message(prompt);
        println!("unwrapping response");
        let return_val = output_stream.await.unwrap();
        println!("unwrapped");
        return_val
    }
}
