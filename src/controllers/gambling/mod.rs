use teloxide::{macros::BotCommands, utils::command};

#[derive(BotCommands, Clone)]
pub enum GambleCommands {
    #[command(aliases = ["guess"])]
    Guess,
    #[command(aliases = ["race"])]
    Race,
}
type HandlerResult = anyhow::Result<()>;

pub mod inline {
    use teloxide::{
        payloads::AnswerInlineQuerySetters, prelude::Requester, types::InlineQuery, Bot,
    };

    use crate::handlers::articles;

    pub async fn inline_gamble(bot: Bot, q: &InlineQuery) -> anyhow::Result<()> {
        let guess_article = articles::gamble_games_article();
        let articles = vec![guess_article.into()];
        bot.answer_inline_query(&q.id, articles)
            .cache_time(0)
            .await?;
        Ok(())
    }
}
// List of games
pub mod guess;
pub mod pigrace;
