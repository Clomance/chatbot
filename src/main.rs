mod chat;

mod openai;

use chat::Chats;
use openai::OpenAI;

use teloxide_core::{
    Bot,
    requests::Requester,
    types::{AllowedUpdate, UpdateKind, BotCommand, BotCommandScope, Update},
};

use tokio::runtime::Builder;

use chat::ChatMode;



const BOT_TOKEN:&'static str="5743583697:AAENl1sDvgMD0DCdEuQmqAmFBvtVrw0vgX0";



fn main(){
    let bot=Bot::new(BOT_TOKEN);

    let rt=Builder::new_current_thread()
        .enable_all()
        .thread_stack_size(1024*1024)
        .worker_threads(2)
        .max_blocking_threads(2)
        .build().unwrap();

    rt.block_on(
        async{
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
                        AllowedUpdate::Message
                    ]
                );

                let Ok(updates)=update_request.await else{
                    continue
                };

                for update in updates{
                    let update_id=update.id;
                    handle_updates(update,&bot,&mut chats).await;
                    last_update_id=Some(update_id+1);
                }
            }
        }
    );
}




async fn handle_updates(update:Update,bot:&Bot,chats:&mut Chats){
    match update.kind{
        UpdateKind::Message(message)=>{
            let chat=chats.get(message.chat.id);

            if let Some(text)=message.text(){
                let text=text
                    .replace("\"","\\\"")
                    .replace("\n","\\n")
                    .replace("\r","\\r")
                    .replace("\t","\\t");

                let mode_changed;

                match text.as_str(){
                    "/chatting"=>{
                        chat.set_mode(ChatMode::Chatting);
                        mode_changed=true
                    }

                    "/tutor"=>{
                        chat.set_mode(ChatMode::Tutor);
                        mode_changed=true
                    }

                    "/homework"=>{
                        chat.set_mode(ChatMode::Homework);
                        mode_changed=true
                    }

                    "/game"=>{
                        chat.set_mode(ChatMode::Game);
                        mode_changed=true
                    }

                    "/playground"=>{
                        chat.set_mode(ChatMode::Playground);
                        return;
                    }

                    "/settings"=>{
                        // chat.set_mode(ChatMode::Settings);
                        return;
                    }

                    _=>{
                        if text.starts_with("/start"){
                            if let Some(args)=text.get(7..){
                                println!("Args: {args}");
                            }
                            else{
                                println!("No args")
                            }

                            return;
                        }

                        mode_changed=false
                    }
                }

                if mode_changed{
                    bot.send_message(message.chat.id,"Режим изменён, подождите немного.").await.unwrap();
                    chat.history().push_question("");
                }
                else{
                    chat.history().push_question(&text);
                }

                let prompt=chat.build_prompt(&text,mode_changed);
                println!("{}",prompt);

                let mut answer=OpenAI::send_prompt(&prompt);

                while let None=answer{
                    answer=OpenAI::send_prompt(&prompt);
                }

                let answer=answer.unwrap();

                println!("{}",answer);

                let answer=answer
                    .replace("\"","\\\"")
                    .replace("\n","\\n")
                    .replace("\r","\\r")
                    .replace("\t","\\t");

                chat.history().push_answer(&answer);
            }
        }
        _=>{}
    }
}