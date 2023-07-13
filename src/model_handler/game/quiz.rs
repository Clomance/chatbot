use crate::{
    chat::User,
    extended_bot_interface::ExtendedBotInterface
};

use teloxide_core::types::ChatId;



pub struct QuizGame<'d>{
    bot:ExtendedBotInterface,
    chat_id:ChatId,
    user:&'d mut User
}

impl<'d> QuizGame<'d>{
    pub fn new(
        bot:ExtendedBotInterface,
        chat_id:ChatId,
        user:&'d mut User
    )->QuizGame<'d>{
        Self{
            bot,
            chat_id,
            user
        }
    }

    pub async fn handle_command_callback(self,callback:&str){
        match callback{
            "quizgamestart"=>{
                self.bot.send_message(self.chat_id,"Данный раздел пока не доступен").await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(self,_message:String){
        self.bot.send_message(self.chat_id,"Данный раздел пока не доступен").await.unwrap();
    }
}