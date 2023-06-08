use crate::{
    prompt::Prompt,
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
Привет, дорогой! Меня создавали в первую очередь быть твоим верным другом. \
Я могу выслушать тебя, дать совет, подбодрить, поговорить с тобой на любые волнующие тебя темы.

Что ты хочешь обсудить, о чем хочешь рассказать?\
";



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
                self.bot.send_message(self.chat.id,GREETING).await.unwrap();
            }

            "reset"=>{
                self.chat.data.chat().history.clear();

                self.bot.send_message(self.chat.id,"История очищена").await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_command_callback(&mut self,callback:&str){
        match callback{
            "start"=>{
                self.bot.send_message(self.chat.id,GREETING).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(&mut self,message:&str){
        let mut messages=Vec::new();

        let data=self.chat.data.chat();

        let prompt=Prompt::load("prompts/chat.json").await;
        messages.push(prompt.text);

        data.history.push_request(message);

        for message in data.history.messages().iter(){
            messages.push(message.to_string())
        }

        let response=OpenAIWrapper::send_chat(&messages,1024);

        println!("Response: {}",response);

        data.history.push_response(&response);

        self.bot.send_message(self.chat.id,response).await.unwrap();
    }
}