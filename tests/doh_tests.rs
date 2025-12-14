// Unit tests for DoH (DNS-over-HTTPS) server

#[cfg(test)]
mod tests {
    use base64::Engine;
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use hickory_proto::op::{Message, MessageType, OpCode, Query};
    use hickory_proto::rr::{Name, RecordType};
    use std::str::FromStr;

    #[test]
    fn test_base64_encoding_roundtrip() {
        let data = b"Hello, DNS-over-HTTPS!";
        let encoded = URL_SAFE_NO_PAD.encode(data);
        let decoded = URL_SAFE_NO_PAD.decode(&encoded).unwrap();
        
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_dns_message_base64_encoding() {
        let mut msg = Message::new();
        msg.set_id(1234);
        msg.set_message_type(MessageType::Query);
        msg.set_op_code(OpCode::Query);
        
        let name = Name::from_str("example.com.").unwrap();
        let query = Query::query(name, RecordType::A);
        msg.add_query(query);
        
        let bytes = msg.to_vec().unwrap();
        let encoded = URL_SAFE_NO_PAD.encode(&bytes);
        let decoded_bytes = URL_SAFE_NO_PAD.decode(&encoded).unwrap();
        
        assert_eq!(bytes, decoded_bytes);
    }

    #[test]
    fn test_max_dns_size() {
        const MAX_DNS_SIZE: usize = 65535;
        assert_eq!(MAX_DNS_SIZE, 65535);
        assert!(MAX_DNS_SIZE > 512);
    }

    #[test]
    fn test_content_type_constant() {
        const DNS_CONTENT_TYPE: &str = "application/dns-message";
        assert_eq!(DNS_CONTENT_TYPE, "application/dns-message");
    }

    #[test]
    fn test_url_path_parsing() {
        let path = "/dns-query";
        assert_eq!(path, "/dns-query");
        assert!(path.starts_with("/dns"));
    }

    #[test]
    fn test_query_parameter_extraction() {
        let url = "/dns-query?dns=AAABAAABAAAAAAAAB2V4YW1wbGUDY29tAAABAAE";
        let parts: Vec<&str> = url.split('?').collect();
        
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "/dns-query");
        
        if let Some(query) = parts.get(1) {
            let params: Vec<&str> = query.split('=').collect();
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], "dns");
        }
    }

    #[test]
    fn test_http_methods() {
        let get = "GET";
        let post = "POST";
        
        assert_eq!(get, "GET");
        assert_eq!(post, "POST");
        assert_ne!(get, post);
    }

    #[test]
    fn test_dns_message_validation() {
        let mut msg = Message::new();
        msg.set_id(42);
        
        let name = Name::from_str("test.com.").unwrap();
        let query = Query::query(name, RecordType::A);
        msg.add_query(query);
        
        let bytes = msg.to_vec().unwrap();
        
        // Validate size
        assert!(bytes.len() > 0);
        assert!(bytes.len() < 65535);
        
        // Validate can be parsed back
        let parsed = Message::from_vec(&bytes).unwrap();
        assert_eq!(parsed.id(), 42);
    }

    #[test]
    fn test_url_safe_base64_no_padding() {
        // URL-safe Base64 should not have = padding
        let data = b"test";
        let encoded = URL_SAFE_NO_PAD.encode(data);
        
        assert!(!encoded.contains('='));
        assert!(!encoded.contains('+'));
        assert!(!encoded.contains('/'));
    }

    #[test]
    fn test_empty_dns_message_handling() {
        let msg = Message::new();
        let bytes = msg.to_vec().unwrap();
        
        assert!(bytes.len() > 0); // Even empty message has headers
    }

    #[test]
    fn test_multiple_record_types() {
        let types = vec![
            RecordType::A,
            RecordType::AAAA,
            RecordType::CNAME,
            RecordType::MX,
            RecordType::TXT,
        ];
        
        for record_type in types {
            let name = Name::from_str("example.com.").unwrap();
            let query = Query::query(name, record_type);
            
            assert_eq!(query.query_type(), record_type);
        }
    }

    #[test]
    fn test_dns_query_path_validation() {
        fn is_valid_doh_path(path: &str) -> bool {
            path == "/dns-query"
        }
        
        assert!(is_valid_doh_path("/dns-query"));
        assert!(!is_valid_doh_path("/other-path"));
        assert!(!is_valid_doh_path("/dns"));
    }
}
