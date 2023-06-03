//#![deny(missing_docs)]

//! BUFR binary data format
//!
//! Module level docs

mod error;
mod tables;

use std::convert::TryInto;
use std::fmt;

use byteorder::{BigEndian, WriteBytesExt};
use derive_builder::Builder;
use getset::{CopyGetters, Getters};

use crate::error::Error;
use crate::tables::TABLE_F3;

/// A parsed BUFR message
pub struct Message {
    total_length: u32,
    version: u8,
    section1: Section1,
    section3: Section3,
    section4: Section4,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BUFR version {}", self.version)?;
        writeln!(f, "     total length: {}", self.total_length)?;
        writeln!(f, "\n")?;
        writeln!(f, "{}", self.section1)?;
        writeln!(f, "\n")?;
        writeln!(f, "{}", self.section3)
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BUFR version {}", self.version)?;
        writeln!(f, "     total length: {}", self.total_length)?;
        writeln!(f, "\n")?;
        writeln!(f, "{}", self.section1)?;
        writeln!(f, "\n")?;
        writeln!(f, "{}", self.section3)?;
        writeln!(f, "\n")?;
        writeln!(f, "{:?}", self.section4)
    }
}

/// Identification Section (section 1) of the BUFR format
#[derive(Debug)]
pub enum Section1 {
    /// Version 3 for the section 1
    V3(Section1v3),
    /// Version 4 for the section 1
    V4(Section1v4),
}

impl fmt::Display for Section1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Section1::V3(_) => writeln!(f, "{:?}", self),
            Section1::V4(s) => writeln!(f, "{}", s),
        }
    }
}

impl Section1 {
    fn decode(buf: &[u8], version: u8) -> Result<Section1, Error> {
        Ok(match version {
            3 => Section1::V3(Section1v3::decode(buf)?),
            4 => Section1::V4(Section1v4::decode(buf)?),
            _ => unimplemented!(),
        })
    }

    fn encode<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        match self {
            Section1::V4(v) => v.encode(wtr),
            _ => unimplemented!(),
        }
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
#[derive(CopyGetters, Getters, Debug)]
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
    #[allow(unused_variables)]
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
#[derive(CopyGetters, Getters, Debug)]
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
            center,
            sub_center,
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

    pub fn encode<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        wtr.write_u24::<BigEndian>(self.length.try_into().unwrap())?;
        wtr.write_u8(self.master_table)?;
        wtr.write_u16::<BigEndian>(self.center)?;
        wtr.write_u16::<BigEndian>(self.sub_center)?;
        wtr.write_u8(self.update_version)?;
        if self.optional_section {
            wtr.write_u8(64)?;
        } else {
            wtr.write_u8(0)?;
        };
        wtr.write_u8(self.data_category)?;
        wtr.write_u8(self.data_subcategory)?;
        wtr.write_u8(self.local_subcategory)?;
        wtr.write_u8(self.master_table_version)?;
        wtr.write_u8(self.local_table_version)?;
        wtr.write_u16::<BigEndian>(self.year)?;
        wtr.write_u8(self.month)?;
        wtr.write_u8(self.day)?;
        wtr.write_u8(self.hour)?;
        wtr.write_u8(self.minute)?;
        wtr.write_u8(self.second)?;
        if !self.local_use.is_empty() {
            wtr.write_all(&self.local_use)?;
        };

        // FIXME: does wtr have a counter?
        Ok(8 + 22 + self.local_use.len())
    }

    /// Length of the Section 1
    pub fn length(&self) -> usize {
        self.length
    }
}

impl fmt::Display for Section1v4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Section 1 (v4)")?;
        writeln!(f, "    Section length: {}", self.length())?;
        writeln!(
            f,
            "    Master table: {} (version: {})",
            self.master_table(),
            self.master_table_version()
        )?;
        writeln!(f, "    Sub-center: {}", self.sub_center())?;
        writeln!(f, "    Center: {}", self.center())?;
        writeln!(f, "    Update version: {}", self.update_version())?;
        writeln!(f, "    Optional section: {}", self.optional_section())?;
        writeln!(f, "    Data category: {}", self.data_category())?;
        writeln!(f, "    Data sub-category: {}", self.data_subcategory())?;
        writeln!(f, "    Local sub-category: {}", self.local_subcategory())?;
        writeln!(f, "    Local table version: {}", self.local_table_version())?;
        writeln!(
            f,
            "    Time: {}-{}-{}T{}:{}:{}",
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second()
        )?;
        writeln!(f, "    Local use: {:x?}", self.local_use())
    }
}

#[cfg(test)]
mod test_section1 {
    use super::Section1v4;

    #[test]
    // A test case from a Spray profile
    fn v4_encode_spray_ph() -> Result<(), Box<dyn std::error::Error>> {
        let section = Section1v4 {
            length: 22,
            master_table: 0,
            // Figure out center
            center: 65535,
            // Figure out sub center code
            sub_center: 65535,
            update_version: 0,
            optional_section: false,
            // Confirm category and subcategory
            data_category: 31,
            data_subcategory: 255,
            local_subcategory: 255,
            master_table_version: 255,
            local_table_version: 255,
            year: 2020,
            month: 10,
            day: 6,
            hour: 19,
            minute: 24,
            second: 0,
            local_use: vec![],
        };
        let mut buf = vec![];
        section.encode(&mut buf)?;
        let ans = vec![
            0, 0, 22, 0, 255, 255, 255, 255, 0, 0, 31, 255, 255, 255, 255, 7, 228, 10, 6, 19, 24, 0,
        ];
        assert_eq!(buf, ans);

        Ok(())
    }
}

/// Data description Section (section 3) of the BUFR format
#[derive(Builder, Debug)]
pub struct Section3 {
    #[builder(setter(skip = true), default = "self.default_length()?")]
    length: usize,
    // 4th byte is reserved
    #[builder(default = "1")]
    n_subsets: u16,
    is_observed: bool,
    #[builder(default = "false")]
    is_compressed: bool,
    descriptors: Vec<Descriptor>,
}

impl fmt::Display for Section3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Section 3")?;
        writeln!(f, "length: {:?}", self.length())?;
        writeln!(f, "is observed: {:?}", self.is_observed())?;
        writeln!(f, "is compressed: {:?}", self.is_compressed())?;

        let mut ident = String::new();
        ident.push_str("    ");

        // let mut n = 0;
        let mut iter = self.descriptors.iter();
        while let Some(d) = iter.next() {
            if d.f == 1 {
                if let Some(dd) = iter.next() {
                    writeln!(f, "{}{:?} + {:?}", ident, d, dd)?;
                }
                // n = if d.y == 0 { d.x + 1 } else { d.x };
                ident.push_str("    ");
            } else if d.f == 3 {
                writeln!(f, "{}{:?}", ident, d)?;

                let child = TABLE_F3
                    .get(&(d.x, d.y))
                    .expect("Failed to get item from F3 table");
                for c in child.iter() {
                    writeln!(f, "{} |_ {}", ident, c)?;
                }
            } else {
                writeln!(f, "{}{:?}", ident, d)?;
            };
        }
        Ok(())
    }
}

impl Section3 {
    fn decode(buf: &[u8]) -> Result<Section3, Error> {
        if buf.len() < 8 {
            return Err(Error::MessageTooShort);
        }
        let length = (usize::from(buf[0]) << 16) + (usize::from(buf[1]) << 8) + usize::from(buf[2]);
        if buf.len() < length {
            return Err(Error::TruncatedMessage);
        }
        // 4th byte reserved, set to zero
        assert_eq!(buf[3], 0);
        // number of descriptors
        let n_subsets = (u16::from(buf[4]) << 8) + u16::from(buf[5]);
        /*
        if length != 7 + 2 * usize::from(n_subsets) {
            return Err(Error::WrongNumberOfDescriptors {
                n_descriptors: n_subsets,
                descriptor_size: usize::from(n_subsets * 2),
                buffer_size: length,
            });
        };
        */

        let (is_observed, is_compressed) = match buf[6] {
            0 => (false, false),
            0b01_000000 => (false, true),
            0b10_000000 => (true, false),
            0b11_000000 => (true, true),
            _ => todo!("error"),
        };

        let mut descriptors = vec![];
        if (length - 7) % 2 != 0 {
            return Err(Error::InvalidSection3Length(length));
        }
        for chunk in buf[7..length].chunks(2) {
            let descriptor = parse_descriptor(chunk.try_into().unwrap());
            descriptors.push(descriptor);
        }

        Ok(Section3 {
            length,
            n_subsets,
            is_observed,
            is_compressed,
            descriptors,
        })
    }

    pub fn encode<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        wtr.write_u24::<BigEndian>(self.length.try_into().unwrap())?;
        wtr.write_u8(0)?;
        wtr.write_u16::<BigEndian>(self.n_subsets)?;
        wtr.write_u8(match (self.is_observed, self.is_compressed) {
            (false, false) => 0,
            (false, true) => 0b01_000000,
            (true, false) => 0b10_000000,
            (true, true) => 0b11_000000,
        })?;
        let mut n: usize = 0;
        for fxy in &self.descriptors {
            n += fxy.encode(wtr)?;
        }
        Ok(7 + n)
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_observed(&self) -> bool {
        self.is_observed
    }

    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }

    pub fn descriptors(&self) -> Vec<Descriptor> {
        self.descriptors.clone()
    }
}

#[cfg(test)]
mod test_section3 {
    use super::{Descriptor, Section3};

    #[test]
    // A test case from a Spray profile
    fn encode_spray_ph() -> Result<(), Box<dyn std::error::Error>> {
        let descriptor = Descriptor { f: 3, x: 15, y: 12 };
        let section = Section3 {
            length: 9,
            n_subsets: 1,
            is_observed: true,
            is_compressed: false,
            descriptors: vec![descriptor],
        };
        let mut buf = vec![];
        section.encode(&mut buf)?;

        Ok(())
    }
}

impl Section3Builder {
    fn default_length(&self) -> Result<usize, Section3BuilderError> {
        match self.descriptors {
            Some(ref x) => Ok(7 + 2 * x.len()),
            _ => Err(Section3BuilderError::from(
                derive_builder::UninitializedFieldError::new("descriptors"),
            )),
        }
    }
}

#[cfg(test)]
mod test_section3_builder {
    use super::{Descriptor, Section3Builder};

    #[test]
    fn build() {
        let section = Section3Builder::default()
            .is_observed(true)
            .descriptors(vec![Descriptor { f: 3, x: 15, y: 12 }])
            .build()
            .unwrap();

        assert_eq!(section.length(), 9);
    }
}

#[derive(Debug)]
pub struct Section4 {
    length: usize,
    data: Vec<u8>,
}

impl Section4 {
    pub fn length(&self) -> usize {
        self.length
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }

    fn decode(buf: &[u8]) -> Result<Section4, Error> {
        if buf.len() < 3 {
            return Err(Error::MessageTooShort);
        }
        let length = (usize::from(buf[0]) << 16) + (usize::from(buf[1]) << 8) + usize::from(buf[2]);
        if buf.len() < length {
            return Err(Error::TruncatedMessage);
        }
        // 4th byte reserved, set to zero
        assert_eq!(buf[3], 0);
        let data = buf[4..length].into();

        Ok(Section4 { length, data })
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

    /// Section 3 of the Message
    pub fn section3(&self) -> &Section3 {
        &self.section3
    }

    /// Section 4 of the Message
    pub fn section4(&self) -> &Section4 {
        &self.section4
    }

    pub fn encode<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        wtr.write_all(b"BUFR")?;
        wtr.write_u24::<BigEndian>(self.total_length)?;

        wtr.write_u8(self.version)?;

        self.section1.encode(wtr)?;

        // FIXME proper size
        Ok(self.total_length as usize)
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

    let section3 = Section3::decode(&buf[offset..])?;
    offset += section3.length();

    let section4 = Section4::decode(&buf[offset..])?;
    offset += section4.length();

    if &buf[offset..(offset + 4)] != b"7777" {
        return Err(Error::EndSection);
    }

    Ok(Message {
        total_length,
        version,
        section1,
        section3,
        section4,
    })
}

/// Struct for a descriptor
#[derive(Debug, Clone, PartialEq)]
pub struct Descriptor {
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
    pub fn encode<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        wtr.write_u8((self.f << 6) + self.x)?;
        wtr.write_u8(self.y)?;
        Ok(2)
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_descriptor, BufferReader, Descriptor};

    #[test]
    fn encode_descriptor_1() {
        let test_data = [0, 0];
        let descriptor = parse_descriptor(test_data);
        let mut result = vec![];
        let n = descriptor.encode(&mut result).unwrap();

        assert_eq!(test_data, result[..]);
        assert_eq!(n, 2);
    }

    #[test]
    fn encode_descriptor_2() {
        let test_data = [0x00, 0];
        let descriptor = parse_descriptor(test_data);
        assert_eq!(descriptor, Descriptor { f: 0, x: 0, y: 0 });
    }

    #[test]
    fn encode_descriptor_001() {
        //let test_data = [0xc1, 1];
        let test_data = [0x00, 1];
        let descriptor = parse_descriptor(test_data);
        assert_eq!(descriptor, Descriptor { f: 0, x: 0, y: 1 });
    }

    #[test]
    fn encode_descriptor_010() {
        let test_data = [0x01, 0];
        let descriptor = parse_descriptor(test_data);
        assert_eq!(descriptor, Descriptor { f: 0, x: 1, y: 0 });
    }

    #[test]
    fn encode_descriptor_3630() {
        let test_data = [0xff, 0];
        let descriptor = parse_descriptor(test_data);
        assert_eq!(descriptor, Descriptor { f: 3, x: 63, y: 0 });
    }

    #[test]
    fn encode_descriptor_3ff() {
        let test_data = [0xff, 0xff];
        let descriptor = parse_descriptor(test_data);
        assert_eq!(
            descriptor,
            Descriptor {
                f: 3,
                x: 63,
                y: 255
            }
        );
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

    #[test]
    fn test_buffer_reader_zero() {
        let buffer = vec![];
        let mut reader = BufferReader::new(&buffer[..]);
        reader.consume(0).unwrap();
    }

    #[test]
    fn test_buffer_reader_1() {
        let buffer = vec![0b11110000, 0b00001111];
        let mut reader = BufferReader::new(&buffer[..]);
        assert_eq!(reader.consume(4).unwrap(), &[0b0000_1111]);
        assert_eq!(reader.consume(8).unwrap(), &[0b0000_0000]);
        assert_eq!(reader.consume(4).unwrap(), &[0b0000_1111]);
    }
}

struct BufferReader<'a> {
    buffer: bitreader::BitReader<'a>,
}

impl<'a> BufferReader<'a> {
    #[allow(dead_code)]
    fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer: bitreader::BitReader::new(buffer),
        }
    }

    #[allow(dead_code)]
    /// offset in bits !!!!
    fn consume(&mut self, width: usize) -> Result<Vec<u8>, Error> {
        let chunks = width / 8;
        let mut result = vec![0; chunks];

        self.buffer
            .read_u8_slice(&mut result[..])
            .expect("Width too large");

        if width > (chunks * 8) {
            let remainder = self
                .buffer
                .read_u8(width as u8 % 8)
                .expect("Width too large");
            result.push(remainder);
        };

        Ok(result)
    }
}

#[allow(dead_code)]
fn width_value_from_table(x: u8, y: u8) -> usize {
    match (x, y) {
        (1, 27) => 80,
        (5, 2) => 2,
        (6, 2) => 2,
        (8, 5) => 4,
        (19, 5) => 9,
        (19, 6) => 14,
        (19, 107) => 4,
        (19, 108) => 3,
        (19, 109) => 4,
        (19, 110) => 4,
        (19, 111) => 7,
        (19, 112) => 7,
        (19, 113) => 4,
        (19, 114) => 7,
        (19, 115) => 6,
        (19, 116) => 7,
        (19, 117) => 3,
        (19, 118) => 7,
        (19, 119) => 3,
        (19, 150) => 32,
        _ => unimplemented!(),
    }
}

#[derive(Debug, PartialEq)]
enum BUFRUnit {
    Numeric,
    CodeTable,
    FlagTable,
    CCITTIA5,
    Year,           // a
    Month,          // mon
    Day,            // d
    Hour,           // h
    Minute,         // min
    Second,         // s
    Degree,         // deg
    MeterPerSecond, // m/s
    DegreeTrue,
    CC1,
    CC12,
    CC11,
    CC14,
    CodeTableOriginator,
    Meter,
    Kelvin,                     // K
    Hertz,                      // Hz
    Kilogram,                   // kg
    KilogramPerLiter,           // kg l-1
    KilogramPerCubicMeter,      // kg m-3
    SquareMeter,                // m^2
    CubicMeter,                 // m^3
    CubicMeterPerSecond,        // m^3
    SquareMeterPerSquareSecond, // m2 s-2
    Pascal,                     // Pa
    Celsius,                    // C
}

/*
type FieldName = String;
enum Values {
    Integer(u32),
    String(String),
    Array(Vec<Value>),
    Float(f64),
    Map(HashMap<FieldName, Values>),
}
*/

/*
magic(0-04-001, 001001100110_0111011) -> Values
0-04-001 = Year (scale=0, reference=0, DataWidth=12, BUFR_Unit=a)

Next test cases:
1111_0000
1111_1111_1111_0000
*/

#[cfg(test)]
mod test_magic {
    use super::{tables::TABLE_F0, BufferReader};

    #[test]
    // 0-04-031: width 8, scale 0, reference 0
    fn width8() -> Result<(), Box<dyn std::error::Error>> {
        let descriptor = TABLE_F0.get(&(4, 31)).expect("No descriptor");
        let buffer = [0b0000_1000];

        let mut reader = BufferReader::new(&buffer);
        let value = reader.consume(descriptor.data_width as usize)?;

        //let (value, offset) = magic(descriptor, &buffer[..], 0);
        assert_eq!(value, &[8]);
        //assert_eq!(offset, 8);

        Ok(())
    }

    #[test]
    // 0-04-002: width 4, scale 0, reference 0
    fn width4() -> Result<(), Box<dyn std::error::Error>> {
        let descriptor = TABLE_F0.get(&(4, 2)).expect("No descriptor");
        let buffer = [0b0000_1000];

        let mut reader = BufferReader::new(&buffer);
        let value = reader.consume(descriptor.data_width as usize)?;
        assert_eq!(value, &[0]);

        let value = reader.consume(descriptor.data_width as usize)?;
        assert_eq!(value, &[8]);

        Ok(())
    }

    #[test]
    // 0-04-001: width 12, scale 0, reference 0
    fn width12() -> Result<(), Box<dyn std::error::Error>> {
        let descriptor = TABLE_F0.get(&(4, 1)).expect("No descriptor");
        let buffer = [0b0000_1111, 0b0000_1111];

        let mut reader = BufferReader::new(&buffer);
        let value = reader.consume(descriptor.data_width as usize)?;
        assert_eq!(value, &[15, 0]);

        Ok(())
    }

    #[test]
    // 0-04-004: width 5, scale 0, reference 0
    fn width_5x2() -> Result<(), Box<dyn std::error::Error>> {
        let descriptor = TABLE_F0.get(&(4, 4)).expect("No descriptor");
        let buffer = [0b0000_1000, 0b0100_1111];
        let mut reader = BufferReader::new(&buffer);

        let value = reader.consume(descriptor.data_width as usize)?;
        assert_eq!(value, &[1]);

        let value = reader.consume(descriptor.data_width as usize)?;
        assert_eq!(value, &[1]);

        Ok(())
    }
}

// scale: The power of 10 by which the element has been multiplied prior to encoding.
// reference: A number to be subtracted from the element, after scaling (if any), and prior to encoding.
#[derive(Debug, PartialEq)]
struct ElementDescriptor {
    name: String,
    unit: BUFRUnit,
    scale: i32,
    reference_value: i32,
    data_width: u16,
}

// Testing WIP
// Expand a sequence descriptor, i.e. F=3.
// Note that it can contain another F=3
/*
fn flatten_sequence_descriptor(x: u8, y: u8) -> Vec<Descriptor> {
    let output = match (x, y) {
        (1, 150) => vec![
            Descriptor(f:0 x:1 y:125),
            Descriptor(f:0 x:1 y:126),
            Descriptor(f:0 x:1 y:127),
            Descriptor(f:0 x:1 y:128)],
        (1, 11) => vec![
            Descriptor(f:0 x:4 y:1),
            Descriptor(f:0 x:4 y:2),
            Descriptor(f:0 x:4 y:3)],
        (1, 13) => vec![
            Descriptor(f:0 x:4 y:4),
            Descriptor(f:0 x:4 y:5),
            Descriptor(f:0 x:4 y:6)],
        (1, 21) => vec![
            Descriptor(f:0 x:5 y:1),
            Descriptor(f:0 x:6 y:1)],
        // Incomplete. Missing descriptors for 15-12
        (15, 12) => vec![
            Descriptor(f:3 x:1 y:150),
            Descriptor(f:0 x:1 y:87),
            Descriptor(f:0 x:1 y:19),
            Descriptor(f:0 x:1 y:36),
            Descriptor(f:0 x:2 y:148),
            Descriptor(f:0 x:1 y:85),
            Descriptor(f:0 x:1 y:86),
            Descriptor(f:0 x:8 y:21),
            Descriptor(f:3 x:1 y:11),
            Descriptor(f:3 x:1 y:13),
            Descriptor(f:3 x:1 y:21),
            Descriptor(f:0 x:11 y:104),
            Descriptor(f:0 x:2 y:169),
            Descriptor(f:0 x:11 y:002),
            Descriptor(f:0 x:11 y:001),
            Descriptor(f:0 x:2 y:169),
            Descriptor(f:0 x:22 y:32),
            Descriptor(f:0 x:22 y:005),
            Descriptor(f:0 x:8 y:21),
            Descriptor(f:0 x:4 y:25),
            Descriptor(f:3 x:1 y:11),
            Descriptor(f:3 x:1 y:13),
            Descriptor(f:3 x:1 y:21),
                ],
        _ => unimplemented!()
    }
    return output;
}
*/

/*
3-01-150
    0-01-125: None
    0-01-126: None
    0-01-127: None
    0-01-128: None
0-01-087: 4802982
0-01-019: Monterey Bay
0-01-036: Monterey Bay Aquarium Research Institute
0-02-148:
0-01-085: Spray
0-01-086: 029
0-08-021: 25
3-01-011
    0-04-001: 2020
    0-04-002: 10
    0-04-003: 06
3-01-013
    0-04-004: 19
    0-04-005: 24
    0-04-006: 00
3-01-021
    0-05-001: 36.803
    0-06-001: -121.860
0-11-104:
 */

/*

use serde::{Serializer, Deserializer};

#[derive(Serializer, Deserializer)]
struct qualquer {
 x: u8,
 y: u8
}

let values: Vec<qualquer> = serde_json::from_reader(buffer)?;

[{x:1, y:2}, {x:2, y:4}]

>>> {'a': 11, 'b': 12}

let guardar: Vec<qualquer> = vec![];

let novo: BTreeMap<u64, String> = guardar.iter().filter(|x| x > 0).map(|y| (y +1, y+2)).collect();

 */
/*
fn width_value_from_table(x: u8, y: u8) -> (usize, Values) {
    match (x, y) {
        (1, 33) => (8, Values::Integer(0)),
        _ => unimplemented!(),
    }
}

type FieldName = String;
enum Values {
    Integer(u32),
    String(String),
    Array(Vec<Value>),
    Float(f64),
    Map(HashMap<FieldName, Value>)
}

struct DataIter<'a> {
  buffer: BufferReader<'a>,
  descriptors: Vec<Descriptor>
  processing_values: Vec<Values>
}

impl Message {
  fn<'a> iter(&'a self) -> DataIter<'a> {
    DataIter {
      buffer: self.section4.data(),
      descriptors: self.descriptors()
    }
  }

    fn all_data(descriptors: &[Descriptor], data: &[u8]) -> DataIter<Item=Values> {
        let data = section4.data();
        let mut reader = BitReader::new(&data);
    }

}

let m: Message = BUFR::decode(...);
for field in m.iter() {

}

3 00 002 -> 0 00 002, 0 00 003

- 0 00 002
- 0 00 003


impl Iterator for DataIter<'a> {
    Item = Values;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

fn consume_descriptor0(descriptor: &Descriptor, data: &[u8]) -> (Values, usize) {
    let (width, type_) = width_value_from_table(descriptor.x, descriptor.y);
    let value = match type_ {
        Values::Integer(_) => Values::Integer(u32::from(data[0])),
        _ => unimplemented!(),
    };

    (value, width)
}
*/
