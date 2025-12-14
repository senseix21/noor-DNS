#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Tls,  // DoT - port 853
}

impl Protocol {
    pub fn parse(input: &str) -> Option<Protocol> {
        if input.eq_ignore_ascii_case("TCP") {
            Some(Protocol::Tcp)
        } else if input.eq_ignore_ascii_case("UDP") {
            Some(Protocol::Udp)
        } else if input.eq_ignore_ascii_case("TLS") {
            Some(Protocol::Tls)
        } else {
            None
        }
    }

    pub fn number(self) -> u32 {
        match self {
            Self::Tcp => 6,
            Self::Udp => 17,
            Self::Tls => 6,  // TLS runs over TCP
        }
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Tcp => write!(f, "TCP"),
            Self::Udp => write!(f, "UDP"),
            Self::Tls => write!(f, "TLS"),
        }
    }
}
