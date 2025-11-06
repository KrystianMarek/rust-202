//! # Factory Pattern
//!
//! Factory pattern using traits for object creation.

/// A trait for creating different types of notifications
///
/// ## Why?
/// Factory pattern with traits provides type-safe object creation.
/// Unlike Python's duck typing or Go's interface{}, Rust ensures
/// all implementations satisfy the contract at compile-time.
///
/// ## Example
/// ```rust
/// use rust_202::oop::patterns::factory::{NotificationFactory, NotificationType};
///
/// let factory = NotificationFactory;
/// let email = factory.create(NotificationType::Email);
/// email.send("Hello!");
/// ```
pub trait Notification {
    /// Send a notification
    fn send(&self, message: &str);

    /// Get notification type
    fn notification_type(&self) -> &str;
}

/// Email notification
pub struct EmailNotification {
    recipient: String,
}

impl EmailNotification {
    /// Create a new email notification
    pub fn new(recipient: &str) -> Self {
        Self {
            recipient: recipient.to_string(),
        }
    }
}

impl Notification for EmailNotification {
    fn send(&self, message: &str) {
        println!("Sending email to {}: {}", self.recipient, message);
    }

    fn notification_type(&self) -> &str {
        "Email"
    }
}

/// SMS notification
pub struct SmsNotification {
    phone: String,
}

impl SmsNotification {
    /// Create a new SMS notification
    pub fn new(phone: &str) -> Self {
        Self {
            phone: phone.to_string(),
        }
    }
}

impl Notification for SmsNotification {
    fn send(&self, message: &str) {
        println!("Sending SMS to {}: {}", self.phone, message);
    }

    fn notification_type(&self) -> &str {
        "SMS"
    }
}

/// Push notification
pub struct PushNotification {
    device_id: String,
}

impl PushNotification {
    /// Create a new push notification
    pub fn new(device_id: &str) -> Self {
        Self {
            device_id: device_id.to_string(),
        }
    }
}

impl Notification for PushNotification {
    fn send(&self, message: &str) {
        println!("Sending push to device {}: {}", self.device_id, message);
    }

    fn notification_type(&self) -> &str {
        "Push"
    }
}

/// Notification types
#[derive(Debug, Clone, Copy)]
pub enum NotificationType {
    /// Email notification
    Email,
    /// SMS notification
    Sms,
    /// Push notification
    Push,
}

/// Factory for creating notifications
///
/// ## Example
/// ```rust
/// use rust_202::oop::patterns::factory::{NotificationFactory, NotificationType};
///
/// let factory = NotificationFactory;
/// let sms = factory.create(NotificationType::Sms);
/// assert_eq!(sms.notification_type(), "SMS");
/// ```
pub struct NotificationFactory;

impl NotificationFactory {
    /// Create a notification based on type
    pub fn create(&self, notification_type: NotificationType) -> Box<dyn Notification> {
        match notification_type {
            NotificationType::Email => Box::new(EmailNotification::new("user@example.com")),
            NotificationType::Sms => Box::new(SmsNotification::new("+1234567890")),
            NotificationType::Push => Box::new(PushNotification::new("device-123")),
        }
    }

    /// Create with custom configuration
    pub fn create_email(&self, recipient: &str) -> Box<dyn Notification> {
        Box::new(EmailNotification::new(recipient))
    }

    /// Create SMS with custom phone
    pub fn create_sms(&self, phone: &str) -> Box<dyn Notification> {
        Box::new(SmsNotification::new(phone))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = NotificationFactory;
        let email = factory.create(NotificationType::Email);
        assert_eq!(email.notification_type(), "Email");

        let sms = factory.create(NotificationType::Sms);
        assert_eq!(sms.notification_type(), "SMS");

        let push = factory.create(NotificationType::Push);
        assert_eq!(push.notification_type(), "Push");
    }

    #[test]
    fn test_custom_creation() {
        let factory = NotificationFactory;
        let email = factory.create_email("custom@example.com");
        email.send("Test message");
    }
}
