use crate::{
    prompt::Prompt,
    openai::OpenAIWrapper,
    chat::{
        Chat,
        History
    }
};

use serde_json::Value;

use teloxide_core::{
    Bot,
    requests::Requester,
    types::{
        InlineKeyboardMarkup,
        InlineKeyboardButton,
        InlineKeyboardButtonKind,
        ReplyMarkup
    }
};



const GREETING:&'static str="\
Давай поиграем! Игры - это отличный способ развеяться, отвлечься от рутины и как это говорят сегодня - \"перезагрузиться\"). \
Научно доказано, что игры помогают нам проще воспринимать и познавать окружающий мир, а знания, полученные в игровой форме, лучше усваиваются и с легкостью применяются в жизни. \
На \"умном\" это звучит \"геймификация\" - gamification.\
";



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

    GuessingGame{
        guess:String
    },

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

    pub fn guessing(guess:&str)->GameMode{
        GameMode::GuessingGame{guess:String::from(guess)}
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
    bot:Bot,
    chat:&'c mut Chat,
}

impl<'c> GameHandler<'c>{
    pub fn new(bot:Bot,chat:&'c mut Chat)->GameHandler<'c>{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle_command(&mut self,command:&str){
        match command{
            "game"=>{
                self.chat.data.game().mode=GameMode::None;

                let mut send_message=self.bot.send_message(self.chat.id,GREETING);
                send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(game_buttons()));
                send_message.await.unwrap();
            }

            "gamerepeat"=>{
                self.chat.data.game().mode=GameMode::None;

                let mut send_message=self.bot.send_message(self.chat.id,"Игровое меню");
                send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(game_buttons()));
                send_message.await.unwrap();
            }

            "reset"=>{
                self.chat.data.game().history.clear();

                self.bot.send_message(self.chat.id,"История очищена").await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_command_callback(&mut self,callback:&str){
        let data=self.chat.data.game();

        data.history.clear();

        match &mut data.mode{
            GameMode::None=>match callback{
                // При выборе режима загадок
                "riddlegame"=>{
                    data.mode=GameMode::riddle("","");

                    let mut send_message=self.bot.send_message(self.chat.id,"Отлично! Загадки очень хороший способ поиграть с пользой.");
                    send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(new_riddle_keyboard()));
                    send_message.await.unwrap();
                }

                // При выборе режима интересных фактов
                "insterestingfacts"=>{
                    data.mode=GameMode::InsterestingFacts;

                    let mut send_message=self.bot.send_message(self.chat.id,"Ты выбрал интересные факты. Нажми ниже кнопку, чтобы начать");
                    send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(new_fact_keyboard()));
                    send_message.await.unwrap();
                }

                // При выборе режима викторины
                "quizgame"=>{
                    data.mode=GameMode::quiz("");

                    self.bot.send_message(self.chat.id,"Данный раздел пока не доступен").await.unwrap();

                    // self.bot.send_message(self.chat.id,"Ты выбрал викторину. Давай начнём. Напиши что-нибудь").await.unwrap();
                }

                "guessinggame"=>{
                    data.mode=GameMode::guessing("");

                    self.bot.send_message(self.chat.id,"Данный раздел пока не доступен").await.unwrap();

                    // self.bot.send_message(self.chat.id,"Ты выбрал угадайку. Давай начнём. Напиши что-нибудь").await.unwrap();
                }

                "tonguetwisters"=>{
                    data.mode=GameMode::TongueTwisters;

                    let mut send_message=self.bot.send_message(self.chat.id,"Ты выбрал скороговорки. Давай начнём. Нажми кнопку");
                    send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(new_tongue_twisters()));
                    send_message.await.unwrap();
                }

                "fairytales"=>{
                    data.mode=GameMode::fairy_tales("",true);

                    let mut send_message=self.bot.send_message(self.chat.id,"Ты выбрал сказки. Давай начнём. Нажми кнопку");
                    send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(fairy_tales()));
                    send_message.await.unwrap();
                }

                _=>{}
            }

            GameMode::RiddleGame{..}=>{
                match callback{
                    // При нажатии на кнопку "Новая загадка"
                    "riddlegamenew"=>{
                        let prompt=Prompt::load("prompts/game/riddle/riddle.json").await;

                        let response=OpenAIWrapper::send_chat(&[prompt.text],1024);

                        let response:Value=serde_json::from_str(&response).unwrap();

                        let riddle=response["riddle"].as_str().unwrap();
                        let answer=response["answer"].as_str().unwrap();

                        data.mode=GameMode::riddle(riddle,answer);

                        self.bot.send_message(self.chat.id,riddle).await.unwrap();
                    }

                    _=>{}
                }
            }

            GameMode::InsterestingFacts=>{
                match callback{
                    // При нажатии на кнопку "Интересный факт"
                    "insterestingfactsnext"=>{
                        let prompt=Prompt::load("prompts/game/insterestingfacts.json").await;

                        let response=OpenAIWrapper::send_chat(&[prompt.text],1024);

                        let mut send_message=self.bot.send_message(self.chat.id,response);
                        send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(new_fact_keyboard()));
                        send_message.await.unwrap();
                    }

                    _=>{}
                }
            }

            GameMode::QuizGame{quiz}=>{}

            GameMode::GuessingGame{guess}=>{}

            GameMode::TongueTwisters=>{
                match callback{
                    "tonguetwistersnext"=>{
                        let prompt=Prompt::load("prompts/game/tonguetwisters.json").await;

                        let response=OpenAIWrapper::send_chat(&[prompt.text],1024);

                        let mut send_message=self.bot.send_message(self.chat.id,response);
                        send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(new_tongue_twisters()));
                        send_message.await.unwrap();
                    }

                    _=>{}
                }
            }

            GameMode::FairyTales{theme,random}=>{
                match callback{
                    "fairytalesnext"=>{
                        if *random{
                            let prompt=Prompt::load("prompts/game/fairytales.json").await;

                            let response=OpenAIWrapper::send_chat(&[prompt.text],1024);

                            let mut send_message=self.bot.send_message(self.chat.id,response);
                            send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(fairy_tales()));
                            send_message.await.unwrap();
                        }
                        else if theme.is_empty(){
                            self.bot.send_message(self.chat.id,"Напиши тему").await.unwrap();
                        }
                        else{
                            let mut prompt=Prompt::load("prompts/game/themedfairytales.json").await;
                            prompt.replace("{theme}",&theme);

                            let response=OpenAIWrapper::send_chat(&[prompt.text],1024);

                            let mut send_message=self.bot.send_message(self.chat.id,response);
                            send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(themed_fairy_tales()));
                            send_message.await.unwrap();
                        }
                    }

                    "fairytalesnewtheme"=>{
                        data.mode=GameMode::fairy_tales("",false);

                        self.bot.send_message(self.chat.id,"Напиши тему").await.unwrap();
                    }

                    "fairytalesresettheme"=>{
                        data.mode=GameMode::fairy_tales("",true);

                        let mut send_message=self.bot.send_message(self.chat.id,"Тема сброшена. Нажми кнопку, чтобы получить сказку");
                        send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(fairy_tales()));
                        send_message.await.unwrap();
                    }

                    _=>{}
                }
            }
        }
    }

    pub async fn handle_message(&mut self,message:&str){
        let mut messages=Vec::new();

        let data=self.chat.data.game();

        match &mut data.mode{
            GameMode::None=>{}

            GameMode::RiddleGame{ref mut riddle,ref mut answer}=>{
                if riddle.is_empty(){
                    self.bot.send_message(self.chat.id,"Нажми на кнопку, чтобы начать играть").await.unwrap();
                }
                else{
                    let mut prompt=Prompt::load("prompts/game/riddle/guess.json").await;

                    prompt.replace("{riddle}",&riddle);
                    prompt.replace("{answer}",&answer);
                    prompt.replace("{guess}",&message);

                    messages.push(prompt.text);

                    let response=OpenAIWrapper::send_chat(&messages,1024);

                    let response:Value=serde_json::from_str(&response).unwrap();

                    let result=response["result"].to_string().to_lowercase();
                    let message=response["message"].as_str().unwrap();

                    println!("[{:?}]",result);

                    if result=="true"{
                        data.mode=GameMode::riddle("","");

                        let mut send_message=self.bot.send_message(self.chat.id,message);
                        send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(new_riddle_keyboard()));
                        send_message.await.unwrap();
                    }
                    else{
                        self.bot.send_message(self.chat.id,message).await.unwrap();
                    }
                }
            }

            GameMode::InsterestingFacts=>{
                self.bot.send_message(self.chat.id,"Нажми на кнопку, чтобы узнать интересный факт").await.unwrap();
            }

            GameMode::QuizGame{quiz}=>{
                self.bot.send_message(self.chat.id,"Данный раздел пока не доступен").await.unwrap();
            }

            GameMode::GuessingGame{guess}=>{
                self.bot.send_message(self.chat.id,"Данный раздел пока не доступен").await.unwrap();
            }

            GameMode::TongueTwisters=>{
                self.bot.send_message(self.chat.id,"Нажми на кнопку, чтобы получить скороговорку").await.unwrap();
            }

            GameMode::FairyTales{theme,random}=>{
                if *random{
                    self.bot.send_message(self.chat.id,"Нажми на кнопку, чтобы получить сказку").await.unwrap();
                }
                else if theme.is_empty(){
                    *theme=message.to_string();

                    let mut send_message=self.bot.send_message(self.chat.id,"Тема принята");
                    send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(themed_fairy_tales()));
                    send_message.await.unwrap();
                }
                else{
                    self.bot.send_message(self.chat.id,"Нажми на кнопку, чтобы получить сказку").await.unwrap();
                }
            }
        }
    }
}



pub fn game_buttons()->InlineKeyboardMarkup{
    let riddle_game=InlineKeyboardButton::new(
        "Загадки",
        InlineKeyboardButtonKind::CallbackData("riddlegame".into())
    );

    let insteresting_facts=InlineKeyboardButton::new(
        "Интересные факты",
        InlineKeyboardButtonKind::CallbackData("insterestingfacts".into())
    );

    let quiz_game=InlineKeyboardButton::new(
        "Викторина",
        InlineKeyboardButtonKind::CallbackData("quizgame".into())
    );

    let guessing_game=InlineKeyboardButton::new(
        "Угадайка",
        InlineKeyboardButtonKind::CallbackData("guessinggame".into())
    );

    let tongue_twisters=InlineKeyboardButton::new(
        "Cкороговорки",
        InlineKeyboardButtonKind::CallbackData("tonguetwisters".into())
    );

    let fairy_tales=InlineKeyboardButton::new(
        "Cказки",
        InlineKeyboardButtonKind::CallbackData("fairytales".into())
    );

    let inline_buttons=vec![
        vec![riddle_game],
        vec![insteresting_facts],
        vec![quiz_game],
        vec![guessing_game],
        vec![tongue_twisters],
        vec![fairy_tales]
    ];

    InlineKeyboardMarkup{
        inline_keyboard:inline_buttons
    }
}



pub fn new_riddle_keyboard()->InlineKeyboardMarkup{
    let next_button=InlineKeyboardButton::new(
        "Новая загадка",
        InlineKeyboardButtonKind::CallbackData("riddlegamenew".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button]])
}


pub fn new_fact_keyboard()->InlineKeyboardMarkup{
    let next_button=InlineKeyboardButton::new(
        "Интересный факт",
        InlineKeyboardButtonKind::CallbackData("insterestingfactsnext".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button]])
}


pub fn new_tongue_twisters()->InlineKeyboardMarkup{
    let next_button=InlineKeyboardButton::new(
        "Скороговорка",
        InlineKeyboardButtonKind::CallbackData("tonguetwistersnext".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button]])
}


pub fn fairy_tales()->InlineKeyboardMarkup{
    let next_button=InlineKeyboardButton::new(
        "Сказка",
        InlineKeyboardButtonKind::CallbackData("fairytalesnext".into())
    );

    let set_button=InlineKeyboardButton::new(
        "Задать тему",
        InlineKeyboardButtonKind::CallbackData("fairytalesnewtheme".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,set_button]])
}


pub fn themed_fairy_tales()->InlineKeyboardMarkup{
    let next_button=InlineKeyboardButton::new(
        "Сказка",
        InlineKeyboardButtonKind::CallbackData("fairytalesnext".into())
    );

    let reset_button=InlineKeyboardButton::new(
        "Сбросить тему",
        InlineKeyboardButtonKind::CallbackData("fairytalesresettheme".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,reset_button]])
}