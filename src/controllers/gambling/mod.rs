use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
pub enum GambleCommands {
    #[command(aliases = ["guess"])]
    Guess,
    #[command(aliases = ["race"])]
    Race,
}
type HandlerResult = anyhow::Result<()>;

pub fn should_cancel_dialog(text: &str) -> bool {
    text.to_lowercase() == "отмена"
        || text.to_lowercase() == "cancel"
        || text.to_lowercase() == "отменить"
        || text.to_lowercase() == "стоп"
}

pub mod inline {
    use teloxide::{prelude::Requester, types::InlineQuery, Bot};

    use crate::handlers::articles;

    pub async fn inline_gamble(bot: Bot, q: &InlineQuery) -> anyhow::Result<()> {
        let guess_article = articles::gamble_games_article();
        let articles = vec![guess_article.into()];
        bot.answer_inline_query(&q.id, articles).await?;
        Ok(())
    }
}
// List of games
pub mod guess;
pub mod pigrace;
