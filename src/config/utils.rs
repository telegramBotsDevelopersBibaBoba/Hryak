use teloxide::{
    dispatching::dialogue::GetChatId,
    payloads::SendMessageSetters,
    prelude::{Requester, ResponseResult},
    types::Message,
    Bot,
};

#[macro_export]
macro_rules! ser_command { // Command serializer
    ($($x:expr),*) => {{
        let mut args = String::new();
        $(
            args.push_str($x);
            args.push(' ');
        )*
        args.trim_end().to_string() // Trim trailing space
    }};
}

#[macro_export]
macro_rules! deser_command {
    ($s:expr) => {{
        let parts: Vec<&str> = $s.split_whitespace().collect();
        parts
    }};
}

pub async fn send_msg(bot: &Bot, msg: &Message, message_text: &str) -> ResponseResult<()> {
    if msg.is_topic_message {
        bot.send_message(msg.chat_id().unwrap(), message_text)
            .parse_mode(teloxide::types::ParseMode::Html)
            .message_thread_id(msg.thread_id.unwrap())
            .await?;
    } else {
        bot.send_message(msg.chat_id().unwrap(), message_text)
            .parse_mode(teloxide::types::ParseMode::Html)
            .await?;
    }
    Ok(())
}
