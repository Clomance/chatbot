use crate::{
    extended_bot_interface::ExtendedBotInterface,
    chat::Chat,
    resources::Resources,
    synthesize_voice_keyboard
};



pub struct PartitialState<'c>{
    bot:ExtendedBotInterface,
    chat:&'c mut Chat
}

impl<'c> PartitialState<'c>{
    pub fn new(bot:ExtendedBotInterface,chat:&'c mut Chat)->PartitialState<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(self,command:&str){
        match command{
            // Перейти в режим 'Простое общение'
            "chat"=>{
                let the_chosen_mode_is_the_current=Resources::load_text("resources/text/general/the_chosen_mode_is_the_current.txt").await;

                self.bot.send_message(self.chat.id,the_chosen_mode_is_the_current,None).await.unwrap();
            }

            // Перейти в раздел 'Настройки'
            "settings"=>{
                let not_avaiable=Resources::load_text("resources/text/general/not_avaiable.txt").await;

                self.bot.send_message(self.chat.id,not_avaiable,Some(synthesize_voice_keyboard(None))).await.unwrap();
            }

            // Перейти в раздел 'Профиль'
            "profile"=>{
                let not_avaiable=Resources::load_text("resources/text/general/not_avaiable.txt").await;

                self.bot.send_message(self.chat.id,not_avaiable,Some(synthesize_voice_keyboard(None))).await.unwrap();
            }

            // Сообщение для других команд
            _=>{
                let access_denied=Resources::load_text("resources/text/general/access_denied.txt").await;

                self.bot.send_message(self.chat.id,access_denied,Some(synthesize_voice_keyboard(None))).await.unwrap();
            }
        }
    }

    pub async fn handle_command_callback(self,_callback:String){
        
    }

    pub async fn handle_message(self,_message:&str){
        
    }
}