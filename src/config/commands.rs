use strum::{Display, EnumString};

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
}

#[derive(Display, EnumString)]
pub enum InlineAdvCommands {
    // Commands with arguments
    #[strum(serialize = "имя", serialize = "name")]
    ChangeName,
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
}
