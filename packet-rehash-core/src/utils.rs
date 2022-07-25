pub mod format {
    use std::fmt::{Display, Formatter};
    use std::time::Duration;

    #[derive(Debug)]
    pub struct FormattedDuration {
        seconds: u64,
        minutes: u64,
        hours: u64,
        days: u64,
    }

    impl FormattedDuration {
        pub fn new(duration : Duration) -> Self {
            let mut t = duration.as_secs();
            let seconds = t % 60;
            t /= 60;
            let minutes = t % 60;
            t /= 60;
            let hours = t % 24;
            t /= 24;
            let days = if t > 0 { t } else { 0 };
            Self {
                seconds,
                minutes,
                hours,
                days,
            }
        }
    }

    impl Display for FormattedDuration {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            if self.days > 0 {
                write!(f, "{}d {:02}:{:02}:{:02}", self.days, self.hours, self.minutes, self.seconds)
            } else {
                write!(f, "{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
            }
        }
    }
}