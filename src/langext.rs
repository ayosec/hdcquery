use chrono::prelude::*;

pub trait DurationExt {
    fn to_human(&self) -> String;
}

impl DurationExt for DateTime<chrono::Utc> {
    fn to_human(&self) -> String {
        let time_ago = Utc::now() - *self;

        if time_ago.num_seconds() < 90 {
            return "just now".to_string();
        }

        if let n @ 0..=120 = time_ago.num_minutes() {
            return format!("{} minutes ago", n);
        }

        if let n @ 0..=72 = time_ago.num_hours() {
            return format!("{} hours ago", n);
        }

        if let n @ 0..=30 = time_ago.num_days() {
            return format!("{} days ago", n);
        }

        if let n @ 0..=20 = time_ago.num_weeks() {
            return format!("{} weeks ago", n);
        }

        self.format("%F").to_string()
    }
}

#[test]
fn compute_durations() {
    macro_rules! check {
        ($num:literal $unit:ident => $expect:expr) => {
            let time = Utc::now() - chrono::Duration::$unit($num);
            assert_eq!(time.to_human(), $expect.to_string());
        };
    }

    check!(10 seconds => "just now");
    check!(30 minutes => "30 minutes ago");
    check!(8 hours => "8 hours ago");
    check!(10 days => "10 days ago");
    check!(15 weeks => "15 weeks ago");

    let long_ago = DateTime::parse_from_rfc3339("2000-01-02T00:00:00Z").unwrap();
    assert_eq!(
        long_ago.with_timezone(&Utc).to_human(),
        "2000-01-02".to_string()
    );
}
