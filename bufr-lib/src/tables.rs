use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
// use std::path::PathBuf;
// use std::str::FromStr;

use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::ElementDescriptor;

pub(crate) type TableF0 = HashMap<(u8, u8), ElementDescriptor>;
pub(crate) type TableF3 = HashMap<(u8, u8), F3>;

pub(crate) static TABLE_F0: Lazy<TableF0> = Lazy::new(|| {
    let mut table = TableF0::default();

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_00.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_01.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    /*
    let data = include_bytes!("../tables/BUFRCREX_TableB_en_02.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_03.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);
    */

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_04.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_05.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    /*
    let data = include_bytes!("../tables/BUFRCREX_TableB_en_06.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_07.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);
    */

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_08.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    /*
    let data = include_bytes!("../tables/BUFRCREX_TableB_en_10.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_11.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_12.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_13.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_14.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_15.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);
    */

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_19.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    /*
    let data = include_bytes!("../tables/BUFRCREX_TableB_en_20.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_21.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_22.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);
    */

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_23.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    /*
    let data = include_bytes!("../tables/BUFRCREX_TableB_en_24.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_25.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);
    */

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_26.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_27.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_28.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_29.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_30.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_31.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    /*
    let data = include_bytes!("../tables/BUFRCREX_TableB_en_33.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);
    */

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_35.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    /*
    let data = include_bytes!("../tables/BUFRCREX_TableB_en_40.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_41.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);
    */

    let data = include_bytes!("../tables/BUFRCREX_TableB_en_42.csv");
    let new_table = parse_table_f0(&data[..]);
    table.extend(new_table);

    table
});

pub(crate) static TABLE_F3: Lazy<TableF3> = Lazy::new(|| {
    let mut table = TableF3::default();

    // FIXME: 18, 21, 22, 40
    //    for n in 0..16 {
    let data = include_bytes!("../tables/BUFR_TableD_en_00.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_01.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_02.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_03.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_04.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_05.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_06.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_07.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_08.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_09.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_10.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_11.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_12.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_13.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_14.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_15.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_16.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_17.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_18.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_19.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_20.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_21.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_22.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_23.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_24.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_25.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_26.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_27.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_28.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_29.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_30.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_31.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_32.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_33.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_34.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/BUFR_TableD_en_35.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    let data = include_bytes!("../tables/glider.csv");
    let new_table = parse_table_f3(&data[..]);
    table.extend(new_table);

    table
});

#[derive(PartialEq, Debug)]
pub enum Descriptor {
    Element(u8, u8),     // F0
    Replication(u8, u8), // F1
    Operator(u8, u8),    // F2
    Sequence(u8, u8),    // F3
}

impl fmt::Display for Descriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (fd, x, y) = match *self {
            Descriptor::Element(x, y) => (0, x, y),
            Descriptor::Replication(x, y) => (1, x, y),
            Descriptor::Operator(x, y) => (2, x, y),
            Descriptor::Sequence(x, y) => (3, x, y),
        };
        write!(f, "Descriptor {{ f: {}, x: {}, y: {} }}", fd, x, y)
    }
}

#[derive(PartialEq, Debug)]
pub(crate) struct F3 {
    descriptors: Vec<Descriptor>,
    title: Option<String>,
}

impl F3 {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Descriptor> {
        self.descriptors.iter()
    }
}

/*
impl Iterator for F3 {
    type Item = Descriptor;

    fn next(&mut self) -> Option<Self::Item> {

    }
}
*/

impl TryFrom<Descriptor> for &ElementDescriptor {
    type Error = crate::Error;

    fn try_from(value: Descriptor) -> Result<Self, Self::Error> {
        match value {
            Descriptor::Element(x, y) => Ok(&TABLE_F0[&(x, y)]),
            _ => unimplemented!(),
        }
    }
}

//Descriptor::Element()

#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct RecordF0 {
    ClassNo: String,      // X
    ClassName_en: String, // Identification: String
    FXY: String,
    ElementName_en: String, // Name: String
    Note_en: Option<String>,
    BUFR_Unit: String,
    BUFR_Scale: i32,
    BUFR_ReferenceValue: i32,
    BUFR_DataWidth_Bits: u16,
    //    CREX_Unit: String,
    //    CREX_Scale: i8,
    //    CREX_DataWidth_Char: u8,
    Status: String, // Operation: String
}

#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct RecordF3 {
    Category: String,
    CategoryOfSequences_en: String,
    FXY1: String,
    Title_en: Option<String>,
    SubTitle_en: Option<String>,
    FXY2: String,
    //ElementName_en: String,
    //ElementDescription_en: Option<String>,
    //Note_en: Option<String>,
    Status: String,
}

impl F3 {
    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.descriptors.len()
    }
}

use crate::BUFRUnit;

impl std::str::FromStr for BUFRUnit {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Code table" => BUFRUnit::CodeTable,
            "Flag table" => BUFRUnit::FlagTable,
            "Numeric" => BUFRUnit::Numeric,
            "a" => BUFRUnit::Year,
            "d" => BUFRUnit::Day,
            "m/s" => BUFRUnit::MeterPerSecond,
            "m s-1" => BUFRUnit::MeterPerSecond,
            "CCITT IA5" => BUFRUnit::CCITTIA5,
            "degree true" => BUFRUnit::DegreeTrue,
            "Code table defined by originating/generating centre" => BUFRUnit::CodeTableOriginator,
            "h" => BUFRUnit::Hour,
            "min" => BUFRUnit::Minute,
            "m" => BUFRUnit::Meter,
            "mon" => BUFRUnit::Month,
            "s" => BUFRUnit::Second,
            "deg" => BUFRUnit::Degree,
            "Common Code table C-1" => BUFRUnit::CC1,
            "Common Code table C-12" => BUFRUnit::CC12,
            "Common Code table C-11" => BUFRUnit::CC11,
            "Common Code table C-14" => BUFRUnit::CC14,
            "K" => BUFRUnit::Kelvin,
            "C" => BUFRUnit::Celsius,
            "Hz" => BUFRUnit::Hertz,
            "kg" => BUFRUnit::Kilogram,
            "kg l-1" => BUFRUnit::KilogramPerLiter,
            "m2" => BUFRUnit::SquareMeter,
            "m3" => BUFRUnit::CubicMeter,
            "m3/s" => BUFRUnit::CubicMeterPerSecond,
            "m3 s-1" => BUFRUnit::CubicMeterPerSecond,
            "m2 s-2" => BUFRUnit::SquareMeterPerSquareSecond,
            "Pa" => BUFRUnit::Pascal,
            _ => unimplemented!("Unrecognized unit: {}", s),
        })
    }
}

impl From<RecordF0> for ElementDescriptor {
    fn from(v: RecordF0) -> Self {
        let unit = v.BUFR_Unit.parse().expect("Unknown unit");

        Self {
            name: v.ElementName_en,
            unit,
            scale: v.BUFR_Scale,
            reference_value: v.BUFR_ReferenceValue,
            data_width: v.BUFR_DataWidth_Bits,
        }
    }
}

#[allow(dead_code)]
fn load_table_f0<P: AsRef<Path>>(filename: P) -> TableF0 {
    let path = filename.as_ref();
    let file = File::open(path).unwrap_or_else(|_| panic!("Error loading file: {:?}", path));
    let reader = BufReader::new(file);
    parse_table_f0(reader)
}

fn parse_table_f0<R: std::io::Read>(reader: R) -> TableF0 {
    let mut table = TableF0::default();

    let mut rdr = csv::Reader::from_reader(reader);
    for result in rdr.deserialize() {
        let record: RecordF0 = result.expect("Error leading record");
        //let f: u8 = record.FXY.get(0..1).expect("").parse().expect("");
        let x: u8 = record.FXY.get(1..=2).expect("").parse().expect("");
        let y: u8 = record.FXY.get(3..).expect("").parse().expect("");
        table.insert((x, y), record.into());
    }
    table
}

#[allow(dead_code)]
fn load_table_f3<P: AsRef<Path>>(filename: P) -> TableF3 {
    let path = filename.as_ref();
    let file = File::open(path).unwrap_or_else(|_| panic!("Error loading file: {:?}", path));
    let reader = BufReader::new(file);
    parse_table_f3(reader)
}

fn parse_table_f3<R: std::io::Read>(reader: R) -> TableF3 {
    let mut table = TableF3::default();

    let mut rdr = csv::Reader::from_reader(reader);
    for result in rdr.deserialize() {
        let record: RecordF3 = result.expect("Error leading record");
        let f: u8 = record.FXY1.get(0..1).expect("").parse().expect("");
        assert_eq!(f, 3);
        let x: u8 = record.FXY1.get(1..=2).expect("").parse().expect("");
        let y: u8 = record.FXY1.get(3..).expect("").parse().expect("");
        table
            .entry((x, y))
            .and_modify(|v| {
                // TODO:
                // - verify that v.title is a superset of record.Title_en
                // - only warn, not assert
                //assert_eq!(record.Title_en, v.title);
                v.descriptors.push(record.clone().into())
            })
            .or_insert({
                let title = record.Title_en.clone();
                F3 {
                    descriptors: vec![record.into()],
                    title,
                }
            });
    }
    table
}

impl From<RecordF3> for Descriptor {
    fn from(record: RecordF3) -> Self {
        let f: u8 = record.FXY2.get(0..1).expect("").parse().expect("");
        let x: u8 = record.FXY2.get(1..=2).expect("").parse().expect("");
        let y: u8 = record.FXY2.get(3..).expect("").parse().expect("");

        match f {
            0 => Descriptor::Element(x, y),
            1 => Descriptor::Replication(x, y),
            2 => Descriptor::Operator(x, y),
            3 => Descriptor::Sequence(x, y),
            _ => unimplemented!("Unknown f: {}", f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{load_table_f0, load_table_f3, Descriptor, TABLE_F0, TABLE_F3};
    use crate::{BUFRUnit, ElementDescriptor, Error};

    use std::path::PathBuf;

    #[test]
    fn validate_load_f0() {
        let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        filename.push("tables/BUFRCREX_TableB_en_01.csv");

        let table = load_table_f0(filename);
        for ((x, y), v) in table.into_iter() {
            if let Ok(ans) = element_descriptor_f0(x, y) {
                assert_eq!(v, ans);
            }
        }
    }

    #[test]
    fn validate_static_f0() {
        for ((x, y), v) in TABLE_F0.iter() {
            if let Ok(ans) = element_descriptor_f0(*x, *y) {
                assert_eq!(v, &ans);
            }
        }
    }

    #[test]
    fn validate_load_f3() {
        let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        filename.push("tables/BUFR_TableD_en_01.csv");

        let table = load_table_f3(filename);
        let record = table.get(&(1, 2)).unwrap();
        assert_eq!(record.len(), 3);
        assert_eq!(
            record.descriptors,
            vec![
                Descriptor::Element(1, 3),
                Descriptor::Element(1, 4),
                Descriptor::Element(1, 5),
            ]
        );
    }

    #[test]
    fn validate_static_f3() {
        let record = TABLE_F3.get(&(1, 2)).unwrap();
        assert_eq!(record.len(), 3);
        assert_eq!(
            record.descriptors,
            vec![
                Descriptor::Element(1, 3),
                Descriptor::Element(1, 4),
                Descriptor::Element(1, 5),
            ]
        );
    }

    // Testing WIP
    // F=0
    fn element_descriptor_f0(x: u8, y: u8) -> Result<ElementDescriptor, Error> {
        let element = match (x, y) {
            (1, 19) => ElementDescriptor {
                name: String::from("Long station or site name"),
                unit: BUFRUnit::CCITTIA5,
                scale: 0,
                reference_value: 0,
                data_width: 256,
            },
            (1, 36) => ElementDescriptor {
                name: String::from("Agency in charge of operating the observing platform"),
                unit: BUFRUnit::CodeTable,
                scale: 0,
                reference_value: 0,
                data_width: 20,
            },
            (1, 85) => ElementDescriptor {
                name: String::from("Observing platform manufacturer's model"),
                unit: BUFRUnit::CCITTIA5,
                scale: 0,
                reference_value: 0,
                data_width: 160,
            },
            (1, 86) => ElementDescriptor {
                name: String::from("Observing platform manufacturer's serial number"),
                unit: BUFRUnit::CCITTIA5,
                scale: 0,
                reference_value: 0,
                data_width: 256,
            },
            (1, 87) => ElementDescriptor {
                name: String::from("WMO marine observing platform extended identifier"),
                unit: BUFRUnit::Numeric,
                scale: 0,
                reference_value: 0,
                data_width: 23,
            },
            (1, 125) => ElementDescriptor {
                name: String::from("WIGOS identifier series"),
                unit: BUFRUnit::Numeric,
                scale: 0,
                reference_value: 0,
                data_width: 4,
            },
            (1, 126) => ElementDescriptor {
                name: String::from("WIGOS issuer of identifier"),
                unit: BUFRUnit::Numeric,
                scale: 0,
                reference_value: 0,
                data_width: 16,
            },
            (1, 127) => ElementDescriptor {
                name: String::from("WIGOS issue number"),
                unit: BUFRUnit::Numeric,
                scale: 0,
                reference_value: 0,
                data_width: 16,
            },
            (1, 128) => ElementDescriptor {
                name: String::from("WIGOS local identifier (character)"),
                unit: BUFRUnit::CCITTIA5,
                scale: 0,
                reference_value: 0,
                data_width: 128,
            },
            (4, 1) => ElementDescriptor {
                name: String::from("Year"),
                unit: BUFRUnit::Year,
                scale: 0,
                reference_value: 0,
                data_width: 12,
            },
            (4, 2) => ElementDescriptor {
                name: String::from("Month"),
                unit: BUFRUnit::Month,
                scale: 0,
                reference_value: 0,
                data_width: 4,
            },
            (4, 3) => ElementDescriptor {
                name: String::from("Day"),
                unit: BUFRUnit::Day,
                scale: 0,
                reference_value: 0,
                data_width: 6,
            },
            (4, 4) => ElementDescriptor {
                name: String::from("Hour"),
                unit: BUFRUnit::Hour,
                scale: 0,
                reference_value: 0,
                data_width: 5,
            },
            (4, 5) => ElementDescriptor {
                name: String::from("Minute"),
                unit: BUFRUnit::Minute,
                scale: 0,
                reference_value: 0,
                data_width: 6,
            },
            (4, 6) => ElementDescriptor {
                name: String::from("Second"),
                unit: BUFRUnit::Second,
                scale: 0,
                reference_value: 0,
                data_width: 6,
            },
            (5, 1) => ElementDescriptor {
                name: String::from("Latitude (high accuracy)"),
                unit: BUFRUnit::Degree,
                scale: 5,
                reference_value: -9000000,
                data_width: 25,
            },
            (6, 1) => ElementDescriptor {
                name: String::from("Longitude (high accuracy)"),
                unit: BUFRUnit::Degree,
                scale: 5,
                reference_value: -18000000,
                data_width: 26,
            },
            (8, 21) => ElementDescriptor {
                name: String::from("Time significance"),
                unit: BUFRUnit::CodeTable,
                scale: 0,
                reference_value: 0,
                data_width: 5,
            },
            _ => return Err(Error::TruncatedMessage), // FIXME
        };
        Ok(element)
    }
}
