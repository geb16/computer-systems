// first we import collections for in-memory counting/correlation
use std::collections::HashMap;

// then we import filesystem + io to read the log file line-by-line
use std::fs::File;
use std::io::{BufRead, BufReader};

// then we import chrono for timestamps that we can sort and print cleanly
use chrono::{DateTime, Utc};

// then we import serde for JSON -> struct parsing
use serde::Deserialize;

// then we define a struct to represent ONE log event line.
// This is the "schema" of our JSON logs.
// Deserialize means: serde can build this struct from JSON.
#[derive(Debug, Deserialize, Clone)]
struct LogEvent {
    ts: DateTime<Utc>,   // ISO timestamp like "2026-03-03T18:00:01Z"
    event: String,       // event type (auth_fail, auth_ok, persistence, etc.)
    user: String,        // username
    ip: String,          // source IP
    service: String,     // system that emitted the log (vpn, db, host, etc.)
    detail: Option<String>, // optional extra info
}

// then we need a struct that holds our detector state.
// Think: "mini SIEM correlation memory".
struct IncidentDetector {
    // failed login counts per (user, ip)
    auth_fail_counts: HashMap<(String, String), u32>,
    // the key is a tuple of (user, ip) and the value is the count of auth_fail events for that pair.
    // threshold for brute-force suspicion
    brute_force_threshold: u32,

    // store "suspicious findings" as human-readable messages
    findings: Vec<String>,

    // store timeline events that matter (for final incident summary)
    timeline: Vec<LogEvent>,
}

// implementing methods for IncidentDetector
impl IncidentDetector {
    // constructor: create a detector with chosen threshold
    fn new(brute_force_threshold: u32) -> Self {
        Self {
            auth_fail_counts: HashMap::new(),
            brute_force_threshold,
            findings: Vec::new(),
            timeline: Vec::new(),
        }
    }

    // we need a function to process each incoming log event.
    // This is where "detection logic" lives.
    fn process(&mut self, ev: LogEvent) {
        // Always keep a timeline copy (we’ll filter/summarize later).
        self.timeline.push(ev.clone());

        match ev.event.as_str() {
            // 1) Detect brute force: repeated auth failures
            "auth_fail" => {
                let key = (ev.user.clone(), ev.ip.clone());
                let count = self.auth_fail_counts.entry(key).or_insert(0);
                *count += 1;

                if *count == self.brute_force_threshold {
                    self.findings.push(format!(
                        "[{}] Suspicious: {} failed logins for user='{}' from ip={}",
                        ev.ts, self.brute_force_threshold, ev.user, ev.ip
                    ));
                }
            }

            // 2) Detect “success after failures” (often a compromise signal)
            "auth_ok" => {
                let key = (ev.user.clone(), ev.ip.clone());
                let fail_count = self.auth_fail_counts.get(&key).copied().unwrap_or(0);

                if fail_count >= self.brute_force_threshold {
                    self.findings.push(format!(
                        "[{}] High risk: successful login after {} failures (user='{}', ip={})",
                        ev.ts, fail_count, ev.user, ev.ip
                    ));
                }
            }

            // 3) Privilege escalation signal
            "privilege_change" => {
                self.findings.push(format!(
                    "[{}] Privilege change observed (user='{}', ip={}, detail={:?})",
                    ev.ts, ev.user, ev.ip, ev.detail
                ));
            }

            // 4) Persistence signal
            "persistence" => {
                self.findings.push(format!(
                    "[{}] Persistence observed (user='{}', ip={}, detail={:?})",
                    ev.ts, ev.user, ev.ip, ev.detail
                ));
            }

            // 5) Data access + Exfil indicators
            "data_access" | "exfil" => {
                self.findings.push(format!(
                    "[{}] Data risk event '{}' (user='{}', ip={}, service={}, detail={:?})",
                    ev.ts, ev.event, ev.user, ev.ip, ev.service, ev.detail
                ));
            }

            // ignore unknown events for now (extend later)
            _ => {}
        }
    }

    // function to print a clean incident report
    fn report(&mut self) {
        // sort timeline by timestamp
        self.timeline.sort_by_key(|e| e.ts);

        println!("=== Incident Triage Report ===");
        println!("Findings:");
        if self.findings.is_empty() {
            println!("  (none)");
        } else {
            for f in &self.findings {
                println!("  - {}", f);
            }
        }

        println!("\nTimeline (sorted):");
        for e in &self.timeline {
            println!(
                "  [{}] {:<16} user='{}' ip={} service={} detail={:?}",
                e.ts, e.event, e.user, e.ip, e.service, e.detail
            );
        }

        // Response guidance (high-level) mapped to NIST IR phases:
        // We keep it general + defensive: detect/contain/eradicate/recover.
        // (NIST incident handling guidance discusses phases like detection/analysis and containment/eradication/recovery.) :contentReference[oaicite:4]{index=4}
        println!("\nRecommended Response (defender playbook):");
        println!("  1) Containment: disable the affected account / revoke VPN session tokens.");
        println!("  2) Containment: block source IP at edge/VPN if appropriate.");
        println!("  3) Eradication: rotate credentials/keys and inspect hosts for persistence artifacts.");
        println!("  4) Recovery: restore services, monitor for re-entry, validate integrity.");
        println!("  5) Post-incident: improve controls (MFA, rate limiting, alert tuning, logging).");
    }
}

// function to read JSONL file into LogEvent items
fn read_logs(path: &str) -> Vec<LogEvent> {
    let file = File::open(path).expect("failed to open log file");
    let reader = BufReader::new(file);

    let mut events = Vec::new();
    for line in reader.lines() {
        let line = line.expect("failed to read line");
        if line.trim().is_empty() {
            continue;
        }
        let ev: LogEvent = serde_json::from_str(&line).expect("invalid JSON line");
        events.push(ev);
    }
    events
}

fn main() {
    // instantiating detector with a threshold of 5 failures for brute-force suspicion
    let mut detector = IncidentDetector::new(5);

    // read logs from file
    let events = read_logs("logs.jsonl");

    // process each event (this is our "correlation loop")
    for ev in events {
        detector.process(ev);
    }

    // print report
    detector.report();
}

// Which exact event sequence in our logs suggests “compromise” more than random user mistakes?
// why we sort the timeline by ts before printing? 
// Sorting the timeline by timestamp (ts) ensures that when we print the events, they are in chronological order. 
// This helps us understand the sequence of events as they actually occurred, 
// which is crucial for incident analysis and triage. 
// It allows us to see how events unfolded over time, identify patterns, and correlate related events more effectively.