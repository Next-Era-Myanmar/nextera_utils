//! ## Parser Extensions for Next Era.
//!
//! Next Era Solution generic parser are implemented in these modules.
//!
pub trait OptionParserExtensions {
    fn to_opt_i32(self) -> Option<i32>;
}

impl OptionParserExtensions for Option<&str> {
    /// ### Parsed form optional immutable str to option i32.
    /// #### If value contain None or Failed, you will get None. If success you get Option<i32>.
    ///
    /// ### Example
    ///
    /// ```
    /// use nextera_utils::parser::OptionParserExtensions;
    /// let x:Option<&str> = Some("200");
    /// let y:Option<i32> = Some(200);
    /// let result = x.to_opt_i32();
    /// assert_eq!(result, y);
    /// ```
    fn to_opt_i32(self) -> Option<i32> {
        match self {
            None => None,
            Some(s) => match s.to_string().parse::<i32>() {
                Ok(r) => Some(r),
                Err(_) => None,
            },
        }
    }
}

pub trait ParserExtensions {
    fn to_opt_u16(self) -> Option<u16>;
}

impl ParserExtensions for String {
    /// ### Parsed form String to u16.
    /// #### If Failed, you will get None. If success you get Option<u16>.
    ///
    /// ### Example
    ///
    /// ```
    /// use nextera_utils::parser::ParserExtensions;
    /// let x:String = String::from("200");
    /// let result = x.to_opt_u16();
    ///
    /// let y:Option<u16> = Some(200);
    /// assert_eq!(result, y);
    /// ```
    fn to_opt_u16(self) -> Option<u16> {
        match self.parse::<u16>() {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }
}
