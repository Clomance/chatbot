use std::{sync::{Arc}};
use tokio::sync::Mutex;

use teloxide_core::{types::{ReplyMarkup}, Bot, requests::Requester};

use crate::{chat::{ChatMode, Chat, TutorMode}, tutor_buttons};

pub struct CommandHandler{
    bot:Bot,
    chat:Arc<Mutex<Chat>>
}

impl CommandHandler{
    pub fn new(bot:Bot,chat:Arc<Mutex<Chat>>)->CommandHandler{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle(self,command:&str){
        let mut command_split=command.split_whitespace();

        let Some(command)=command_split.next()else{return};

        let mut chat=self.chat.lock().await;

        match command{
            "start"=>{
                if let Some(args)=command_split.next(){
                    println!("Args: {args}");
                }
                else{
                    println!("No args")
                }
            }

            "chatting"=>{
                chat.set_mode(ChatMode::Chatting);
            }

            "tutor"=>{
                chat.set_mode(ChatMode::Tutor(TutorMode::None));
                let mut send_message=self.bot.send_message(chat.id,"Выбери то, чем хочешь заняться.");
                send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(tutor_buttons()));
                send_message.await.unwrap();
            }

            "game"=>{
                chat.set_mode(ChatMode::Game);
            }

            "playground"=>{
                chat.set_mode(ChatMode::Playground);
                return;
            }

            "settings"=>{
                // chat.set_mode(ChatMode::Settings);
                return;
            }

            "ping"=>{
                self.bot.send_message(chat.id,"pong").await.unwrap();
            }

            _=>{}
        }
    }
}