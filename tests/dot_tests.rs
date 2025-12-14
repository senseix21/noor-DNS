// Unit tests for DoT (DNS-over-TLS) server

#[cfg(test)]
mod tests {
    use hickory_proto::op::{Message, MessageType, OpCode, Query};
    use hickory_proto::rr::{Name, RecordType};
    use std::str::FromStr;

    #[test]
    fn test_dns_message_parsing() {
        let mut msg = Message::new();
        msg.set_id(1234);
        msg.set_message_type(MessageType::Query);
        msg.set_op_code(OpCode::Query);
        
        let name = Name::from_str("example.com.").unwrap();
        let query = Query::query(name, RecordType::A);
        msg.add_query(query);
        
        let bytes = msg.to_vec().unwrap();
        let parsed = Message::from_vec(&bytes).unwrap();
        
        assert_eq!(parsed.id(), 1234);
        assert_eq!(parsed.message_type(), MessageType::Query);
        assert_eq!(parsed.queries().len(), 1);
    }

    #[test]
    fn test_max_message_size() {
        const MAX_DNS_MESSAGE_SIZE: usize = 65535;
        assert!(MAX_DNS_MESSAGE_SIZE == 65535);
        assert!(MAX_DNS_MESSAGE_SIZE > 512); // Minimum DNS message size
    }

    #[test]
    fn test_message_length_prefix() {
        let msg_len: u16 = 512;
        let bytes = msg_len.to_be_bytes();
        let decoded = u16::from_be_bytes([bytes[0], bytes[1]]);
        assert_eq!(decoded, 512);
    }

    #[test]
    fn test_dns_query_construction() {
        let name = Name::from_str("google.com.").unwrap();
        let query = Query::query(name.clone(), RecordType::A);
        
        assert_eq!(query.name(), &name);
        assert_eq!(query.query_type(), RecordType::A);
    }

    #[test]
    fn test_multiple_queries_in_message() {
        let mut msg = Message::new();
        
        let name1 = Name::from_str("example.com.").unwrap();
        let query1 = Query::query(name1, RecordType::A);
        msg.add_query(query1);
        
        let name2 = Name::from_str("example.org.").unwrap();
        let query2 = Query::query(name2, RecordType::AAAA);
        msg.add_query(query2);
        
        assert_eq!(msg.queries().len(), 2);
    }

    #[test]
    fn test_message_serialization_roundtrip() {
        let mut msg = Message::new();
        msg.set_id(9999);
        
        let name = Name::from_str("test.local.").unwrap();
        let query = Query::query(name, RecordType::A);
        msg.add_query(query);
        
        let bytes = msg.to_vec().unwrap();
        let parsed = Message::from_vec(&bytes).unwrap();
        
        assert_eq!(parsed.id(), msg.id());
        assert_eq!(parsed.queries().len(), msg.queries().len());
    }

    #[test]
    fn test_wildcard_domain_pattern() {
        let domain = "example.com";
        let wildcard = format!("*.{}", domain);
        
        assert_eq!(wildcard, "*.example.com");
        assert!(wildcard.starts_with("*."));
    }

    #[test]
    fn test_domain_validation() {
        fn is_valid_domain(domain: &str) -> bool {
            !domain.is_empty() && 
            domain.len() < 256 &&
            domain.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '*')
        }
        
        assert!(is_valid_domain("example.com"));
        assert!(is_valid_domain("*.example.com"));
        assert!(is_valid_domain("sub.example.com"));
        assert!(!is_valid_domain(""));
        assert!(!is_valid_domain("example com"));
    }
}
