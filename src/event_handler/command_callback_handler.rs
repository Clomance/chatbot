use crate::{
    yandex::YandexSpeechKit,
    module::NewsSubscriptionCheck,
    extended_bot_interface::ExtendedBotInterface,
    chat::{
        Chat,
        ChatState, Chats,
    },
    state_handler::{
        InitialState,
        PartitialState,
        FullState
    }
};

use std::sync::Arc;

use tokio::sync::{Mutex, RwLock};

use teloxide_core::{
    requests::Requester,
    types::{
        Message,
        ChatAction,
        ChatKind,
        MessageKind,
        MediaKind
    }
};



pub struct CommandCallbackHandler{
    bot:ExtendedBotInterface,
    callback:String,
    message:Message,
    chat:Arc<Mutex<Chat>>,
    chats:Arc<RwLock<Chats>>
}

impl CommandCallbackHandler{
    pub fn new(
        bot:ExtendedBotInterface,
        callback:String,
        message:Message,
        chat:Arc<Mutex<Chat>>,
        chats:Arc<RwLock<Chats>>
    )->CommandCallbackHandler{
        Self{
            bot,
            callback,
            message,
            chat,
            chats
        }
    }

    pub async fn handle(self){
        match self.message.chat.kind{
            // Закрытый чат или канал
            ChatKind::Private(_)=>{
                let MessageKind::Common(message)=self.message.kind else {return};

                if !self.chat.lock().await.is_admin(){
                    if let Some(user)=message.from{
                        if !NewsSubscriptionCheck::check(&self.bot,user.id).await{
                            self.bot.send_message(self.message.chat.id,"Подпишитесь на наш телеграм канал (@gptkids), чтобы получить доступ").await.unwrap();
                            return
                        }
                    }
                    else{
                        return
                    }
                }

                let mut chat=self.chat.lock().await;

                match self.callback.as_str(){
                    "synthesize"=>{
                        if let MediaKind::Text(media)=&message.media_kind{
                            let message=self.bot.send_message(chat.id,"Записывает голосовое сообщение...").await.unwrap();
                            self.bot.bot.send_chat_action(chat.id,ChatAction::RecordVoice).await.unwrap();

                            let voice=YandexSpeechKit::synthesize(&media.text).await;

                            if voice.is_empty(){
                                self.bot.send_message(chat.id,"Ошибка").await.unwrap();
                            }
                            else{
                                self.bot.bot.delete_message(chat.id,message.id).await.unwrap();
                                self.bot.send_voice(chat.id,voice).await.unwrap();
                            }
                        }
                    }

                    _=>match chat.state{
                        ChatState::Initial=>InitialState::new(self.bot,&mut chat).handle_command_callback(self.callback).await,

                        ChatState::PartiallyFunctional=>PartitialState::new(self.bot,&mut chat).handle_command_callback(self.callback).await,

                        ChatState::FullyFunctional=>FullState::new(self.bot,&mut chat,self.chats).handle_command_callback(self.callback).await,
                    }
                }
            }

            // Открытый канал
            ChatKind::Public(_chat)=>{
                
            }
        }
    }
}