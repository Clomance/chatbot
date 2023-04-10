mod openai;

use openai::OpenAI;

use teloxide_core::{
    Bot,
    requests::Requester,
    types::{AllowedUpdate, UpdateKind},
};

use tokio::runtime::Builder;



const BOT_TOKEN:&'static str="6058214276:AAFzDlKUr33C3In7jMM5cz6kUDqh85OzFqE";



fn main(){
    // let file_id=OpenAI::upload_file("data.jsonl","fine-tune");
    // OpenAI::create_fine_tune("file-vcv84KKd2vhIjGBkjUDgJHB8");
    // OpenAI::get_fine_tunes();

    let bot=Bot::new(BOT_TOKEN);

    let rt=Builder::new_current_thread()
        .enable_all()
        .thread_stack_size(1024*1024)
        .worker_threads(1)
        .max_blocking_threads(1)
        .build().unwrap();

    rt.block_on(
        async{
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
                            if let Some(text)=message.text(){
                                let answer=OpenAI::send_prompt(&text);
                                bot.send_message(message.chat.id,answer).await.unwrap();
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