use std::path::Path;



pub struct Prompt{
    pub text:String
}

impl Prompt{
    pub async fn load<P:AsRef<Path>>(path:P)->Prompt{
        Self{
            text:tokio::fs::read_to_string(path).await.unwrap().replace("\r\n","").replace("\n","")
        }
    }

    pub fn replace(&mut self,from:&str,to:&str){
        self.text=self.text.replace(from,to)
    }
}