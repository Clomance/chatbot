use std::time::Duration;

use attohttpc::{MultipartBuilder, MultipartFile};

use serde_json::{Value, json};



const API_KEY:&'static str="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";

// "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"


pub struct OpenAI;

impl OpenAI{
    pub fn send_prompt(prompt:&str)->String{
        let url="https://api.openai.com/v1/completions";

        let payload=json!({
            "model":"text-davinci-003",
            "prompt":prompt,
            "max_tokens":1024,
            "best_of":3
        });

        println!("{payload}");

        let response=attohttpc::post(url)
            .timeout(Duration::from_secs(1200))
            .header("Authorization",format!("Bearer {API_KEY}"))
            .json(&payload).unwrap()
            .connect_timeout(Duration::from_secs(1200))
            .read_timeout(Duration::from_secs(1200))
            .send().unwrap();

        let text=response.text_utf8().unwrap();

        println!("{}",text);

        let payload:Value=serde_json::from_str(&text).unwrap();

        let choices=&payload["choices"];

        let message=choices[0]["text"].as_str().unwrap();

        message.to_string()
    }

    pub fn send_message(message:&str)->String{
        let url="https://api.openai.com/v1/chat/completions";

        let payload=json!({
            "model":"gpt-3.5-turbo",
            "messages":[
                {
                    "role":"user",
                    "content":"
                        I want you to act as an assistant of Добрый company.
                        This company sells different kinds of juice.
                        You will consult people with a Добрый juice topic.
                    "
                },
                {"role":"assistant","content":"Certainly!"},
                {"role":"user","content":message}
            ]
        });

        println!("{payload}");

        let response=attohttpc::post(url)
            .timeout(Duration::from_secs(1200))
            .header("Authorization",format!("Bearer {API_KEY}"))
            .json(&payload).unwrap()
            .connect_timeout(Duration::from_secs(1200))
            .read_timeout(Duration::from_secs(1200))
            .send().unwrap();

        let text=response.text_utf8().unwrap();

        println!("{}",text);

        let payload:Value=serde_json::from_str(&text).unwrap();

        let choices=&payload["choices"];

        let message=choices[0]["message"]["content"].as_str().unwrap();

        message.to_string()
    }

    pub fn upload_file(path:&str,purpose:&str)->String{
        let url="https://api.openai.com/v1/files";

        let file=std::fs::read(path).unwrap();

        let multipart=MultipartBuilder::new()
            .with_text("purpose",purpose)
            .with_file(MultipartFile::new("file",&file).with_filename("data.jsonl"))
            .build().unwrap();

        let response=attohttpc::post(url)
            .timeout(Duration::from_secs(1200))
            .header("Authorization",format!("Bearer {API_KEY}"))
            .body(multipart)
            .send().unwrap();

        let text=response.text_utf8().unwrap();

        println!("{}",text);

        let payload:Value=serde_json::from_str(&text).unwrap();

        payload["id"].as_str().unwrap().to_string()
    }

    pub fn create_fine_tune(file_id:&str){
        let url="https://api.openai.com/v1/fine-tunes";

        let payload=json!({
            "training_file":file_id,
            "model":"davinci"
        });

        let response=attohttpc::post(url)
            .timeout(Duration::from_secs(1200))
            .header("Authorization",format!("Bearer {API_KEY}"))
            .json(&payload).unwrap()
            .send().unwrap();

        let text=response.text_utf8().unwrap();

        println!("{}",text);
    }

    pub fn get_fine_tunes(){
        let url="https://api.openai.com/v1/fine-tunes";

        let response=attohttpc::get(url)
            .timeout(Duration::from_secs(1200))
            .header("Authorization",format!("Bearer {API_KEY}"))
            .send().unwrap();

        let text=response.text_utf8().unwrap();

        println!("{}",text);
    }

    pub fn create_embedding(input:&str)->Vec<f32>{
        let url="https://api.openai.com/v1/embeddings";

        let payload=json!({
            "input":input,
            "model":"text-embedding-ada-002"
        });

        let response=attohttpc::post(url)
            .timeout(Duration::from_secs(1200))
            .header("Authorization",format!("Bearer {API_KEY}"))
            .json(&payload).unwrap()
            .send().unwrap();

        let text=response.text_utf8().unwrap();

        let payload:Value=serde_json::from_str(&text).unwrap();

        let mut vectors=Vec::new();

        for vector in payload["data"].as_array().unwrap()[0]["embedding"].as_array().unwrap(){
            vectors.push(vector.as_f64().unwrap() as f32)
        }

        vectors
    }
}