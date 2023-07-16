use serde_json::Value;
use teloxide_core::types::{
    InlineKeyboardButton,
    InlineKeyboardButtonKind,
    InlineKeyboardMarkup,
    KeyboardButton,
    KeyboardMarkup
};



pub fn build_keyboard(path:&str)->KeyboardMarkup{
    // Загрузка данных
    let rows:Vec<Value>=serde_json::from_slice(
        &std::fs::read(path).unwrap()
    ).unwrap();

    let mut keyboard=Vec::new();
    for row in rows{
        let row=row.as_array().unwrap();

        let mut buttons=Vec::with_capacity(row.len());

        for button in row{
            let text=button["text"].as_str().unwrap().to_string();

            let button=KeyboardButton::new(text);

            buttons.push(button)
        }

        keyboard.push(buttons)
    }

    KeyboardMarkup::new(keyboard)
}



pub fn build_inline_keyboard(path:&str)->InlineKeyboardMarkup{
    // Загрузка данных
    let rows:Vec<Value>=serde_json::from_slice(
        &std::fs::read(path).unwrap()
    ).unwrap();

    let mut keyboard=Vec::new();
    for row in rows{
        let row=row.as_array().unwrap();

        let mut buttons=Vec::with_capacity(row.len());

        for button in row{
            let text=button["text"].as_str().unwrap().to_string();
            let data=button["data"].as_str().unwrap().to_string();

            let button=InlineKeyboardButton::new(
                text,
                InlineKeyboardButtonKind::CallbackData(data)
            );

            buttons.push(button)
        }

        keyboard.push(buttons)
    }

    InlineKeyboardMarkup{
        inline_keyboard:keyboard
    }
}