#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_tokens() {
        let mut wallet = Wallet::new();
        let user1 = "user1".to_string();
        let user2 = "user2".to_string();

        wallet.receive_tokens(user1.clone(), 100);
        assert_eq!(wallet.get_balance(&user1), 100);

        assert!(wallet.send_tokens(user1.clone(), user2.clone(), 50).is_ok());
        assert_eq!(wallet.get_balance(&user1), 50);
        assert_eq!(wallet.get_balance(&user2), 50);
    }

    #[test]
    fn test_insufficient_funds() {
        let mut wallet = Wallet::new();
        let user1 = "user1".to_string();
        let user2 = "user2".to_string();

        wallet.receive_tokens(user1.clone(), 30);
        assert!(wallet.send_tokens(user1.clone(), user2.clone(), 50).is_err());
    }
}
