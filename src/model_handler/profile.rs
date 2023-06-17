use crate::{
    synthesize_voice_button,
    synthesize_voice_keyboard,
    extended_bot_interface::ExtendedBotInterface,
    chat::{
        Chat
    }
};

use teloxide_core::types::{
    InlineKeyboardButton,
    InlineKeyboardButtonKind,
    InlineKeyboardMarkup
};



pub struct ProfileHandler<'c>{
    bot:ExtendedBotInterface,
    chat:&'c mut Chat,
}

impl<'c> ProfileHandler<'c>{
    pub fn new(bot:ExtendedBotInterface,chat:&'c mut Chat)->ProfileHandler<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(&mut self,command:&str){
        match command{
            "profile"=>{
                let token_usage=InlineKeyboardButton::new(
                    "Использовано токенов",
                    InlineKeyboardButtonKind::CallbackData("tokenusage".into())
                );

                let inline_buttons=vec![
                    vec![synthesize_voice_button(None)],
                    vec![token_usage]
                ];

                let keyboard=InlineKeyboardMarkup{
                    inline_keyboard:inline_buttons
                };

                self.bot.send_message(self.chat.id,"Профиль",Some(keyboard)).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_command_callback(&mut self,callback:&str){
        match callback{
            "tokenusage"=>{
                let text=format!("Использовано токенов: {}",self.chat.user.token_used);

                self.bot.send_message(self.chat.id,text,Some(synthesize_voice_keyboard(None))).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(&mut self,_message:&str){

    }
}