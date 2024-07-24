use super::Message;

trait EventHandler {
    fn on_ready(&self);
    fn on_message(&self, message: Message);
}