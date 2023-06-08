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
    bot:Bot,
    chat:&'c mut Chat,
}

impl<'c> HomeworkHandler<'c>{
    pub fn new(bot:Bot,chat:&'c mut Chat)->HomeworkHandler<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(&mut self,command:&str){
        match command{
            "homework"=>{
                self.bot.send_message(self.chat.id,"Здесь мы можем просто пообщаться").await.unwrap();
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

        let data=self.chat.data.homework();

        data.history.push_request(message);

        let prompt=std::fs::read_to_string("prompts/study/homework/homework.json").unwrap().replace('\n',"");
        messages.push(prompt);

        for message in data.history.messages().iter(){
            messages.push(message.to_string())
        }

        let response=OpenAIWrapper::send_chat(&messages,1024);

        println!("Response: {}",response);

        data.history.push_response(
            &response
            .replace("\"","\\\"")
            .replace("\n","\\n")
            .replace("\r","\\r")
            .replace("\t","\\t")
        );

        self.bot.send_message(self.chat.id,response).await.unwrap();
    }
}