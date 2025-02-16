use chrono::{NaiveDateTime, Utc};

pub struct Time;

impl Time {
    /// ### Get current utc time in naive time.
    ///
    /// ### Example
    ///
    /// ```
    /// use nextera_utils::time::Time;
    /// println!("{}" ,Time::get_utc().to_string());
    /// ```
    pub fn get_utc() -> NaiveDateTime {
        // Get the current UTC time
        let utc_time = Utc::now();
        // Convert it to a naive UTC datetime
        utc_time.naive_utc()
    }

    /// ### Get current naive time.
    ///
    /// ### Example
    ///
    /// ```
    /// use nextera_utils::time::Time;
    /// println!("{}" ,Time::get_now().to_string());
    /// ```
    pub fn get_now() -> NaiveDateTime {
        // Get the Naive Datetime
        NaiveDateTime::now()
    }
}