use crate::{
    synthesize_voice_button,
    synthesize_voice_keyboard,
    chat::Chat,
    extended_bot_interface::ExtendedBotInterface
};

use super::GameHandler;

use teloxide_core::{
    types::{
        InlineKeyboardMarkup,
        InlineKeyboardButton,
        InlineKeyboardButtonKind
    }
};



pub struct GuessingGame<'c>{
    bot:ExtendedBotInterface,
    chat:&'c mut Chat,
}

impl<'c> GuessingGame<'c>{
    pub fn new(handler:GameHandler<'c>)->GuessingGame<'c>{
        Self{
            bot:handler.bot,
            chat:handler.chat
        }
    }

    pub async fn handle_command(self,command:&str){

    }

    pub async fn handle_command_callback(self,callback:&str){
        match callback{
            "guessinggame"=>{
                self.bot.send_message(self.chat.id,"Ты выбрал угадайку",Some(new_guessing_game_keyboard())).await.unwrap();
            }

            "guessinggamestart"=>{
                self.bot.send_message(self.chat.id,"Я загадал слово. Попробуй его отгадать",Some(synthesize_voice_keyboard(None))).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(self,message:&str){

    }
}



pub fn new_guessing_game_keyboard()->InlineKeyboardMarkup{
    let next_button=InlineKeyboardButton::new(
        "Начать",
        InlineKeyboardButtonKind::CallbackData("guessinggamestart".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,synthesize_voice_button(None)]])
}