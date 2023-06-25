use crate::{
    synthesize_voice_button,
    extended_bot_interface::ExtendedBotInterface,
    chat::Chat,
    prompt::Prompt,
    openai::OpenAIWrapper,
    resources::Resources,
};

use super::{
    GameHandler,
    GameMode
};

use serde_json::Value;

use teloxide_core::{
    requests::Requester,
    types::{
        InlineKeyboardMarkup,
        InlineKeyboardButton,
        InlineKeyboardButtonKind
    }
};



pub struct RiddleGame<'c>{
    bot:ExtendedBotInterface,
    chat:&'c mut Chat
}

impl<'c> RiddleGame<'c>{
    pub fn new(handler:GameHandler<'c>)->RiddleGame<'c>{
        Self{
            bot:handler.bot,
            chat:handler.chat
        }
    }

    pub async fn handle_command_callback(self,callback:&str){
        let data=self.chat.data.game();

        match callback{
            "riddlegamestart"=>{
                let greeting=Resources::load_text("resources/text/game/riddle/greeting.txt").await;

                self.bot.send_message(self.chat.id,greeting).reply_markup(riddle_keyboard(None)).await.unwrap();
            }

            // При нажатии на кнопку "Новая загадка"
            "riddlegamecontinue"=>{
                let sent_message=self.bot.send_message(self.chat.id,"Пишет загадку...").await.unwrap();

                let prompt=Prompt::load("prompts/game/riddle/riddle.json").await;

                let response=OpenAIWrapper::send_chat(&[prompt.text],1024).await;

                self.chat.user.token_usage.total+=response.usage;

                let response:Value=serde_json::from_str(&response.message).unwrap();

                let riddle=response["riddle"].as_str().unwrap();
                let answer=response["answer"].as_str().unwrap();

                data.mode=GameMode::riddle(riddle,answer);

                self.bot.bot.delete_message(self.chat.id,sent_message.id).await.unwrap();
                self.bot.send_message(self.chat.id,riddle).reply_markup(riddle_keyboard(Some("Озвучить загадку"))).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(self,riddle:String,answer:String,message:&str){
        let mut messages=Vec::new();

        if riddle.is_empty(){
            self.bot.send_message(self.chat.id,"Нажми на кнопку, чтобы начать играть").reply_markup(synthesize_voice_button(None)).await.unwrap();
        }
        else{
            let sent_message=self.bot.send_message(self.chat.id,"Думает...").await.unwrap();

            let mut prompt=Prompt::load("prompts/game/riddle/guess.json").await;

            prompt.replace("{riddle}",&riddle);
            prompt.replace("{answer}",&answer);
            prompt.replace("{guess}",&message);

            messages.push(prompt.text);

            let response=OpenAIWrapper::send_chat(&messages,1024).await;

            self.chat.user.token_usage.total+=response.usage;

            let response:Value=serde_json::from_str(&response.message).unwrap();

            let result=response["result"].to_string().to_lowercase();
            let message=response["message"].as_str().unwrap();

            self.bot.bot.delete_message(self.chat.id,sent_message.id).await.unwrap();
            if result=="true" || result=="\"true\"" || result=="\'true\'"{
                self.chat.data.game().mode=GameMode::riddle("","");

                self.bot.send_message(self.chat.id,message).reply_markup(riddle_keyboard(None)).await.unwrap();
            }
            else{
                self.bot.send_message(self.chat.id,message).reply_markup(synthesize_voice_button(None)).await.unwrap();
            }
        }
    }
}


pub fn riddle_keyboard(voice_button_text:Option<&str>)->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(voice_button_text);

    let next_button=InlineKeyboardButton::new(
        "Новая загадка",
        InlineKeyboardButtonKind::CallbackData("riddlegamecontinue".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,voice]])
}