use crate::{
    synthesize_voice_button,
    chat::User,
    prompt::Prompt,
    resources::Resources,
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



pub struct FairyTalesData{
    random:bool,
    theme:String
}

impl FairyTalesData{
    pub fn new()->FairyTalesData{
        Self{
            random:true,
            theme:String::new()
        }
    }
}



pub struct FairyTales<'d>{
    bot:ExtendedBotInterface,
    chat_id:ChatId,
    user:&'d mut User,
    data:&'d mut FairyTalesData
}

impl<'d> FairyTales<'d>{
    pub fn new(
        bot:ExtendedBotInterface,
        chat_id:ChatId,
        user:&'d mut User,
        data:&'d mut FairyTalesData
    )->FairyTales<'d>{
        Self{
            bot,
            chat_id,
            user,
            data
        }
    }

    pub async fn handle_command_callback(self,callback:&str){
        match callback{
            "fairytalesstart"=>{
                self.bot.send_message(self.chat_id,"Ты выбрал сказки. Давай начнём. Нажми кнопку").reply_markup(fairy_tales_start_keyboard()).await.unwrap();
            }

            "fairytalescontinue"=>{
                if self.data.random{
                    let sent_message=self.bot.send_message(self.chat_id,"Пишет сказку...").await.unwrap();

                    let prompt=Prompt::load("resources/prompts/game/fairytales.json").await;

                    let response=OpenAIWrapper::send_chat(&[prompt.text],1024).await;

                    self.user.token_usage.total+=response.usage;

                    self.bot.bot.delete_message(self.chat_id,sent_message.id).await.unwrap();
                    self.bot.send_message(self.chat_id,response.message).reply_markup(fairy_tales_continue_keyboard()).await.unwrap();
                }
                else if self.data.theme.is_empty(){
                    self.bot.send_message(self.chat_id,"Напиши тему").reply_markup(synthesize_voice_button(None)).await.unwrap();
                }
                else{
                    let sent_message=self.bot.send_message(self.chat_id,"Пишет сказку...").await.unwrap();

                    let mut prompt=Prompt::load("resources/prompts/game/themedfairytales.json").await;
                    prompt.replace("{theme}",&self.data.theme);

                    let response=OpenAIWrapper::send_chat(&[prompt.text],1024).await;

                    self.user.token_usage.total+=response.usage;

                    self.bot.bot.delete_message(self.chat_id,sent_message.id).await.unwrap();
                    self.bot.send_message(self.chat_id,response.message).reply_markup(themed_fairy_tales_continue_keyboard()).await.unwrap();
                }
            }

            "fairytalesnewtheme"=>{
                self.data.random=false;
                self.data.theme.clear();

                self.bot.send_message(self.chat_id,"Напиши тему").reply_markup(synthesize_voice_button(None)).await.unwrap();
            }

            "fairytalesresettheme"=>{
                self.data.random=true;
                self.data.theme.clear();

                let theme_reset=Resources::load_text("resources/text/game/fairy_tales/theme_reset.txt").await;

                self.bot.send_message(self.chat_id,theme_reset).reply_markup(fairy_tales_start_keyboard()).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(self,message:String){
        if self.data.random{
            self.bot.send_message(self.chat_id,"Нажми на кнопку, чтобы получить сказку").reply_markup(synthesize_voice_button(None)).await.unwrap();
        }
        else if self.data.theme.is_empty(){
            self.data.theme=message.to_string();

            let theme_set=Resources::load_text("resources/text/game/fairy_tales/theme_set.txt").await;

            self.bot.send_message(self.chat_id,theme_set).reply_markup(themed_fairy_tales_start_keyboard()).await.unwrap();
        }
        else{
            self.bot.send_message(self.chat_id,"Нажми на кнопку, чтобы получить сказку").reply_markup(synthesize_voice_button(None)).await.unwrap();
        }
    }
}



pub fn fairy_tales_start_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(Some("Озвучить"));

    let next_button=InlineKeyboardButton::new(
        "Сказка",
        InlineKeyboardButtonKind::CallbackData("fairytalescontinue".into())
    );

    let set_button=InlineKeyboardButton::new(
        "Задать тему",
        InlineKeyboardButtonKind::CallbackData("fairytalesnewtheme".into())
    );

    InlineKeyboardMarkup::new(vec![vec![voice],vec![next_button,set_button]])
}


pub fn fairy_tales_continue_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(Some("Озвучить сказку"));

    let next_button=InlineKeyboardButton::new(
        "Сказка",
        InlineKeyboardButtonKind::CallbackData("fairytalescontinue".into())
    );

    let set_button=InlineKeyboardButton::new(
        "Задать тему",
        InlineKeyboardButtonKind::CallbackData("fairytalesnewtheme".into())
    );

    InlineKeyboardMarkup::new(vec![vec![voice],vec![next_button,set_button]])
}


pub fn themed_fairy_tales_start_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(Some("Озвучить"));

    let next_button=InlineKeyboardButton::new(
        "Сказка",
        InlineKeyboardButtonKind::CallbackData("fairytalescontinue".into())
    );

    let reset_button=InlineKeyboardButton::new(
        "Сбросить тему",
        InlineKeyboardButtonKind::CallbackData("fairytalesresettheme".into())
    );

    InlineKeyboardMarkup::new(vec![vec![voice],vec![next_button,reset_button]])
}


pub fn themed_fairy_tales_continue_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(Some("Озвучить сказку"));

    let next_button=InlineKeyboardButton::new(
        "Сказка",
        InlineKeyboardButtonKind::CallbackData("fairytalescontinue".into())
    );

    let reset_button=InlineKeyboardButton::new(
        "Сбросить тему",
        InlineKeyboardButtonKind::CallbackData("fairytalesresettheme".into())
    );

    InlineKeyboardMarkup::new(vec![vec![voice],vec![next_button,reset_button]])
}