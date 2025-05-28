use std::collections::HashMap;
use std::time::{Duration, Instant};
use dashmap::DashMap;

/// A rate limiter that caps the number of events per minute for each event type
/// 
/// This implementation uses DashMap for high-performance concurrent access without explicit locking.
/// Each event type is tracked separately with its own counter and time window.
/// 
/// # Example
/// 
/// ```rust
/// use std::sync::Arc;
/// use tauri_plugin_rudderstack::{AnalyticsExt, rate_limiters::PerEventCap};
/// 
/// // Allow maximum 100 events per minute for each event type
/// let rate_limiter = PerEventCap::new(100);
/// let rate_limiter_fn = Arc::new(move |msg| rate_limiter.should_allow(msg));
/// 
/// // Register the rate limiter
/// // app.set_rate_limiter(rate_limiter_fn);
/// ```
pub struct PerEventCap {
    events_per_minute: u32,
    event_counters: DashMap<String, EventCounter>,
}

#[derive(Debug)]
struct EventCounter {
    count: u32,
    window_start: Instant,
}

impl EventCounter {
    fn new() -> Self {
        Self {
            count: 0,
            window_start: Instant::now(),
        }
    }

    fn reset_if_expired(&mut self) {
        if self.window_start.elapsed() >= Duration::from_secs(60) {
            self.count = 0;
            self.window_start = Instant::now();
        }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn current_count(&self) -> u32 {
        self.count
    }
}

impl PerEventCap {
    /// Create a new PerEventCap rate limiter
    /// 
    /// # Arguments
    /// * `events_per_minute` - Maximum number of events allowed per minute for each event type
    pub fn new(events_per_minute: u32) -> Self {
        Self {
            events_per_minute,
            event_counters: DashMap::new(),
        }
    }

    /// Check if an event should be allowed based on the rate limit
    /// Returns true if the event should be sent, false if it should be dropped
    pub fn should_allow(&self, message: &rudderanalytics::message::Message) -> bool {
        let event_type = self.extract_event_type(message).to_string();
        
        // Use entry API to get or insert a new counter
        let mut counter = self.event_counters.entry(event_type).or_insert_with(EventCounter::new);
        
        // Reset counter if the time window has expired
        counter.reset_if_expired();
        
        // Check if we're within the limit
        if counter.current_count() < self.events_per_minute {
            counter.increment();
            true
        } else {
            false
        }
    }

    /// Extract the event type from a RudderStack message
    fn extract_event_type(&self, message: &rudderanalytics::message::Message) -> String {
        match message {
            rudderanalytics::message::Message::Track(track) => {
                track.event.clone()
            }
            rudderanalytics::message::Message::Identify(_) => "identify".to_string(),
            rudderanalytics::message::Message::Page(page) => {
                page.name.clone()
            }
            rudderanalytics::message::Message::Screen(screen) => {
                screen.name.clone()
            }
            rudderanalytics::message::Message::Group(_) => "group".to_string(),
            rudderanalytics::message::Message::Alias(_) => "alias".to_string(),
            rudderanalytics::message::Message::Batch(_) => "batch".to_string(),
        }
    }

    /// Get current statistics for all event types
    /// Returns a HashMap with event type as key and current count as value
    pub fn get_stats(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        
        for mut entry in self.event_counters.iter_mut() {
            let (event_type, counter) = entry.pair_mut();
            counter.reset_if_expired();
            stats.insert(event_type.clone(), counter.current_count());
        }
        
        stats
    }

    /// Reset all counters
    pub fn reset(&self) {
        self.event_counters.clear();
    }
}

impl crate::rudder_wrapper::RateLimiter for PerEventCap {
    fn let_pass(&self, msg: &rudderanalytics::message::Message) -> bool {
        self.should_allow(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_per_event_cap_basic() {
        let rate_limiter = PerEventCap::new(2); // Allow 2 events per minute

        let track_message = rudderanalytics::message::Message::Track(
            rudderanalytics::message::Track {
                event: "test_event".to_string(),
                ..Default::default()
            }
        );

        // First two events should be allowed
        assert!(rate_limiter.should_allow(&track_message));
        assert!(rate_limiter.should_allow(&track_message));

        // Third event should be blocked
        assert!(!rate_limiter.should_allow(&track_message));
    }

    #[test]
    fn test_per_event_cap_different_events() {
        let rate_limiter = PerEventCap::new(1); // Allow 1 event per minute

        let track_message1 = rudderanalytics::message::Message::Track(
            rudderanalytics::message::Track {
                event: "event1".to_string(),
                ..Default::default()
            }
        );

        let track_message2 = rudderanalytics::message::Message::Track(
            rudderanalytics::message::Track {
                event: "event2".to_string(),
                ..Default::default()
            }
        );

        // Different event types should have separate counters
        assert!(rate_limiter.should_allow(&track_message1));
        assert!(rate_limiter.should_allow(&track_message2));

        // Second occurrence of each should be blocked
        assert!(!rate_limiter.should_allow(&track_message1));
        assert!(!rate_limiter.should_allow(&track_message2));
    }

    #[test]
    fn test_per_event_cap_identify_message() {
        let rate_limiter = PerEventCap::new(1);

        let identify_message = rudderanalytics::message::Message::Identify(
            rudderanalytics::message::Identify {
                user_id: Some("test_user".to_string()),
                ..Default::default()
            }
        );

        assert!(rate_limiter.should_allow(&identify_message));
        assert!(!rate_limiter.should_allow(&identify_message));
    }

    #[test]
    fn test_per_event_cap_stats() {
        let rate_limiter = PerEventCap::new(5);

        let track_message = rudderanalytics::message::Message::Track(
            rudderanalytics::message::Track {
                event: "test_event".to_string(),
                ..Default::default()
            }
        );

        // Send 3 events
        rate_limiter.should_allow(&track_message);
        rate_limiter.should_allow(&track_message);
        rate_limiter.should_allow(&track_message);

        let stats = rate_limiter.get_stats();
        assert_eq!(stats.get("test_event"), Some(&3));
    }

    #[test]
    fn test_per_event_cap_reset() {
        let rate_limiter = PerEventCap::new(1);

        let track_message = rudderanalytics::message::Message::Track(
            rudderanalytics::message::Track {
                event: "test_event".to_string(),
                ..Default::default()
            }
        );

        // Use up the limit
        assert!(rate_limiter.should_allow(&track_message));
        assert!(!rate_limiter.should_allow(&track_message));

        // Reset and try again
        rate_limiter.reset();
        assert!(rate_limiter.should_allow(&track_message));
    }
} 