use std::{sync::{Arc}};
use tokio::sync::Mutex;

use teloxide_core::{Bot, requests::Requester};

use crate::{chat::{ChatMode, Chat, TutorMode}};

pub struct CommandCallbackHandler{
    bot:Bot,
    chat:Arc<Mutex<Chat>>
}

impl CommandCallbackHandler{
    pub fn new(bot:Bot,chat:Arc<Mutex<Chat>>)->CommandCallbackHandler{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle(self,callback:&str){
        let mut chat=self.chat.lock().await;
        
        match callback{
            "math"=>{
                chat.set_mode(ChatMode::Tutor(TutorMode::Math));
                self.bot.send_message(chat.id,"Ты выбрал математику. Давай начнём. Напиши что-нибудь").await.unwrap();
            }
    
            "english"=>{
                chat.set_mode(ChatMode::Tutor(TutorMode::English));
                self.bot.send_message(chat.id,"Ты выбрал английский язык. Давай начнём. Напиши что-нибудь").await.unwrap();
            }
    
            "finance"=>{
                chat.set_mode(ChatMode::Tutor(TutorMode::Finance));
                self.bot.send_message(chat.id,"Ты выбрал финасовую граммотность. Давай начнём. Напиши что-нибудь").await.unwrap();
            }
    
            "astronomy"=>{
                chat.set_mode(ChatMode::Tutor(TutorMode::Astronomy));
                self.bot.send_message(chat.id,"Ты выбрал астрономию. Давай начнём. Напиши что-нибудь").await.unwrap();
            }
    
            "enviroment"=>{
                chat.set_mode(ChatMode::Tutor(TutorMode::Enviroment));
                self.bot.send_message(chat.id,"Ты выбрал окружающий мир. Давай начнём. Напиши что-нибудь").await.unwrap();
            }
    
            "selfcare"=>{
                chat.set_mode(ChatMode::Tutor(TutorMode::SelfCare));
                self.bot.send_message(chat.id,"Ты выбрал здоровье и гигиена. Давай начнём. Напиши что-нибудь").await.unwrap();
            }
    
            _=>{}
        }
    }
}