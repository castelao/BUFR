#![deny(missing_docs)]

//! Module level docs

/// Struct for a descriptor
struct Descriptor {
    f: u8,
    x: u8,
    y: u8,
}

/// Parses a descriptor from a BUFR message.
///
/// A Descriptor has 3 fields:
/// - f (2 bits):
/// - x (2 bits):
/// - y (8 bits):
fn parse_descriptor(buf: [u8; 2]) -> Descriptor {
    let f = (buf[0] & 0b11000000) >> 6;
    let x = buf[0] & 0b00111111;
    let y = buf[1];

    Descriptor { f, x, y }
}

impl Descriptor {
    fn encode_descriptor(&self) -> [u8; 2] {
        let mut buf = [0u8; 2];
        buf[0] = (self.f << 6) + self.x;
        buf[1] = self.y;
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::parse_descriptor;

    #[test]
    fn encode_descriptor_1() {
        let test_data = [0, 0];
        let descriptor = parse_descriptor(test_data);
        let result = descriptor.encode_descriptor();

        assert_eq!(test_data, result);
    }
    #[test]
    fn parse_descriptor_1() {
        let test_data = [0, 0];
        let descriptor = parse_descriptor(test_data);

        assert_eq!(descriptor.f, 0);
        assert_eq!(descriptor.x, 0);
        assert_eq!(descriptor.y, 0);
    }

    #[test]
    fn parse_descriptor_2() {
        let test_data = [0, 1];
        let descriptor = parse_descriptor(test_data);

        assert_eq!(descriptor.f, 0);
        assert_eq!(descriptor.x, 0);
        assert_eq!(descriptor.y, 1);
    }

    #[test]
    fn parse_descriptor_3() {
        let test_data = [0b10101010, 1];
        let descriptor = parse_descriptor(test_data);

        assert_eq!(descriptor.f, 2);
        assert_eq!(descriptor.x, 42);
        assert_eq!(descriptor.y, 1);
    }
}
