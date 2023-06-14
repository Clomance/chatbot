use crate::{
    yandex::YandexSpeechKit,
    chat::{
        ChatMode,
        Chat,
        ChatState,
        StudyMode
    },
    model::{
        GameHandler,
        TutorHandler,
        ChatHandler,
        HomeworkHandler,
        PlaygroundHandler
    }
};

use std::sync::Arc;

use tokio::sync::Mutex;

use teloxide_core::{
    Bot,
    requests::Requester,
    types::{
        Message,
        InputFile
    }
};



pub struct CommandCallbackHandler{
    bot:Bot,
    callback:String,
    message:Message,
    chat:Arc<Mutex<Chat>>
}

impl CommandCallbackHandler{
    pub fn new(bot:Bot,callback:String,message:Message,chat:Arc<Mutex<Chat>>)->CommandCallbackHandler{
        Self{
            bot,
            callback,
            message,
            chat
        }
    }

    pub async fn handle(self){
        let mut chat=self.chat.lock().await;

        match self.callback.as_str(){
            "synthesize"=>{
                println!("Synthesize");
                if let Some(_text)=self.message.text(){
                    println!("Sent");
                    let voice=YandexSpeechKit::synthesize("Привет").await;

                    if voice.is_empty(){
                        self.bot.send_message(chat.id,"Ошибка").await.unwrap();
                    }
                    else{
                        self.bot.send_voice(chat.id,InputFile::memory(voice)).await.unwrap();
                    }
                }

                return
            }

            _=>{}
        }

        match chat.state{
            ChatState::Initial=>match self.callback.as_str(){
                "start"=>{
                    chat.state=ChatState::FullyFunctional(ChatMode::Chat);

                    ChatHandler::new(self.bot,&mut chat).handle_command_callback(&self.callback).await;
                }

                _=>{}
            }

            ChatState::PartiallyFunctional=>{}

            ChatState::FullyFunctional(mode)=>{
                match mode{
                    ChatMode::Chat=>ChatHandler::new(self.bot,&mut chat).handle_command_callback(&self.callback).await,

                    ChatMode::Study(mode)=>match mode{
                        StudyMode::None=>{
                            match self.callback.as_str(){
                                "tutor"=>{
                                    chat.set_mode(ChatMode::Study(StudyMode::Tutor));

                                    TutorHandler::new(self.bot,&mut chat).handle_command_callback(&self.callback).await
                                }

                                "homework"=>{
                                    chat.set_mode(ChatMode::Study(StudyMode::Homework));

                                    HomeworkHandler::new(self.bot,&mut chat).handle_command_callback(&self.callback).await
                                }

                                _=>{}
                            }
                        }

                        StudyMode::Tutor=>TutorHandler::new(self.bot,&mut chat).handle_command_callback(&self.callback).await,

                        StudyMode::Homework=>HomeworkHandler::new(self.bot,&mut chat).handle_command_callback(&self.callback).await,
                    }

                    ChatMode::Game=>GameHandler::new(self.bot,&mut chat).handle_command_callback(&self.callback).await,

                    ChatMode::Playground=>PlaygroundHandler::new(self.bot,&mut chat).handle_command_callback(&self.callback).await
                }
            }
        }
    }
}