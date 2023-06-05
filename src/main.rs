mod chat;

mod openai;

use chat::{Chats, Chat};

use openai::OpenAI;

use serde_json::Value;

use teloxide_core::{
    Bot,
    requests::Requester,
    types::{
        AllowedUpdate,
        UpdateKind,
        BotCommand,
        BotCommandScope,
        Update,
        Message,
        ChatId,
        InlineKeyboardButton,
        InlineKeyboardButtonKind,
        ReplyMarkup,
        InlineKeyboardMarkup
    }
};

use chat::ChatMode;

use crate::chat::TutorMode;



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
            BotCommand::new("game","Игры"),
            BotCommand::new("playground","Свободный режим"),
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
                    let chat=chats.get(message.chat.id);

                    handle_command_callback(&data,bot,message.chat.id,chat).await;

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



async fn handle_command_callback(callback:&str,bot:&Bot,chat_id:ChatId,chat:&mut Chat){
    match callback{
        "math"=>{
            chat.set_mode(ChatMode::Tutor(TutorMode::Math));
            bot.send_message(chat_id,"Ты выбрал математику. Давай начнём. Напиши что-нибудь").await.unwrap();
        }

        "rus"=>{
            chat.set_mode(ChatMode::Tutor(TutorMode::Rus));
            bot.send_message(chat_id,"Ты выбрал русский язык. Давай начнём. Напиши что-нибудь").await.unwrap();
        }

        "society"=>{

        }

        "enviroment"=>{

        }

        _=>{}
    }
}



async fn handle_message(message:Message,bot:&Bot,chat:&mut Chat){
    if let Some(text)=message.text(){
        if text.starts_with("/"){
            if let Some(command)=text.get(1..){
                handle_command(command,bot,message.chat.id,chat).await;
            }

            return
        }

        let text=text
            .replace("\"","\\\"")
            .replace("\n","\\n")
            .replace("\r","\\r")
            .replace("\t","\\t");

        chat.history().push_question(&text);

        let mode=chat.mode();

        let mut messages=Vec::new();

        match mode{
            ChatMode::Chatting=>{
                let prompt=std::fs::read_to_string("prompts/chatting.json").unwrap().replace('\n',"");
                messages.push(prompt)
            }

            ChatMode::Tutor(mode)=>{
                match mode{
                    TutorMode::None=>{
                        return
                    }

                    TutorMode::Math=>{
                        let prompt=std::fs::read_to_string("prompts/tutor/math.json").unwrap().replace('\n',"");
                        messages.push(prompt)
                    }

                    TutorMode::Rus=>{
                        let prompt=std::fs::read_to_string("prompts/tutor/rus.json").unwrap().replace('\n',"");
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

        let answer=loop{
            if let Some(answer)=OpenAI::send_chat(&messages, 1024){
                break answer
            }
            println!("Ошибка");
        };

        if let ChatMode::Chatting=mode{
            let response:Value=serde_json::from_str(&answer.replace('\'',"\"")).unwrap();

            if let Some(abilities)=response["abilities"].as_i64(){
                if abilities>=9{
                    bot.send_message(message.chat.id,"Я умею чирикать").await.unwrap();
                    return
                }
            }

            if let Some(play)=response["play"].as_i64(){
                if play>=9{
                    let play_button=InlineKeyboardButton::new(
                        "Играть",
                        InlineKeyboardButtonKind::CallbackData("play".into())
                    );

                    let mut send_message=bot.send_message(message.chat.id,"Если хочешь поиграть, то мы можешь перейти в игровой режим.");

                    send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup::new(vec![vec![play_button]])));

                    send_message.await.unwrap();
                    return
                }
            }
        }

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



async fn handle_command(command:&str,bot:&Bot,chat_id:ChatId,chat:&mut Chat){
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
            chat.set_mode(ChatMode::Tutor(TutorMode::None));
            let mut send_message=bot.send_message(chat_id,"Выбери то, чем хочешь заняться.");
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
            bot.send_message(chat_id,"pong").await.unwrap();
        }

        _=>{}
    }
}


fn tutor_buttons()->InlineKeyboardMarkup{
    let math=InlineKeyboardButton::new(
        "Математика",
        InlineKeyboardButtonKind::CallbackData("math".into())
    );

    let english=InlineKeyboardButton::new(
        "Английский",
        InlineKeyboardButtonKind::CallbackData("rus".into())
    );

    // let usdc_button=InlineKeyboardButton::new(
    //     "Общество",
    //     InlineKeyboardButtonKind::CallbackData("society".into())
    // );

    // let none_button=InlineKeyboardButton::new(
    //     "Окружающий мир",
    //     InlineKeyboardButtonKind::CallbackData("enviroment".into())
    // );

    let inline_buttons=vec![
        vec![math],
        vec![english],
        // vec![usdc_button],
        // vec![none_button]
    ];

    InlineKeyboardMarkup{
        inline_keyboard:inline_buttons
    }
}