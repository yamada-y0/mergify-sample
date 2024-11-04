fn main() {
    println!("{}", get_message());
}

fn get_message() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use crate::get_message;

    #[test]
    fn test_get_message() {
        assert_eq!(get_message(), "Hello world!");
    }
}
