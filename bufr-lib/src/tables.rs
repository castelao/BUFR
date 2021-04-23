use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Record {
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

#[cfg(test)]
mod tests {
    use super::Record;

    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::PathBuf;

    #[test]
    fn test_load() {
        let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        filename.push("tables/BUFRCREX_TableB_en_01.csv");

        let file = File::open(&filename).expect(&format!("Error loading file: {:?}", &filename));
        let mut reader = BufReader::new(file);

        let mut table: HashMap<(u8, u8), Record> = HashMap::default();

        let mut rdr = csv::Reader::from_reader(reader);
        for result in rdr.deserialize() {
            let record: Record = result.expect("Error leading record");
            //let f: u8 = record.FXY.get(0..1).expect("").parse().expect("");
            let x: u8 = record.FXY.get(1..=2).expect("").parse().expect("");
            let y: u8 = dbg!(record.FXY.get(3..)).expect("").parse().expect("");
            table.insert((x, y), record);
        }
        dbg!(table);
        assert!(false);
    }
}
