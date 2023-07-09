pub struct Duration {
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
}

impl Duration {
    pub fn from_chrono_duration(d: chrono::Duration) -> Duration {
        let mut seconds = d.num_seconds();
        let hours = seconds / 3600;
        seconds -= hours * 3600;
        let minutes = seconds / 60;
        seconds -= minutes * 60;

        Duration {
            hours,
            minutes,
            seconds,
        }
    }
}
