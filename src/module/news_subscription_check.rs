use crate::extended_bot_interface::ExtendedBotInterface;

use teloxide_core::{
    requests::Requester,
    types::{
        UserId,
        ChatMemberKind
    }
};



pub struct NewsSubscriptionCheck;

impl NewsSubscriptionCheck{
    pub async fn check(bot:&ExtendedBotInterface,user_id:UserId)->bool{
        let maybe_member=bot.bot.get_chat_member("@gptkids".to_string(),user_id).await;
        if let Ok(member)=maybe_member{
            if let ChatMemberKind::Left=member.kind{
                return false
            }
            if let ChatMemberKind::Banned(_)=member.kind{
                return false
            }
        }
        else{
            return false
        }

        true
    }
}