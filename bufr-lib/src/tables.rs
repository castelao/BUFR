use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::ElementDescriptor;

pub type TableF0 = HashMap<(u8, u8), ElementDescriptor>;
pub type TableF3 = HashMap<(u8, u8), Vec<F3>>;

static TABLE_F0: Lazy<TableF0> = Lazy::new(|| {
    let data = include_bytes!("../tables/BUFRCREX_TableB_en_01.csv");
    parse_table_f0(&data[..])
});

pub enum Descriptor {
    Element(u8, u8),           // F0
    Replication(u8, u8),       // F1
    Operator(u8, u8),          // F2
    Sequence(Vec<Descriptor>), // F3
}

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

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
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

pub struct F3 {
    fxy2: Descriptor,
    title: Option<String>,
}

use crate::BUFRUnit;

impl std::str::FromStr for BUFRUnit {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Code table" => BUFRUnit::CodeTable,
            "Numeric" => BUFRUnit::Numeric,
            "a" => BUFRUnit::Year,
            "d" => BUFRUnit::Day,
            "m/s" => BUFRUnit::MeterPerSecond,
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

pub fn load_table_f0<P: AsRef<Path>>(filename: P) -> TableF0 {
    let path = filename.as_ref();
    let file = File::open(path).expect(&format!("Error loading file: {:?}", path));
    let reader = BufReader::new(file);
    parse_table_f0(reader)
}

pub fn parse_table_f0<R: std::io::Read>(reader: R) -> TableF0 {
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

pub fn load_table_f3<P: AsRef<Path>>(filename: P) -> TableF3 {
    let path = filename.as_ref();
    let file = File::open(path).expect(&format!("Error loading file: {:?}", path));
    let reader = BufReader::new(file);

    let mut table = TableF3::default();

    let mut rdr = csv::Reader::from_reader(reader);
    for result in rdr.deserialize() {
        let record: RecordF3 = result.expect("Error leading record");
        let f: u8 = record.FXY1.get(0..1).expect("").parse().expect("");
        assert_eq!(f, 3);
        let x: u8 = record.FXY1.get(1..=2).expect("").parse().expect("");
        let y: u8 = record.FXY1.get(3..).expect("").parse().expect("");
        //table.insert((f, x, y), record);
        table
            .entry((x, y))
            .and_modify(|v| v.push(record.clone().into()))
            .or_insert(vec![record.into()]);
    }
    table
}

impl From<RecordF3> for F3 {
    fn from(v: RecordF3) -> Self {
        Self {
            fxy2: v.FXY2.parse().unwrap(),
            title: v.Title_en,
        }
    }
}

impl FromStr for Descriptor {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // FIXME Continue from here
        /*
        let f: u8 = record.FXY1.get(0..1).expect("").parse().expect("");
        let x: u8 = record.FXY1.get(1..=2).expect("").parse().expect("");
        let y: u8 = record.FXY1.get(3..).expect("").parse().expect("");

        match f {
          0 => Descriptor::Element,
          3 => Descriptor::Sequence,
        */
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::{load_table_f0, load_table_f3, TABLE_F0};
    use crate::{BUFRUnit, ElementDescriptor, Error};

    use std::path::PathBuf;

    #[test]
    fn test_load_f0() {
        let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        filename.push("tables/BUFRCREX_TableB_en_01.csv");

        let table = load_table_f0(filename);
        //        assert_eq!(table.get(&(1, 154)).unwrap().BUFR_DataWidth_Bits, 12u16);
    }

    #[test]
    fn validate_load_f0() {
        let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        filename.push("tables/BUFRCREX_TableB_en_01.csv");

        let table = load_table_f0(filename);
        for ((x, y), v) in table.into_iter() {
            if let Ok(ans) = element_descriptor_f0(x, y) {
                //                assert_eq!(v.BUFR_DataWidth_Bits, ans.data_width);
                assert_eq!(v, ans);
            }
        }
    }

    #[test]
    fn validate_static_f0() {
        for ((x, y), v) in TABLE_F0.iter() {
            if let Ok(ans) = element_descriptor_f0(*x, *y) {
                //                assert_eq!(v.BUFR_DataWidth_Bits, ans.data_width);
                assert_eq!(v, &ans);
            }
        }
    }

    #[test]
    fn test_load_f3() {
        let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        filename.push("tables/BUFR_TableD_en_01.csv");

        let table = load_table_f3(filename);
        let record = table.get(&(1, 2)).unwrap();
        assert_eq!(record.len(), 3);
        assert_eq!(
            record
                .into_iter()
                .map(|r| r.FXY2.clone())
                .collect::<Vec<_>>(),
            vec![
                String::from("001003"),
                String::from("001004"),
                String::from("001005")
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
