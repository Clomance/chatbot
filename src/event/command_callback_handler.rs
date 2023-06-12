use crate::{
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

use teloxide_core::Bot;



pub struct CommandCallbackHandler{
    bot:Bot,
    chat:Arc<Mutex<Chat>>
}

impl CommandCallbackHandler{
    pub fn new(bot:Bot,chat:Arc<Mutex<Chat>>)->CommandCallbackHandler{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle(self,callback:&str){
        let mut chat=self.chat.lock().await;

        match chat.state{
            ChatState::Initial=>match callback{
                "start"=>{
                    chat.state=ChatState::FullyFunctional(ChatMode::Chat);

                    ChatHandler::new(self.bot,&mut chat).handle_command_callback(callback).await;
                }

                _=>{}
            }

            ChatState::PartiallyFunctional=>{}

            ChatState::FullyFunctional(mode)=>match mode{
                ChatMode::Chat=>ChatHandler::new(self.bot,&mut chat).handle_command_callback(callback).await,

                ChatMode::Study(mode)=>match mode{
                    StudyMode::None=>{
                        match callback{
                            "tutor"=>{
                                chat.set_mode(ChatMode::Study(StudyMode::Tutor));

                                TutorHandler::new(self.bot,&mut chat).handle_command_callback(callback).await
                            }

                            "homework"=>{
                                chat.set_mode(ChatMode::Study(StudyMode::Homework));

                                HomeworkHandler::new(self.bot,&mut chat).handle_command_callback(callback).await
                            }

                            _=>{}
                        }
                    }

                    StudyMode::Tutor=>TutorHandler::new(self.bot,&mut chat).handle_command_callback(callback).await,

                    StudyMode::Homework=>HomeworkHandler::new(self.bot,&mut chat).handle_command_callback(callback).await,
                }

                ChatMode::Game=>GameHandler::new(self.bot,&mut chat).handle_command_callback(&callback).await,

                ChatMode::Playground=>PlaygroundHandler::new(self.bot,&mut chat).handle_command_callback(callback).await
            }
        }
    }
}