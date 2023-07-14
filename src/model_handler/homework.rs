use crate::{
    synthesize_voice_button,
    resources::{
        Resources,
        Prompt
    },
    openai::OpenAIWrapper,
    extended_bot_interface::ExtendedBotInterface,
    chat::{
        Chat,
        History
    }
};

use teloxide_core::requests::Requester;



pub struct HomeworkModel{
    history:History
}

impl HomeworkModel{
    pub fn new()->HomeworkModel{
        Self{
            history:History::new()
        }
    }
}



pub struct HomeworkHandler<'c>{
    bot:ExtendedBotInterface,
    chat:&'c mut Chat,
}

impl<'c> HomeworkHandler<'c>{
    pub fn new(bot:ExtendedBotInterface,chat:&'c mut Chat)->HomeworkHandler<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(&mut self,command:&str){
        match command{
            "reset"=>{
                self.chat.data.homework().history.clear();

                self.bot.send_message(self.chat.id,"История очищена").reply_markup(synthesize_voice_button(None)).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_command_callback(&mut self,callback:&str){
        match callback{
            "homework"=>{
                let greeting=Resources::load_text("resources/text/study/homework/greeting.txt").await;

                self.bot.send_message(self.chat.id,greeting).reply_markup(synthesize_voice_button(None)).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(&mut self,message:&str){
        let sent_message=self.bot.send_message(self.chat.id,"Набирает сообщение...").await.unwrap();

        let mut messages=Vec::new();

        let data=self.chat.data.homework();

        data.history.push_request(message);

        let prompt=Prompt::load("resources/prompts/study/homework/homework.json").await;
        messages.push(prompt.text);

        for message in data.history.messages().iter(){
            messages.push(message.to_string())
        }

        let response=OpenAIWrapper::send_chat(&messages,1024).await;

        self.chat.user.token_usage.total+=response.usage;

        data.history.push_response(&response.message);

        self.bot.bot.delete_message(self.chat.id,sent_message.id).await.unwrap();
        self.bot.send_message(self.chat.id,response.message).reply_markup(synthesize_voice_button(None)).await.unwrap();
    }
}