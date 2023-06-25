use crate::{
    synthesize_voice_button,
    chat::Chat,
    extended_bot_interface::ExtendedBotInterface
};

use teloxide_core::types::{
    InlineKeyboardButton,
    InlineKeyboardButtonKind
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
                    InlineKeyboardButtonKind::CallbackData("tokensused".into())
                );

                let inline_buttons=vec![
                    vec![synthesize_voice_button(None)],
                    vec![token_usage]
                ];

                self.bot.send_message(self.chat.id,"Профиль").reply_markup(inline_buttons).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_command_callback(&mut self,callback:&str){
        match callback{
            "tokensused"=>{
                let text=format!("Использовано токенов: {}",self.chat.user.token_usage.total);

                self.bot.send_message(self.chat.id,text).reply_markup(synthesize_voice_button(None)).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(&mut self,_message:&str){

    }
}