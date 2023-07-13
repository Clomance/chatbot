use std::future::IntoFuture;

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
        InputFile,
        InlineKeyboardButton
    }
};



pub trait ReplyMarkupWrap{
    fn wrap(self)->ReplyMarkup;
}

impl ReplyMarkupWrap for InlineKeyboardButton{
    fn wrap(self)->ReplyMarkup{
        ReplyMarkup::InlineKeyboard(
            InlineKeyboardMarkup{
                inline_keyboard:vec![vec![self]]
            }
        )
    }
}

impl ReplyMarkupWrap for Vec<InlineKeyboardButton>{
    fn wrap(self)->ReplyMarkup{
        ReplyMarkup::InlineKeyboard(
            InlineKeyboardMarkup{
                inline_keyboard:vec![self]
            }
        )
    }
}

impl ReplyMarkupWrap for Vec<Vec<InlineKeyboardButton>>{
    fn wrap(self)->ReplyMarkup{
        ReplyMarkup::InlineKeyboard(
            InlineKeyboardMarkup{
                inline_keyboard:self
            }
        )
    }
}

impl ReplyMarkupWrap for InlineKeyboardMarkup{
    fn wrap(self)->ReplyMarkup{
        ReplyMarkup::InlineKeyboard(self)
    }
}

impl ReplyMarkupWrap for ReplyMarkup{
    fn wrap(self)->ReplyMarkup{
        self
    }
}



pub struct ExtendedSendMessage{
    pub inner:JsonRequest<SendMessage>
}

impl ExtendedSendMessage{
    pub fn new(send_message:JsonRequest<SendMessage>)->ExtendedSendMessage{
        Self{
            inner:send_message
        }
    }

    pub fn reply_markup<R:ReplyMarkupWrap>(mut self,reply_markup:R)->ExtendedSendMessage{
        self.inner.reply_markup=Some(reply_markup.wrap());
        self
    }
}

impl IntoFuture for ExtendedSendMessage{
    type Output=<JsonRequest<SendMessage> as IntoFuture>::Output;
    type IntoFuture=<JsonRequest<SendMessage> as IntoFuture>::IntoFuture;

    fn into_future(self)->Self::IntoFuture{
        self.inner.into_future()
    }
}



/// Расширенная версия интерефейса бота.
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

    pub fn send_message<T:Into<String>>(&self,chat_id:ChatId,text:T)->ExtendedSendMessage{
        ExtendedSendMessage::new(self.bot.send_message(chat_id,text))
    }

    pub fn send_voice(&self,chat_id:ChatId,voice:Vec<u8>)->MultipartRequest<SendVoice>{
        self.bot.send_voice(chat_id,InputFile::memory(voice))
    }
}