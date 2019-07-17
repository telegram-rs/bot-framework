use std::env;
use bot_framework::{BotWrapper, BotHandler};
use telegram_bot::prelude::*;
use telegram_bot::types::*;
use telegram_bot::Api;

struct MyBot;

impl BotHandler for MyBot {
    fn inline_query(&self, api: &Api, query: InlineQuery) {
        let input_text_message_content = InputTextMessageContent {
            message_text: query.query.clone(),
            parse_mode: Some(telegram_bot::ParseMode::Markdown),
            disable_web_page_preview: true,
        };

        let mut article = InlineQueryResultArticle::new(
            format!("{}", query.from.id),
            format!("Hello, User!"),
            input_text_message_content,
        );
        article.description(format!("This is an inline query result article"));

        let mut ans = query.answer(vec![]);
        ans.add_inline_result(article);
        api.spawn(ans);
    }
}

fn main() {
    let token = env::var("TELEGRAM_BOT_KEY").expect("TELEGRAM_BOT_KEY not found in env");
    let bot = BotWrapper::new_with_handler(token, MyBot);
    bot.run();
}
