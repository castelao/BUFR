#![deny(missing_docs)]

//! Module level docs

/// Struct for a triad
struct Triad {
    f: u8,
    x: u8,
    y: u8,
}

/// Parses a triad from a BUFR message.
///
/// A Triad has 3 fields:
/// - f (2 bits):
/// - x (2 bits):
/// - y (8 bits):
fn parse_triad(buf: [u8; 2]) -> Triad {
    let f = (buf[0] & 0b11000000) >> 6;
    let x = buf[0] & 0b00111111;
    let y = buf[1];

    Triad { f, x, y }
}

impl Triad {
    fn encode_triad(&self) -> [u8; 2] {
        let mut buf = [0u8; 2];
        buf[0] = (self.f << 6) + self.x;
        buf[1] = self.y;
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::parse_triad;

    #[test]
    fn encode_triad_1() {
        let test_data = [0, 0];
        let triad = parse_triad(test_data);
        let result = triad.encode_triad();

        assert_eq!(test_data, result);
    }
    #[test]
    fn parse_triad_1() {
        let test_data = [0, 0];
        let triad = parse_triad(test_data);

        assert_eq!(triad.f, 0);
        assert_eq!(triad.x, 0);
        assert_eq!(triad.y, 0);
    }

    #[test]
    fn parse_triad_2() {
        let test_data = [0, 1];
        let triad = parse_triad(test_data);

        assert_eq!(triad.f, 0);
        assert_eq!(triad.x, 0);
        assert_eq!(triad.y, 1);
    }

    #[test]
    fn parse_triad_3() {
        let test_data = [0b10101010, 1];
        let triad = parse_triad(test_data);

        assert_eq!(triad.f, 2);
        assert_eq!(triad.x, 42);
        assert_eq!(triad.y, 1);
    }
}
