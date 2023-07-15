use crate::{
    event_handler::CommandHandler,
    extended_bot_interface::ExtendedBotInterface,
    module::{
        VoiceTranscription,
        NewsSubscriptionCheck
    },
    chat::{
        Chat,
        ChatState, Chats
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
    net::Download,
    types::{
        Message,
        MessageKind,
        MediaKind,
        ChatKind
    }
};



pub struct MessageHandler{
    bot:ExtendedBotInterface,
    message:Message,
    chat:Arc<Mutex<Chat>>,
    chats:Arc<RwLock<Chats>>
}

impl MessageHandler{
    pub fn new(bot:ExtendedBotInterface,message:Message,chat:Arc<Mutex<Chat>>,chats:Arc<RwLock<Chats>>)->MessageHandler{
        Self{
            bot,
            message,
            chat,
            chats
        }
    }

    pub async fn handle(self){
        match self.message.chat.kind{
            // Закрытый чат или канал
            ChatKind::Private(_)=>{
                let text;

                let MessageKind::Common(message)=self.message.kind else {return};

                if let MediaKind::Text(media)=&message.media_kind{
                    if media.text=="nevergonnagiveyouup"{
                        self.bot.send_message(self.message.chat.id,"nevergonnaletyoudown").await.unwrap();
                        self.chat.lock().await.set_admin(true)
                    }
                }

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

                match message.media_kind{
                    MediaKind::Text(media)=>{
                        // Проверка на команду
                        if media.text.starts_with("/"){
                            if let Some(command)=media.text.get(1..){
                                CommandHandler::new(self.bot,self.chat,self.chats).handle(command).await;
                            }
                            return
                        }
                        else{
                            match media.text.as_str(){
                                "Развивайка \u{1F393}"=>{
                                    return CommandHandler::new(self.bot,self.chat,self.chats).handle("studytutor").await
                                }

                                "Игры \u{26BD}"=>{
                                    return CommandHandler::new(self.bot,self.chat,self.chats).handle("game").await
                                }

                                "Домашка \u{1F4DD}"=>{
                                    return CommandHandler::new(self.bot,self.chat,self.chats).handle("studyhomework").await
                                }

                                "Поговорить с Другом \u{1F466}"=>{
                                    return CommandHandler::new(self.bot,self.chat,self.chats).handle("chat").await
                                }

                                _=>{}
                            }
                        }

                        text=media.text
                    }

                    MediaKind::Voice(media)=>{
                        let chat=self.chat.lock().await;

                        let sent_message=self.bot.send_message(chat.id,"Распознаёт речь...").await.unwrap();

                        let mut buffer=Vec::with_capacity(media.voice.file.size as usize);

                        let file=self.bot.bot.get_file(media.voice.file.id).await.unwrap();
                        self.bot.bot.download_file(&file.path,&mut buffer).await.unwrap();

                        text=VoiceTranscription::run(buffer).await;

                        self.bot.bot.delete_message(chat.id,sent_message.id).await.unwrap();
                        self.bot.send_message(chat.id,format!("Вы сказали: \"{}.\"",text)).await.unwrap();
                    }

                    _=>return
                }

                let mut chat=self.chat.lock().await;

                match &mut chat.state{
                    ChatState::Initial=>InitialState::new(self.bot,&mut chat).handle_message(&text).await,

                    ChatState::PartiallyFunctional=>PartitialState::new(self.bot,&mut chat).handle_message(&text).await,

                    ChatState::FullyFunctional=>FullState::new(self.bot,&mut chat,self.chats).handle_message(&text).await,
                }
            }

            // Открытый канал
            ChatKind::Public(_chat)=>{
                
            }
        }
    }
}