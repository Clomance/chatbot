use std::sync::Arc;
use tokio::sync::Mutex;

use teloxide_core::{
    Bot,
    requests::Requester,
    types::{
        Update,
        UpdateKind
    }
};

use crate::chat::Chats;

use super::{
    MessageHandler,
    CommandCallbackHandler
};

pub struct UpdateHandler{
    bot:Bot,
    update:Update,
    chats:Arc<Mutex<Chats>>
}

impl UpdateHandler{
    pub fn new(bot:Bot,update:Update,chats:Arc<Mutex<Chats>>)->UpdateHandler{
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
                        let chat=self.chats.lock().await.get(message.chat.id);
                        CommandCallbackHandler::new(self.bot.clone(),chat).handle(&data).await;
                    }
                }

                let _=self.bot.answer_callback_query(query.id).await;
            }

            UpdateKind::Message(message)=>{
                let chat=self.chats.lock().await.get(message.chat.id);
                MessageHandler::new(self.bot,message,chat).handle().await;
            }

            _=>{}
        }
    }
}