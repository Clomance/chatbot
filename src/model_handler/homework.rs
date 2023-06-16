use crate::{
    synthesize_voice_keyboard,
    prompt::Prompt,
    openai::OpenAIWrapper,
    chat::{
        Chat,
        History
    },
    extended_bot_interface::ExtendedBotInterface,
    resources::Resources
};



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

                self.bot.send_message(self.chat.id,"История очищена",Some(synthesize_voice_keyboard(None))).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_command_callback(&mut self,callback:&str){
        match callback{
            "homework"=>{
                let greeting=Resources::load_text("resources/text/study/homework/greeting.txt").await;

                self.bot.send_message(self.chat.id,greeting,Some(synthesize_voice_keyboard(None))).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(&mut self,message:&str){
        let mut messages=Vec::new();

        let data=self.chat.data.homework();

        data.history.push_request(message);

        let prompt=Prompt::load("prompts/study/homework/homework.json").await;
        messages.push(prompt.text);

        for message in data.history.messages().iter(){
            messages.push(message.to_string())
        }

        let response=OpenAIWrapper::send_chat(&messages,1024);

        println!("Response: {}",response);

        data.history.push_response(&response);

        self.bot.send_message(self.chat.id,response,Some(synthesize_voice_keyboard(None))).await.unwrap();
    }
}