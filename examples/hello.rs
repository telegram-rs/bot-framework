use bot_framework::BotWrapper;
use std::env;
use telegram_bot::prelude::*;

fn main() {
    let token = env::var("TELEGRAM_BOT_KEY").expect("TELEGRAM_BOT_KEY not found in env");
    let mut bot = BotWrapper::new(token);
    bot.command("hello".into(), |api, msg| {
        api.spawn(msg.text_reply(format!("Hello, {}!", &msg.from.first_name)));
    });

    bot.run();
}
