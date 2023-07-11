use crate::platform::api::PlatformAPI;

pub struct MessengerAPI {}
impl PlatformAPI for MessengerAPI {
    fn typing_on(&self, sender_id: &String) {}
    fn typing_off(&self, sender_id: &String) {}
}
