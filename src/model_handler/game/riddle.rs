use crate::{
    synthesize_voice_button,
    chat::User,
    prompt::Prompt,
    openai::OpenAIWrapper,
    resources::Resources,
    extended_bot_interface::ExtendedBotInterface
};

use serde_json::Value;

use teloxide_core::{
    requests::Requester,
    types::{
        InlineKeyboardMarkup,
        InlineKeyboardButton,
        InlineKeyboardButtonKind, ChatId
    }
};



pub struct RiddleGameData{
    riddle:String,
    answer:String
}

impl RiddleGameData{
    pub fn new()->RiddleGameData{
        Self{
            riddle:String::new(),
            answer:String::new()
        }
    }
}



pub struct RiddleGame<'d>{
    bot:ExtendedBotInterface,
    chat_id:ChatId,
    user:&'d mut User,
    data:&'d mut RiddleGameData
}

impl<'d> RiddleGame<'d>{
    pub fn new(
        bot:ExtendedBotInterface,
        chat_id:ChatId,
        user:&'d mut User,
        data:&'d mut RiddleGameData
    )->RiddleGame<'d>{
        Self{
            bot,
            chat_id,
            user,
            data
        }
    }

    pub async fn handle_command_callback(self,callback:&str){
        match callback{
            "riddlegamestart"=>{
                let greeting=Resources::load_text("resources/text/game/riddle/greeting.txt").await;

                self.bot.send_message(self.chat_id,greeting).reply_markup(riddle_keyboard(None)).await.unwrap();
            }

            // При нажатии на кнопку "Новая загадка"
            "riddlegamecontinue"=>{
                let sent_message=self.bot.send_message(self.chat_id,"Пишет загадку...").await.unwrap();

                let prompt=Prompt::load("resources/prompts/game/riddle/riddle.json").await;

                let response=OpenAIWrapper::send_chat(&[prompt.text],1024).await;

                self.user.token_usage.total+=response.usage;

                let response:Value=serde_json::from_str(&response.message).unwrap();

                self.data.riddle=response["riddle"].as_str().unwrap().to_string();
                self.data.answer=response["answer"].as_str().unwrap().to_string();

                self.bot.bot.delete_message(self.chat_id,sent_message.id).await.unwrap();
                self.bot.send_message(self.chat_id,&self.data.riddle).reply_markup(riddle_keyboard(Some("Озвучить загадку"))).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(self,message:String){
        let mut messages=Vec::new();

        if self.data.riddle.is_empty(){
            self.bot.send_message(self.chat_id,"Нажми на кнопку, чтобы начать играть").reply_markup(synthesize_voice_button(None)).await.unwrap();
        }
        else{
            let sent_message=self.bot.send_message(self.chat_id,"Думает...").await.unwrap();

            let mut prompt=Prompt::load("resources/prompts/game/riddle/guess.json").await;

            prompt.replace("{riddle}",&self.data.riddle);
            prompt.replace("{answer}",&self.data.answer);
            prompt.replace("{guess}",&message);

            messages.push(prompt.text);

            let response=OpenAIWrapper::send_chat(&messages,1024).await;

            self.user.token_usage.total+=response.usage;

            let response:Value=serde_json::from_str(&response.message).unwrap();

            let result=response["result"].to_string().to_lowercase();
            let message=response["message"].as_str().unwrap();

            self.bot.bot.delete_message(self.chat_id,sent_message.id).await.unwrap();
            if result=="true" || result=="\"true\"" || result=="\'true\'"{
                self.data.riddle.clear();
                self.data.answer.clear();

                self.bot.send_message(self.chat_id,message).reply_markup(riddle_keyboard(None)).await.unwrap();
            }
            else{
                self.bot.send_message(self.chat_id,message).reply_markup(synthesize_voice_button(None)).await.unwrap();
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