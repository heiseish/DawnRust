pub mod request;
use crate::messenger::api::MessengerAPI;
use crate::platform::api::PlatformAPI;
use crate::{error, info};
/// Handle request from *Facebook Messenger* and *Telegram* platform form
/// The parameters are both normalize into `request::Request` type
///
/// Example
///
/// ```
/// let req = request::Request::new();
/// handle_request(req);
/// ```
pub fn handle_request(req: request::Request) {
    info!("{:?}", req);
    match req.platform {
        "telegram" => {
            if let Some(api) = req.telegram_api {
                _handle_request(req, api);
            } else {
                error!("No valid telegram API found");
            }
        }
        "messenger" => {
            let api_handler = MessengerAPI {};
            _handle_request(req, &api_handler);
        }
        _ => {
            error!("Cannot find a suitable platform for {}", req.platform);
        }
    }
}

fn _handle_request<T>(req: request::Request, api: &T)
where
    T: PlatformAPI,
{
    api.typing_on(&req.sender_id);
    api.typing_off(&req.sender_id);
}
