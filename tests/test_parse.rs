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
