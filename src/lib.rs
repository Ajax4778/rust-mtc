mod mtc {
    extern crate chrono;
    use self::chrono::prelude::*;

    const UT_JULIAN_UNIX: f64 = 2_440_587.5;
    const TT_JULIAN_2000: f64 = 2_451_545.0;
    const UT_TT_SECONDS_OFFSET: f64 = 69.184;

    const SECONDS_IN_DAY: f64 = 86_400.0;

    pub fn now() -> String {
        let ut_julian = UT_JULIAN_UNIX + days_since_unix(UTC::now());
        let days_since_2000 = ut_julian - TT_JULIAN_2000 + (UT_TT_SECONDS_OFFSET / SECONDS_IN_DAY);

        format(mars_sol_date(days_since_2000))
    }

    pub fn at(datetime: DateTime<UTC>) -> String {
        let ut_julian = UT_JULIAN_UNIX + days_since_unix(datetime);
        let days_since_2000 = ut_julian - TT_JULIAN_2000 + (UT_TT_SECONDS_OFFSET / SECONDS_IN_DAY);

        format(mars_sol_date(days_since_2000))
    }

    fn days_since_unix(datetime: DateTime<UTC>) -> f64 {
        let unix = UTC.ymd(1970, 1, 1).and_hms(0, 0, 0);
        let mills = datetime.signed_duration_since(unix).num_milliseconds() as f64;
        mills / (SECONDS_IN_DAY * 1000_f64)
    }

    fn mars_sol_date(days_since_2000: f64) -> f64 {
        ((days_since_2000 - 4.5) / 1.027491252) + 44_796_f64 - 0.00096
    }

    fn format(mars_date: f64) -> String {
        let date = mars_date.ceil();

        let hours = (24_f64 * mars_date) % 24_f64;
        let h = hours.floor();

        let minutes = (hours - h) * 60.0;
        let m = minutes.floor();

        let seconds = (minutes - m) * 60.0;
        let s = seconds.floor();
        format!("{:02}:{:02}:{:02} on Mars Sol Date {}", h, m, s, date)
    }
}
