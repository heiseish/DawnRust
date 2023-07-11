pub trait PlatformAPI {
    fn typing_on(&self, sender_id: &String) {}
    fn typing_off(&self, sender_id: &String) {}
}
