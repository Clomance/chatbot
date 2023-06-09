use super::HISTORY_TOKEN_LIMIT;



pub struct History{
    messages:Vec<String>,
    tokens:usize
}

impl History{
    pub fn new()->History{
        Self{
            messages:Vec::new(),
            tokens:0usize
        }
    }

    pub fn messages(&self)->&Vec<String>{
        &self.messages
    }

    pub fn push_request(&mut self,request:&str){
        let request=request
            .replace("\\","\\\\")
            .replace("\"","\\\"")
            .replace("\\\"","\"")
            .replace("\"","\\\"")
            .replace("\r\n","\\n")
            .replace("\n","\\n")
            .replace("\r","\\r")
            .replace("\t","\\t");

        let message=format!("{{\"role\":\"user\",\"content\":\"{}\"}}",request);

        let tokens=gpt_encoder::Encoder::new().encode(message.clone());

        self.tokens+=tokens.len();
        self.remove_old();

        self.messages.push(message);
    }

    pub fn push_response(&mut self,response:&str){
        let response=response
            .replace("\\","\\\\")
            .replace("\"","\\\"")
            .replace("\\\"","\"")
            .replace("\"","\\\"")
            .replace("\r\n","\\n")
            .replace("\n","\\n")
            .replace("\r","\\r")
            .replace("\t","\\t");

        let message=format!("{{\"role\":\"assistant\",\"content\":\"{}\"}}",response);

        let tokens=gpt_encoder::Encoder::new().encode(message.clone());

        self.tokens+=tokens.len();
        self.remove_old();

        self.messages.push(message);
    }

    pub fn remove_old(&mut self){
        while self.tokens>=HISTORY_TOKEN_LIMIT{
            let message=self.messages.remove(0);

            let tokens=gpt_encoder::Encoder::new().encode(message);

            self.tokens-=tokens.len();
        }
    }

    pub fn clear(&mut self){
        self.messages.clear()
    }
}