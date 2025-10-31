//! # Builder Pattern
//!
//! Fluent builder pattern for complex object construction.

/// A user configuration builder
///
/// ## Why?
/// The builder pattern in Rust leverages ownership to ensure
/// objects are valid when constructed. Unlike Python's kwargs
/// or Go's functional options, Rust builders can enforce
/// required fields at compile-time.
///
/// ## Example
/// ```rust
/// use rust_202::oop::patterns::builder::UserBuilder;
///
/// let user = UserBuilder::new("alice")
///     .email("alice@example.com")
///     .age(30)
///     .build();
///
/// assert_eq!(user.username(), "alice");
/// assert_eq!(user.email(), Some("alice@example.com"));
/// ```
#[derive(Debug, Clone)]
pub struct User {
    username: String,
    email: Option<String>,
    age: Option<u32>,
    active: bool,
}

impl User {
    /// Get username
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get email
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    /// Get age
    pub fn age(&self) -> Option<u32> {
        self.age
    }

    /// Check if active
    pub fn is_active(&self) -> bool {
        self.active
    }
}

/// Builder for User
pub struct UserBuilder {
    username: String,
    email: Option<String>,
    age: Option<u32>,
    active: bool,
}

impl UserBuilder {
    /// Create a new builder with required username
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
            email: None,
            age: None,
            active: true,
        }
    }

    /// Set email (fluent interface)
    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    /// Set age (fluent interface)
    pub fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }

    /// Set active status
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Build the User
    pub fn build(self) -> User {
        User {
            username: self.username,
            email: self.email,
            age: self.age,
            active: self.active,
        }
    }
}

/// HTTP request builder example
///
/// ## Example
/// ```rust
/// use rust_202::oop::patterns::builder::HttpRequestBuilder;
///
/// let request = HttpRequestBuilder::new("https://api.example.com")
///     .method("POST")
///     .header("Content-Type", "application/json")
///     .body(r#"{"key": "value"}"#)
///     .build();
///
/// assert_eq!(request.url(), "https://api.example.com");
/// assert_eq!(request.method(), "POST");
/// ```
#[derive(Debug, Clone)]
pub struct HttpRequest {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl HttpRequest {
    /// Get URL
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get method
    pub fn method(&self) -> &str {
        &self.method
    }

    /// Get headers
    pub fn headers(&self) -> &[(String, String)] {
        &self.headers
    }

    /// Get body
    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
}

/// Builder for HTTP requests
pub struct HttpRequestBuilder {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl HttpRequestBuilder {
    /// Create a new request builder
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            method: "GET".to_string(),
            headers: Vec::new(),
            body: None,
        }
    }

    /// Set HTTP method
    pub fn method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }

    /// Add a header
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    /// Set request body
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    /// Build the request
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_builder() {
        let user = UserBuilder::new("alice")
            .email("alice@example.com")
            .age(30)
            .build();

        assert_eq!(user.username(), "alice");
        assert_eq!(user.email(), Some("alice@example.com"));
        assert_eq!(user.age(), Some(30));
        assert!(user.is_active());
    }

    #[test]
    fn test_user_builder_minimal() {
        let user = UserBuilder::new("bob").build();
        assert_eq!(user.username(), "bob");
        assert_eq!(user.email(), None);
        assert!(user.is_active());
    }

    #[test]
    fn test_http_request_builder() {
        let request = HttpRequestBuilder::new("https://api.example.com")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(r#"{"key": "value"}"#)
            .build();

        assert_eq!(request.url(), "https://api.example.com");
        assert_eq!(request.method(), "POST");
        assert_eq!(request.headers().len(), 1);
        assert!(request.body().is_some());
    }
}
