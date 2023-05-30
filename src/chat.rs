use std::collections::HashMap;

use teloxide_core::types::ChatId;



const HISTORY_TOKEN_LIMIT:usize=3800;



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
    lengths:Vec<usize>,
    tokens:usize
}

impl History{
    pub fn new()->History{
        Self{
            text:String::new(),
            lengths:Vec::new(),
            tokens:0usize
        }
    }

    pub fn is_empty(&self)->bool{
        self.text.is_empty()
    }

    pub fn text(&self)->&str{
        if self.text.is_empty(){
            "Empty."
        }
        else{
            &self.text
        }
    }

    pub fn push_question(&mut self,question:&str){
        let tokens=gpt_encoder::Encoder::new().encode(question.to_string());

        self.tokens+=tokens.len();
        self.remove_old();

        self.lengths.push(question.len()+4);

        self.text.push_str("M:");
        self.text.push_str(question);
        self.text.push_str("\\n");
    }

    pub fn push_answer(&mut self,answer:&str){
        let tokens=gpt_encoder::Encoder::new().encode(answer.to_string());

        self.tokens+=tokens.len();
        self.remove_old();

        self.lengths.push(answer.len()+4);

        self.text.push_str("Y:");
        self.text.push_str(answer);
        self.text.push_str("\\n");
    }

    pub fn remove_old(&mut self){
        while self.tokens>=HISTORY_TOKEN_LIMIT{
            let len=self.lengths.remove(0);

            let text=self.text[0..len].to_string();
            let tokens=gpt_encoder::Encoder::new().encode(text);

            self.tokens-=tokens.len();

            self.text.replace_range(0..len,"");
        }
    }

    pub fn clear(&mut self){
        self.text.clear()
    }
}



#[derive(Clone,Copy)]
#[repr(usize)]
pub enum ChatMode{
    Chatting,
    Tutor,
    Homework,
    Game,
    Playground,
    Settings,
    ChatModeCount
}



#[derive(Clone,Copy)]
pub enum Language{
    Russian,
    English
}


pub struct Chat{
    flags:ChatFlags,
    mode:ChatMode,
    language:Language,
    history:Vec<History>,
}

impl Chat{
    pub fn new()->Chat{
        unsafe{
            let c=std::mem::transmute::<ChatMode,usize>(ChatMode::ChatModeCount);

            let mut history=Vec::with_capacity(c);

            for _ in 0..c{
                history.push(History::new())
            }

            Self{
                flags:ChatFlags::new(),
                mode:ChatMode::Chatting,
                language:Language::Russian,
                history
            }
        }
    }

    pub fn history(&mut self)->&mut History{
        unsafe{
            &mut self.history[std::mem::transmute::<ChatMode,usize>(self.mode)]
        }
    }

    pub fn mode(&self)->ChatMode{
        self.mode
    }

    pub fn set_mode(&mut self,mode:ChatMode){
        self.mode=mode
    }

    pub fn build_prompt(&mut self,message:&str,mode_changed:bool)->String{
        match self.mode{
            ChatMode::Chatting=>if mode_changed{
               format!(
                    "You are my friend.We're quite close.We're kids.\
                    This is our chat history (M - Me,Y - You):{}\\n\
                    Write something to continue chatting with me or start it.\
                    The main language is russian.Y:",
                    self.history().text()
                )
            }
            else{
                format!(
                    "You are my friend.We're quite close.We're kids.\
                    This is our chat history (M - Me,Y - You):{}\\n\
                    M:{message}\\nY:",
                    self.history().text()
                )
            }

            ChatMode::Tutor=>if mode_changed{
                format!(
                    "You are a tutor for kids.\
                    I am a kid.\
                    You are up to teach me something I want.\
                    This is our chat history (M - Me,Y - You):{}\\n\
                    Write something to continue chatting with me or start it.\
                    The main language is russian.Y:",
                    self.history().text()
                )
            }
            else{
                format!(
                    "You are a tutor for kids.\
                    I am a kid.\
                    You are up to teach me something I want.\
                    This is our chat history (M - Me,Y - You):{}\\n\
                    M:{message}\\nY:",
                    self.history().text()
                )
            }

            ChatMode::Homework=>if mode_changed{
                format!(
                    "You are a tutor for kids.\
                    You help kids with homework.\
                    I am a kid.\
                    This is our chat history (M - Me,Y - You):{}\\n\
                    Write something to continue chatting with me or start it.\
                    The main language is russian.Y:",
                    self.history().text()
                )
            }
            else{
                format!(
                    "You are a tutor for kids.\
                    You help kids with homework.\
                    I am a kid.\
                    This is our chat history (M - Me,Y - You):{}\\n\
                    M:{message}\\nY:",
                    self.history().text()
                )
            }

            ChatMode::Game=>if mode_changed{
                format!(
                    "We gonna play games.\
                    You suggest and play a chat game with me.\
                    I am a kid.\
                    This is our chat history (M - Me,Y - You):{}\\n\
                    Write something to continue chatting with me or start it.\
                    The main language is russian.Y:",
                    self.history().text()
                )
            }
            else{
                format!(
                    "We gonna play games.\
                    You suggest and play a chat game with me.\
                    I am a kid.\
                    This is our chat history (M - Me,Y - You):{}\\n\
                    M:{message}\\nY:",
                    self.history().text()
                )
            }

            ChatMode::Playground=>{
                format!(
                    "Chat history (M - Me,Y - You):{}\\n\
                    M:{message}\\nY:",
                    self.history().text()
                )
            }

            ChatMode::Settings=>unreachable!(),

            _=>unreachable!()
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
            self.chats.insert(chat_id,Chat::new());
        }
        self.chats.get_mut(&chat_id).unwrap()
    }
}