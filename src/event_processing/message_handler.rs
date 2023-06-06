use crate::{
    openai::OpenAI,
    event_processing::CommandHandler,
    chat::{
        Chat,
        ChatMode,
        TutorMode,
        ChatState,
        ChatAction
    }
};

use std::sync::Arc;
use tokio::sync::Mutex;

use serde_json::Value;
use teloxide_core::{
    Bot,
    requests::Requester,
    types::{
        Message,
        InlineKeyboardButton,
        InlineKeyboardMarkup,
        InlineKeyboardButtonKind,
        ReplyMarkup
    }
};



pub struct MessageHandler{
    bot:Bot,
    message:Message,
    chat:Arc<Mutex<Chat>>
}

impl MessageHandler{
    pub fn new(bot:Bot,message:Message,chat:Arc<Mutex<Chat>>)->MessageHandler{
        Self{
            bot,
            message,
            chat
        }
    }

    pub async fn handle(self){
        let Some(text)=self.message.text()else{return};

        if text.starts_with("/"){
            if let Some(command)=text.get(1..){
                CommandHandler::new(self.bot,self.chat).handle(command).await;
            }
            return
        }

        let mut chat=self.chat.lock().await;

        match chat.state{
            ChatState::Initial(action)=>{
                match action{
                    ChatAction::Greet=>{}

                    ChatAction::AskName=>{
                        chat.user.name=text.to_string();
                        self.bot.send_message(chat.id,"Сколько тебе лет?").await.unwrap();
                        chat.state=ChatState::Initial(ChatAction::AskAge)
                    }

                    ChatAction::AskAge=>{
                        if let Ok(age)=text.parse(){
                            chat.user.age=age;

                            let mut send_message=self.bot.send_message(chat.id,"А ты мальчик или девочка?");

                            let male_button=InlineKeyboardButton::new(
                                "Мальчик",
                                InlineKeyboardButtonKind::CallbackData("sexmale".into())
                            );
                            let female_button=InlineKeyboardButton::new(
                                "Девочка",
                                InlineKeyboardButtonKind::CallbackData("sexfemale".into())
                            );

                            send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup::new(vec![vec![male_button,female_button]])));

                            send_message.await.unwrap();

                            chat.state=ChatState::Initial(ChatAction::AskSex)
                        }
                        else{
                            self.bot.send_message(chat.id,"Немного не понял. Напиши свой возраст ещё раз.").await.unwrap();
                        }
                    }

                    ChatAction::AskSex=>{
                        chat.state=ChatState::FullyFunctional(ChatMode::Chatting)
                    }
                }
            }

            ChatState::PartiallyFunctional=>{
                
            }

            ChatState::FullyFunctional(mode)=>{
                let text=text
                    .replace("\"","\\\"")
                    .replace("\n","\\n")
                    .replace("\r","\\r")
                    .replace("\t","\\t");

                chat.history().push_request(&text);

                let mut messages=Vec::new();

                match mode{
                    ChatMode::Chatting=>{
                        let prompt=std::fs::read_to_string("prompts/chatting.json").unwrap().replace('\n',"");
                        messages.push(prompt)
                    }

                    ChatMode::Tutor(mode)=>{
                        chat.history().clear();
                        match mode{
                            TutorMode::None=>{
                                return
                            }

                            TutorMode::Math=>{
                                let prompt=std::fs::read_to_string("prompts/tutor/math.json").unwrap().replace('\n',"");
                                messages.push(prompt)
                            }

                            TutorMode::English=>{
                                let prompt=std::fs::read_to_string("prompts/tutor/english.json").unwrap().replace('\n',"");
                                messages.push(prompt)
                            }

                            TutorMode::Finance=>{
                                let prompt=std::fs::read_to_string("prompts/tutor/finance.json").unwrap().replace('\n',"");
                                messages.push(prompt)
                            }

                            TutorMode::Astronomy=>{
                                let prompt=std::fs::read_to_string("prompts/tutor/astronomy.json").unwrap().replace('\n',"");
                                messages.push(prompt)
                            }

                            TutorMode::Enviroment=>{
                                let prompt=std::fs::read_to_string("prompts/tutor/enviroment.json").unwrap().replace('\n',"");
                                messages.push(prompt)
                            }

                            TutorMode::SelfCare=>{
                                let prompt=std::fs::read_to_string("prompts/tutor/selfcare.json").unwrap().replace('\n',"");
                                messages.push(prompt)
                            }
                        }
                    }

                    ChatMode::Game=>{
                        let prompt=std::fs::read_to_string("prompts/game.json").unwrap().replace('\n',"");
                        messages.push(prompt)
                    }

                    ChatMode::Playground=>{}

                    _=>return
                }

                let history=chat.history().messages();

                for message in history.iter(){
                    messages.push(message.to_string())
                }

                let response=loop{
                    if let Some(response)=OpenAI::send_chat(&messages,1024){
                        break response
                    }
                    println!("Ошибка");
                };

                if let ChatMode::Chatting=mode{
                    let response:Value=serde_json::from_str(&response.replace('\'',"\"")).unwrap();

                    if let Some(abilities)=response["abilities"].as_i64(){
                        if abilities>=9{
                            self.bot.send_message(chat.id,"Я умею чирикать").await.unwrap();
                            return
                        }
                    }

                    if let Some(play)=response["play"].as_i64(){
                        if play>=9{
                            let play_button=InlineKeyboardButton::new(
                                "Играть",
                                InlineKeyboardButtonKind::CallbackData("play".into())
                            );

                            let mut send_message=self.bot.send_message(chat.id,"Если хочешь поиграть, то мы можешь перейти в игровой режим.");

                            send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup::new(vec![vec![play_button]])));

                            send_message.await.unwrap();
                            return
                        }
                    }
                }

                println!("Response: {}",response);

                chat.history().push_response(
                    &response
                    .replace("\"","\\\"")
                    .replace("\n","\\n")
                    .replace("\r","\\r")
                    .replace("\t","\\t")
                );

                self.bot.send_message(chat.id,response).await.unwrap();
            }
        }
    }
}