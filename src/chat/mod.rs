use crate::model::{
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



pub const HISTORY_TOKEN_LIMIT:usize=2000;



#[derive(Clone,Copy)]
pub enum ChatState{
    Initial,
    PartiallyFunctional,
    FullyFunctional(ChatMode)
}



#[derive(Clone,Copy)]
pub enum ChatMode{
    Chat,
    Study(StudyMode),
    Game,
    Playground
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

            ChatMode::Playground=>self.playground=ManuallyDrop::new(PlaygroundModel::new())
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

                ChatMode::Playground=>ManuallyDrop::drop(&mut self.playground)
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


pub struct Chat{
    pub id:ChatId,
    pub state:ChatState,
    pub data:ModeData
}

impl Chat{
    pub fn new(id:ChatId)->Chat{
        Self{
            id,
            state:ChatState::Initial,
            data:ModeData::new()
        }
    }

    pub fn set_mode(&mut self,mode:ChatMode){
        if let ChatState::FullyFunctional(current_mode)=&mut self.state{
            self.data.clear(*current_mode);
            *current_mode=mode;
            self.data.set(*current_mode);
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

    pub fn get(&mut self,chat_id:ChatId)->Arc<Mutex<Chat>>{
        if self.chats.get(&chat_id).is_none(){
            self.chats.insert(chat_id,Arc::new(Mutex::new(Chat::new(chat_id))));
        }
        self.chats.get(&chat_id).unwrap().clone()
    }
}