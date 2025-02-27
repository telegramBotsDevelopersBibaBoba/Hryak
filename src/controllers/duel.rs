pub mod inline {
    use sqlx::MySqlPool;
    use teloxide::{
        payloads::AnswerInlineQuerySetters,
        prelude::Requester,
        types::{InlineQuery, InlineQueryResult},
        Bot,
    };

    use crate::handlers::articles;

    pub async fn inline_duel(
        bot: Bot,
        q: &InlineQuery,
        pool: &MySqlPool,
        bid: f64,
    ) -> anyhow::Result<()> {
        let duel =
            articles::inline_duel_article(&pool, q.from.id.0, q.from.mention().unwrap(), bid)
                .await?;
        let articles = vec![InlineQueryResult::Article(duel)];

        bot.answer_inline_query(&q.id, articles)
            .cache_time(0)
            .await?;
        Ok(())
    }
}

pub mod callback {
    use std::time::Duration;

    use anyhow::anyhow;
    use sqlx::MySqlPool;
    use teloxide::{
        payloads::AnswerCallbackQuerySetters,
        prelude::{Request, Requester},
        types::CallbackQuery,
        Bot,
    };
    use tokio::time::sleep;

    use crate::{
        config::consts,
        controllers,
        db::{economydb, pigdb},
    };

    pub async fn callbak_start_duel(
        bot: &Bot,
        q: &CallbackQuery,
        data: &[&str],
        part_id: u64,
        pool: &MySqlPool,
    ) -> anyhow::Result<()> {
        if data.is_empty() {
            bot.edit_message_text_inline(
                q.inline_message_id
                    .as_ref()
                    .ok_or(anyhow!("No data in start duel"))?,
                "Ошибка при дуэли. Отмена",
            )
            .send()
            .await?;
            return Ok(());
        }

        let host_id = data[0].trim().parse::<u64>()?;

        if host_id == part_id {
            bot.answer_callback_query(&q.id)
                .text("Нельзя дуэлить себя идиот гадэмн бля")
                .send()
                .await?;
            return Ok(());
        }

        if !pigdb::pig_exists(pool, part_id).await {
            bot.answer_callback_query(&q.id)
                .text("У тебя нет свиньи! Подуэлиться не получиться.\nИспользуй бота, чтобы она создалась автоматически")
                .send()
                .await?;

            return Ok(());
        }

        let bid = data[2]
            .trim()
            .parse::<f64>()
            .unwrap_or(consts::DUEL_DEFAULT_BID);
        let part_balance = economydb::get_balance(pool, part_id).await?;
        if part_balance < bid {
            bot.answer_callback_query(&q.id)
                .text("Недостаточно денег!")
                .send()
                .await?;
            return Ok(());
        }

        bot.edit_message_text_inline(
            q.inline_message_id.as_ref().unwrap(),
            "Готовимся к дуэли...",
        )
        .send()
        .await?;

        sleep(Duration::from_millis(400)).await;

        bot.edit_message_text_inline(
            q.inline_message_id.as_ref().unwrap(),
            "Рассчитываем шансы...",
        )
        .send()
        .await?;

        let pig_first = pigdb::get_pig_by_user_id(pool, host_id).await?;
        let pig_second = pigdb::get_pig_by_user_id(pool, part_id).await?;

        sleep(Duration::from_secs(1)).await;

        bot.edit_message_text_inline(
            q.inline_message_id.as_ref().ok_or(anyhow!("Error"))?,
            "Битва началась...",
        )
        .send()
        .await?;

        if pig_first.duel(&pig_second) {
            controllers::pig::proccess_duel_results(pool, host_id, part_id, bid).await?;

            let msg = format!("Победителем оказался: {}", data[1]);
            bot.edit_message_text_inline(
                q.inline_message_id.as_ref().ok_or(anyhow!("Error"))?,
                msg,
            )
            .send()
            .await?;

            bot.answer_callback_query(&q.id)
                .text("Вы проиграли!")
                .send()
                .await?;
        } else {
            controllers::pig::proccess_duel_results(pool, part_id, host_id, bid).await?;

            let msg = format!("Победителем оказался: {}", q.from.mention().unwrap());
            bot.edit_message_text_inline(
                q.inline_message_id.as_ref().ok_or(anyhow!("Error"))?,
                msg,
            )
            .send()
            .await?;
            bot.answer_callback_query(&q.id)
                .text("Вы выиграли!")
                .send()
                .await?;
        }

        Ok(())
    }
}
