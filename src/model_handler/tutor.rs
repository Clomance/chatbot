use crate::{
    synthesize_voice_button,
    prompt::Prompt,
    openai::OpenAIWrapper,
    resources::Resources,
    extended_bot_interface::ExtendedBotInterface,
    chat::{
        Chat,
        History
    }
};

use teloxide_core::{
    requests::Requester,
    types::{
        InlineKeyboardMarkup,
        InlineKeyboardButton,
        InlineKeyboardButtonKind
    }
};



pub enum TutorMode{
    None,
    Math,
    English,
    Finance,
    Astronomy,
    Enviroment,
    SelfCare
}



pub struct TutorModel{
    history:History,
    mode:TutorMode
}

impl TutorModel{
    pub fn new()->TutorModel{
        Self{
            history:History::new(),
            mode:TutorMode::None
        }
    }
}



pub struct TutorHandler<'c>{
    bot:ExtendedBotInterface,
    chat:&'c mut Chat,
}

impl<'c> TutorHandler<'c>{
    pub fn new(bot:ExtendedBotInterface,chat:&'c mut Chat)->TutorHandler<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(&mut self,command:&str){
        match command{
            "reset"=>{
                self.chat.data.tutor().history.clear();

                self.bot.send_message(self.chat.id,"История очищена").reply_markup(synthesize_voice_button(None)).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_command_callback(&mut self,callback:&str){
        let data=self.chat.data.tutor();

        data.history.clear();

        let greeting=match callback{
            "tutor"=>{
                let greeting=Resources::load_text("resources/text/study/tutor/greeting.txt").await;

                self.bot.send_message(self.chat.id,greeting).reply_markup(tutor_buttons()).await.unwrap();

                return
            }

            "math"=>{
                data.mode=TutorMode::Math;

                Resources::load_text("resources/text/study/tutor/math/greeting.txt").await
            }

            "english"=>{
                data.mode=TutorMode::English;

                Resources::load_text("resources/text/study/tutor/english/greeting.txt").await
            }

            "finance"=>{
                data.mode=TutorMode::Finance;

                Resources::load_text("resources/text/study/tutor/finance/greeting.txt").await
            }

            "astronomy"=>{
                data.mode=TutorMode::Astronomy;

                Resources::load_text("resources/text/study/tutor/astronomy/greeting.txt").await
            }

            "enviroment"=>{
                data.mode=TutorMode::Enviroment;

                Resources::load_text("resources/text/study/tutor/enviroment/greeting.txt").await
            }

            "selfcare"=>{
                data.mode=TutorMode::SelfCare;

                Resources::load_text("resources/text/study/tutor/selfcare/greeting.txt").await
            }

            _=>return
        };

        self.bot.send_message(self.chat.id,greeting).reply_markup(synthesize_voice_button(None)).await.unwrap();
    }

    pub async fn handle_message(&mut self,message:&str){
        let sent_message=self.bot.send_message(self.chat.id,"Набирает сообщение...").reply_markup(synthesize_voice_button(None)).await.unwrap();

        let mut messages=Vec::new();

        let data=self.chat.data.tutor();

        match data.mode{
            TutorMode::None=>{
                return
            }

            TutorMode::Math=>{
                let prompt=Prompt::load("prompts/study/tutor/math.json").await;
                messages.push(prompt.text)
            }

            TutorMode::English=>{
                let prompt=Prompt::load("prompts/study/tutor/english.json").await;
                messages.push(prompt.text)
            }

            TutorMode::Finance=>{
                let prompt=Prompt::load("prompts/study/tutor/finance.json").await;
                messages.push(prompt.text)
            }

            TutorMode::Astronomy=>{
                let prompt=Prompt::load("prompts/study/tutor/astronomy.json").await;
                messages.push(prompt.text)
            }

            TutorMode::Enviroment=>{
                let prompt=Prompt::load("prompts/study/tutor/enviroment.json").await;
                messages.push(prompt.text)
            }

            TutorMode::SelfCare=>{
                let prompt=Prompt::load("prompts/study/tutor/selfcare.json").await;
                messages.push(prompt.text)
            }
        }

        data.history.push_request(message);

        for message in data.history.messages().iter(){
            messages.push(message.to_string())
        }

        let response=OpenAIWrapper::send_chat(&messages,1024).await;

        self.chat.user.token_usage.total+=response.usage;

        data.history.push_response(&response.message);

        self.bot.bot.delete_message(self.chat.id,sent_message.id).await.unwrap();
        self.bot.send_message(self.chat.id,response.message).reply_markup(synthesize_voice_button(None)).await.unwrap();
    }
}



pub fn tutor_buttons()->InlineKeyboardMarkup{
    let math=InlineKeyboardButton::new(
        "Математика",
        InlineKeyboardButtonKind::CallbackData("math".into())
    );

    let english=InlineKeyboardButton::new(
        "Английский",
        InlineKeyboardButtonKind::CallbackData("english".into())
    );

    let finance=InlineKeyboardButton::new(
        "Финансовая грамотность",
        InlineKeyboardButtonKind::CallbackData("finance".into())
    );

    let astronomy=InlineKeyboardButton::new(
        "Астрономия",
        InlineKeyboardButtonKind::CallbackData("astronomy".into())
    );

    let enviroment=InlineKeyboardButton::new(
        "Окружающий мир",
        InlineKeyboardButtonKind::CallbackData("enviroment".into())
    );

    let self_care=InlineKeyboardButton::new(
        "Здоровье и гигиена",
        InlineKeyboardButtonKind::CallbackData("selfcare".into())
    );

    let inline_buttons=vec![
        vec![synthesize_voice_button(None)],
        vec![math],
        vec![english],
        vec![finance],
        vec![astronomy],
        vec![enviroment],
        vec![self_care]
    ];

    InlineKeyboardMarkup{
        inline_keyboard:inline_buttons
    }
}