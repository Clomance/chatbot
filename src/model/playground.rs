use crate::{
    openai::OpenAIWrapper,
    chat::{
        Chat,
        History
    }
};

use teloxide_core::{
    Bot,
    requests::Requester
};



const GREETING:&'static str="\
Режим без ограничений\
";



pub struct PlaygroundModel{
    history:History
}


impl PlaygroundModel{
    pub fn new()->PlaygroundModel{
        Self{
            history:History::new()
        }
    }
}



pub struct PlaygroundHandler<'c>{
    bot:Bot,
    chat:&'c mut Chat,
}

impl<'c> PlaygroundHandler<'c>{
    pub fn new(bot:Bot,chat:&'c mut Chat)->PlaygroundHandler<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(&mut self,command:&str){
        match command{
            "playground"=>{
                self.bot.send_message(self.chat.id,GREETING).await.unwrap();
            }

            "reset"=>{
                self.chat.data.playground().history.clear();

                self.bot.send_message(self.chat.id,"История очищена").await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_command_callback(&mut self,callback:&str){
        match callback{
            _=>{}
        }
    }

    pub async fn handle_message(&mut self,message:&str){
        let mut messages=Vec::new();

        let data=self.chat.data.playground();

        data.history.push_request(message);

        for message in data.history.messages().iter(){
            messages.push(message.to_string())
        }

        let response=OpenAIWrapper::send_chat(&messages,1024);

        data.history.push_response(&response);

        self.bot.send_message(self.chat.id,response).await.unwrap();
    }
}