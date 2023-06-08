use crate::{
    model::{
        GameHandler,
        ChatHandler,
        PlaygroundHandler,
        HomeworkHandler,
        TutorHandler
    },
    chat::{
        Chat,
        ChatState,
        ChatMode,
        StudyMode
    }
};

use std::sync::Arc;

use tokio::sync::Mutex;

use teloxide_core::{
    Bot,
    requests::Requester,
    types::{
        ReplyMarkup,
        InlineKeyboardMarkup,
        InlineKeyboardButton,
        InlineKeyboardButtonKind
    }
};



const GREETING:&'static str="\
Приветствуем Вас в чат-боте GPTkids!😌

Это не просто бот, а полноценная площадка для детского развития, получения новых знаний в игровой форме и общения с нейросетью.

GPTKids разработан с компонентом ChatGPT (нейросеть от OpenAI) специально для детей 6-12 лет и их родителей. Хотя полезен он будет всем вне зависимости от возраста!😉

Уже сейчас GPTkids может быть репетитором по следующим направлениям:
 • английский язык;
 • математика;
 • литература;
 • и многое-многое другое!

Важно, что к разработке GPTkids мы привлекли высококвалифицированных детских психологов, чтобы Вы не боясь могли использовать чат-бота для развития Вашего ребенка!😍

Более того, мы постарались сделать GPTkids полезным в случаях, когда у ребенка что-то не получается в учебе или в общении со сверстниками. Он может подбодрить, что-то посоветовать и в целом положительно повлиять на настроение😎

Мы уверены, что GPTKids станет верным другом и помощником для вашего ребенка!

С уважением, команда 8bit NFT Studio!🤓\
";



const THE_CHOSEN_MODE_IS_THE_CURRENT:&'static str="Выбранный режим является текущим";



pub struct CommandHandler{
    bot:Bot,
    chat:Arc<Mutex<Chat>>
}

impl CommandHandler{
    pub fn new(bot:Bot,chat:Arc<Mutex<Chat>>)->CommandHandler{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle(self,command:&str){
        let mut command_split=command.split_whitespace();

        let Some(command)=command_split.next()else{return};

        let mut chat=self.chat.lock().await;

        if command=="ping"{
            self.bot.send_message(chat.id,"pong").await.unwrap();
            return
        }

        match &mut chat.state{
            // Команды начального состояния
            ChatState::Initial=>{
                match command{
                    "start"=>{
                        if let Some(args)=command_split.next(){
                            println!("Args: {args}");
                        }
                        else{
                            println!("No args")
                        }

                        let mut send_message=self.bot.send_message(chat.id,GREETING);

                        let start=InlineKeyboardButton::new(
                            "Старт",
                            InlineKeyboardButtonKind::CallbackData("start".into())
                        );

                        send_message.reply_markup=Some(
                            ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup{inline_keyboard:vec![vec![start]]})
                        );

                        send_message.await.unwrap();
                    }

                    // Сообщение при для других команд
                    _=>{
                        self.bot.send_message(chat.id,"У тебя пока нет доступа к этому режиму").await.unwrap();
                    }
                }
            }

            // Команды частичного функционала
            ChatState::PartiallyFunctional=>{
                match command{
                    // Перейти в режим 'Простое общение'
                    "chat"=>{
                        self.bot.send_message(chat.id,THE_CHOSEN_MODE_IS_THE_CURRENT).await.unwrap();
                    }

                    // Перейти в раздел 'Настройки'
                    "settings"=>{
                        self.bot.send_message(chat.id,"Данный раздел пока недоступен").await.unwrap();
                    }

                    // Перейти в раздел 'Профиль'
                    "profile"=>{
                        self.bot.send_message(chat.id,"Данный раздел пока недоступен").await.unwrap();
                    }

                    // Сообщение для других команд
                    _=>{
                        self.bot.send_message(chat.id,"У тебя пока нет доступа к этому режиму").await.unwrap();
                    }
                }
            }

            // Команды полного функционала
            ChatState::FullyFunctional(mode)=>{
                match command{
                    // Перейти в режим 'Простое общение'
                    "chat"=>{
                        if let ChatMode::Chat=mode{
                            self.bot.send_message(chat.id,THE_CHOSEN_MODE_IS_THE_CURRENT).await.unwrap();
                        }
                        else{
                            chat.set_mode(ChatMode::Chat);

                            ChatHandler::new(self.bot,&mut chat).handle_command(command).await;
                        }
                    }

                    // Перейти в режим 'Учёба'
                    "study"=>{
                        chat.set_mode(ChatMode::Study(StudyMode::None));

                        let mut send_message=self.bot.send_message(chat.id,"Выбери режим");
                        send_message.reply_markup=Some(ReplyMarkup::InlineKeyboard(study_buttons()));
                        send_message.await.unwrap();
                    }

                    // Перейти в режим 'Игры'
                    "game"=>{
                        if let ChatMode::Game=mode{
                            GameHandler::new(self.bot,&mut chat).handle_command("gamerepeat").await
                        }
                        else{
                            chat.set_mode(ChatMode::Game);

                            GameHandler::new(self.bot,&mut chat).handle_command(command).await
                        }
                    }

                    // Перейти в режим 'Свободный режим'
                    "playground"=>{
                        if let ChatMode::Playground=mode{
                            self.bot.send_message(chat.id,THE_CHOSEN_MODE_IS_THE_CURRENT).await.unwrap();
                        }
                        else{
                            chat.set_mode(ChatMode::Playground);

                            PlaygroundHandler::new(self.bot,&mut chat).handle_command(command).await
                        }
                    }

                    // Перейти в раздел 'Настройки'
                    "settings"=>{
                        self.bot.send_message(chat.id,"Данный раздел пока недоступен").await.unwrap();
                    }

                    // Перейти в раздел 'Профиль'
                    "profile"=>{
                        self.bot.send_message(chat.id,"Данный раздел пока недоступен").await.unwrap();
                    }

                    // Остальные команды ("reset")
                    _=>match mode{
                        ChatMode::Chat=>ChatHandler::new(self.bot,&mut chat).handle_command(command).await,

                        ChatMode::Study(mode)=>match mode{
                            StudyMode::None=>{}

                            StudyMode::Tutor=>TutorHandler::new(self.bot,&mut chat).handle_command(command).await,

                            StudyMode::Homework=>HomeworkHandler::new(self.bot,&mut chat).handle_command(command).await,
                        }

                        ChatMode::Game=>GameHandler::new(self.bot,&mut chat).handle_command(command).await,

                        ChatMode::Playground=>PlaygroundHandler::new(self.bot,&mut chat).handle_command(command).await,
                    }
                }
            }
        }
    }
}



pub fn study_buttons()->InlineKeyboardMarkup{
    let tutor=InlineKeyboardButton::new(
        "Репетитор",
        InlineKeyboardButtonKind::CallbackData("tutor".into())
    );

    let homework=InlineKeyboardButton::new(
        "Помощь с домашним заданием",
        InlineKeyboardButtonKind::CallbackData("homework".into())
    );

    let inline_buttons=vec![
        vec![tutor],
        vec![homework]
    ];

    InlineKeyboardMarkup{
        inline_keyboard:inline_buttons
    }
}