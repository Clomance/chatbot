mod chat;
use std::{
    sync::Arc,
    time::Duration
};

use chat::Chats;

mod openai;

mod event;
use event::UpdateHandler;

mod model;

mod prompt;

use teloxide_core::{
    Bot,
    requests::Requester,
    types::{
        AllowedUpdate,
        BotCommand,
        BotCommandScope
    }
};

use tokio::sync::Mutex;



const TELEGRAM_BOT_TOKEN:&'static str="5743583697:AAENl1sDvgMD0DCdEuQmqAmFBvtVrw0vgX0";



#[tokio::main]
async fn main(){
    println!("BotStart");

    let bot=Bot::new(TELEGRAM_BOT_TOKEN);

    let mut delete=bot.delete_my_commands();
    delete.scope=Some(BotCommandScope::Default);
    delete.await.unwrap();

    bot.set_my_commands(
        vec![
            BotCommand::new("chat","Простое общение"),
            BotCommand::new("study","Учёба"),
            BotCommand::new("game","Игры"),
            BotCommand::new("playground","Свободный режим"),
            BotCommand::new("reset","Очистить историю"),
            BotCommand::new("settings","Настройки"),
            BotCommand::new("profile","Профиль"),
        ]
    ).await.unwrap();

    println!("CommandsSet");

    let chats=Arc::new(Mutex::new(Chats::new()));

    println!("UpdateLoopStart");

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
            last_update_id=Some(update.id+1);

            let bot=bot.clone();
            let chats=chats.clone();

            tokio::spawn(UpdateHandler::new(bot,update,chats).handle());
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}