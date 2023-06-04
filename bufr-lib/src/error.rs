//#![deny(missing_docs)]

/// Possible errors when parsing BUFR messages
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Message too short to even contain the total size
    #[error("BUFR message is too short, less than 8 bytes")]
    MessageTooShort,

    /// Wrong magic number
    #[error("BUFR message should start with BUFR")]
    MagicNumber,

    /// Wrong end section
    #[error("BUFR message should end with 7777")]
    EndSection,

    /// Message shorter than specified on section 0
    #[error("Message shorter than expected, is it truncated?")]
    TruncatedMessage,

    /// BUFR version not supported
    #[error("BUFR version {0} not supported")]
    VersionNotSupported(u8),

    /*
    /// Number of descriptors doesn't match the size
    #[error("Expected {n_descriptors} descriptors, requiring {descriptor_size}B but buffer size {buffer_size}B")]
    WrongNumberOfDescriptors {
        n_descriptors: u16,
        descriptor_size: usize,
        buffer_size: usize,
    },
    */
    #[error("Section 3 must be larger than 7 and an odd number, got {0}")]
    InvalidSection3Length(usize),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
