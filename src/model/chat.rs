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
    bot:Bot,
    chat:&'c mut Chat,
}

impl<'c> ChatHandler<'c>{
    pub fn new(bot:Bot,chat:&'c mut Chat)->ChatHandler<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(&mut self,command:&str){
        match command{
            "chat"=>{
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

        let data=self.chat.data.chat();

        let prompt=std::fs::read_to_string("prompts/chat.json").unwrap().replace('\n',"");
        messages.push(prompt);

        data.history.push_request(message);

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