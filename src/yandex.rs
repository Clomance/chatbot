use std::time::{
    Duration,
    SystemTime
};

use serde_json::{
    Value,
    json
};

use tokio::sync::RwLock;



const IAM_TOKEN_LIFETIME:Duration=Duration::from_secs(4000);



static mut OAUTH_TOKEN:String=String::new();

static mut FOLDER_ID:String=String::new();

static mut IAM_TOKEN:RwLock<IamToken>=RwLock::const_new(IamToken::new());



pub struct IamToken{
    creation:SystemTime,
    token:String
}

impl IamToken{
    pub const fn new()->IamToken{
        Self{
            creation:SystemTime::UNIX_EPOCH,
            token:String::new()
        }
    }

    pub fn is_old(&self)->bool{
        SystemTime::now().duration_since(self.creation).unwrap()>IAM_TOKEN_LIFETIME
    }

    pub async fn update(&mut self){
        self.creation=SystemTime::now();
        self.token=YandexSpeechKit::iam_token().await
    }
}



pub struct YandexSpeechKit;

impl YandexSpeechKit{
    pub fn set_oauth_token(token:&str){
        unsafe{
            OAUTH_TOKEN=token.to_string()
        }
    }

    pub fn set_folder_id(id:&str){
        unsafe{
            FOLDER_ID=id.to_string()
        }
    }

    pub async fn iam_token()->String{
        let url="https://iam.api.cloud.yandex.net/iam/v1/tokens";

        let object=json!({
            "yandexPassportOauthToken": unsafe{&OAUTH_TOKEN}
        });

        let response=reqwest::Client::new()
            .post(url)
            .json(&object)
            .timeout(Duration::from_secs(1200))
            .send().await.unwrap();

        let response:Value=response.json().await.unwrap();

        println!("{:?}",response);

        response["iamToken"].as_str().unwrap().to_string()
    }

    pub async fn synthesize(text:&str)->Vec<u8>{
        let url=unsafe{format!(
            "https://tts.api.cloud.yandex.net/speech/v1/tts:synthesize?text={}&folderId={FOLDER_ID}",
            text
        )};

        let token=unsafe{
            let mut token=IAM_TOKEN.write().await;

            if token.is_old(){
                token.update().await
            }

            token.token.clone()
        };

        let response=reqwest::Client::new()
            .post(url)
            .header("Authorization",format!("Bearer {token}"))
            .timeout(Duration::from_secs(1200))
            .send().await.unwrap();

        if response.status().is_success(){
            response.bytes().await.unwrap().to_vec()
        }
        else{
            let response:Value=response.json().await.unwrap();

            println!("{:?}",response);

            Vec::new()
        }
    }
}