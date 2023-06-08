use crate::{
    prompt::Prompt,
    openai::OpenAIWrapper,
    chat::{
        Chat,
        History
    },
};


use teloxide_core::{
    Bot,
    requests::Requester,
    types::{
        InlineKeyboardMarkup,
        InlineKeyboardButton,
        InlineKeyboardButtonKind,
        ReplyMarkup
    }
};



const GREETING:&'static str="\
Отлично! Я искусственный интеллект, но могу стать твоим верным помощником в постижении полезных знаний, изучении языков, математики, физики, астрономии, обсудить интересные научные факты.

Выбери темы, знания о которых тебе сейчас нужны:\
";

const MATH_GREETING:&'static str="\
Математика - царица наук, благодаря инструментам которой, функционируют и другие, точные науки. \
Она пригодится тебе всегда и везде, чтобы ни говорили люди вокруг о ней.
С чего начнем изучение математики? \
Могу тебе помочь объяснить решение некоторых задач.\
";

const ENGLISH_GREETING:&'static str="\
Hi there! Очень рад твоим стремлениям к изучениям языков! \
Мы живем на одной большой Планете, однако история сложилась так, что в мире существуют более 7 тысяч языков, которые относятся к 142 языковым группам. \
Английский по праву язык мира, так как распространен в большинстве стран. \
Также и Я, твой друг, написан языком программирования с использованием команд на английском языке!
Ну что же, Lets go!\
";

const FINANCE_GREETING:&'static str="\
Ты выбрал финасовую граммотность. Давай начнём. Напиши что-нибудь\
";

const ASTRONOMY_GREETING:&'static str="\
Прекрасно! Наверное, ничто не придавало столько энтузиазма в изучении науки, как изучение космоса. \
Человек всегда стремился к звездам, изучению планет, космических объектов. \
Человек ступил на Луну и уже сегодня готовятся миссии на ближайшие планеты Солнечной системы.

Напиши, что ты хочешь узнать из раздела астрономии, о юный исследователь.\
";

const ENVIROMENT_GREETING:&'static str="\
Ты выбрал окружающий мир. Давай начнём. Напиши что-нибудь\
";

const SELF_CARE_GREETING:&'static str="\
Ты выбрал здоровье и гигиена. Давай начнём. Напиши что-нибудь\
";



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
    bot:Bot,
    chat:&'c mut Chat,
}

impl<'c> TutorHandler<'c>{
    pub fn new(bot:Bot,chat:&'c mut Chat)->TutorHandler<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(&mut self,command:&str){
        match command{
            "reset"=>{
                self.chat.data.tutor().history.clear();

                self.bot.send_message(self.chat.id,"История очищена").await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_command_callback(&mut self,callback:&str){
        let data=self.chat.data.tutor();

        data.history.clear();

        match callback{
            "tutor"=>{
                let mut send_message=self.bot.send_message(self.chat.id,GREETING);
                send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(tutor_buttons()));
                send_message.await.unwrap();
            }

            "math"=>{
                data.mode=TutorMode::Math;
                self.bot.send_message(self.chat.id,MATH_GREETING).await.unwrap();
            }

            "english"=>{
                data.mode=TutorMode::English;
                self.bot.send_message(self.chat.id,ENGLISH_GREETING).await.unwrap();
            }

            "finance"=>{
                data.mode=TutorMode::Finance;
                self.bot.send_message(self.chat.id,FINANCE_GREETING).await.unwrap();
            }

            "astronomy"=>{
                data.mode=TutorMode::Astronomy;
                self.bot.send_message(self.chat.id,ASTRONOMY_GREETING).await.unwrap();
            }

            "enviroment"=>{
                data.mode=TutorMode::Enviroment;
                self.bot.send_message(self.chat.id,ENVIROMENT_GREETING).await.unwrap();
            }

            "selfcare"=>{
                data.mode=TutorMode::SelfCare;
                self.bot.send_message(self.chat.id,SELF_CARE_GREETING).await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(&mut self,message:&str){
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

        let response=OpenAIWrapper::send_chat(&messages,1024);

        println!("Response: {}",response);

        data.history.push_response(&response);

        self.bot.send_message(self.chat.id,response).await.unwrap();
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