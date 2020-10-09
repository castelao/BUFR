#![deny(missing_docs)]

//! Module level docs

/// Possible errors when parsing BUFR messages
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Message too short to even contain the total size
    #[error("BUFR message is too short, less than 8 bytes")]
    MessageTooShort,

    /// Wrong magic number
    #[error("BUFR message should start with BUFR")]
    MagicNumber,

    /// Message shorter than specified on section 0
    #[error("Message shorter than expected, is it truncated?")]
    TruncatedMessage,

    /// BUFR version not supported
    #[error("BUFR version {0} not supported")]
    VersionNotSupported(u8),
}

/// A parsed BUFR message
pub struct Message {
    total_length: u32,
    version: u8
}

impl Message {
    /// Total length of the message including all sections
    pub fn total_length(&self) -> u32 {
        self.total_length
    }

    /// BUFR protocol version
    pub fn version(&self) -> u8 {
        self.version
    }
}

/// Decode a BUFR message
pub fn decode(buf: &[u8]) -> Result<Message, Error> {
    // section 0
    if buf.len() < 8 {
        return Err(Error::MessageTooShort);
    }

    // TODO: verify BUFR start [0x42 0x55 0x46 0x52]
    if &buf[0..4] != b"BUFR" {
        return Err(Error::MagicNumber);
    }
    // TODO: total length [0x0 0x0 0x92]
    let total_length: u32 = (u32::from(buf[4]) << 16) + (u32::from(buf[5]) << 8) + u32::from(buf[6]);

    if (buf.len() as u32) < total_length {
        return Err(Error::TruncatedMessage)
    }
    // TODO: version [0x04]
    let version: u8 = buf[7];
    match version {
        3 | 4 => (),
        v => return Err(Error::VersionNotSupported(v))
    };
    Ok(Message {total_length, version})
}

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
