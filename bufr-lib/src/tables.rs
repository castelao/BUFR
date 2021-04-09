use serde::Deserialize;

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

        let mut table: HashMap<String, Record> = HashMap::default();

        let mut rdr = csv::Reader::from_reader(reader);
        for result in rdr.deserialize() {
            let record: Record = result.expect("Error leading record");
            println!("{:?}", record);
            //table.insert((record.FXY).clone(), record);
        }
        dbg!(table);
        assert!(false);
    }
}
