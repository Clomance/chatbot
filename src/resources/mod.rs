mod keyboard;
pub use keyboard::*;

mod prompt;
pub use prompt::*;

use std::path::Path;



pub struct Resources;

impl Resources{
    pub async fn load_text<P:AsRef<Path>>(path:P)->String{
        tokio::fs::read_to_string(path).await.unwrap()
    }
}