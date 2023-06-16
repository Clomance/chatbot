use crate::{
    model_handler::ChatHandler,
    synthesize_voice_button,
    resources::Resources,
    synthesize_voice_keyboard,
    extended_bot_interface::ExtendedBotInterface,
    chat::{
        Chat,
        ChatState
    }
};

use teloxide_core::types::{
    InlineKeyboardButton,
    InlineKeyboardButtonKind,
    InlineKeyboardMarkup
};



pub struct InitialState<'c>{
    bot:ExtendedBotInterface,
    chat:&'c mut Chat
}

impl<'c> InitialState<'c>{
    pub fn new(bot:ExtendedBotInterface,chat:&'c mut Chat)->InitialState<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(self,command:&str){
        let mut command_split=command.split_whitespace();

        let Some(command)=command_split.next()else{return};

        match command{
            "start"=>{
                if let Some(args)=command_split.next(){
                    println!("Args: {args}");
                }
                else{
                    println!("No args")
                }

                let start=InlineKeyboardButton::new(
                    "Старт",
                    InlineKeyboardButtonKind::CallbackData("start".into())
                );

                let voice=synthesize_voice_button(None);

                let greeting=Resources::load_text("resources/text/general/start/greeting.txt").await;

                self.bot.send_message(self.chat.id,greeting,Some(InlineKeyboardMarkup{inline_keyboard:vec![vec![start,voice]]})).await.unwrap();
            }

            // Сообщение при для других команд
            _=>{
                let access_denied=Resources::load_text("resources/text/general/access_denied.txt").await;

                self.bot.send_message(self.chat.id,access_denied,Some(synthesize_voice_keyboard(None))).await.unwrap();
            }
        }
    }

    pub async fn handle_command_callback(self,callback:String){
        match callback.as_str(){
            "start"=>{
                self.chat.state=ChatState::FullyFunctional;

                ChatHandler::new(self.bot,self.chat).handle_command_callback(callback).await;
            }

            _=>{}
        }
    }

    pub async fn handle_message(self,_message:&str){
        self.bot.send_message(self.chat.id,"Напиши /start, чтобы начать",None).await.unwrap();
    }
}