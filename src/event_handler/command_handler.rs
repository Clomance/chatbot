use crate::{
    extended_bot_interface::ExtendedBotInterface,
    chat::{
        Chat,
        ChatState
    },
    state_handler::{
        InitialState,
        PartitialState,
        FullState
    }
};

use std::sync::Arc;

use tokio::sync::Mutex;



pub struct CommandHandler{
    bot:ExtendedBotInterface,
    chat:Arc<Mutex<Chat>>
}

impl CommandHandler{
    pub fn new(bot:ExtendedBotInterface,chat:Arc<Mutex<Chat>>)->CommandHandler{
        Self{
            bot,
            chat
        }
    }

    pub async fn handle(self,command:&str){
        let mut chat=self.chat.lock().await;

        match &mut chat.state{
            // Команды начального состояния
            ChatState::Initial=>InitialState::new(self.bot,&mut chat).handle_command(command).await,

            // Команды частичного функционала
            ChatState::PartiallyFunctional=>PartitialState::new(self.bot,&mut chat).handle_command(command).await,

            // Команды полного функционала
            ChatState::FullyFunctional=>FullState::new(self.bot,&mut chat).handle_command(command).await
        }
    }
}