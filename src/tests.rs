#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_tokens() {
        let mut token = init();
        set_balance("alice".to_string(), 100);

        assert!(send("bob".to_string(), 50));
        assert_eq!(get_balance("alice".to_string()), 50);
        assert_eq!(get_balance("bob".to_string()), 50);

        assert!(!send("bob".to_string(), 100));
    }

    #[test]
    fn test_receive_tokens() {
        let mut token = init();
        set_balance("alice".to_string(), 100);

        assert!(send("bob".to_string(), 50));
        assert_eq!(get_balance("bob".to_string()), 50);
    }
}
