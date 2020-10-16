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

    assert_eq!(message.total_length(), 146);
    assert_eq!(message.version(), 4);

    let section1 = message.section1();
    assert_eq!(section1.length(), 22);
    match section1 {
        bufr::Section1::V3(data) => {
            assert_eq!(data.master_table(), 8);
        }
        bufr::Section1::V4(data) => {
            assert_eq!(data.master_table(), 0);
            assert_eq!(data.center(), 34);
            assert_eq!(data.sub_center(), 0);
            assert_eq!(data.update_version(), 0);
            assert_eq!(data.optional_section(), false);
            assert_eq!(data.data_category(), 12);
            assert_eq!(data.data_subcategory(), 7);
            assert_eq!(data.local_subcategory(), 255);
            assert_eq!(data.master_table_version(), 12);
            assert_eq!(data.local_table_version(), 255);
            assert_eq!(data.year(), 2004);
            assert_eq!(data.month(), 6);
            assert_eq!(data.day(), 16);
            assert_eq!(data.hour(), 0);
            assert_eq!(data.minute(), 0);
            assert_eq!(data.second(), 0);
            assert_eq!(data.local_use(), &[]);
        }
    }
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

#[test]
fn bad_magic() {
    let buf = [0; 8];
    match bufr::decode(&buf) {
        Err(bufr::Error::MagicNumber) => (),
        _ => assert!(false),
    };
}

#[test]
fn wrong_message_size() {
    let buf = [b'B', b'U', b'F', b'R', 0, 0, 9, 4];
    match bufr::decode(&buf) {
        Err(bufr::Error::TruncatedMessage) => (),
        _ => assert!(false),
    };
}

#[test]
fn version_not_supported() {
    let buf = [b'B', b'U', b'F', b'R', 0, 0, 8, 2];
    match bufr::decode(&buf) {
        Err(bufr::Error::VersionNotSupported(2)) => (),
        _ => assert!(false),
    };
}
