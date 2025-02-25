use strum::{Display, EnumString};
use teloxide::utils::command::{self, BotCommands};
#[derive(Display, EnumString)]
pub enum InlineCommands {
    // Single-word commands
    #[strum(serialize = "hryak", serialize = "хряк")]
    Hryak,
    #[strum(serialize = "shop", serialize = "магазин")]
    Shop,
    #[strum(serialize = "name", serialize = "имя")]
    Name,
    #[strum(serialize = "duel", serialize = "дуэль")]
    Duel,
    #[strum(serialize = "баланс", serialize = "balance")]
    Balance,

    // Gambling
    #[strum(serialize = "gamble", serialize = "азарт")]
    Gamble,
    #[strum(serialize = "guess", serialize = "угадывание")]
    GuessGamble,
}

#[derive(Display, EnumString)]
pub enum InlineAdvCommands {
    // Commands with arguments
    #[strum(serialize = "имя", serialize = "name")]
    ChangeName,
    #[strum(serialize = "duel", serialize = "дуэль")]
    Duel,
}

#[derive(Display, EnumString)]
pub enum CallbackCommands {
    #[strum(serialize = "shop")]
    Shop,
    #[strum(serialize = "duel")]
    StartDuel,
}

#[derive(Display, EnumString)]
pub enum FeedbackCommands {
    #[strum(serialize = "имя", serialize = "name")]
    ChangeName,
    #[strum(serialize = "угадывание", serialize = "guess")]
    GuessGamble,
}

#[derive(BotCommands, Clone)]
pub enum EconomyCommands {
    #[command(aliases = ["income", "daily"], description = "Получить ежедневную денежную помощь от США")]
    DailyIncome,

    #[command(parse_with = "split", aliases = ["pay"], description = "Перевести деньги кому-нибудь")]
    Pay { mention: String, amount: f64 },
}
