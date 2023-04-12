mod openai;

use openai::OpenAI;

use openssl::ssl::{SslMethod, SslConnector, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use serde_json::Value;
// use serde_json::Value;
use teloxide_core::{
    Bot,
    requests::Requester,
    types::{AllowedUpdate, UpdateKind},
};

use tokio::runtime::Builder;
use tokio_postgres::types::Type;



const BOT_TOKEN:&'static str="6058214276:AAFzDlKUr33C3In7jMM5cz6kUDqh85OzFqE";



fn main(){
    // let file_id=OpenAI::upload_file("data.jsonl","fine-tune");
    // OpenAI::create_fine_tune("file-vcv84KKd2vhIjGBkjUDgJHB8");
    // OpenAI::get_fine_tunes();

    let samples:Vec<Value>=serde_json::from_str(include_str!("../sample.json")).unwrap();

    let mut sample_vectors=Vec::new();

    for sample in &samples{
        let vector=OpenAI::create_embedding(&sample.as_str().unwrap());
        sample_vectors.push(vector)
    }

    let database_url=if let Ok(database_url)=std::env::var("DATABASE_URL"){
        database_url
    }
    else{
        String::new()
    };

    let mut builder=SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let connector=MakeTlsConnector::new(builder.build());

    let bot=Bot::new(BOT_TOKEN);

    let rt=Builder::new_current_thread()
        .enable_all()
        .thread_stack_size(1024*1024)
        .worker_threads(2)
        .max_blocking_threads(2)
        .build().unwrap();

    rt.block_on(
        async{
            let (client,connection)=tokio_postgres::connect(
                &database_url,
                connector,
            ).await.unwrap();

            tokio::spawn(async move {
                if let Err(e)=connection.await{
                    println!("connection error: {}", e);
                }
            });

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
                                if text.starts_with("запрос:")||text.starts_with("Запрос:"){
                                    let text=&text[13..];

                                    let text=text.replace("\"","\'");

                                    let prompt=format!(
                                        "A table name: 'employees'.\
                                        The first field: 'name', type is varchar.\
                                        The second field: 'sex', type is varchar, possible values are 'мужской' (male) 'женский' (female).\
                                        The third field: 'position' (means a job position), type is varchar.\
                                        The forth field: 'age', type integer.\
                                        Given this database table schema, write only a sql query for this question:{text}."
                                    );

                                    let answer=OpenAI::send_prompt(&prompt);

                                    let rows=client.query(&answer,&[]).await.unwrap();

                                    println!("{rows:?}");

                                    let mut data=String::new();
                                    for row in rows{
                                        for column in row.columns(){
                                            let name=column.name();
                                            let column_type=column.type_();
                                            match column_type.clone(){
                                                Type::VARCHAR=>{
                                                    let value:String=row.get(name);
                                                    data+=&value;
                                                    data+=" "
                                                }

                                                Type::INT4=>{
                                                    let value:i32=row.get(name);
                                                    data+=&value.to_string();
                                                    data+=" "
                                                }

                                                Type::INT8=>{
                                                    let value:i64=row.get(name);
                                                    data+=&value.to_string();
                                                    data+=" "
                                                }

                                                _=>{}
                                            }
                                        }
                                        data+="\n"
                                    }

                                    println!("{data}");

                                    if data.trim().is_empty(){
                                        bot.send_message(message.chat.id,"Ничего").await.unwrap();
                                    }
                                    else{
                                        bot.send_message(message.chat.id,data).await.unwrap();
                                    }
                                }
                                else if text.starts_with("Поиск:")||text.starts_with("поиск:"){
                                    let text=&text[11..];
                                    let vector=OpenAI::create_embedding(text);

                                    let mut max_similarity=0f32;
                                    let mut id=0;

                                    for (c,sample) in sample_vectors.iter().enumerate(){
                                        let similarity=cosine_similarity(sample,&vector);

                                        if max_similarity<similarity{
                                            max_similarity=similarity;
                                            id=c
                                        }

                                        println!("{similarity}");
                                    }

                                    if max_similarity>0.70{
                                        let file=samples[id].as_str().unwrap();
                                        let file=file.replace("\"","\'");

                                        let prompt=format!(
                                            "{file}\nAdapt this text for this question:{text}\nYour answer:"
                                        );
    
                                        let answer=OpenAI::send_prompt(&prompt);
                                        bot.send_message(message.chat.id,answer).await.unwrap();
                                    }
                                    else{
                                        bot.send_message(message.chat.id,"Ничего").await.unwrap();
                                    }
                                }
                                else{
                                    // "curie:ft-personal-2023-04-09-12-01-05"
                                    let text=text.replace("\"","\'");

                                    let prompt=format!(
                                        "You are a russian chat assistant of Добрый company.\
                                        This company sells different kinds of juice.\
                                        You answer people questions and tell them about it's products if they ask for them.\n\n\
                                        The first question:{text}\nYour answer:"
                                    );

                                    let answer=OpenAI::send_prompt(&prompt);
                                    bot.send_message(message.chat.id,answer).await.unwrap();
                                }
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

fn cosine_similarity(vector1:&Vec<f32>,vector2:&Vec<f32>)->f32{
    let mut sa=0f32;
    let mut sb=0f32;
    let mut sab=0f32;

    for (&axe1,&axe2) in vector1.iter().zip(vector2.iter()){
        sa+=axe1*axe1;
        sb+=axe2*axe2;

        sab+=axe1*axe2;
    }

    let mut cos=(sab*sab)/(sa*sb);

    if sab.is_sign_negative(){
        cos*=-1f32
    }

    cos
}