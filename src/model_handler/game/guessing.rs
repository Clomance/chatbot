use crate::{
    synthesize_voice_button,
    chat::{History, User},
    extended_bot_interface::ExtendedBotInterface,
    prompt::Prompt,
    openai::OpenAIWrapper
};

use serde_json::Value;

use teloxide_core::{
    requests::Requester,
    types::{
        InlineKeyboardMarkup,
        InlineKeyboardButton,
        InlineKeyboardButtonKind,
        ChatId
    }
};



pub struct GuessingGameData{
    secret:String,
    history:History
}

impl GuessingGameData{
    pub fn new(secret:&str)->GuessingGameData{
        Self{
            secret:String::from(secret),
            history:History::new()
        }
    }
}



pub struct GuessingGame<'d>{
    bot:ExtendedBotInterface,
    chat_id:ChatId,
    user:&'d mut User,
    data:&'d mut GuessingGameData
}

impl<'d> GuessingGame<'d>{
    pub fn new(bot:ExtendedBotInterface,chat_id:ChatId,user:&'d mut User,data:&'d mut GuessingGameData)->GuessingGame<'d>{
        Self{
            bot,
            chat_id,
            user,
            data
        }
    }

    pub async fn handle_command(self,command:&str){
        match command{
            "reset"=>{
                self.data.history.clear();

                self.bot.send_message(self.chat_id,"История очищена").reply_markup(synthesize_voice_button(None)).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_command_callback(self,callback:&str){
        match callback{
            "guessinggamestart"=>{
                self.bot.send_message(self.chat_id,"Ты выбрал угадайку").reply_markup(new_guessing_game_keyboard()).await.unwrap();
            }

            "guessinggamenew"=>{
                self.data.history.clear();

                let prompt=Prompt::load("prompts/game/guessing/secret.json").await;

                let response=OpenAIWrapper::send_chat(&vec![prompt.text],1024).await;

                let message:Value=serde_json::from_str(&response.message).unwrap();

                self.data.secret=message["secret"].as_str().unwrap().to_string();

                self.user.token_usage.total+=response.usage;

                self.bot.send_message(self.chat_id,"Я загадал слово. Попробуй его отгадать").reply_markup(synthesize_voice_button(None)).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(self,message:String){
        if self.data.secret.is_empty(){
            self.bot.send_message(self.chat_id,"Нажми на кнопку, чтобы начать").reply_markup(new_guessing_game_keyboard()).await.unwrap();
        }
        else{
            let sent_message=self.bot.send_message(self.chat_id,"Набирает сообщение...").await.unwrap();

            let mut messages=Vec::new();

            let mut prompt=Prompt::load("prompts/game/guessing/guess.json").await;
            prompt.replace("{secret}",&self.data.secret);

            messages.push(prompt.text);

            self.data.history.push_request(&message);

            for message in self.data.history.messages().iter(){
                messages.push(message.to_string())
            }

            let response=OpenAIWrapper::send_chat(&messages,1024).await;

            let message:Value=serde_json::from_str(&response.message).unwrap();
            let end=message["end"].as_bool().unwrap();
            let message=message["message"].as_str().unwrap();

            self.user.token_usage.total+=response.usage;

            self.bot.bot.delete_message(self.chat_id,sent_message.id).await.unwrap();

            if end{
                self.data.secret.clear();
                self.data.history.clear();

                self.bot.send_message(self.chat_id,message).reply_markup(guessing_game_continue_keyboard()).await.unwrap();
            }
            else{
                self.data.history.push_response(&response.message);

                self.bot.send_message(self.chat_id,message).reply_markup(synthesize_voice_button(None)).await.unwrap();
            }
        }
    }
}



pub fn new_guessing_game_keyboard()->InlineKeyboardMarkup{
    let next_button=InlineKeyboardButton::new(
        "Начать",
        InlineKeyboardButtonKind::CallbackData("guessinggamenew".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,synthesize_voice_button(None)]])
}

pub fn guessing_game_continue_keyboard()->InlineKeyboardMarkup{
    let next_button=InlineKeyboardButton::new(
        "Начать заново",
        InlineKeyboardButtonKind::CallbackData("guessinggamenew".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,synthesize_voice_button(None)]])
}