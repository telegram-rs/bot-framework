use std::collections::HashMap;

use futures::stream::Stream;
use telegram_bot::types::{Message, MessageKind, Update, UpdateKind};
use telegram_bot::Api;
use tokio_core::reactor::Core;

pub struct BotWrapper {
    api: Api,
    core: Core,
    commands: HashMap<String, Box<dyn Fn(&Api, &Message) -> ()>>,
}

impl BotWrapper {
    fn handle_update(
        api: &Api,
        commands: &HashMap<String, Box<dyn Fn(&Api, &Message) -> ()>>,
        update: Update,
    ) {
        if let UpdateKind::Message(ref msg) = update.kind {
            if let MessageKind::Text {
                ref data,
                entities: _,
            } = msg.kind
            {
                let data = data.clone().split_off(1);
                let data = data.split_whitespace().next().unwrap_or("");
                commands.get(data).map(|command| command(api, msg));
            }
        }
    }

    pub fn new(token: String) -> BotWrapper {
        let core = Core::new().expect("Failed to execute tokio core");

        let api = Api::configure(token.clone())
            .build(core.handle())
            .expect("Failed to spawn bot threads");
        BotWrapper {
            api,
            core,
            commands: HashMap::new(),
        }
    }

    pub fn run(self) {
        let BotWrapper {
            api,
            mut core,
            commands,
        } = self;
        let update_stream = api
            .stream()
            .for_each(|update| Ok(BotWrapper::handle_update(&api, &commands, update)));

        core.run(update_stream).expect("Failed to run core reactor");
    }

    pub fn command<F>(&mut self, command: String, handle: F)
    where
        F: 'static + Fn(&Api, &Message) -> (),
    {
        self.commands.insert(command, Box::new(handle));
    }
}
