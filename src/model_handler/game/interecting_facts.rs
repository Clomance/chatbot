use crate::{
    synthesize_voice_button,
    chat::User,
    resources::{
        Resources,
        Prompt
    },
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



pub struct InterestingFacts<'d>{
    bot:ExtendedBotInterface,
    chat_id:ChatId,
    user:&'d mut User
}

impl<'d> InterestingFacts<'d>{
    pub fn new(
        bot:ExtendedBotInterface,
        chat_id:ChatId,
        user:&'d mut User
    )->InterestingFacts<'d>{
        Self{
            bot,
            chat_id,
            user
        }
    }

    pub async fn handle_command_callback(self,callback:&str){
        match callback{
            "insterestingfactsstart"=>{
                let greeting=Resources::load_text("resources/text/game/insteresting_facts/greeting.txt").await;

                self.bot.send_message(self.chat_id,greeting).reply_markup(interecting_facts_start_keyboard()).await.unwrap();
            }

            // При нажатии на кнопку "Интересный факт"
            "insterestingfactscontinue"=>{
                let sent_message=self.bot.send_message(self.chat_id,"Пишет интересный факт...").await.unwrap();

                let prompt=Prompt::load("resources/prompts/game/interestingfacts.json").await;

                let response=OpenAIWrapper::send_chat(&[prompt.text],1024).await;

                self.user.token_usage.total+=response.usage;

                self.bot.bot.delete_message(self.chat_id,sent_message.id).await.unwrap();
                self.bot.send_message(self.chat_id,response.message).reply_markup(interecting_facts_continue_keyboard()).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(self,_message:String){
        self.bot.send_message(self.chat_id,"Нажми на кнопку, чтобы узнать интересный факт").reply_markup(synthesize_voice_button(None)).await.unwrap();
    }
}



pub fn interecting_facts_start_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(None);

    let next_button=InlineKeyboardButton::new(
        "Интересный факт",
        InlineKeyboardButtonKind::CallbackData("insterestingfactscontinue".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,voice]])
}


pub fn interecting_facts_continue_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(Some("Озвучить факт"));

    let next_button=InlineKeyboardButton::new(
        "Интересный факт",
        InlineKeyboardButtonKind::CallbackData("insterestingfactscontinue".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,voice]])
}