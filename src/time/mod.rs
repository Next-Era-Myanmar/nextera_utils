use chrono::{Local, NaiveDateTime, Utc};

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
        Local::now().naive_local()
    }

    /// ### Get supported timezones.
    ///
    /// ### Example
    ///
    /// ```
    /// use nextera_utils::time::Time;
    /// println!("{}" ,Time::get_supported_timezones().len().to_string());
    /// ```
    pub fn get_supported_timezones() -> Vec<String> {
        vec![
            "UTC-12:00".to_string(),
            "UTC-11:00".to_string(),
            "UTC-10:00".to_string(),
            "UTC-09:30".to_string(),
            "UTC-09:00".to_string(),
            "UTC-08:00".to_string(),
            "UTC-07:00".to_string(),
            "UTC-06:00".to_string(),
            "UTC-05:00".to_string(),
            "UTC-04:30".to_string(),
            "UTC-04:00".to_string(),
            "UTC-03:30".to_string(),
            "UTC-03:00".to_string(),
            "UTC-02:00".to_string(),
            "UTC-01:00".to_string(),
            "UTC+00:00".to_string(),
            "UTC+01:00".to_string(),
            "UTC+02:00".to_string(),
            "UTC+03:00".to_string(),
            "UTC+03:30".to_string(),
            "UTC+04:00".to_string(),
            "UTC+04:30".to_string(),
            "UTC+05:00".to_string(),
            "UTC+05:30".to_string(),
            "UTC+05:45".to_string(),
            "UTC+06:00".to_string(),
            "UTC+06:30".to_string(),
            "UTC+07:00".to_string(),
            "UTC+08:00".to_string(),
            "UTC+08:30".to_string(),
            "UTC+08:45".to_string(),
            "UTC+09:00".to_string(),
            "UTC+09:30".to_string(),
            "UTC+10:00".to_string(),
            "UTC+10:30".to_string(),
            "UTC+11:00".to_string(),
            "UTC+11:30".to_string(),
            "UTC+12:00".to_string(),
            "UTC+12:45".to_string(),
            "UTC+13:00".to_string(),
            "UTC+14:00".to_string(),
        ]
    }

    /// ### Get Valid timezone.
    ///
    /// ### Example
    /// if valid you will get the same timezone, if not you will get UTC+00:00
    /// ```
    /// use nextera_utils::time::Time;
    /// println!("{}" ,Time::validate_timezone("UTC+06:00").to_string());
    /// ```
    pub fn validate_timezone(input_offset: &str) -> String {
        let supported_offsets = Time::get_supported_timezones();
        if supported_offsets.contains(&input_offset.to_string()) {
            input_offset.to_string()
        } else {
            "UTC+00:00".to_string()
        }
    }

    /// ### Convert utc datetime to other datetime with destination timezone.
    /// if success you will get the converted datetime, if not you will get the original datetime
    /// ### Example
    ///
    /// ```
    /// use nextera_utils::time::Time;
    /// let utc = Time::get_utc();
    /// println!("{}" ,Time::convert_timezone(utc,"UTC+06:00".to_string()).to_string());
    /// ```
    pub fn convert_timezone(
        utc_datetime: NaiveDateTime,
        destination_timezone: String,
    ) -> NaiveDateTime {
        // Parse the destination timezone offset
        let offset_parts: Vec<&str> = destination_timezone.split(':').collect();
        if offset_parts.len() != 2 {
            return utc_datetime; // Invalid timezone format, return original datetime
        }

        if offset_parts[0].len() != 6 {
            return utc_datetime; // Invalid timezone format, return original datetime
        }

        let offset_sign = offset_parts[0].chars().nth(3);

        if offset_sign != Some('-') && offset_sign != Some('+') {
            return utc_datetime; // Invalid timezone format, return original datetime
        }

        let hours_offset: i64 = offset_parts[0][4..6].parse().unwrap_or(0);
        let minutes_offset: i64 = offset_parts[1].parse().unwrap_or(0);

        // Calculate the total offset in minutes
        let total_offset_minutes = (hours_offset * 60) + minutes_offset;

        // Convert UTC datetime to destination timezone
        match offset_sign {
            Some('-') => {
                // UTC - offset
                utc_datetime - chrono::Duration::minutes(total_offset_minutes)
            }
            Some('+') => {
                // UTC + offset
                utc_datetime + chrono::Duration::minutes(total_offset_minutes)
            }
            _ => utc_datetime, // Invalid sign, return original datetime
        }
    }
}
