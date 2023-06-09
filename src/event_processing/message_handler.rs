use crate::{
    event_processing::CommandHandler,
    model::{
        GameHandler,
        TutorHandler,
        ChatHandler,
        HomeworkHandler,
        PlaygroundHandler
    },
    chat::{
        Chat,
        ChatMode,
        ChatState,
        StudyMode
    }
};

use std::sync::Arc;

use tokio::sync::Mutex;

use teloxide_core::{
    Bot,
    types::Message, requests::Requester
};



pub struct MessageHandler{
    bot:Bot,
    message:Message,
    chat:Arc<Mutex<Chat>>
}

impl MessageHandler{
    pub fn new(bot:Bot,message:Message,chat:Arc<Mutex<Chat>>)->MessageHandler{
        Self{
            bot,
            message,
            chat
        }
    }

    pub async fn handle(self){
        let Some(text)=self.message.text()else{return};

        if text.starts_with("/"){
            if let Some(command)=text.get(1..){
                CommandHandler::new(self.bot,self.chat).handle(command).await;
            }
            return
        }

        let mut chat=self.chat.lock().await;

        match &mut chat.state{
            ChatState::Initial=>{
                self.bot.send_message(chat.id,"Напиши /start, чтобы начать").await.unwrap();
            }

            ChatState::PartiallyFunctional=>{}

            ChatState::FullyFunctional(mode)=>{
                let text=text
                    .replace("\"","\\\"")
                    .replace("\n","\\n")
                    .replace("\r","\\r")
                    .replace("\t","\\t");

                match mode{
                    ChatMode::Chat=>ChatHandler::new(self.bot,&mut chat).handle_message(&text).await,

                    ChatMode::Study(mode)=>match mode{
                        StudyMode::None=>{}

                        StudyMode::Tutor=>TutorHandler::new(self.bot,&mut chat).handle_message(&text).await,

                        StudyMode::Homework=>HomeworkHandler::new(self.bot,&mut chat).handle_message(&text).await,
                    }

                    ChatMode::Game=>GameHandler::new(self.bot,&mut chat).handle_message(&text).await,

                    ChatMode::Playground=>PlaygroundHandler::new(self.bot,&mut chat).handle_message(&text).await,
                }
            }
        }
    }
}