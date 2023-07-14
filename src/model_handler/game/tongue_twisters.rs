use crate::{
    synthesize_voice_button,
    chat::User,
    resources::Prompt,
    openai::OpenAIWrapper,
    extended_bot_interface::ExtendedBotInterface
};

use teloxide_core::{
    requests::Requester,
    types::{
        ChatId,
        InlineKeyboardMarkup,
        InlineKeyboardButton,
        InlineKeyboardButtonKind
    }
};



pub struct TongueTwisters<'d>{
    bot:ExtendedBotInterface,
    chat_id:ChatId,
    user:&'d mut User
}

impl<'d> TongueTwisters<'d>{
    pub fn new(
        bot:ExtendedBotInterface,
        chat_id:ChatId,
        user:&'d mut User
    )->TongueTwisters<'d>{
        Self{
            bot,
            chat_id,
            user
        }
    }

    pub async fn handle_command_callback(self,callback:&str){
        match callback{
            "tonguetwistersstart"=>{
                self.bot.send_message(self.chat_id,"Ты выбрал скороговорки. Давай начнём. Нажми кнопку").reply_markup(tongue_twisters_start_keyboard()).await.unwrap();
            }

            "tonguetwisterscontinue"=>{
                let sent_message=self.bot.send_message(self.chat_id,"Пишет скороговорку...").await.unwrap();

                let prompt=Prompt::load("resources/prompts/game/tonguetwisters.json").await;

                let response=OpenAIWrapper::send_chat(&[prompt.text],1024).await;

                self.user.token_usage.total+=response.usage;

                self.bot.bot.delete_message(self.chat_id,sent_message.id).await.unwrap();
                self.bot.send_message(self.chat_id,response.message).reply_markup(tongue_twisters_continue_keyboard()).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(self,_message:String){
        self.bot.send_message(self.chat_id,"Нажми на кнопку, чтобы получить скороговорку").reply_markup(synthesize_voice_button(None)).await.unwrap();
    }
}



pub fn tongue_twisters_start_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(None);

    let next_button=InlineKeyboardButton::new(
        "Скороговорка",
        InlineKeyboardButtonKind::CallbackData("tonguetwisterscontinue".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,voice]])
}

pub fn tongue_twisters_continue_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(Some("Озвучить скороговорку"));

    let next_button=InlineKeyboardButton::new(
        "Скороговорка",
        InlineKeyboardButtonKind::CallbackData("tonguetwisterscontinue".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,voice]])
}