use std::sync::Arc;
use tauri_plugin_rudderstack::{RateLimiterFn, PerEventCap};

/// Example showing how to use the rate limiter functionality
/// 
/// This example demonstrates:
/// 1. Basic rate limiter using a custom function
/// 2. PerEventCap rate limiter for limiting events per minute by event type
/// 3. How to register and remove rate limiters
fn main() {
    // Example 1: Basic rate limiter that drops every 10th event
    let _basic_rate_limiter: Arc<RateLimiterFn> = Arc::new(|_msg| {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        count % 10 != 9 // Drop every 10th event
    });

    // Example 2: PerEventCap rate limiter - allows max 100 events per minute per event type
    let per_event_cap = PerEventCap::new(100);
    let _per_event_rate_limiter: Arc<RateLimiterFn> = Arc::new(move |msg| {
        per_event_cap.should_allow(msg)
    });

    // Example 3: Custom rate limiter based on event type
    let _selective_rate_limiter: Arc<RateLimiterFn> = Arc::new(|msg| {
        match msg {
            rudderanalytics::message::Message::Track(track) => {
                // Allow all track events except "spam_event"
                track.event != "spam_event"
            }
            rudderanalytics::message::Message::Identify(_) => {
                // Limit identify events to every other one
                static IDENTIFY_COUNTER: std::sync::atomic::AtomicUsize = 
                    std::sync::atomic::AtomicUsize::new(0);
                let count = IDENTIFY_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                count % 2 == 0
            }
            _ => true, // Allow all other event types
        }
    });

    println!("Rate limiter examples created!");
    println!("To use in a Tauri app:");
    println!("1. app.set_rate_limiter(_basic_rate_limiter);");
    println!("2. app.set_rate_limiter(_per_event_rate_limiter);");
    println!("3. app.set_rate_limiter(_selective_rate_limiter);");
    println!("4. app.remove_rate_limiter(); // To remove the rate limiter");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_per_event_cap_example() {
        let per_event_cap = PerEventCap::new(2);
        
        let track_message = rudderanalytics::message::Message::Track(
            rudderanalytics::message::Track {
                event: "test_event".to_string(),
                ..Default::default()
            }
        );

        // First two should pass
        assert!(per_event_cap.should_allow(&track_message));
        assert!(per_event_cap.should_allow(&track_message));
        
        // Third should be blocked
        assert!(!per_event_cap.should_allow(&track_message));
        
        // Check stats
        let stats = per_event_cap.get_stats();
        assert_eq!(stats.get("test_event"), Some(&2));
    }

    #[test]
    fn test_selective_rate_limiter() {
        let selective_rate_limiter: Arc<RateLimiterFn> = Arc::new(|msg| {
            match msg {
                rudderanalytics::message::Message::Track(track) => {
                    track.event != "spam_event"
                }
                _ => true,
            }
        });

        let good_track = rudderanalytics::message::Message::Track(
            rudderanalytics::message::Track {
                event: "good_event".to_string(),
                ..Default::default()
            }
        );

        let spam_track = rudderanalytics::message::Message::Track(
            rudderanalytics::message::Track {
                event: "spam_event".to_string(),
                ..Default::default()
            }
        );

        assert!(selective_rate_limiter(&good_track));
        assert!(!selective_rate_limiter(&spam_track));
    }
} 