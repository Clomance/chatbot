mod chat;
use chat::Chats;

mod resources;

mod extended_bot_interface;
use extended_bot_interface::ExtendedBotInterface;

mod openai;
use openai::OpenAI;

mod yandex;
use yandex::YandexSpeechKit;

mod event_handler;
use event_handler::UpdateHandler;

mod model_handler;

mod module;

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
        InlineKeyboardButton,
        InlineKeyboardButtonKind
    }
};

use tokio::sync::RwLock;

use serde_json::Value;



#[tokio::main]
async fn main(){
    println!("Launched");

    // Загрузка настроек
    let settings:Value=serde_json::from_slice(
        &std::fs::read("settings/api.json").expect("No settings file")
    ).expect("Wrong settings format");

    // Получение и установка токена телеграм-бота
    let bot=ExtendedBotInterface::new(settings["TELEGRAM_BOT_TOKEN"].as_str().unwrap()); // 5743583697:AAENl1sDvgMD0DCdEuQmqAmFBvtVrw0vgX0

    // Получение и установка ключа для интерфейса OpenAI
    OpenAI::set_token(settings["OPENAI_API_KEY"].as_str().unwrap());

    // Получение и установка токенов для интерфейса Яндекса
    YandexSpeechKit::set_oauth_token(settings["YANDEX_OAUTH_TOKEN"].as_str().unwrap());
    YandexSpeechKit::set_folder_id(settings["YANDEX_FOLDER_ID"].as_str().unwrap());

    println!("Loaded");

    // Удаление старых команд
    let mut delete=bot.bot.delete_my_commands();
    delete.scope=Some(BotCommandScope::Default);
    delete.await.unwrap();

    // Установка новых команд
    bot.bot.set_my_commands(
        [
            BotCommand::new("chat","Простое общение"),
            BotCommand::new("study","Учёба"),
            BotCommand::new("game","Игры"),
            BotCommand::new("playground","Свободный режим"),
            BotCommand::new("reset","Очистить историю"),
            BotCommand::new("settings","Настройки"),
            BotCommand::new("profile","Профиль"),
            BotCommand::new("etc","Дополнительно")
        ]
    ).await.unwrap();

    println!("Configured");

    let chats=Arc::new(RwLock::new(Chats::new()));

    println!("Started");

    let mut last_update_id=None;
    loop{
        // Получение обновлений (только сообщения и нажатия кнопок)
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

            // Выделение параллельной задачи для обработки обновления
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