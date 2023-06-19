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
        MediaKind
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
        let text;
        if let MessageKind::Common(message)=self.message.kind{
            match message.media_kind{
                MediaKind::Text(media)=>{
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
        }
        else{
            return
        }

        if text.starts_with("/"){
            if let Some(command)=text.get(1..){
                CommandHandler::new(self.bot,self.chat).handle(command).await;
            }
            return
        }

        let mut chat=self.chat.lock().await;

        match &mut chat.state{
            ChatState::Initial=>InitialState::new(self.bot,&mut chat).handle_message(&text).await,

            ChatState::PartiallyFunctional=>PartitialState::new(self.bot,&mut chat).handle_message(&text).await,

            ChatState::FullyFunctional=>FullState::new(self.bot,&mut chat).handle_message(&text).await,
        }
    }
}