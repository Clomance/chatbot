use crate::{
    openai::OpenAI,
    event_handler::CommandHandler,
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

use std::{
    sync::Arc,
    io::Cursor
};

use mp3lame_encoder::{
    Builder,
    Id3Tag,
    MonoPcm,
    FlushNoGap
};

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
        let mut voice=false;
        if let MessageKind::Common(message)=self.message.kind{
            match message.media_kind{
                MediaKind::Text(media)=>{
                    text=media.text
                }

                MediaKind::Voice(media)=>{
                    let mut buffer=Vec::with_capacity(media.voice.file.size as usize);

                    let file=self.bot.bot.get_file(media.voice.file.id).await.unwrap();
                    self.bot.bot.download_file(&file.path,&mut buffer).await.unwrap();

                    let (samples,_)=ogg_opus::decode::<Cursor<Vec<u8>>,48000>(Cursor::new(buffer)).unwrap();

                    let mut mp3_encoder = Builder::new().expect("Create LAME builder");
                    mp3_encoder.set_num_channels(1).expect("set channels");
                    mp3_encoder.set_sample_rate(44_100).expect("set sample rate");
                    mp3_encoder.set_brate(mp3lame_encoder::Bitrate::Kbps192).expect("set brate");
                    mp3_encoder.set_quality(mp3lame_encoder::Quality::Best).expect("set quality");
                    mp3_encoder.set_id3_tag(Id3Tag {
                        title: b"My title",
                        artist: &[],
                        album: b"My album",
                        year: b"Current year",
                        comment: b"Just my comment",
                    });
                    let mut mp3_encoder=mp3_encoder.build().expect("To initialize LAME encoder");

                    let input = MonoPcm(&samples);

                    let mut mp3_out_buffer = Vec::new();
                    mp3_out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(samples.len()));
                    let encoded_size=mp3_encoder.encode(input,mp3_out_buffer.spare_capacity_mut()).expect("To encode");
                    unsafe {
                        mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
                    }

                    let encoded_size=mp3_encoder.flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut()).expect("to flush");
                    unsafe {
                        mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
                    }

                    text=OpenAI::send_voice(&mp3_out_buffer);

                    println!("{}",text);

                    voice=true;
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

        if voice{
            self.bot.send_message(chat.id,format!("Вы сказали: \"{}.\"",text),None).await.unwrap();
        }

        match &mut chat.state{
            ChatState::Initial=>InitialState::new(self.bot,&mut chat).handle_message(&text).await,

            ChatState::PartiallyFunctional=>PartitialState::new(self.bot,&mut chat).handle_message(&text).await,

            ChatState::FullyFunctional=>FullState::new(self.bot,&mut chat).handle_message(&text).await,
        }
    }
}