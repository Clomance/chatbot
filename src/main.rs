mod chat;
use chat::Chats;

mod resources;

mod extended_bot_interface;
use extended_bot_interface::ExtendedBotInterface;

mod openai;

mod yandex;

mod event_handler;
use event_handler::UpdateHandler;

mod model_handler;

mod state_handler;

mod prompt;

use std::{
    sync::Arc,
    time::Duration
};

use teloxide_core::{
    requests::Requester,
    types::{
        AllowedUpdate,
        BotCommand,
        BotCommandScope,
        InlineKeyboardMarkup,
        InlineKeyboardButton,
        InlineKeyboardButtonKind
    }
};

use tokio::sync::RwLock;



const TELEGRAM_BOT_TOKEN:&'static str="6058214276:AAFzDlKUr33C3In7jMM5cz6kUDqh85OzFqE";
// NoSenseDevBot - 6058214276:AAFzDlKUr33C3In7jMM5cz6kUDqh85OzFqE
// GptKids - 5743583697:AAENl1sDvgMD0DCdEuQmqAmFBvtVrw0vgX0



#[tokio::main]
async fn main(){
    println!("BotStart");

    let bot=ExtendedBotInterface::new(TELEGRAM_BOT_TOKEN);

    let mut delete=bot.bot.delete_my_commands();
    delete.scope=Some(BotCommandScope::Default);
    delete.await.unwrap();

    bot.bot.set_my_commands(
        [
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

    let chats=Arc::new(RwLock::new(Chats::new()));

    println!("UpdateLoopStart");

    let mut last_update_id=None;
    loop{
        let mut update_request=bot.bot.get_updates();
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



pub fn synthesize_voice_button(text:Option<&str>)->InlineKeyboardButton{
    let text=if let Some(text)=text{
        text
    }
    else{
        "Озвучить"
    };

    InlineKeyboardButton::new(
        text,
        InlineKeyboardButtonKind::CallbackData("synthesize".into())
    )
}



pub fn synthesize_voice_keyboard(text:Option<&str>)->InlineKeyboardMarkup{
    let synthesize=synthesize_voice_button(text);

    let inline_buttons=vec![
        vec![synthesize]
    ];

    InlineKeyboardMarkup{
        inline_keyboard:inline_buttons
    }
}