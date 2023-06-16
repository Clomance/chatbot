use crate::synthesize_voice_button;

use teloxide_core::types::{
    InlineKeyboardMarkup,
    InlineKeyboardButton,
    InlineKeyboardButtonKind
};



pub fn fairy_tales_start_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(Some("Озвучить"));

    let next_button=InlineKeyboardButton::new(
        "Сказка",
        InlineKeyboardButtonKind::CallbackData("fairytalescontinue".into())
    );

    let set_button=InlineKeyboardButton::new(
        "Задать тему",
        InlineKeyboardButtonKind::CallbackData("fairytalesnewtheme".into())
    );

    InlineKeyboardMarkup::new(vec![vec![voice],vec![next_button,set_button]])
}


pub fn fairy_tales_continue_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(Some("Озвучить сказку"));

    let next_button=InlineKeyboardButton::new(
        "Сказка",
        InlineKeyboardButtonKind::CallbackData("fairytalescontinue".into())
    );

    let set_button=InlineKeyboardButton::new(
        "Задать тему",
        InlineKeyboardButtonKind::CallbackData("fairytalesnewtheme".into())
    );

    InlineKeyboardMarkup::new(vec![vec![voice],vec![next_button,set_button]])
}


pub fn themed_fairy_tales_continue_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(Some("Озвучить сказку"));

    let next_button=InlineKeyboardButton::new(
        "Сказка",
        InlineKeyboardButtonKind::CallbackData("fairytalescontinue".into())
    );

    let reset_button=InlineKeyboardButton::new(
        "Сбросить тему",
        InlineKeyboardButtonKind::CallbackData("fairytalesresettheme".into())
    );

    InlineKeyboardMarkup::new(vec![vec![voice],vec![next_button,reset_button]])
}