use crate::{
    openai::OpenAIWrapper,
    chat::{
        Chat,
        History
    }
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
            _=>{}
        }
    }

    pub async fn handle_command_callback(&mut self,callback:&str){
        let data=self.chat.data.tutor();

        data.history.clear();

        match callback{
            "tutor"=>{
                let mut send_message=self.bot.send_message(self.chat.id,"Выбери то, чем хочешь заняться.");
                send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(tutor_buttons()));
                send_message.await.unwrap();
            }

            "math"=>{
                data.mode=TutorMode::Math;
                self.bot.send_message(self.chat.id,"Ты выбрал математику. Давай начнём. Напиши что-нибудь").await.unwrap();
            }

            "english"=>{
                data.mode=TutorMode::English;
                self.bot.send_message(self.chat.id,"Ты выбрал английский язык. Давай начнём. Напиши что-нибудь").await.unwrap();
            }

            "finance"=>{
                data.mode=TutorMode::Finance;
                self.bot.send_message(self.chat.id,"Ты выбрал финасовую граммотность. Давай начнём. Напиши что-нибудь").await.unwrap();
            }

            "astronomy"=>{
                data.mode=TutorMode::Astronomy;
                self.bot.send_message(self.chat.id,"Ты выбрал астрономию. Давай начнём. Напиши что-нибудь").await.unwrap();
            }

            "enviroment"=>{
                data.mode=TutorMode::Enviroment;
                self.bot.send_message(self.chat.id,"Ты выбрал окружающий мир. Давай начнём. Напиши что-нибудь").await.unwrap();
            }

            "selfcare"=>{
                data.mode=TutorMode::SelfCare;
                self.bot.send_message(self.chat.id,"Ты выбрал здоровье и гигиена. Давай начнём. Напиши что-нибудь").await.unwrap();
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
                let prompt=std::fs::read_to_string("prompts/study/tutor/math.json").unwrap().replace('\n',"");
                messages.push(prompt)
            }

            TutorMode::English=>{
                let prompt=std::fs::read_to_string("prompts/study/tutor/english.json").unwrap().replace('\n',"");
                messages.push(prompt)
            }

            TutorMode::Finance=>{
                let prompt=std::fs::read_to_string("prompts/study/tutor/finance.json").unwrap().replace('\n',"");
                messages.push(prompt)
            }

            TutorMode::Astronomy=>{
                let prompt=std::fs::read_to_string("prompts/study/tutor/astronomy.json").unwrap().replace('\n',"");
                messages.push(prompt)
            }

            TutorMode::Enviroment=>{
                let prompt=std::fs::read_to_string("prompts/study/tutor/enviroment.json").unwrap().replace('\n',"");
                messages.push(prompt)
            }

            TutorMode::SelfCare=>{
                let prompt=std::fs::read_to_string("prompts/study/tutor/selfcare.json").unwrap().replace('\n',"");
                messages.push(prompt)
            }
        }

        data.history.push_request(message);

        for message in data.history.messages().iter(){
            messages.push(message.to_string())
        }

        let response=OpenAIWrapper::send_chat(&messages,1024);

        println!("Response: {}",response);

        data.history.push_response(
            &response
            .replace("\"","\\\"")
            .replace("\n","\\n")
            .replace("\r","\\r")
            .replace("\t","\\t")
        );

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