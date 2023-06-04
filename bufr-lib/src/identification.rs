//#![deny(missing_docs)]

use std::convert::TryInto;
use std::fmt;

use byteorder::{BigEndian, WriteBytesExt};
use derive_builder::Builder;
use getset::{CopyGetters, Getters};

use crate::error::Error;

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
    pub(super) fn decode(buf: &[u8], version: u8) -> Result<Section1, Error> {
        Ok(match version {
            3 => Section1::V3(Section1v3::decode(buf)?),
            4 => Section1::V4(Section1v4::decode(buf)?),
            _ => unimplemented!(),
        })
    }

    pub(super) fn encode<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
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
#[derive(Builder, CopyGetters, Getters, Debug)]
pub struct Section1v4 {
    #[builder(setter(skip = true), default = "self.default_length()?")]
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
    #[builder(default = "vec![]")]
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

#[cfg(test)]
mod test_section1_builder {
    use super::Section1v4Builder;

    #[test]
    fn build() {
        let section = Section1v4Builder::default()
            .master_table(0)
            .sub_center(0)
            .center(0)
            .update_version(0)
            .optional_section(false)
            .data_category(0)
            .data_subcategory(6)
            .local_subcategory(0)
            .master_table_version(39)
            .local_table_version(0)
            .year(2018)
            .month(2)
            .day(7)
            .hour(23)
            .minute(23)
            .second(42)
            .build()
            .unwrap();

        assert_eq!(section.length(), 22);
    }
}

impl Section1v4Builder {
    fn default_length(&self) -> Result<usize, Section1v4BuilderError> {
        // Only if local_use is false
        Ok(22)
        /*
        match self.local_use {
            Some(ref x) => Ok(7 + 2 * x.len()),
            _ => Err(Section1v4BuilderError::from(
                derive_builder::UninitializedFieldError::new("local_use"),
            )),
        }
        */
    }
}
