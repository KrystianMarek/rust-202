//! Design patterns example

use rust_202::oop::patterns::{
    CompressionContext, Config, GzipCompression, NotificationFactory, NotificationType, UserBuilder,
};

fn main() {
    println!("=== Design Patterns Examples ===\n");

    // Singleton pattern
    println!("1. Singleton Pattern - Global Config:");
    Config::set("api_url", "https://api.example.com");
    Config::set("api_key", "secret123");
    println!("   API URL: {:?}", Config::get("api_url"));
    println!("   API Key: {:?}\n", Config::get("api_key"));

    // Factory pattern
    println!("2. Factory Pattern - Notifications:");
    let factory = NotificationFactory;
    let email = factory.create(NotificationType::Email);
    let sms = factory.create(NotificationType::Sms);
    print!("   ");
    email.send("Your order has shipped!");
    print!("   ");
    sms.send("Verification code: 123456");
    println!();

    // Builder pattern
    println!("3. Builder Pattern - User Creation:");
    let user = UserBuilder::new("alice")
        .email("alice@example.com")
        .age(30)
        .active(true)
        .build();

    println!("   Username: {}", user.username());
    println!("   Email: {:?}", user.email());
    println!("   Age: {:?}", user.age());
    println!("   Active: {}\n", user.is_active());

    // Strategy pattern
    println!("4. Strategy Pattern - Compression:");
    let context = CompressionContext::new(Box::new(GzipCompression));
    let data = "Hello, World!";
    let compressed = context.compress(data);
    let decompressed = context.decompress(&compressed);
    println!("   Original: {}", data);
    println!("   Decompressed: {}\n", decompressed);

    println!("=== End of Patterns Examples ===");
}
