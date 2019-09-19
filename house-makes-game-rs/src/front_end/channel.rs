//! A more general and simplified interface for sender/receiver (compared to [std::sync::mpsc::channel]) 
//! which is used to communicate with the renderer of the front end.

use std::sync::mpsc;

pub trait Sender<T: Send>: Send {
    fn send(&self, msg: T) -> Result<(), ()>;
}

pub trait Receiver<T: Send>: Send {
    fn recv(&self) -> Result<T, ()>;
}

impl<T: Send> Sender<T> for mpsc::Sender<T> {
    fn send(&self, msg: T) -> Result<(), ()> {
        mpsc::Sender::send(self, msg)
            .map_err(|_| ())
    }
}

impl<T: Send> Receiver<T> for mpsc::Receiver<T> {
    fn recv(&self) -> Result<T, ()> {
        mpsc::Receiver::recv(self)
            .map_err(|_| ())
    }
}
