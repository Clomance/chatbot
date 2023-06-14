use std::time::{
    Duration,
    SystemTime
};

use serde_json::{
    Value,
    json
};

use tokio::sync::RwLock;



const OAUTH_TOKEN:&'static str="y0_AgAAAABr_dOTAATuwQAAAADlZWMyvt6pwDoQSQyD_ZGqevdufCINKd0";

const FOLDER_ID:&'static str="b1galt7gva67moa8daor";

const IAM_TOKEN_LIFETIME:Duration=Duration::from_secs(4000);



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

    pub fn update(&mut self){
        self.token=YandexSpeechKit::token()
    }
}


pub struct YandexSpeechKit;

impl YandexSpeechKit{
    pub fn token()->String{
        let url="https://iam.api.cloud.yandex.net/iam/v1/tokens";

        let object=json!({
            "yandexPassportOauthToken": OAUTH_TOKEN
        });

        let response=attohttpc::post(url)
            .timeout(Duration::from_secs(1200))
            .json(&object).unwrap()
            .send().unwrap();

        let response:Value=response.json_utf8().unwrap();

        println!("{:?}",response);

        response["iamToken"].as_str().unwrap().to_string()
    }

    pub async fn synthesize(text:&str)->Vec<u8>{
        let url=format!(
            "https://tts.api.cloud.yandex.net/speech/v1/tts:synthesize?text={}&folderId={FOLDER_ID}",
            text
        );

        let token=unsafe{
            let mut token=IAM_TOKEN.write().await;

            if token.is_old(){
                token.update()
            }

            token.token.clone()
        };

        let response=attohttpc::post(url)
            .timeout(Duration::from_secs(1200))
            .header("Authorization",format!("Bearer {token}"))
            .send().unwrap();

        if response.is_success(){
            response.bytes().unwrap()
        }
        else{
            let response:Value=response.json_utf8().unwrap();

            println!("{:?}",response);

            Vec::new()
        }
    }
}