//#![deny(missing_docs)]

//! Module level docs

use std::convert::TryInto;

use getset::{CopyGetters, Getters};

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

    /// Number of descriptors doesn't match the size
    #[error("Expected {n_descriptors} descriptors, requiring {descriptor_size}B but buffer size {buffer_size}B")]
    WrongNumberOfDescriptors {
        n_descriptors: u16,
        descriptor_size: usize,
        buffer_size: usize,
    },
}

/// A parsed BUFR message
pub struct Message {
    total_length: u32,
    version: u8,
    section1: Section1,
    section3: Section3,
}

/// Identification Section (section 1) of the BUFR format
pub enum Section1 {
    /// Version 3 for the section 1
    V3(Section1v3),
    /// Version 4 for the section 1
    V4(Section1v4),
}

impl Section1 {
    fn decode(buf: &[u8], version: u8) -> Result<Section1, Error> {
        Ok(match version {
            3 => Section1::V3(Section1v3::decode(&buf[..])?),
            4 => Section1::V4(Section1v4::decode(&buf[..])?),
            _ => unimplemented!(),
        })
    }

    /// Length of the Section 1
    pub fn length(&self) -> usize {
        match self {
            Section1::V3(v) => v.length(),
            Section1::V4(v) => v.length(),
        }
    }

    /// Optional section 2 switch
    pub fn optional_section(&self) -> bool {
        match self {
            Section1::V3(v) => v.optional_section(),
            Section1::V4(v) => v.optional_section(),
        }
    }
}

/// Version 3 variant of the Section 1
#[derive(CopyGetters, Getters)]
pub struct Section1v3 {
    length: usize,
    master_table: u8,
    #[getset(get_copy = "pub")]
    sub_center: u8,
    #[getset(get_copy = "pub")]
    center: u8,
    #[getset(get_copy = "pub")]
    update_version: u8,
    #[getset(get_copy = "pub")]
    optional_section: bool,
    #[getset(get_copy = "pub")]
    data_category: u8,
    #[getset(get_copy = "pub")]
    data_subcategory: u8,
    // In the future change to a table object
    #[getset(get_copy = "pub")]
    master_table_version: u8,
    #[getset(get_copy = "pub")]
    local_table_version: u8,
    #[getset(get_copy = "pub")]
    year: u8,
    #[getset(get_copy = "pub")]
    month: u8,
    #[getset(get_copy = "pub")]
    day: u8,
    #[getset(get_copy = "pub")]
    hour: u8,
    #[getset(get_copy = "pub")]
    minute: u8,
    #[getset(get = "pub")]
    local_use: Vec<u8>,
}

impl Section1v3 {
    fn decode(buf: &[u8]) -> Result<Section1v3, Error> {
        unimplemented!()
    }

    /// Length of the Section 1
    pub fn length(&self) -> usize {
        self.length
    }

    /// Master table for this section
    pub fn master_table(&self) -> u8 {
        self.master_table
    }
}

/// Version 4 variant of the Section 1
#[allow(missing_docs)]
#[derive(CopyGetters, Getters)]
pub struct Section1v4 {
    length: usize,
    #[getset(get_copy = "pub")]
    master_table: u8,
    #[getset(get_copy = "pub")]
    sub_center: u16,
    #[getset(get_copy = "pub")]
    center: u16,
    #[getset(get_copy = "pub")]
    update_version: u8,
    #[getset(get_copy = "pub")]
    optional_section: bool,
    #[getset(get_copy = "pub")]
    data_category: u8,
    #[getset(get_copy = "pub")]
    data_subcategory: u8,
    #[getset(get_copy = "pub")]
    local_subcategory: u8,
    // In the future change to a table object
    #[getset(get_copy = "pub")]
    master_table_version: u8,
    #[getset(get_copy = "pub")]
    local_table_version: u8,
    #[getset(get_copy = "pub")]
    year: u16,
    #[getset(get_copy = "pub")]
    month: u8,
    #[getset(get_copy = "pub")]
    day: u8,
    #[getset(get_copy = "pub")]
    hour: u8,
    #[getset(get_copy = "pub")]
    minute: u8,
    #[getset(get_copy = "pub")]
    second: u8,
    #[getset(get = "pub")]
    local_use: Vec<u8>,
}

impl Section1v4 {
    fn decode(buf: &[u8]) -> Result<Section1v4, Error> {
        let length = (usize::from(buf[0]) << 16) + (usize::from(buf[1]) << 8) + usize::from(buf[2]);
        let master_table: u8 = buf[3];
        let center: u16 = (u16::from(buf[4]) << 8) + u16::from(buf[5]);
        let sub_center: u16 = (u16::from(buf[6]) << 8) + u16::from(buf[7]);
        let update_version: u8 = buf[8];
        let optional_section: bool = match buf[9] {
            64 => true,
            0 => false,
            _ => todo!("error"),
        };
        let data_category: u8 = buf[10];
        let data_subcategory: u8 = buf[11];
        let local_subcategory: u8 = buf[12];
        // In the future change to a table object
        let master_table_version: u8 = buf[13];
        let local_table_version: u8 = buf[14];
        let year: u16 = (u16::from(buf[15]) << 8) + u16::from(buf[16]);
        let month: u8 = buf[17];
        let day: u8 = buf[18];
        let hour: u8 = buf[19];
        let minute: u8 = buf[20];
        let second: u8 = buf[21];
        let local_use = if length > 21 {
            buf[22..length].into()
        } else {
            vec![]
        };

        Ok(Section1v4 {
            length,
            master_table,
            sub_center,
            center,
            update_version,
            optional_section,
            data_category,
            data_subcategory,
            local_subcategory,
            master_table_version,
            local_table_version,
            year,
            month,
            day,
            hour,
            minute,
            second,
            local_use,
        })
    }

    /// Length of the Section 1
    pub fn length(&self) -> usize {
        self.length
    }
}

/// Data description Section (section 3) of the BUFR format
pub struct Section3 {
    length: usize,
    // 4th byte is reserved
    is_observed: bool,
    is_compressed: bool,
    descriptors: Vec<Descriptor>,
}

impl Section3 {
    fn decode(buf: &[u8]) -> Result<Section3, Error> {
        if buf.len() < 8 {
            return Err(Error::MessageTooShort);
        }
        let length = (usize::from(buf[0]) << 16) + (usize::from(buf[1]) << 8) + usize::from(buf[2]);
        if (buf.len() as usize) < length {
            return Err(Error::TruncatedMessage);
        }
        // 4th byte reserved, set to zero
        assert_eq!(buf[3], 0);
        // number of descriptors
        let n_subsets = (u16::from(buf[4]) << 8) + u16::from(buf[5]);
        if length != 7 + 2 * usize::from(n_subsets) {
            return Err(Error::WrongNumberOfDescriptors {
                n_descriptors: n_subsets,
                descriptor_size: usize::from(n_subsets * 2),
                buffer_size: length,
            });
        };

        let (is_observed, is_compressed) = match buf[6] {
            0 => (false, false),
            0b01_000000 => (false, true),
            0b10_000000 => (true, false),
            0b11_000000 => (true, true),
            _ => todo!("error"),
        };

        let mut descriptors = vec![];
        for chunk in buf[8..length].chunks(2) {
            let descriptor = parse_descriptor(chunk.try_into().unwrap());
            descriptors.push(descriptor);
        }

        Ok(Section3 {
            length,
            is_observed,
            is_compressed,
            descriptors,
        })
    }
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

    /// Section 1 of the Message
    pub fn section1(&self) -> &Section1 {
        &self.section1
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
    let total_length: u32 =
        (u32::from(buf[4]) << 16) + (u32::from(buf[5]) << 8) + u32::from(buf[6]);

    if (buf.len() as u32) < total_length {
        return Err(Error::TruncatedMessage);
    }
    // TODO: version [0x04]
    let version: u8 = buf[7];
    match version {
        3 | 4 => (),
        v => return Err(Error::VersionNotSupported(v)),
    };

    let mut offset: usize = 8;

    let section1 = Section1::decode(&buf[offset..], version)?;
    offset += section1.length();

    if section1.optional_section() {
        //Section2::decode(&buf[offset..]) -> n_consumed
        unimplemented!()
    }

    let section3 = Section3::decode(&buf[offset..], version)?;
    offset += section3.length();

    Ok(Message {
        total_length,
        version,
        section1,
        section3,
    })
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
    let f = buf[0] & 0b11000000 >> 6;
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
