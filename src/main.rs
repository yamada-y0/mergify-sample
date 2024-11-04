fn main() {
    println!("{}", get_message());
}

fn get_message() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use crate::get_message;

    #[test]
    fn test_get_message() {
        let five_min = time::Duration::from_secs(300);
        thread::sleep(five_min);
        assert_eq!(get_message(), "Hello, world!");
    }
}
