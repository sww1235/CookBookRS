use super::Error;

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

///Terminal Events
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Event {
    /// Tick
    Tick,
    /// Key press
    Key(KeyEvent),
    /// Mouse click/scroll
    Mouse(MouseEvent),
    ///Terminal Resize
    Resize(u16, u16),
}

///Terminal Event Handler
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
#[allow(dead_code)] //TODO: figure out why this is detecting dead code
pub struct EventHandler {
    /// Event sender channel
    sender: mpsc::Sender<Event>,
    /// Event receiver channel
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// constructs a new instance of [`EventHandler`]
    ///
    /// # Panics
    /// This function doesn't actually panic itself, but the thread spawned inside may panic
    #[allow(clippy::expect_used)] //TODO: maybe fix this?
    #[must_use]
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or(tick_rate);

                    if event::poll(timeout).expect("failed to poll new events") {
                        #[allow(clippy::match_same_arms)] //TODO: remove this eventually
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            CrosstermEvent::FocusGained => Ok(()), //TODO: add something here
                            CrosstermEvent::FocusLost => Ok(()),   //TODO: add something here
                            CrosstermEvent::Paste(_) => Ok(()),    //TODO: add something here
                        }
                        .expect("failed to send terminal event");
                    }
                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self { sender, receiver, handler }
    }
    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    ///
    /// # Errors
    /// - [`std::io::Error errors`]
    /// - [`std::sync::mpsc::RecvError`] errors
    pub fn next(&self) -> Result<Event, Error> {
        Ok(self.receiver.recv()?)
    }
}
