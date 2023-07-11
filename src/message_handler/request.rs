use crate::telegram::api::TelegramAPI;

#[derive(Debug)]
pub enum RequestType {
    Text(String),
    Image(String, String),
    File(String),
    Video(String, String),
    Url(String),
    Location(String),
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Request<'a> {
    pub data: RequestType,
    pub sender_id: String,
    pub platform: &'static str,

    #[derivative(Debug = "ignore")]
    pub telegram_api: Option<&'a TelegramAPI>,
}
impl<'a> Request<'a> {
    pub fn new() -> Request<'a> {
        Request {
            data: RequestType::Text(String::new()),
            sender_id: String::new(),
            platform: "telegram",
            telegram_api: None,
        }
    }
}
