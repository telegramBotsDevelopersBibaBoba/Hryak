use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
pub enum GambleComamnds {
    #[command(aliases = ["guess"])]
    Guess,
}
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub mod guess;
