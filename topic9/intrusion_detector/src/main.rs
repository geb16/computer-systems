// HashMap stores login attempt counts per IP
use std::collections::HashMap;

// time module for timestamps
use std::time::{SystemTime, UNIX_EPOCH};
// UNIX_EPOCH is the reference point for time calculations

// structure to represent a login attempt
#[derive(Debug)]
struct LoginAttempt {
    ip: String,
    success: bool,
    #[allow(dead_code)]
    timestamp: u64,
}

// intrusion detection system state
struct IntrusionDetector {
    failed_attempts: HashMap<String, u32>,
    threshold: u32,
}

impl IntrusionDetector {
    // constructor
    fn new(threshold: u32) -> Self {
        Self {
            failed_attempts: HashMap::new(),
            threshold,
        }
    }

    // process a login attempt
    fn process_attempt(&mut self, attempt: LoginAttempt) {
        if attempt.success {
            // reset failed count on successful login
            self.failed_attempts.remove(&attempt.ip);
        } else {
            let count = self.failed_attempts.entry(attempt.ip.clone()).or_insert(0);
            *count += 1;

            if *count >= self.threshold {
                println!("⚠ Suspicious activity detected from IP: {}", attempt.ip);
            }
        }
    }
}

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn main() {
    let mut detector = IntrusionDetector::new(5);

    // simulate repeated failed attempts
    for _ in 0..6 {
        let attempt = LoginAttempt {
            ip: "192.168.1.10".to_string(),
            success: false,
            timestamp: current_time(),
        };

        detector.process_attempt(attempt);
    }
}
//  In our detector, what data structure tracks suspicious activity?
// The `IntrusionDetector` struct uses a `HashMap<String, u32>` 
// called `failed_attempts` to track the number of failed login attempts for each IP address.
//Why do we reset failed attempts after a successful login?
// We reset failed attempts after a successful login to prevent false positives.
// If a user successfully logs in, it indicates that they are likely legitimate, 
// so we clear their failed attempt count to avoid mistakenly flagging them as suspicious in the future.
//What