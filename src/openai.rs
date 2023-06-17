use std::time::Duration;

use reqwest::multipart::{Form, Part};
use serde_json::{
    // json,
    Value
};



const API_KEY:&'static str="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";



pub struct Completition{
    pub message:String,
    pub usage:u64
}

impl Completition{
    pub fn new(message:String,usage:u64)->Completition{
        Self{
            message,
            usage
        }
    }
}



pub struct OpenAI;

impl OpenAI{
    // pub fn send_prompt<S:AsRef<str>>(prompt:&S,max_tokens:u16)->Option<String>{
    //     Self::send_prompt_array(&[prompt],max_tokens).pop()
    // }

    // pub fn send_prompt_array<S:AsRef<str>>(prompts:&[S],max_tokens:u16)->Vec<String>{
    //     let url="https://api.openai.com/v1/completions";

    //     let mut payload=format!("\
    //         {{\
    //             \"model\":\"text-davinci-003\",\
    //             \"max_tokens\":{max_tokens},\
    //             \"best_of\":2,\
    //             \"prompt\":[\
    //     ");

    //     for prompt in prompts{
    //         payload.push('\"');
    //         payload.push_str(prompt.as_ref());
    //         payload.push('\"');
    //         payload.push(',');
    //     }

    //     if !prompts.is_empty(){
    //         payload.pop();
    //     }

    //     payload.push_str("]}");

    //     let response=attohttpc::post(url)
    //         .timeout(Duration::from_secs(1200))
    //         .header("Authorization",format!("Bearer {API_KEY}"))
    //         .header("Content-Type","application/json;charset=utf-8")
    //         .body(Text(payload))
    //         .connect_timeout(Duration::from_secs(1200))
    //         .read_timeout(Duration::from_secs(1200))
    //         .send().unwrap();

    //     let text=response.text_utf8().unwrap();

    //     let mut payload:Value=serde_json::from_str(&text).unwrap();

    //     let usage=&payload["usage"];
    //     println!("{usage:?}");

    //     let Some(choices)=payload["choices"].as_array_mut() else {return Vec::new()};

    //     let mut messages=Vec::with_capacity(choices.len());

    //     for choice in choices{
    //         match choice["text"].take(){
    //             Value::String(message)=>{
    //                 messages.push(message)
    //             }
    //             _=>{}
    //         }
    //     }

    //     messages
    // }

    pub async fn send_chat<S:AsRef<str>>(messages:&[S],max_tokens:u16)->Option<Completition>{
        let url="https://api.openai.com/v1/chat/completions";

        let mut payload=format!("\
            {{\
                \"model\":\"gpt-3.5-turbo\",\
                \"max_tokens\":{max_tokens},\
                \"messages\":[\
        ");

        for message in messages{
            payload.push_str(message.as_ref());
            payload.push(',');
        }

        if !messages.is_empty(){
            payload.pop();
        }

        payload.push_str("]}");

        println!("OpenAI request: {payload}");

        let response=reqwest::Client::new()
            .post(url)
            .header("Authorization",format!("Bearer {API_KEY}"))
            .header("Content-Type","application/json;charset=utf-8")
            .body(payload)
            .timeout(Duration::from_secs(1200))
            .send().await.unwrap();

        let response=response.text().await.unwrap();

        println!("OpenAI response: {response}");

        let payload:Value=serde_json::from_str(&response).unwrap();

        let Some(choices)=payload["choices"].as_array() else {return None};

        let message=choices[0]["message"]["content"].as_str().unwrap();
        let usage=payload["usage"]["total_tokens"].as_i64().unwrap() as u64;

        Some(Completition::new(message.to_string(),usage))
    }

    // pub fn upload_file(path:&str,purpose:&str)->String{
    //     let url="https://api.openai.com/v1/files";

    //     let file=std::fs::read(path).unwrap();

    //     let multipart=MultipartBuilder::new()
    //         .with_text("purpose",purpose)
    //         .with_file(MultipartFile::new("file",&file).with_filename("data.jsonl"))
    //         .build().unwrap();

    //     let response=attohttpc::post(url)
    //         .timeout(Duration::from_secs(1200))
    //         .header("Authorization",format!("Bearer {API_KEY}"))
    //         .body(multipart)
    //         .send().unwrap();

    //     let text=response.text_utf8().unwrap();

    //     println!("{}",text);

    //     let payload:Value=serde_json::from_str(&text).unwrap();

    //     payload["id"].as_str().unwrap().to_string()
    // }

    // pub fn create_fine_tune(file_id:&str){
    //     let url="https://api.openai.com/v1/fine-tunes";

    //     let payload=json!({
    //         "training_file":file_id,
    //         "model":"davinci"
    //     });

    //     let response=attohttpc::post(url)
    //         .timeout(Duration::from_secs(1200))
    //         .header("Authorization",format!("Bearer {API_KEY}"))
    //         .json(&payload).unwrap()
    //         .send().unwrap();

    //     let text=response.text_utf8().unwrap();

    //     println!("{}",text);
    // }

    // pub fn get_fine_tunes(){
    //     let url="https://api.openai.com/v1/fine-tunes";

    //     let response=attohttpc::get(url)
    //         .timeout(Duration::from_secs(1200))
    //         .header("Authorization",format!("Bearer {API_KEY}"))
    //         .send().unwrap();

    //     let text=response.text_utf8().unwrap();

    //     println!("{}",text);
    // }

    // pub fn create_embedding(input:&str)->Vec<f32>{
    //     let url="https://api.openai.com/v1/embeddings";

    //     let payload=format!("\
    //         {{\
    //             \"model\":\"text-embedding-ada-002\",\
    //             \"input\":\"{input}\"\
    //         }}\
    //     ");

    //     let response=attohttpc::post(url)
    //         .timeout(Duration::from_secs(1200))
    //         .header("Authorization",format!("Bearer {API_KEY}"))
    //         .header("Content-Type","application/json;charset=utf-8")
    //         .body(Text(payload))
    //         .send().unwrap();

    //     let text=response.text_utf8().unwrap();

    //     let payload:Value=serde_json::from_str(&text).unwrap();

    //     let usage=&payload["usage"];
    //     println!("{usage:?}");

    //     let mut vectors=Vec::new();

    //     for vector in payload["data"].as_array().unwrap()[0]["embedding"].as_array().unwrap(){
    //         vectors.push(vector.as_f64().unwrap() as f32)
    //     }

    //     vectors
    // }

    pub async fn send_voice(voice:Vec<u8>)->String{
        let url="https://api.openai.com/v1/audio/transcriptions";

        let response=reqwest::Client::new()
            .post(url)
            .multipart(
                Form::new()
                .text("model","whisper-1")
                .text("language","ru")
                .part("file",Part::bytes(voice).file_name("voice.mp3").mime_str("audio/mp3").unwrap())
            )
            .header("Authorization",format!("Bearer {API_KEY}"))
            .timeout(Duration::from_secs(1200))
            .send().await.unwrap();

        let text=response.text().await.unwrap();

        println!("OpenAI response: {}",text);

        let payload:Value=serde_json::from_str(&text).unwrap();

        payload["text"].as_str().unwrap().to_string()
    }
}



pub struct OpenAIWrapper;

impl OpenAIWrapper{
    pub async fn send_chat<S:AsRef<str>>(messages:&[S],max_tokens:u16)->Completition{
        loop{
            if let Some(response)=OpenAI::send_chat(&messages,max_tokens).await{
                break response
            }
            println!("Ошибка");
        }
    }
}