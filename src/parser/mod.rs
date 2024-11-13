pub trait OptionParserExtensions {
    fn to_i32(self) -> Option<i32>;
}

impl OptionParserExtensions for Option<&str> {
    fn to_i32(self) -> Option<i32> {
        match self {
            None => None,
            Some(s) => match s.to_string().parse::<i32>() {
                Ok(r) => Some(r),
                Err(_) => None,
            },
        }
    }
}
