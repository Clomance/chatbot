use crate::{    
    synthesize_voice_button,
    keyboard::build_inline_keyboard,
    resources::Resources,
    extended_bot_interface::ExtendedBotInterface,
    chat::{
        Chat,
        History
    },
};

mod riddle;
use riddle::{
    RiddleGame,
    RiddleGameData
};

mod interecting_facts;
use interecting_facts::InterestingFacts;

mod guessing;
use guessing::{
    GuessingGame,
    GuessingGameData
};

mod tongue_twisters;
use tongue_twisters::TongueTwisters;

mod fairy_tales;
use fairy_tales::{
    FairyTales,
    FairyTalesData
};

mod quiz;
use quiz::QuizGame;



pub enum GameMode{
    None,

    RiddleGame(RiddleGameData),

    InterestingFacts,

    QuizGame,

    GuessingGame(GuessingGameData),

    TongueTwisters,

    FairyTales(FairyTalesData)
}



pub struct GameModel{
    history:History,
    mode:GameMode
}

impl GameModel{
    pub fn new()->GameModel{
        Self{
            history:History::new(),
            mode:GameMode::None
        }
    }
}



pub struct GameHandler<'c>{
    bot:ExtendedBotInterface,
    chat:&'c mut Chat
}

impl<'c> GameHandler<'c>{
    pub fn new(bot:ExtendedBotInterface,chat:&'c mut Chat)->GameHandler<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(self,command:&str){
        match command{
            "game"=>{
                self.chat.data.game().mode=GameMode::None;

                let greeting=Resources::load_text("resources/text/game/greeting.txt").await;

                self.bot.send_message(self.chat.id,greeting).reply_markup(build_inline_keyboard("resources/keyboard/game.js")).await.unwrap();
            }

            "gamerepeat"=>{
                self.chat.data.game().mode=GameMode::None;

                let menu=Resources::load_text("resources/text/game/menu.txt").await;

                self.bot.send_message(self.chat.id,menu).reply_markup(build_inline_keyboard("resources/keyboard/game.js")).await.unwrap();
            }

            "reset"=>{
                self.chat.data.game().history.clear();

                self.bot.send_message(self.chat.id,"История очищена").reply_markup(synthesize_voice_button(None)).await.unwrap();
            }

            _=>match &mut self.chat.data.game().mode{
                GameMode::GuessingGame(data)=>GuessingGame::new(self.bot,self.chat.id,&mut self.chat.user,data).handle_command(command).await,

                _=>{}
            }
        }
    }

    pub async fn handle_command_callback(self,callback:&str){
        let data=self.chat.data.game();

        data.history.clear();

        match &mut data.mode{
            GameMode::None=>match callback{
                // При выборе режима "Загадки"
                "riddlegamestart"=>{
                    let mut riddle_game_data=RiddleGameData::new();

                    RiddleGame::new(self.bot,self.chat.id,&mut self.chat.user,&mut riddle_game_data).handle_command_callback(callback).await;

                    data.mode=GameMode::RiddleGame(riddle_game_data);
                }

                // При выборе режима "Интересные факты"
                "interestingfactsstart"=>{
                    InterestingFacts::new(self.bot,self.chat.id,&mut self.chat.user).handle_command_callback(callback).await;

                    data.mode=GameMode::InterestingFacts;
                }

                // При выборе режима "Викторина"
                "quizgamestart"=>{
                    QuizGame::new(self.bot,self.chat.id,&mut self.chat.user).handle_command_callback(callback).await;

                    data.mode=GameMode::None;
                }

                // При выборе режима "Угадайка"
                "guessinggamestart"=>{
                    let mut guessing_game_data=GuessingGameData::new();

                    GuessingGame::new(self.bot,self.chat.id,&mut self.chat.user,&mut guessing_game_data).handle_command_callback(callback).await;

                    data.mode=GameMode::GuessingGame(guessing_game_data);
                }

                // При выборе режима "Скороговорки"
                "tonguetwistersstart"=>{
                    TongueTwisters::new(self.bot,self.chat.id,&mut self.chat.user).handle_command_callback(callback).await;

                    data.mode=GameMode::TongueTwisters;
                }

                // При выборе режима "Сказки"
                "fairytalesstart"=>{
                    let mut fairy_tales_data=FairyTalesData::new();

                    FairyTales::new(self.bot,self.chat.id,&mut self.chat.user,&mut fairy_tales_data).handle_command_callback(callback).await;

                    data.mode=GameMode::FairyTales(fairy_tales_data);
                }

                _=>{}
            }

            GameMode::RiddleGame(data)=>RiddleGame::new(self.bot,self.chat.id,&mut self.chat.user,data).handle_command_callback(callback).await,

            GameMode::InterestingFacts=>InterestingFacts::new(self.bot,self.chat.id,&mut self.chat.user).handle_command_callback(callback).await,

            GameMode::QuizGame=>QuizGame::new(self.bot,self.chat.id,&mut self.chat.user).handle_command_callback(callback).await,

            GameMode::GuessingGame(data)=>GuessingGame::new(self.bot,self.chat.id,&mut self.chat.user,data).handle_command_callback(callback).await,

            GameMode::TongueTwisters=>TongueTwisters::new(self.bot,self.chat.id,&mut self.chat.user).handle_command_callback(callback).await,

            GameMode::FairyTales(data)=>FairyTales::new(self.bot,self.chat.id,&mut self.chat.user,data).handle_command_callback(callback).await,
        }
    }

    pub async fn handle_message(self,message:String){
        match &mut self.chat.data.game().mode{
            GameMode::None=>{}

            GameMode::RiddleGame(data)=>RiddleGame::new(self.bot,self.chat.id,&mut self.chat.user,data).handle_message(message).await,

            GameMode::InterestingFacts=>InterestingFacts::new(self.bot,self.chat.id,&mut self.chat.user).handle_message(message).await,

            GameMode::QuizGame=>QuizGame::new(self.bot,self.chat.id,&mut self.chat.user).handle_message(message).await,

            GameMode::GuessingGame(data)=>GuessingGame::new(self.bot,self.chat.id,&mut self.chat.user,data).handle_message(message).await,

            GameMode::TongueTwisters=>TongueTwisters::new(self.bot,self.chat.id,&mut self.chat.user).handle_message(message).await,

            GameMode::FairyTales(data)=>FairyTales::new(self.bot,self.chat.id,&mut self.chat.user,data).handle_message(message).await,
        }
    }
}