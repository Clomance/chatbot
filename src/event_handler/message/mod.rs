use crate::{
    event_handler::CommandHandler,
    module::VoiceTranscription,
    extended_bot_interface::ExtendedBotInterface,
    chat::{
        Chat,
        ChatState
    },
    state_handler::{
        InitialState,
        PartitialState,
        FullState
    }
};

use std::sync::Arc;

use tokio::sync::Mutex;

use teloxide_core::{
    requests::Requester,
    net::Download,
    types::{
        Message,
        MessageKind,
        MediaKind, ChatKind
    }
};



pub struct MessageHandler{
    bot:ExtendedBotInterface,
    message:Message,
    chat:Arc<Mutex<Chat>>
}

impl MessageHandler{
    pub fn new(bot:ExtendedBotInterface,message:Message,chat:Arc<Mutex<Chat>>)->MessageHandler{
        Self{
            bot,
            message,
            chat
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
                        self.chat.lock().await.set_admin(true)
                    }
                }

                if !self.chat.lock().await.is_admin(){
                    if let Some(user)=message.from{
                        if self.bot.bot.get_chat_member("@gptkids".to_string(),user.id).await.is_err(){
                            self.bot.send_message(self.message.chat.id,"Подпишитесь на наш телеграм канал (@gptkids), чтобы получить доступ",None).await.unwrap();
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
                                CommandHandler::new(self.bot,self.chat).handle(command).await;
                            }
                            return
                        }

                        text=media.text
                    }

                    MediaKind::Voice(media)=>{
                        let chat=self.chat.lock().await;

                        let sent_message=self.bot.send_message(chat.id,"Распознаёт речь...",None).await.unwrap();

                        let mut buffer=Vec::with_capacity(media.voice.file.size as usize);

                        let file=self.bot.bot.get_file(media.voice.file.id).await.unwrap();
                        self.bot.bot.download_file(&file.path,&mut buffer).await.unwrap();

                        text=VoiceTranscription::run(buffer).await;

                        self.bot.bot.delete_message(chat.id,sent_message.id).await.unwrap();
                        self.bot.send_message(chat.id,format!("Вы сказали: \"{}.\"",text),None).await.unwrap();
                    }

                    _=>return
                }

                let mut chat=self.chat.lock().await;

                match &mut chat.state{
                    ChatState::Initial=>InitialState::new(self.bot,&mut chat).handle_message(&text).await,

                    ChatState::PartiallyFunctional=>PartitialState::new(self.bot,&mut chat).handle_message(&text).await,

                    ChatState::FullyFunctional=>FullState::new(self.bot,&mut chat).handle_message(&text).await,
                }
            }

            // Открытый канал
            ChatKind::Public(_chat)=>{
                
            }
        }
    }
}