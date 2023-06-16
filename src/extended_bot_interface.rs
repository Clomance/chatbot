use teloxide_core::{
    Bot,
    requests::{
        Requester,
        JsonRequest,
        MultipartRequest
    },
    payloads::{
        SendMessage,
        SendVoice
    },
    types::{
        ChatId,
        InlineKeyboardMarkup,
        ReplyMarkup,
        InputFile
    }
};



#[derive(Clone)]
pub struct ExtendedBotInterface{
    pub bot:Bot
}

impl ExtendedBotInterface{
    pub fn new(token:&str)->ExtendedBotInterface{
        Self{
            bot:Bot::new(token)
        }
    }

    pub fn send_message<T:Into<String>>(&self,chat_id:ChatId,text:T,inline_keyboard:Option<InlineKeyboardMarkup>)->JsonRequest<SendMessage>{
        let mut send_message=self.bot.send_message(chat_id,text);

        if let Some(keyboard)=inline_keyboard{
            send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(keyboard));
        }

        send_message
    }

    pub fn send_voice(&self,chat_id:ChatId,voice:Vec<u8>)->MultipartRequest<SendVoice>{
        self.bot.send_voice(chat_id,InputFile::memory(voice))
    }
}