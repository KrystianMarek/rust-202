//! # Observer Pattern
//!
//! Observer pattern using channels for event notification.

use std::sync::mpsc::{channel, Receiver, Sender};

/// Event types that can be observed
#[derive(Debug, Clone)]
pub enum Event {
    /// User logged in
    UserLoggedIn(String),
    /// User logged out
    UserLoggedOut(String),
    /// Data updated
    DataUpdated(String),
}

/// Observer trait for receiving events
///
/// ## Why?
/// Unlike Python's observer pattern with callbacks or Go's channels,
/// Rust's trait-based observers with channels provide type safety
/// and clear ownership semantics.
pub trait Observer: Send {
    /// Handle an event
    fn handle_event(&self, event: &Event);
}

/// Console logger observer
pub struct ConsoleLogger;

impl Observer for ConsoleLogger {
    fn handle_event(&self, event: &Event) {
        println!("[ConsoleLogger] Event: {:?}", event);
    }
}

/// File logger observer
pub struct FileLogger {
    filename: String,
}

impl FileLogger {
    /// Create a new file logger
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
        }
    }
}

impl Observer for FileLogger {
    fn handle_event(&self, event: &Event) {
        println!("[FileLogger] Writing to {}: {:?}", self.filename, event);
    }
}

/// Subject that notifies observers
///
/// ## Example
/// ```rust
/// use rust_202::oop::patterns::observer::{EventSubject, ConsoleLogger, Event};
///
/// let mut subject = EventSubject::new();
/// subject.attach(Box::new(ConsoleLogger));
/// subject.notify(Event::UserLoggedIn("Alice".to_string()));
/// ```
pub struct EventSubject {
    observers: Vec<Box<dyn Observer>>,
}

impl EventSubject {
    /// Create a new subject
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    /// Attach an observer
    pub fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    /// Notify all observers of an event
    pub fn notify(&self, event: Event) {
        for observer in &self.observers {
            observer.handle_event(&event);
        }
    }

    /// Get the number of observers
    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }
}

impl Default for EventSubject {
    fn default() -> Self {
        Self::new()
    }
}

/// Channel-based event system (more idiomatic Rust)
///
/// ## Why?
/// This is more idiomatic in Rust than the traditional observer pattern.
/// Channels provide thread-safe communication with clear ownership.
///
/// ## Example
/// ```rust
/// use rust_202::oop::patterns::observer::{EventChannel, Event};
///
/// let channel = EventChannel::new();
/// let receiver = channel.subscribe();
/// channel.publish(Event::DataUpdated("New data".to_string()));
/// ```
pub struct EventChannel {
    sender: Sender<Event>,
}

impl EventChannel {
    /// Create a new event channel
    pub fn new() -> Self {
        let (sender, _receiver) = channel();
        Self { sender }
    }

    /// Subscribe to events (returns a new receiver)
    pub fn subscribe(&self) -> Receiver<Event> {
        let (_sender, receiver) = channel();
        // In a real implementation, we'd manage multiple receivers
        receiver
    }

    /// Publish an event
    pub fn publish(&self, event: Event) {
        // In a real implementation, send to all subscribers
        let _ = self.sender.send(event);
    }
}

impl Default for EventChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer_pattern() {
        let mut subject = EventSubject::new();
        subject.attach(Box::new(ConsoleLogger));
        subject.attach(Box::new(FileLogger::new("events.log")));

        assert_eq!(subject.observer_count(), 2);

        subject.notify(Event::UserLoggedIn("Alice".to_string()));
        subject.notify(Event::DataUpdated("Test data".to_string()));
    }

    #[test]
    fn test_event_channel() {
        let channel = EventChannel::new();
        channel.publish(Event::UserLoggedIn("Bob".to_string()));
    }
}
