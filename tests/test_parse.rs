use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

#[test]
fn simple_parse() -> Result<(), Box<dyn std::error::Error>> {
    let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filename.push("tests/data/wmo_sarep.bufr");

    let file = File::open(&filename).expect(&format!("Error loading file: {:?}", &filename));
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let message = bufr::decode(&buffer)?;
    // TODO: check message values

    Ok(())
}

#[test]
fn too_short() {
    let buf = [0; 8];
    for i in 0..8 {
        match bufr::decode(&buf[0..i]) {
            Err(bufr::Error::MessageTooShort) => (),
            _ => assert!(false),
        };
    }
}
