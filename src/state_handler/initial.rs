use crate::{
    model_handler::ChatHandler,
    synthesize_voice_button,
    resources::Resources,
    extended_bot_interface::ExtendedBotInterface,
    chat::{
        Chat,
        ChatState, Chats
    }
};

use std::sync::Arc;

use tokio::sync::RwLock;

use teloxide_core::{types::{
    InlineKeyboardButton,
    InlineKeyboardButtonKind,
    ReplyMarkup,
    ChatId, KeyboardButton, KeyboardMarkup
}, requests::Requester};



pub struct InitialState<'c>{
    bot:ExtendedBotInterface,
    chat:&'c mut Chat
}

impl<'c> InitialState<'c>{
    pub fn new(
        bot:ExtendedBotInterface,
        chat:&'c mut Chat
    )->InitialState<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(self,command:&str,chats:Arc<RwLock<Chats>>){
        let mut command_split=command.split_whitespace();

        let Some(command)=command_split.next()else{return};

        match command{
            "start"=>{
                let buttons=vec![
                    vec![
                        KeyboardButton::new("Развивайка \u{1F393}"),
                        KeyboardButton::new("Игры \u{26BD}"),
                    ],
                    vec![
                        KeyboardButton::new("Домашка \u{1F4DD}"),
                        KeyboardButton::new("Поговорить с Другом \u{1F466}")
                    ]
                ];

                let mut send_message=self.bot.send_message(self.chat.id,"Обработка...").inner;
                send_message.reply_markup=Some(ReplyMarkup::Keyboard(
                    KeyboardMarkup::new(buttons).resize_keyboard(true).one_time_keyboard(true)
                ));
                let sent_message=send_message.await.unwrap();

                self.bot.bot.delete_message(self.chat.id,sent_message.id).await.unwrap();

                if let Some(args)=command_split.next(){
                    println!("Args: {args}");

                    if let Ok(chat_id)=args.parse::<i64>(){
                        let chat_id=ChatId(chat_id);
                        if chat_id!=self.chat.id{
                            let chats=chats.read().await;
                            if let Some(chat)=chats.get(chat_id){
                                chat.lock().await.user.people_invited+=1;

                                self.chat.set_invited(true);

                                self.bot.send_message(self.chat.id,"Вы перешли по реферальной ссылке - Вам будут доступны бонусы.").reply_markup(synthesize_voice_button(None)).await.unwrap();
                            }
                        }
                    }
                }
                else{
                    println!("No args")
                }

                let start=InlineKeyboardButton::new(
                    "Старт",
                    InlineKeyboardButtonKind::CallbackData("start".into())
                );

                let voice=synthesize_voice_button(None);

                let greeting=Resources::load_text("resources/text/general/start/greeting.txt").await;

                self.bot.send_message(self.chat.id,greeting).reply_markup(vec![start,voice]).await.unwrap();
            }

            // Сообщение при для других команд`
            _=>{
                let access_denied=Resources::load_text("resources/text/general/access_denied.txt").await;

                self.bot.send_message(self.chat.id,access_denied).reply_markup(synthesize_voice_button(None)).await.unwrap();
            }
        }
    }

    pub async fn handle_command_callback(self,callback:String){
        match callback.as_str(){
            "start"=>{
                self.chat.state=ChatState::FullyFunctional;

                ChatHandler::new(self.bot,self.chat).handle_command_callback(callback).await;
            }

            _=>{}
        }
    }

    pub async fn handle_message(self,_message:&str){
        self.bot.send_message(self.chat.id,"Напиши /start, чтобы начать").await.unwrap();
    }
}