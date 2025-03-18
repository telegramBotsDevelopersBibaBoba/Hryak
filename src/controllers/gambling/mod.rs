use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
pub enum GambleCommands {
    #[command(aliases = ["guess"])]
    Guess,
    #[command(aliases = ["race"])]
    Race,
    #[command(aliases = ["treasurehunt"])]
    TreasureHunt,
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

    use super::{guess, pigrace};

    pub async fn inline_gamble(bot: Bot, q: &InlineQuery) -> anyhow::Result<()> {
        let guess_game_msg = format!("Смысл игры в том, чтобы угадать загаданное ботом число от 0 до 100. Победа дает <ставка> * {}$\n\nЧтобы начать игру введите /guess. После ввода нужно будет ввести ставку и число, ответив на сообщение бота (На любом этапе игры можно ответить 'отмена', остановив таким образом игру).", guess::GUESS_BID_MULTIPLIER);
        let guess_game_article = articles::make_article("guess-game", "Угадывание числа", &guess_game_msg, "Попробуйте угадать число", "https://i.fbcd.co/products/resized/resized-360-240/c-1000-designbundle-filthy-rich-pig-fat-2-11-11-a82d6109eb213397774b6f90288b5f94b677f16e03e1f0737ab5e87dee9cc164.jpg".into());

        let pigrace_msg = format!("Смысл игры в том, чтобы поставить деньги на какую-то из свиней. Если выбранная вами свинья пройдет наибольшее расстояние, то вы получите денежный приз - <ставка> * {}$\n\nЧтобы начать игру введите /race. После ввода нужно будет ввести ставку и айди свиньи, ответив на сообщение бота, после чего победитель определится в реальном времени (На любом этапе игры можно ответить 'отмена', остановив таким образом игру).", pigrace::RACE_BID_MULTIPLIER);
        let pigrace_article = articles::make_article("pigrace-info", "Гонка свиней", &pigrace_msg, "Поставьте деньги на самую спортивную из свиней!", "https://as1.ftcdn.net/jpg/05/72/12/08/1000_F_572120864_Nzjwk0uvWrh7NeGbokIh6St3n0qtHLRr.jpg".into());

        let treasurehunt_msg = format!("Смысл игры в том, чтобы отправить свою свиньи в какую-то из локаций, в которой она может найти призы.\nЧтобы начать игру введите /treasurehunt. После ввода нужно будет ввести ставку и айди локации, ответив на сообщение бота, после чего свинья отправится в путешествие, из которого, конечно, вернется, но, возможно, с пустыми копытами (На любом этапе игры можно ответить 'отмена', остановив таким образом игру).");
        let treasurehunt_article = articles::make_article("treasurehunt-info", "Охота за сокровищами", &treasurehunt_msg, "Отправьте свинью на поиск сокровищ", "https://images.squarespace-cdn.com/content/v1/532aa86ae4b025b2a07ff10f/1562152965768-NCQTTDRO5GV5QBQZ95H6/swimming+pigs+bahamas+abaco+piggyville".into());

        let articles = vec![
            guess_game_article.into(),
            pigrace_article.into(),
            treasurehunt_article.into(),
        ];
        bot.answer_inline_query(&q.id, articles).await?;
        Ok(())
    }
}
// List of games
pub mod guess;
pub mod pigrace;
pub mod treasurehunt;
