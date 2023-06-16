use crate::synthesize_voice_button;

use teloxide_core::types::{
    InlineKeyboardMarkup,
    InlineKeyboardButton,
    InlineKeyboardButtonKind
};



pub fn interecting_facts_start_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(None);

    let next_button=InlineKeyboardButton::new(
        "Интересный факт",
        InlineKeyboardButtonKind::CallbackData("insterestingfactscontinue".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,voice]])
}


pub fn interecting_facts_continue_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(Some("Озвучить факт"));

    let next_button=InlineKeyboardButton::new(
        "Интересный факт",
        InlineKeyboardButtonKind::CallbackData("insterestingfactscontinue".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,voice]])
}