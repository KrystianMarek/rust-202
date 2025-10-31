//! # Async State Machine
//!
//! State pattern with async transitions.

/// Connection states
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    /// Disconnected state
    Disconnected,
    /// Connecting state
    Connecting,
    /// Connected state
    Connected,
    /// Error state
    Error(String),
}

/// Async state machine for connections
///
/// ## Why?
/// Async state machines model complex workflows with compile-time state
/// validation. Unlike Python's async state machines (runtime checks) or
/// Go's select-based state management, Rust enforces correctness at compile-time.
///
/// ## Example
/// ```rust
/// # async fn example() {
/// use rust_202::r#async::patterns::{ConnectionStateMachine, ConnectionState};
///
/// let mut machine = ConnectionStateMachine::new();
/// assert_eq!(machine.current_state(), &ConnectionState::Disconnected);
///
/// machine.connect().await;
/// assert_eq!(machine.current_state(), &ConnectionState::Connected);
/// # }
/// ```
pub struct ConnectionStateMachine {
    state: ConnectionState,
}

impl ConnectionStateMachine {
    /// Create a new state machine
    pub fn new() -> Self {
        Self {
            state: ConnectionState::Disconnected,
        }
    }

    /// Get current state
    pub fn current_state(&self) -> &ConnectionState {
        &self.state
    }

    /// Transition to connecting
    pub async fn connect(&mut self) {
        match self.state {
            ConnectionState::Disconnected => {
                self.state = ConnectionState::Connecting;
                // Simulate async connection
                self.state = ConnectionState::Connected;
            }
            _ => {
                self.state = ConnectionState::Error("Invalid transition".to_string());
            }
        }
    }

    /// Transition to disconnected
    pub async fn disconnect(&mut self) {
        self.state = ConnectionState::Disconnected;
    }
}

impl Default for ConnectionStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(test, feature = "async-tokio"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_state_machine() {
        let mut machine = ConnectionStateMachine::new();
        assert_eq!(machine.current_state(), &ConnectionState::Disconnected);

        machine.connect().await;
        assert_eq!(machine.current_state(), &ConnectionState::Connected);

        machine.disconnect().await;
        assert_eq!(machine.current_state(), &ConnectionState::Disconnected);
    }
}

