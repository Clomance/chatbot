mod chat;

mod openai;

use chat::Chats;
use openai::OpenAI;

use teloxide_core::{
    Bot,
    requests::Requester,
    types::{AllowedUpdate, UpdateKind},
};

use tokio::runtime::Builder;



const BOT_TOKEN:&'static str="6113109632:AAEvVdzWlmyb8gYfZC8JyGDpDFzY_Dha2-c";



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

                    match update.kind{
                        UpdateKind::Message(message)=>{
                            let chat=chats.get(message.chat.id);

                            if let Some(text)=message.text(){
                                let text=text.replace("\"","\'");

                                let prompt=format!(
                                    "You are a russian chat teacher.\
                                    You answer people questions.\
                                    This is your chat history with this user (U - User, Y - You):{}\\n\
                                    U:{text}\\nY:(write just our answer)",
                                    chat.history()
                                );

                                println!("{}",prompt);

                                let answer=OpenAI::send_prompt(&prompt);
                                
                                bot.send_message(message.chat.id,&answer).await.unwrap();

                                chat.history.push_question(&text);
                                chat.history.push_question(&answer.replace("\n","\\n").replace("\"","\'"));
                            }
                        }
                        _=>{}
                    }

                    last_update_id=Some(update_id+1);
                }
            }
        }
    );
}