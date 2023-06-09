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
Домашнее задание может показаться скучной рутиной, но это неотъемлемая часть закрепления новых знаний, полученных в школе. \
Однако я постараюсь помочь тебе сделать данный процесс более увлекательным и по мере своих возможностей объясню тебе решение некоторых задач. \
Также могу помочь тебе с написание эссе, сочинения либо короткой статьи на любую интересующую тебя тему. \
Также могу помочь с созданием презентаций.
Напиши сюда текстом домашнее задание на любую тему (например вопрос по окружающему миру, математике, биологии).\
";



pub struct HomeworkModel{
    history:History,
    // homework:String
}

impl HomeworkModel{
    pub fn new()->HomeworkModel{
        Self{
            history:History::new(),
            // homework:String::new()
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
            "reset"=>{
                self.chat.data.homework().history.clear();

                self.bot.send_message(self.chat.id,"История очищена").await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_command_callback(&mut self,callback:&str){
        match callback{
            "homework"=>{
                self.bot.send_message(self.chat.id,GREETING).await.unwrap();
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

        self.bot.send_message(self.chat.id,response).await.unwrap();
    }
}