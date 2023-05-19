use std::collections::HashMap;

use teloxide_core::types::ChatId;



const HISTORY_TOKEN_LIMIT:usize=1024;



#[derive(Clone,Copy,Debug)]
#[repr(u64)]
pub enum ChatFlag{
    Subscibed
}



pub struct ChatFlags{
    inner:u64
}

impl ChatFlags{
    pub const fn new()->ChatFlags{
        Self{
            inner:0u64
        }
    }

    pub fn set_raw(&mut self,inner:u64){
        self.inner=inner
    }

    pub const fn inner(&self)->u64{
        self.inner
    }

    pub fn set(&mut self,flag:ChatFlag){
        self.inner|=flag as u64
    }

    pub fn remove(&mut self,flag:ChatFlag){
        self.inner&=!(flag as u64)
    }

    pub const fn check(&self,flag:ChatFlag)->bool{
        (self.inner&(flag as u64))==flag as u64
    }
}



pub struct History{
    text:String,
    tokens:usize
}

impl History{
    pub fn new()->History{
        Self{
            text:String::new(),
            tokens:0usize
        }
    }

    pub fn push_question(&mut self,question:&str){
        self.text.push_str("U:");
        self.text.push_str(question);
        self.text.push_str("\\n");
    }

    pub fn push_answer(&mut self,answer:&str){
        self.text.push_str("Y:");
        self.text.push_str(answer);
        self.text.push_str("\\n");
    }
}



pub struct Chat{
    flags:ChatFlags,
    pub history:History
}

impl Chat{
    pub fn new()->Chat{
        Self{
            flags:ChatFlags::new(),
            history:History::new()
        }
    }

    pub fn history(&self)->&str{
        &self.history.text
    }
}



pub struct Chats{
    chats:HashMap<ChatId,Chat>
}

impl Chats{
    pub fn new()->Chats{
        Self{
            chats:HashMap::new()
        }
    }

    pub fn get(&mut self,chat_id:ChatId)->&mut Chat{
        if self.chats.get_mut(&chat_id).is_none(){
            self.chats.insert(chat_id,Chat::new());
        }
        self.chats.get_mut(&chat_id).unwrap()
    }
}