mod chat;

mod openai;

use chat::{Chats, Chat};
use openai::OpenAI;

use teloxide_core::{
    Bot,
    requests::Requester,
    types::{AllowedUpdate, UpdateKind, BotCommand, BotCommandScope, Update, Message},
};

use chat::ChatMode;



const BOT_TOKEN:&'static str="5743583697:AAENl1sDvgMD0DCdEuQmqAmFBvtVrw0vgX0";



#[tokio::main]
async fn main(){
    let bot=Bot::new(BOT_TOKEN);

    println!("openai bot start");

    let mut delete=bot.delete_my_commands();
    delete.scope=Some(BotCommandScope::Default);
    delete.await.unwrap();

    bot.set_my_commands(
        vec![
            BotCommand::new("chatting","Простое общение"),
            BotCommand::new("tutor","Репетитор"),
            BotCommand::new("homework","Помощь с домашним задание"),
            BotCommand::new("game","Игры"),
            BotCommand::new("playground","Свобожный режим"),
            BotCommand::new("settings","Настройки")
        ]
    ).await.unwrap();

    let mut chats=Chats::new();

    let mut last_update_id=None;
    loop{
        let mut update_request=bot.get_updates();
        update_request.offset=last_update_id;
        update_request.allowed_updates=Some(
            vec![
                AllowedUpdate::Message,
                AllowedUpdate::CallbackQuery
            ]
        );

        let Ok(updates)=update_request.await else{
            continue
        };

        for update in updates{
            let update_id=update.id;
            handle_update(update,&bot,&mut chats).await;
            last_update_id=Some(update_id+1);
        }
    }
}



async fn handle_update(update:Update,bot:&Bot,chats:&mut Chats){
    match update.kind{
        UpdateKind::CallbackQuery(query)=>{
            if let Some(message)=query.message{
                if let Some(data)=query.data{
                    let _chat=chats.get(message.chat.id);

                    match data.as_str(){
                        _=>{}
                    }
                }
            }

            let _=bot.answer_callback_query(query.id).await;
        }

        UpdateKind::Message(message)=>{
            let chat=chats.get(message.chat.id);

            handle_message(message,bot,chat).await;
        }

        _=>{}
    }
}



async fn handle_message(message:Message,bot:&Bot,chat:&mut Chat){
    if let Some(text)=message.text(){
        if text.starts_with("/"){
            if let Some(command)=text.get(1..){
                handle_command(command,bot,chat).await;
            }

            return
        }

        let text=text
            .replace("\"","\\\"")
            .replace("\n","\\n")
            .replace("\r","\\r")
            .replace("\t","\\t");

        // let mode_changed;

        chat.history().push_question(&text);

        let prompt=chat.build_prompt(&text,false);
        println!("{}",prompt);

        let answer=loop{
            if let Some(answer)=OpenAI::send_prompt(&prompt){
                break answer
            }
            println!("Ошибка");
        };

        println!("{}",answer);

        let answer=answer
            .replace("\"","\\\"")
            .replace("\n","\\n")
            .replace("\r","\\r")
            .replace("\t","\\t");

        chat.history().push_answer(&answer);

        bot.send_message(message.chat.id,answer).await.unwrap();
    }
}



async fn handle_command(command:&str,_bot:&Bot,chat:&mut Chat){
    if command.starts_with("start"){
        if let Some(args)=command.get(7..){
            println!("Args: {args}");
        }
        else{
            println!("No args")
        }

        return;
    }

    match command{
        "chatting"=>{
            chat.set_mode(ChatMode::Chatting);
        }

        "tutor"=>{
            chat.set_mode(ChatMode::Tutor);
        }

        "homework"=>{
            chat.set_mode(ChatMode::Homework);
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

        _=>{}
    }
}



fn create_tutor_menu(){
    
}