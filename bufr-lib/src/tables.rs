use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct RecordF0 {
    ClassNo: String,
    ClassName_en: String,
    FXY: String,
    ElementName_en: String,
    Note_en: Option<String>,
    BUFR_Unit: String,
    BUFR_Scale: i8,
    BUFR_ReferenceValue: i64,
    BUFR_DataWidth_Bits: u16,
    //    CREX_Unit: String,
    //    CREX_Scale: i8,
    //    CREX_DataWidth_Char: u8,
    Status: String,
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

pub type TableF0 = HashMap<(u8, u8), RecordF0>;
pub type TableF3 = HashMap<(u8, u8), Vec<RecordF3>>;

pub fn load_table_f0<P: AsRef<Path>>(filename: P) -> TableF0 {
    let path = filename.as_ref();
    let file = File::open(path).expect(&format!("Error loading file: {:?}", path));
    let reader = BufReader::new(file);

    let mut table = TableF0::default();

    let mut rdr = csv::Reader::from_reader(reader);
    for result in rdr.deserialize() {
        let record: RecordF0 = result.expect("Error leading record");
        //let f: u8 = record.FXY.get(0..1).expect("").parse().expect("");
        let x: u8 = record.FXY.get(1..=2).expect("").parse().expect("");
        let y: u8 = record.FXY.get(3..).expect("").parse().expect("");
        table.insert((x, y), record);
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
            .and_modify(|v| v.push(record.clone()))
            .or_insert(vec![record]);
    }
    table
}

#[cfg(test)]
mod tests {
    use super::{load_table_f0, load_table_f3};

    use std::path::PathBuf;

    #[test]
    fn test_load_f0() {
        let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        filename.push("tables/BUFRCREX_TableB_en_01.csv");

        let table = load_table_f0(filename);
        assert_eq!(table.get(&(1, 154)).unwrap().BUFR_DataWidth_Bits, 12u16);
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
}
