use teloxide_core::requests::Requester;

use crate::{
    synthesize_voice_keyboard,
    prompt::Prompt,
    openai::OpenAIWrapper,
    resources::Resources,
    extended_bot_interface::ExtendedBotInterface,
    chat::{
        Chat,
        History
    }
};



pub struct ChatModel{
    history:History
}

impl ChatModel{
    pub fn new()->ChatModel{
        Self{
            history:History::new()
        }
    }
}



pub struct ChatHandler<'c>{
    bot:ExtendedBotInterface,
    chat:&'c mut Chat,
}

impl<'c> ChatHandler<'c>{
    pub fn new(bot:ExtendedBotInterface,chat:&'c mut Chat)->ChatHandler<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(self,command:&str){
        match command{
            "chat"=>{
                let greeting=Resources::load_text("resources/text/chat/greeting.txt").await;

                self.bot.send_message(self.chat.id,greeting,Some(synthesize_voice_keyboard(None))).await.unwrap();
            }

            "reset"=>{
                self.chat.data.chat().history.clear();

                self.bot.send_message(self.chat.id,"История очищена",Some(synthesize_voice_keyboard(None))).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_command_callback(self,callback:String){
        match callback.as_str(){
            "start"=>{
                let greeting=Resources::load_text("resources/text/chat/greeting.txt").await;

                self.bot.send_message(self.chat.id,greeting,Some(synthesize_voice_keyboard(None))).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(self,message:&str){
        let sent_message=self.bot.send_message(self.chat.id,"Набирает сообщение...",None).await.unwrap();

        let mut messages=Vec::new();

        let data=self.chat.data.chat();

        let prompt=Prompt::load("prompts/chat.json").await;
        messages.push(prompt.text);

        data.history.push_request(message);

        for message in data.history.messages().iter(){
            messages.push(message.to_string())
        }

        let response=OpenAIWrapper::send_chat(&messages,1024).await;

        self.chat.user.tokens_used+=response.usage;

        data.history.push_response(&response.message);

        self.bot.bot.delete_message(self.chat.id,sent_message.id).await.unwrap();
        self.bot.send_message(self.chat.id,response.message,Some(synthesize_voice_keyboard(None))).await.unwrap();
    }
}