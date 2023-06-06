use std::collections::HashMap;

use teloxide_core::types::ChatId;



const HISTORY_TOKEN_LIMIT:usize=2000;



// #[derive(Clone,Copy,Debug)]
// #[repr(u64)]
// pub enum ChatFlag{
//     Subscibed
// }



// pub struct ChatFlags{
//     inner:u64
// }

// impl ChatFlags{
//     pub const fn new()->ChatFlags{
//         Self{
//             inner:0u64
//         }
//     }

//     pub fn set_raw(&mut self,inner:u64){
//         self.inner=inner
//     }

//     pub const fn inner(&self)->u64{
//         self.inner
//     }

//     pub fn set(&mut self,flag:ChatFlag){
//         self.inner|=flag as u64
//     }

//     pub fn remove(&mut self,flag:ChatFlag){
//         self.inner&=!(flag as u64)
//     }

//     pub const fn check(&self,flag:ChatFlag)->bool{
//         (self.inner&(flag as u64))==flag as u64
//     }
// }



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

    pub fn is_empty(&self)->bool{
        self.messages.is_empty()
    }

    pub fn messages(&self)->&Vec<String>{
        &self.messages
    }

    pub fn push_question(&mut self,question:&str){
        let message=format!("{{\"role\":\"user\",\"content\":\"{}\"}}",question);

        let tokens=gpt_encoder::Encoder::new().encode(message.clone());

        self.tokens+=tokens.len();
        self.remove_old();

        self.messages.push(message);
    }

    pub fn push_answer(&mut self,answer:&str){
        let message=format!("{{\"role\":\"assistant\",\"content\":\"{}\"}}",answer);

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


#[derive(Debug,Clone,Copy)]
pub enum ChatAction{
    Greet,
    AskName,
    AskSex,
    AskAge,
    Chat,
}



#[derive(Debug,Clone,Copy)]
#[repr(usize)]
pub enum ChatMode{
    Chatting,
    Tutor(TutorMode),
    Game,
    Playground,
    ChatModeCount
}

#[derive(Debug,Clone,Copy)]
#[repr(usize)]
pub enum TutorMode{
    None,
    Math,
    English,
    Finance,
    Astronomy,
    Enviroment,
    SelfCare
}



pub struct User{
    pub name:String,
    pub sex:bool,
    pub age:u8
}

impl User{
    pub fn new()->User{
        Self{
            name:String::new(),
            sex:false,
            age:0u8
        }
    }
}


pub struct Chat{
    pub id:ChatId,
    // flags:ChatFlags,
    pub mode:ChatMode,
    history:Vec<History>,
    pub user:User,
}

impl Chat{
    pub fn new(id:ChatId)->Chat{
        unsafe{
            let c:[usize;2]=std::mem::transmute(ChatMode::ChatModeCount);

            let mut history=Vec::with_capacity(c[0]);

            for _ in 0..c[0]{
                history.push(History::new())
            }

            Self{
                id,
                // flags:ChatFlags::new(),
                mode:ChatMode::Chatting,
                history,
                user:User::new()
            }
        }
    }

    pub fn history(&mut self)->&mut History{
        unsafe{
            let c:[usize;2]=std::mem::transmute(self.mode);
            &mut self.history[c[0]]
        }
    }

    pub fn set_mode(&mut self,mode:ChatMode){
        self.mode=mode
    }

    pub fn clear_all_history(&mut self){
        for history in &mut self.history{
            history.clear()
        }
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
            self.chats.insert(chat_id,Chat::new(chat_id));
        }
        self.chats.get_mut(&chat_id).unwrap()
    }
}