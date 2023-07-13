#![feature(decl_macro)]

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

mod keyboard;

mod web_server;
use web_server::WebServer;

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

    let bot;
    {
        // Загрузка настроек
        let settings:Value=serde_json::from_slice(
            &std::fs::read("settings/api.json").expect("No settings file")
        ).expect("Wrong settings format");

        // Получение и установка токена телеграм-бота
        let field=if cfg!(debug_assertions){
            "TELEGRAM_TEST_BOT_TOKEN"
        }
        else{
            "TELEGRAM_BOT_TOKEN"
        };
        bot=ExtendedBotInterface::new(settings[field].as_str().unwrap());

        // Получение и установка ключа для интерфейса OpenAI
        OpenAI::set_token(settings["OPENAI_API_KEY"].as_str().unwrap());

        // Получение и установка токенов для интерфейса Яндекса
        YandexSpeechKit::set_oauth_token(settings["YANDEX_OAUTH_TOKEN"].as_str().unwrap());
        YandexSpeechKit::set_folder_id(settings["YANDEX_FOLDER_ID"].as_str().unwrap());
    }

    {
        // Загрузка списка команд
        let command_menu:Vec<Value>=serde_json::from_slice(
            &std::fs::read("resources/menu/command_menu.json").expect("No settings file")
        ).expect("Wrong settings format");

        let mut commands=Vec::new();
        for command in command_menu{
            let name=command["command"].as_str().unwrap();
            let description=command["description"].as_str().unwrap();
            commands.push(BotCommand::new(name,description))
        }

        // Удаление старых команд
        let mut delete=bot.bot.delete_my_commands();
        delete.scope=Some(BotCommandScope::Default);
        delete.await.unwrap();

        // Установка новых команд
        bot.bot.set_my_commands(commands).await.unwrap();
    }

    println!("Configured");

    let chats=Arc::new(RwLock::new(Chats::new()));

    tokio::spawn(WebServer::new().run());

    println!("Started");

    let mut last_update_id=None;
    loop{
        // Чтобы не превысить лимит запросов телеграма и сам сервер
        // (лимиты телеграма носят рекомендательных характер и не имеют строгих ограничений,
        // 10 запросов в секунду - оптимальный вариант)
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Настройка запроса обновлений (только сообщения и нажатия кнопок)
        let mut update_request=bot.bot.get_updates();
        update_request.offset=last_update_id;
        update_request.allowed_updates=Some(
            vec![
                AllowedUpdate::Message,
                AllowedUpdate::CallbackQuery
            ]
        );

        // Получение обновлений, при ошибке цикл начинается заново
        let Ok(updates)=update_request.await else{continue};

        // Обработка полученных обновлений
        for update in updates{
            last_update_id=Some(update.id+1);

            let bot=bot.clone();
            let chats=chats.clone();

            // Передача обновления новой задаче для параллельного выполения
            tokio::spawn(UpdateHandler::new(bot,update,chats).handle());
        }
    }
}


/// Создаёт кнопку для синтеза речи.
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