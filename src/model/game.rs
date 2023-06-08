use crate::{
    openai::OpenAIWrapper,
    chat::{
        Chat,
        History
    }
};

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



pub enum GameMode{
    None,

    RiddleGame{
        riddle:String
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
        theme:String
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

                let mut send_message=self.bot.send_message(self.chat.id,"Выбери то, чем хочешь заняться.");
                send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(game_buttons()));
                send_message.await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_command_callback(&mut self,callback:&str){
        let data=self.chat.data.game();

        match callback{
            "riddlegame"=>{
                // data.mode=GameMode::RiddleGame;
                self.bot.send_message(self.chat.id,"Ты выбрал загадки. Давай начнём. Напиши что-нибудь").await.unwrap();
            }

            "insterestingfacts"=>{
                data.mode=GameMode::InsterestingFacts;

                let next_button=InlineKeyboardButton::new(
                    "Интересный факт",
                    InlineKeyboardButtonKind::CallbackData("insterestingfactsnext".into())
                );

                let mut send_message=self.bot.send_message(self.chat.id,"Ты выбрал интересные факты. Нажми ниже кнопку, чтобы начать");

                send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup::new(vec![vec![next_button]])));

                send_message.await.unwrap();
            }

            "insterestingfactsnext"=>{
                let prompt=std::fs::read_to_string("prompts/game/insterestingfacts.json").unwrap().replace('\n',"");

                let messages=[prompt];

                let response=OpenAIWrapper::send_chat(&messages,1024);

                let next_button=InlineKeyboardButton::new(
                    "Интересный факт",
                    InlineKeyboardButtonKind::CallbackData("insterestingfactsnext".into())
                );

                let mut send_message=self.bot.send_message(self.chat.id,response);

                send_message.reply_markup=Some(
                    ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup::new(vec![vec![next_button]]))
                );

                send_message.await.unwrap();
            }

            "quizgame"=>{
                // chat.set_mode(ChatMode::Game(GameMode::QuizGame));
                self.bot.send_message(self.chat.id,"Ты выбрал викторину. Давай начнём. Напиши что-нибудь").await.unwrap();
            }

            "guessinggame"=>{
                // chat.set_mode(ChatMode::Game(GameMode::GuessingGame));
                self.bot.send_message(self.chat.id,"Ты выбрал угадайку. Давай начнём. Напиши что-нибудь").await.unwrap();
            }

            "tonguetwisters"=>{
                // chat.set_mode(ChatMode::Game(GameMode::TongueTwisters));
                self.bot.send_message(self.chat.id,"Ты выбрал скороговорки. Давай начнём. Напиши что-нибудь").await.unwrap();
            }

            "fairytales"=>{
                // chat.set_mode(ChatMode::Game(GameMode::FairyTales));
                self.bot.send_message(self.chat.id,"Ты выбрал сказки. Давай начнём. Напиши что-нибудь").await.unwrap();
            }

            _=>{}
        }
    }

    pub async fn handle_message(&mut self,message:&str){
        let mut messages=Vec::new();

        let data=self.chat.data.game();
        data.history.clear();

        match data.mode{
            GameMode::None=>{
                return
            }

            GameMode::RiddleGame{ref riddle}=>{
                let prompt=std::fs::read_to_string("prompts/game/game.json").unwrap().replace('\n',"");
                messages.push(prompt)
            }

            GameMode::InsterestingFacts=>{
                // let prompt=std::fs::read_to_string("prompts/game/game.json").unwrap().replace('\n',"");
                // messages.push(prompt)
            }

            GameMode::QuizGame{ref quiz}=>{
                let prompt=std::fs::read_to_string("prompts/game/game.json").unwrap().replace('\n',"");
                messages.push(prompt)
            }

            GameMode::GuessingGame{ref guess}=>{
                let prompt=std::fs::read_to_string("prompts/game/game.json").unwrap().replace('\n',"");
                messages.push(prompt)
            }

            GameMode::TongueTwisters=>{
                let prompt=std::fs::read_to_string("prompts/game/game.json").unwrap().replace('\n',"");
                messages.push(prompt)
            }

            GameMode::FairyTales{ref theme}=>{
                let prompt=std::fs::read_to_string("prompts/game/game.json").unwrap().replace('\n',"");
                messages.push(prompt)
            }
        }

        data.history.push_request(message);

        for message in data.history.messages().iter(){
            messages.push(message.to_string())
        }

        let response=OpenAIWrapper::send_chat(&messages,1024);

        println!("Response: {}",response);

        data.history.push_response(
            &response
            .replace("\"","\\\"")
            .replace("\n","\\n")
            .replace("\r","\\r")
            .replace("\t","\\t")
        );

        self.bot.send_message(self.chat.id,response).await.unwrap();
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