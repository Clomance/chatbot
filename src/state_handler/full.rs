use crate::{
    resources::Resources,
    synthesize_voice_button,
    extended_bot_interface::ExtendedBotInterface,
    chat::{
        ChatMode,
        StudyMode,
        Chat
    },
    model_handler::{
        ChatHandler,
        TutorHandler,
        GameHandler,
        HomeworkHandler,
        PlaygroundHandler,
        ProfileHandler
    }
};



use teloxide_core::types::{
    InlineKeyboardMarkup,
    InlineKeyboardButton,
    InlineKeyboardButtonKind
};



pub struct FullState<'c>{
    bot:ExtendedBotInterface,
    chat:&'c mut Chat
}

impl<'c> FullState<'c>{
    pub fn new(bot:ExtendedBotInterface,chat:&'c mut Chat)->FullState<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(self,command:&str){
        match command{
            // Перейти в раздел 'Простое общение'
            "chat"=>{
                if let ChatMode::Chat=self.chat.mode{
                    let the_chosen_mode_is_the_current=Resources::load_text("resources/text/general/the_chosen_mode_is_the_current.txt").await;

                    self.bot.send_message(self.chat.id,the_chosen_mode_is_the_current).reply_markup(synthesize_voice_button(None)).await.unwrap();
                }
                else{
                    self.chat.set_mode(ChatMode::Chat);

                    ChatHandler::new(self.bot,self.chat).handle_command(command).await;
                }
            }

            // Перейти в раздел 'Учёба'
            "study"=>{
                self.chat.set_mode(ChatMode::Study(StudyMode::None));

                self.bot.send_message(self.chat.id,"Выбери режим").reply_markup(study_buttons()).await.unwrap();
            }

            // Перейти в раздел 'Игры'
            "game"=>{
                if let ChatMode::Game=self.chat.mode{
                    GameHandler::new(self.bot,self.chat).handle_command("gamerepeat").await
                }
                else{
                    self.chat.set_mode(ChatMode::Game);

                    GameHandler::new(self.bot,self.chat).handle_command(command).await
                }
            }

            // Перейти в раздел 'Свободный режим'
            "playground"=>{
                if let ChatMode::Playground=self.chat.mode{
                    let the_chosen_mode_is_the_current=Resources::load_text("resources/text/general/the_chosen_mode_is_the_current.txt").await;

                    self.bot.send_message(self.chat.id,the_chosen_mode_is_the_current).reply_markup(synthesize_voice_button(None)).await.unwrap();
                }
                else{
                    self.chat.set_mode(ChatMode::Playground);

                    PlaygroundHandler::new(self.bot,self.chat).handle_command(command).await
                }
            }

            // Перейти в раздел 'Настройки'
            "settings"=>{
                let not_avaiable=Resources::load_text("resources/text/general/not_avaiable.txt").await;

                self.bot.send_message(self.chat.id,not_avaiable).reply_markup(synthesize_voice_button(None)).await.unwrap();
            }

            // Перейти в раздел 'Профиль'
            "profile"=>{
                self.chat.set_mode(ChatMode::Profile);

                ProfileHandler::new(self.bot,self.chat).handle_command(command).await
            }

            // Перейти в раздел 'Дополнительно'
            "etc"=>{
                // self.chat.set_mode(ChatMode::Etc);

                // EtcHandler::new(self.bot,self.chat).handle_command(command).await
            }

            // Остальные команды ("reset")
            _=>match self.chat.mode{
                ChatMode::Chat=>ChatHandler::new(self.bot,self.chat).handle_command(command).await,

                ChatMode::Study(mode)=>match mode{
                    StudyMode::None=>{}

                    StudyMode::Tutor=>TutorHandler::new(self.bot,self.chat).handle_command(command).await,

                    StudyMode::Homework=>HomeworkHandler::new(self.bot,self.chat).handle_command(command).await,
                }

                ChatMode::Game=>GameHandler::new(self.bot,self.chat).handle_command(command).await,

                ChatMode::Playground=>PlaygroundHandler::new(self.bot,self.chat).handle_command(command).await,

                ChatMode::Profile=>ProfileHandler::new(self.bot,self.chat).handle_command(command).await,
            }
        }
    }

    pub async fn handle_command_callback(self,callback:String){
        match self.chat.mode{
            ChatMode::Chat=>ChatHandler::new(self.bot,self.chat).handle_command_callback(callback).await,

            ChatMode::Study(mode)=>match mode{
                StudyMode::None=>{
                    match callback.as_str(){
                        "tutor"=>{
                            self.chat.set_mode(ChatMode::Study(StudyMode::Tutor));

                            TutorHandler::new(self.bot,self.chat).handle_command_callback(&callback).await
                        }

                        "homework"=>{
                            self.chat.set_mode(ChatMode::Study(StudyMode::Homework));

                            HomeworkHandler::new(self.bot,self.chat).handle_command_callback(&callback).await
                        }

                        _=>{}
                    }
                }

                StudyMode::Tutor=>TutorHandler::new(self.bot,self.chat).handle_command_callback(&callback).await,

                StudyMode::Homework=>HomeworkHandler::new(self.bot,self.chat).handle_command_callback(&callback).await,
            }

            ChatMode::Game=>GameHandler::new(self.bot,self.chat).handle_command_callback(&callback).await,

            ChatMode::Playground=>PlaygroundHandler::new(self.bot,self.chat).handle_command_callback(&callback).await,

            ChatMode::Profile=>ProfileHandler::new(self.bot,self.chat).handle_command_callback(&callback).await,
        }
    }

    pub async fn handle_message(self,message:&str){
        let text=message
            .replace("\"","\\\"")
            .replace("\n","\\n")
            .replace("\r","\\r")
            .replace("\t","\\t");

        match self.chat.mode{
            ChatMode::Chat=>ChatHandler::new(self.bot,self.chat).handle_message(&text).await,

            ChatMode::Study(mode)=>match mode{
                StudyMode::None=>{}

                StudyMode::Tutor=>TutorHandler::new(self.bot,self.chat).handle_message(&text).await,

                StudyMode::Homework=>HomeworkHandler::new(self.bot,self.chat).handle_message(&text).await,
            }

            ChatMode::Game=>GameHandler::new(self.bot,self.chat).handle_message(text).await,

            ChatMode::Playground=>PlaygroundHandler::new(self.bot,self.chat).handle_message(&text).await,

            ChatMode::Profile=>ProfileHandler::new(self.bot,self.chat).handle_message(&text).await,
        }
    }
}



pub fn study_buttons()->InlineKeyboardMarkup{
    let tutor=InlineKeyboardButton::new(
        "Репетитор",
        InlineKeyboardButtonKind::CallbackData("tutor".into())
    );

    let homework=InlineKeyboardButton::new(
        "Помощь с домашним заданием",
        InlineKeyboardButtonKind::CallbackData("homework".into())
    );

    let inline_buttons=vec![
        vec![synthesize_voice_button(None)],
        vec![tutor],
        vec![homework]
    ];

    InlineKeyboardMarkup{
        inline_keyboard:inline_buttons
    }
}