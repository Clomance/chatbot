use std::sync::Arc;
use tokio::sync::Mutex;

use teloxide_core::{types::{ReplyMarkup, InlineKeyboardMarkup, InlineKeyboardButton, InlineKeyboardButtonKind}, Bot, requests::Requester};

use crate::{chat::{ChatMode, Chat, TutorMode, ChatState, ChatAction}};

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

                if let ChatState::Initial(ChatAction::Greet)=chat.state{
                    self.bot.send_message(chat.id,"Привет😎\nДавай познакомимся!\nКак тебя зовут?").await.unwrap();
                    chat.state=ChatState::Initial(ChatAction::AskName)
                }
                else{
                    self.bot.send_message(chat.id,"Бот уже запущен").await.unwrap();
                }
            }

            "chatting"=>{
                chat.set_mode(ChatMode::Chatting);
            }

            "studying"=>{
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
                return;
            }

            "ping"=>{
                self.bot.send_message(chat.id,"pong").await.unwrap();
            }

            _=>{}
        }
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