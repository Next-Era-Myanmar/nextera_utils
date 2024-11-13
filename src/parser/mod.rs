pub trait OptionParserExtensions {
    fn to_i32(&self) -> Option<i32>;
}

impl OptionParserExtensions for Option<&str> {
    fn to_i32(&self) -> Option<i32> {
        if self != None {
            match self?.to_string().parse::<i32>() {
                Ok(d) => Some(d),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}