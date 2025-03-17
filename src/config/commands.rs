use strum::{Display, EnumString};
use teloxide::utils::command::BotCommands;
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
    #[strum(serialize = "азарт", serialize = "gamble")]
    Gamble,
    #[strum(serialize = "inventory", serialize = "inv", serialize = "инвентарь")]
    Inventory,
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
    DuelStart,
    #[strum(serialize = "action")]
    DuelAction,
    #[strum(serialize = "buff")]
    BuffUse,
    #[strum(serialize = "dpage")]
    SwitchPageDuel,
}

#[derive(Display, EnumString)]
pub enum FeedbackCommands {
    #[strum(serialize = "имя", serialize = "name")]
    ChangeName,
}

#[derive(BotCommands, Clone)]
pub enum EconomyCommands {
    #[command(aliases = ["income", "daily"], description = "Получить ежедневную денежную помощь от США")]
    DailyIncome,

    #[command(parse_with = "split", aliases = ["pay"], description = "Перевести деньги кому-нибудь")]
    Pay { mention: String, amount: f64 },
}
