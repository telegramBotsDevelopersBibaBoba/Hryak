use strum::{Display, EnumString};

#[derive(Display, EnumString)]
pub enum InlineCommands {
    #[strum(serialize = "hryak")]
    Hryak,
}
