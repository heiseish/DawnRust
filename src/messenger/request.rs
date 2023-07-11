#[derive(Serialize, Deserialize)]
pub struct MessengerSender {
    pub id: String,
}
#[derive(Serialize, Deserialize)]
pub struct MessengerPayload {
    pub payload: String,
}
impl Default for MessengerPayload {
    fn default() -> MessengerPayload {
        MessengerPayload {
            payload: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MessengerLocation {
    lat: f32,
    long: f32,
}

#[derive(Serialize, Deserialize)]
pub struct MessengerLocationPayload {
    pub coordinates: Option<MessengerLocation>,
}
// Missing document payload
#[derive(Serialize, Deserialize)]
pub struct MessengerAttachment {
    #[serde(rename = "type")]
    pub type_name: String,
    pub payload: Option<MessengerLocationPayload>,
}

impl Default for MessengerAttachment {
    fn default() -> MessengerAttachment {
        MessengerAttachment {
            type_name: String::new(),
            payload: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub text: Option<String>,
    pub attachments: Option<Vec<MessengerAttachment>>,
    pub quick_reply: Option<MessengerPayload>,
}

#[derive(Serialize, Deserialize)]
pub struct MessengerMessage {
    pub sender: MessengerSender,
    pub message: Message,
}
#[derive(Serialize, Deserialize)]
pub struct MessengerEvent {
    pub messaging: Vec<MessengerMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct MessengerRequest {
    pub entry: Vec<MessengerEvent>,
}

impl MessengerEvent {
    pub fn new() -> MessengerEvent {
        return MessengerEvent { messaging: vec![] };
    }

    pub fn get_events_ref(&self) -> &Vec<MessengerMessage> {
        self.messaging.as_ref()
    }
}
