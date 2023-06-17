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



pub struct User{
    pub token_used:u64,
}

impl User{
    pub const fn new()->User{
        Self{
            token_used:0u64
        }
    }
}


pub struct Chat{
    pub id:ChatId,
    pub state:ChatState,
    pub mode:ChatMode,
    pub data:ModeData,
    pub user:User
}

impl Chat{
    pub fn new(id:ChatId)->Chat{
        Self{
            id,
            state:ChatState::Initial,
            mode:ChatMode::Chat,
            data:ModeData::new(),
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