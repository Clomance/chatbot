use crate::model_handler::{
    GameModel,
    TutorModel,
    ChatModel,
    PlaygroundModel,
    HomeworkModel
};

mod history;
pub use history::History;

use std::{
    sync::Arc,
    collections::HashMap,
    mem::ManuallyDrop
};

use tokio::sync::Mutex;

use teloxide_core::types::ChatId;



pub const HISTORY_TOKEN_LIMIT:usize=2500;



#[derive(Clone,Copy)]
pub enum ChatState{
    Initial,
    PartiallyFunctional,
    FullyFunctional
}



#[derive(Clone,Copy)]
pub enum ChatMode{
    Chat,
    Study(StudyMode),
    Game,
    Playground,
    Profile
}



#[derive(Clone,Copy)]
pub enum StudyMode{
    None,
    Tutor,
    Homework
}


pub union ModeData{
    chat:ManuallyDrop<ChatModel>,
    homework:ManuallyDrop<HomeworkModel>,
    tutor:ManuallyDrop<TutorModel>,
    game:ManuallyDrop<GameModel>,
    playground:ManuallyDrop<PlaygroundModel>
}

impl ModeData{
    pub fn new()->ModeData{
        Self{
            chat:ManuallyDrop::new(ChatModel::new())
        }
    }

    fn set(&mut self,mode:ChatMode){
        match mode{
            ChatMode::Chat=>self.chat=ManuallyDrop::new(ChatModel::new()),

            ChatMode::Study(mode)=>match mode{
                StudyMode::None=>{}

                StudyMode::Tutor=>self.tutor=ManuallyDrop::new(TutorModel::new()),

                StudyMode::Homework=>self.homework=ManuallyDrop::new(HomeworkModel::new()),
            }

            ChatMode::Game=>self.game=ManuallyDrop::new(GameModel::new()),

            ChatMode::Playground=>self.playground=ManuallyDrop::new(PlaygroundModel::new()),

            ChatMode::Profile=>{}
        }
    }

    fn clear(&mut self,mode:ChatMode){
        unsafe{
            match mode{
                ChatMode::Chat=>self.chat=ManuallyDrop::new(ChatModel::new()),

                ChatMode::Study(mode)=>match mode{
                    StudyMode::None=>{}

                    StudyMode::Tutor=>ManuallyDrop::drop(&mut self.tutor),

                    StudyMode::Homework=>ManuallyDrop::drop(&mut self.homework),
                }

                ChatMode::Game=>ManuallyDrop::drop(&mut self.game),

                ChatMode::Playground=>ManuallyDrop::drop(&mut self.playground),

                ChatMode::Profile=>{}
            }
        }
    }

    pub fn chat(&mut self)->&mut ChatModel{
        unsafe{
            &mut self.chat
        }
    }

    pub fn game(&mut self)->&mut GameModel{
        unsafe{
            &mut self.game
        }
    }

    pub fn homework(&mut self)->&mut HomeworkModel{
        unsafe{
            &mut self.homework
        }
    }

    pub fn tutor(&mut self)->&mut TutorModel{
        unsafe{
            &mut self.tutor
        }
    }

    pub fn playground(&mut self)->&mut PlaygroundModel{
        unsafe{
            &mut self.playground
        }
    }
}



#[derive(Clone,Copy,Debug)]
#[repr(u64)]
pub enum ChatFlag{
    Admin=1<<0,
    Invited=1<<1
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

    pub fn set(&mut self,flag:ChatFlag){
        self.inner|=flag as u64
    }

    pub fn remove(&mut self,flag:ChatFlag){
        self.inner&=!(flag as u64)
    }

    pub fn is_set(&self,flag:ChatFlag)->bool{
        (self.inner&(flag as u64))==flag as u64
    }
}



pub struct TokenUsage{
    pub total:u32
}

impl TokenUsage{
    pub const fn new()->TokenUsage{
        Self{
            total:0u32
        }
    }
}



pub struct User{
    pub token_usage:TokenUsage,
    pub people_invited:u32
}

impl User{
    pub const fn new()->User{
        Self{
            token_usage:TokenUsage::new(),
            people_invited:0u32
        }
    }
}


pub struct Chat{
    pub id:ChatId,
    pub state:ChatState,
    pub mode:ChatMode,
    pub data:ModeData,
    pub flags:ChatFlags,
    pub user:User
}

impl Chat{
    pub fn new(id:ChatId)->Chat{
        Self{
            id,
            state:ChatState::Initial,
            mode:ChatMode::Chat,
            data:ModeData::new(),
            flags:ChatFlags::new(),
            user:User::new()
        }
    }

    pub fn set_mode(&mut self,mode:ChatMode){
        if let ChatState::FullyFunctional=&mut self.state{
            self.data.clear(self.mode);
            self.mode=mode;
            self.data.set(self.mode);
        }
    }

    pub fn set_admin(&mut self,set:bool){
        if set{
            self.flags.set(ChatFlag::Admin)
        }
        else{
            self.flags.remove(ChatFlag::Admin)
        }
    }

    pub fn set_invited(&mut self,set:bool){
        if set{
            self.flags.set(ChatFlag::Invited)
        }
        else{
            self.flags.remove(ChatFlag::Invited)
        }
    }

    pub fn is_admin(&self)->bool{
        self.flags.is_set(ChatFlag::Admin)
    }

    pub fn was_invited(&self)->bool{
        self.flags.is_set(ChatFlag::Invited)
    }
}



pub struct Chats{
    chats:HashMap<ChatId,Arc<Mutex<Chat>>>
}

impl Chats{
    pub fn new()->Chats{
        Self{
            chats:HashMap::new()
        }
    }

    pub fn push(&mut self,chat_id:ChatId){
        self.chats.insert(chat_id,Arc::new(Mutex::new(Chat::new(chat_id))));
    }

    pub fn get(&self,chat_id:ChatId)->Option<Arc<Mutex<Chat>>>{
        if let Some(chat)=self.chats.get(&chat_id){
            Some(chat.clone())
        }
        else{
            None
        }
    }
}