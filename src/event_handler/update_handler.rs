use crate::{chat::Chats, extended_bot_interface::ExtendedBotInterface};

use super::{
    MessageHandler,
    CommandCallbackHandler
};

use std::sync::Arc;

use tokio::sync::RwLock;

use teloxide_core::{
    requests::Requester,
    types::{
        Update,
        UpdateKind
    }
};



pub struct UpdateHandler{
    bot:ExtendedBotInterface,
    update:Update,
    chats:Arc<RwLock<Chats>>
}

impl UpdateHandler{
    pub fn new(bot:ExtendedBotInterface,update:Update,chats:Arc<RwLock<Chats>>)->UpdateHandler{
        Self{
            bot,
            update,
            chats
        }
    }

    pub async fn handle(self){
        match self.update.kind{
            UpdateKind::CallbackQuery(query)=>{
                if let Some(message)=query.message{
                    if let Some(data)=query.data{
                        let chat=self.chats.write().await.get(message.chat.id);
                        CommandCallbackHandler::new(self.bot.clone(),data,message,chat).handle().await;
                    }
                }

                let _=self.bot.bot.answer_callback_query(query.id).await;
            }

            UpdateKind::Message(message)=>{
                let chat=self.chats.write().await.get(message.chat.id);
                MessageHandler::new(self.bot,message,chat).handle().await;
            }

            _=>{}
        }
    }
}