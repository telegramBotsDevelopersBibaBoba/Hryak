use strum::{Display, EnumString};

#[derive(Display, EnumString)]
pub enum InlineCommands {
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
pub enum CallbackCommands {
    #[strum(serialize = "shop")]
    Shop,
}

#[derive(Display, EnumString)]
pub enum InlineAdvCommands {
    #[strum(serialize = "имя", serialize = "name")]
    ChangeName,
}

#[derive(Display, EnumString)]
pub enum FeedbackCommands {
    #[strum(serialize = "имя", serialize = "name")]
    ChangeName,
}
