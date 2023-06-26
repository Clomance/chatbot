use crate::{
    synthesize_voice_button,
    resources::Resources,
    extended_bot_interface::ExtendedBotInterface,
    prompt::Prompt,
    openai::OpenAIWrapper,
    chat::{
        Chat,
        History
    }
};

mod riddle;
use riddle::{
    RiddleGame
};

mod interecting_facts;
use interecting_facts::{
    interecting_facts_start_keyboard,
    interecting_facts_continue_keyboard
};

mod guessing;
use guessing::{
    GuessingGame,
    GuessingGameData
};

mod tongue_twisters;
use tongue_twisters::{
    tongue_twisters_start_keyboard,
    tongue_twisters_continue_keyboard
};

mod fairy_tales;
use fairy_tales::{
    fairy_tales_start_keyboard,
    fairy_tales_continue_keyboard,
    themed_fairy_tales_start_keyboard,
    themed_fairy_tales_continue_keyboard
};

use teloxide_core::{
    requests::Requester,
    types::{
        InlineKeyboardMarkup,
        InlineKeyboardButton,
        InlineKeyboardButtonKind
    }
};



pub enum GameMode{
    None,

    RiddleGame{
        riddle:String,
        answer:String
    },

    InsterestingFacts,

    QuizGame{
        quiz:String
    },

    GuessingGame(GuessingGameData),

    TongueTwisters,

    FairyTales{
        theme:String,
        random:bool
    }
}

impl GameMode{
    pub fn riddle(riddle:&str,answer:&str)->GameMode{
        GameMode::RiddleGame{
            riddle:String::from(riddle),
            answer:String::from(answer)
        }
    }

    pub fn quiz(quiz:&str)->GameMode{
        GameMode::QuizGame{quiz:String::from(quiz)}
    }

    pub fn guessing(secret:&str)->GameMode{
        GameMode::GuessingGame(GuessingGameData::new(secret))
    }

    pub fn fairy_tales(theme:&str,random:bool)->GameMode{
        GameMode::FairyTales{
            theme:String::from(theme),
            random
        }
    }
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

                self.bot.send_message(self.chat.id,greeting).reply_markup(game_buttons()).await.unwrap();
            }

            "gamerepeat"=>{
                self.chat.data.game().mode=GameMode::None;

                let menu=Resources::load_text("resources/text/game/menu.txt").await;

                self.bot.send_message(self.chat.id,menu).reply_markup(game_buttons()).await.unwrap();
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
                    data.mode=GameMode::riddle("","");

                    RiddleGame::new(self).handle_command_callback(callback).await;
                }

                // При выборе режима "Интересные факты"
                "insterestingfactsstart"=>{
                    data.mode=GameMode::InsterestingFacts;

                    let greeting=Resources::load_text("resources/text/game/insteresting_facts/greeting.txt").await;

                    self.bot.send_message(self.chat.id,greeting).reply_markup(interecting_facts_start_keyboard()).await.unwrap();
                }

                // При выборе режима "Викторина"
                "quizgamestart"=>{
                    data.mode=GameMode::quiz("");

                    self.bot.send_message(self.chat.id,"Данный раздел пока не доступен").await.unwrap();
                }

                // При выборе режима "Угадайка"
                "guessinggamestart"=>{
                    data.mode=GameMode::guessing("");

                    if let GameMode::GuessingGame(data)=&mut data.mode{
                        GuessingGame::new(self.bot,self.chat.id,&mut self.chat.user,data).handle_command_callback(callback).await
                    }
                }

                // При выборе режима "Скороговорки"
                "tonguetwistersstart"=>{
                    data.mode=GameMode::TongueTwisters;

                    self.bot.send_message(self.chat.id,"Ты выбрал скороговорки. Давай начнём. Нажми кнопку").reply_markup(tongue_twisters_start_keyboard()).await.unwrap();
                }

                // При выборе режима "Сказки"
                "fairytalesstart"=>{
                    data.mode=GameMode::fairy_tales("",true);

                    self.bot.send_message(self.chat.id,"Ты выбрал сказки. Давай начнём. Нажми кнопку").reply_markup(fairy_tales_start_keyboard()).await.unwrap();
                }

                _=>{}
            }

            GameMode::RiddleGame{..}=>RiddleGame::new(self).handle_command_callback(callback).await,

            GameMode::InsterestingFacts=>{
                match callback{
                    // При нажатии на кнопку "Интересный факт"
                    "insterestingfactscontinue"=>{
                        let sent_message=self.bot.send_message(self.chat.id,"Пишет интересный факт...").await.unwrap();

                        let prompt=Prompt::load("resources/prompts/game/insterestingfacts.json").await;

                        let response=OpenAIWrapper::send_chat(&[prompt.text],1024).await;

                        self.chat.user.token_usage.total+=response.usage;

                        self.bot.bot.delete_message(self.chat.id,sent_message.id).await.unwrap();
                        self.bot.send_message(self.chat.id,response.message).reply_markup(interecting_facts_continue_keyboard()).await.unwrap();
                    }

                    _=>{}
                }
            }

            GameMode::QuizGame{..}=>{}

            GameMode::GuessingGame(data)=>GuessingGame::new(self.bot,self.chat.id,&mut self.chat.user,data).handle_command_callback(callback).await,

            GameMode::TongueTwisters=>{
                match callback{
                    "tonguetwisterscontinue"=>{
                        let sent_message=self.bot.send_message(self.chat.id,"Пишет скороговорку...").await.unwrap();

                        let prompt=Prompt::load("resources/prompts/game/tonguetwisters.json").await;

                        let response=OpenAIWrapper::send_chat(&[prompt.text],1024).await;

                        self.chat.user.token_usage.total+=response.usage;

                        self.bot.bot.delete_message(self.chat.id,sent_message.id).await.unwrap();
                        self.bot.send_message(self.chat.id,response.message).reply_markup(tongue_twisters_continue_keyboard()).await.unwrap();
                    }

                    _=>{}
                }
            }

            GameMode::FairyTales{theme,random}=>{
                match callback{
                    "fairytalescontinue"=>{
                        if *random{
                            let sent_message=self.bot.send_message(self.chat.id,"Пишет сказку...").await.unwrap();

                            let prompt=Prompt::load("resources/prompts/game/fairytales.json").await;

                            let response=OpenAIWrapper::send_chat(&[prompt.text],1024).await;

                            self.chat.user.token_usage.total+=response.usage;

                            self.bot.bot.delete_message(self.chat.id,sent_message.id).await.unwrap();
                            self.bot.send_message(self.chat.id,response.message).reply_markup(fairy_tales_continue_keyboard()).await.unwrap();
                        }
                        else if theme.is_empty(){
                            self.bot.send_message(self.chat.id,"Напиши тему").reply_markup(synthesize_voice_button(None)).await.unwrap();
                        }
                        else{
                            let sent_message=self.bot.send_message(self.chat.id,"Пишет сказку...").await.unwrap();

                            let mut prompt=Prompt::load("resources/prompts/game/themedfairytales.json").await;
                            prompt.replace("{theme}",&theme);

                            let response=OpenAIWrapper::send_chat(&[prompt.text],1024).await;

                            self.chat.user.token_usage.total+=response.usage;

                            self.bot.bot.delete_message(self.chat.id,sent_message.id).await.unwrap();
                            self.bot.send_message(self.chat.id,response.message).reply_markup(themed_fairy_tales_continue_keyboard()).await.unwrap();
                        }
                    }

                    "fairytalesnewtheme"=>{
                        data.mode=GameMode::fairy_tales("",false);

                        self.bot.send_message(self.chat.id,"Напиши тему").reply_markup(synthesize_voice_button(None)).await.unwrap();
                    }

                    "fairytalesresettheme"=>{
                        data.mode=GameMode::fairy_tales("",true);

                        let theme_reset=Resources::load_text("resources/text/game/fairy_tales/theme_reset.txt").await;

                        self.bot.send_message(self.chat.id,theme_reset).reply_markup(fairy_tales_start_keyboard()).await.unwrap();
                    }

                    _=>{}
                }
            }
        }
    }

    pub async fn handle_message(self,message:String){
        match &mut self.chat.data.game().mode{
            GameMode::None=>{}

            GameMode::RiddleGame{ref mut riddle,ref mut answer}=>{
                let riddle=riddle.clone();
                let answer=answer.clone();
                RiddleGame::new(self).handle_message(riddle,answer,&message).await;
            }

            GameMode::InsterestingFacts=>{
                self.bot.send_message(self.chat.id,"Нажми на кнопку, чтобы узнать интересный факт").reply_markup(synthesize_voice_button(None)).await.unwrap();
            }

            GameMode::QuizGame{..}=>{
                self.bot.send_message(self.chat.id,"Данный раздел пока не доступен").reply_markup(synthesize_voice_button(None)).await.unwrap();
            }

            GameMode::GuessingGame(data)=>GuessingGame::new(self.bot,self.chat.id,&mut self.chat.user,data).handle_message(message).await,

            GameMode::TongueTwisters=>{
                self.bot.send_message(self.chat.id,"Нажми на кнопку, чтобы получить скороговорку").reply_markup(synthesize_voice_button(None)).await.unwrap();
            }

            GameMode::FairyTales{theme,random}=>{
                if *random{
                    self.bot.send_message(self.chat.id,"Нажми на кнопку, чтобы получить сказку").reply_markup(synthesize_voice_button(None)).await.unwrap();
                }
                else if theme.is_empty(){
                    *theme=message.to_string();

                    let theme_set=Resources::load_text("resources/text/game/fairy_tales/theme_set.txt").await;

                    self.bot.send_message(self.chat.id,theme_set).reply_markup(themed_fairy_tales_start_keyboard()).await.unwrap();
                }
                else{
                    self.bot.send_message(self.chat.id,"Нажми на кнопку, чтобы получить сказку").reply_markup(synthesize_voice_button(None)).await.unwrap();
                }
            }
        }
    }
}



pub fn game_buttons()->InlineKeyboardMarkup{
    let riddle_game=InlineKeyboardButton::new(
        "Загадки",
        InlineKeyboardButtonKind::CallbackData("riddlegamestart".into())
    );

    let insteresting_facts=InlineKeyboardButton::new(
        "Интересные факты",
        InlineKeyboardButtonKind::CallbackData("insterestingfactsstart".into())
    );

    let quiz_game=InlineKeyboardButton::new(
        "Викторина",
        InlineKeyboardButtonKind::CallbackData("quizgamestart".into())
    );

    let guessing_game=InlineKeyboardButton::new(
        "Угадайка",
        InlineKeyboardButtonKind::CallbackData("guessinggamestart".into())
    );

    let tongue_twisters=InlineKeyboardButton::new(
        "Cкороговорки",
        InlineKeyboardButtonKind::CallbackData("tonguetwistersstart".into())
    );

    let fairy_tales=InlineKeyboardButton::new(
        "Cказки",
        InlineKeyboardButtonKind::CallbackData("fairytalesstart".into())
    );

    let inline_buttons=vec![
        vec![synthesize_voice_button(None)],
        vec![riddle_game,insteresting_facts],
        vec![quiz_game,guessing_game],
        vec![tongue_twisters,fairy_tales],
    ];

    InlineKeyboardMarkup{
        inline_keyboard:inline_buttons
    }
}