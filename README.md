Rust Telegram Bot Framework
=========================
[![License](https://img.shields.io/github/license/telegram-rs/bot-framework.svg)]()
[![Crates.io](https://img.shields.io/crates/v/bot-framework.svg)](https://crates.io/crates/bot-framework)

<table>
  <tbody>
    <tr>
      <td><b>Documentation:</b></td>
      <td><a href="https://docs.rs/bot-framework/">Latest crates.io version</a></td>
    </tr>
  </tbody>
</table>

A library for writing your own [Telegram](https://telegram.org/) bots. More information [here](https://core.telegram.org/bots). Official API [here](https://core.telegram.org/bots/api).

## Example
Here is a simple example of handling user command (see [`example/hello.rs`](https://github.com/telegram-rs/bot-framework/blob/master/examples/hello.rs)):

``` rust
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
```
You can find a bigger examples in the `examples`.

## Usage
This library is available via `crates.io`. In order to use it, just add this to your `Cargo.toml`:

```
bot-framework = "0.0.1"
```

The library allows you to do E2E-testing of your bot easily: just specify `TELEGRAM_API_URL` environment variable to point to your fake Telegram test server.

## Collaboration
Yes please! Every type of contribution is welcome: Create issues, hack some code or make suggestions. Don't know where to start? Good first issues are tagged with [up for grab](https://github.com/telegram-rs/telegram-bot/issues?q=is%3Aissue+is%3Aopen+label%3A%22up+for+grab%22).
