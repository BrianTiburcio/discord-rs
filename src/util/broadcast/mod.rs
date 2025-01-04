use std::sync::mpsc::{self, Sender, Receiver};

pub struct Broadcast<T> {
    pub(crate) sender: Sender<T>,
    pub(crate) receivers: Vec<Receiver<T>>,
}

// impl<T> Broadcast<T>
// where T: Send + 'static
// {
//     pub fn new(receiver_count: u8) -> Self {
//         assert!(receiver_count > 0, "Broadcast must have at least one receiver");

//         let (sx, rx) = mpsc::channel::<T>();
//         let mut receivers = Vec::new();
        
//         for _ in 0..receiver_count {
//             receivers.push(rx.clone());
//         }

//         Broadcast {
//             sender: sx,
//             receivers
//         }
//     }

//     pub fn send(&self, data: T) {
//         self.sender.send(data).unwrap();
//     }

//     pub fn recv(&self) -> T {
//         self.receivers.recv().unwrap()
//     }
// }