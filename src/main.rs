mod chat;
use std::{sync::Arc, time::Duration};

use chat::Chats;

mod openai;

mod event_processing;

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

use crate::event_processing::UpdateHandler;



const BOT_TOKEN:&'static str="5743583697:AAENl1sDvgMD0DCdEuQmqAmFBvtVrw0vgX0";



#[tokio::main]
async fn main(){
    let bot=Bot::new(BOT_TOKEN);

    println!("BotStart");

    let mut delete=bot.delete_my_commands();
    delete.scope=Some(BotCommandScope::Default);
    delete.await.unwrap();

    bot.set_my_commands(
        vec![
            BotCommand::new("chatting","Болтовня"),
            BotCommand::new("studying","Учёба"),
            BotCommand::new("game","Игры"),
            BotCommand::new("playground","Свободный режим"),
            BotCommand::new("settings","Настройки")
        ]
    ).await.unwrap();

    println!("CommandsSet");

    let chats=Arc::new(Mutex::new(Chats::new()));

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

            tokio::spawn(async move{
                UpdateHandler::new(bot,update,chats).handle().await
            });
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}