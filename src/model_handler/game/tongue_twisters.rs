use crate::synthesize_voice_button;

use teloxide_core::types::{
    InlineKeyboardMarkup,
    InlineKeyboardButton,
    InlineKeyboardButtonKind
};



pub fn tongue_twisters_start_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(None);

    let next_button=InlineKeyboardButton::new(
        "Скороговорка",
        InlineKeyboardButtonKind::CallbackData("tonguetwisterscontinue".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,voice]])
}

pub fn tongue_twisters_continue_keyboard()->InlineKeyboardMarkup{
    let voice=synthesize_voice_button(Some("Озвучить скороговорку"));

    let next_button=InlineKeyboardButton::new(
        "Скороговорка",
        InlineKeyboardButtonKind::CallbackData("tonguetwisterscontinue".into())
    );

    InlineKeyboardMarkup::new(vec![vec![next_button,voice]])
}