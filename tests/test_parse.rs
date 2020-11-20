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

    let section3 = message.section3();
    assert_eq!(section3.length(), 67);

    let descriptors = section3.descriptors();
    assert_eq!(descriptors.len(), 30);
    //3 1 1
    assert_eq!(descriptors[0].encode(), [0b11_000001, 1]);
    //3 1 11
    assert_eq!(descriptors[1].encode(), [0b11_000001, 11]);
    //3 1 12
    assert_eq!(descriptors[2].encode(), [0b11_000001, 12]);
    //0 1 7
    assert_eq!(descriptors[3].encode(), [0b00_000001, 7]);
    //0 1 33
    assert_eq!(descriptors[4].encode(), [0b00_000001, 33]);
    //0 25 150
    assert_eq!(descriptors[5].encode(), [0b00_011001, 150]);
    //1 22 0
    assert_eq!(descriptors[6].encode(), [0b01_010110, 0]);
    //0 31 1
    assert_eq!(descriptors[7].encode(), [0b00_011111, 1]);
    //0 1 27
    assert_eq!(descriptors[8].encode(), [0b00_000001, 27]);
    //0 19 150
    assert_eq!(descriptors[9].encode(), [0b00_010011, 150]);
    //1 19 106
    assert_eq!(descriptors[10].encode(), [0b00_010011, 106]);
    //0 8 5
    assert_eq!(descriptors[11].encode(), [0b00_001000, 5]);
    //0 5 2
    assert_eq!(descriptors[12].encode(), [0b00_000101, 2]);
    //0 6 2
    assert_eq!(descriptors[13].encode(), [0b00_000110, 2]);
    //0 8 5
    assert_eq!(descriptors[14].encode(), [0b00_001000, 5]);
    //0 19 107
    assert_eq!(descriptors[15].encode(), [0b00_010011, 107]);
    //0 19 5
    assert_eq!(descriptors[16].encode(), [0b00_010011, 5]);
    //0 19 6
    assert_eq!(descriptors[17].encode(), [0b00_010011, 6]);
    //0 19 108
    assert_eq!(descriptors[18].encode(), [0b00_010011, 108]);
    //0 19 109
    assert_eq!(descriptors[19].encode(), [0b00_010011, 109]);
    //0 19 110
    assert_eq!(descriptors[20].encode(), [0b00_010011, 110]);
    //0 19 111
    assert_eq!(descriptors[21].encode(), [0b00_010011, 111]);
    //0 19 112
    assert_eq!(descriptors[22].encode(), [0b00_010011, 112]);
    //0 19 113
    assert_eq!(descriptors[23].encode(), [0b00_010011, 113]);
    //0 19 114
    assert_eq!(descriptors[24].encode(), [0b00_010011, 114]);
    //0 19 115
    assert_eq!(descriptors[25].encode(), [0b00_010011, 115]);
    //0 19 116
    assert_eq!(descriptors[26].encode(), [0b00_010011, 116]);
    //0 19 117
    assert_eq!(descriptors[27].encode(), [0b00_010011, 117]);
    //0 19 118
    assert_eq!(descriptors[28].encode(), [0b00_010011, 118]);
    //0 19 119
    assert_eq!(descriptors[29].encode(), [0b00_010011, 119]);

    let section4 = message.section4();
    assert_eq!(section4.length(), 45);

    let data = section4.data();
    assert_eq!(data.len(), 41);

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
