use crate::message_handler::{handle_request, request};
use crate::platform::api::PlatformAPI;
use futures::StreamExt;
use crate::info;
use std::io::Error;
use telegram_bot::*;
pub struct TelegramAPI {
    api: Api,
}

impl TelegramAPI {
    pub fn new() -> TelegramAPI {
        let token = dotenv!("TELEGRAM_TOKEN");
        return TelegramAPI {
            api: Api::new(token),
        };
    }
    pub async fn start_listening(&mut self) -> Result<(), Error> {
        info!("Loading telegram endpoint");
        let mut stream = self.api.stream();
        while let Some(update) = stream.next().await {
            if let Ok(update_msg) = update {
                if let UpdateKind::Message(message) = update_msg.kind {
                    if let MessageKind::Text { ref data, .. } = message.kind {
                        // Print received text message to stdout.
                        let mut msg_request = request::Request::new();
                        msg_request.sender_id = message.from.first_name.clone();
                        msg_request.platform = "telegram";
                        msg_request.data = request::RequestType::Text(data.clone());
                        msg_request.telegram_api = Some(self);
                        handle_request(msg_request).await?;
                    }
                }
            }
        }
        Ok(());
    }
}
impl PlatformAPI for TelegramAPI {
    fn typing_on(&self, sender_id: &String) {}
    fn typing_off(&self, sender_id: &String) {}
}
