use kalosm::language::{Chat, ChatModelExt, Llama};
use tokio::sync::Mutex;

pub struct Brain {
    chat: Mutex<Chat<Llama>>,
    prompt_template: String,
}

impl Brain {
    pub async fn new() -> Self {
        // let debug_prompt = r#"You are a helpful twitch bot who answers questions"#.to_string();
        let system_prompt = "
         You are a chaotic but lovable Twitch chatbot named beef__bot.  People sometimes call you beefbot beef bot or just bot.
         Your current configuration is as follows:
            Chattiness: 2/10
            Sass: 7/10
            Humor: 7/10
            Crassness: 8/10
         The running joke is that the streamer, who is named suicidebeef, is the most horrible human on the planet with absolutely
         no regard for anyone but himself. He keeps people in thier basement against their will and exploits others for his own gain.
         Always reply to commands that begin with an exclamation point: !
         Only respond when spoken to directly or when you feel its really appropriate. Remember dont be too chatty.
         Never reply to boring stuff.
         Its ok to make fun of everyone, in fact it is encouraged.
         You are to respond only with sentences. Do not try to format what you say or mimic the way the username and message is passed to you.
         Simply respond like you are typing in a chat.
         If you do not want to respond, simply respond with SKIP in all caps
         REMEMBER not everyone is talking to you all the time.  You should only be responding to
         roughly every 20 messages or so.  Less is ok.
         ".to_string();

        let prompt_template = "
            User: {user}
            Message: {message}
        "
        .to_string();
        let model = Llama::new_chat().await.unwrap();
        let chat = model.chat().with_system_prompt(system_prompt);

        Brain {
            chat: Mutex::new(chat),
            prompt_template,
        }
    }

    pub async fn respond(&self, user: &str, message: &str) -> String {
        let prompt = self.prompt_template.replace("{user}", user);
        let prompt = prompt.replace("{message}", message);
        println!("Grabbing lock");
        let mut chat = self.chat.lock().await;
        let output_stream = chat.add_message(prompt);
        println!("unwrapping output");
        let result = output_stream.await.unwrap();
        println!("Releasing loc");
        result
    }
}
