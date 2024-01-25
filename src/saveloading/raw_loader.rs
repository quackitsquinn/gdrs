use std::{
    error::Error,
    fmt::{Debug, Display},
    fs::File,
    io::{Read, Write},
};

use crate::saveloading::XOR_SAVE_KEY;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use flate2::bufread::GzDecoder;

/// Loads a save from a raw byte array. This preforms all of the steps to decode the save, but does not do any further processing.
/// Returns a XML string of the save.
pub fn decode_save_bytes(bytes: Vec<u8>) -> Result<String, DecodeError> {
    // The first step is to xor the bytes with the key.
    let xored_bytes = bytes
        .iter()
        .map(|byte| byte ^ XOR_SAVE_KEY)
        .collect::<Vec<u8>>();
    // TODO: add a check if the save is mac or not, and decrypt the aes accordingly.
    // Load the bytes into a string because its a base64 string.
    let mut base64_string = String::from_utf8(xored_bytes)?;
    // Remove any nuls from the string.
    base64_string = base64_string.replace("\u{0}", "");
    // Remove any base64 padding because its tends to be inconsistent.
    base64_string = base64_string.replace("=", "");
    // Decode the base64 string into bytes.
    let decoded_bytes = URL_SAFE_NO_PAD.decode(base64_string.as_bytes())?;
    // lastly, we gzip expand the bytes into a string.
    let mut reader = GzDecoder::new(decoded_bytes.as_slice());
    let mut xml_string = String::new();
    reader.read_to_string(&mut xml_string)?;
    Ok(xml_string)
}
/// An error that can occur while decoding a save.
#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    /// The decoder was unable to decode a string from utf8.
    #[error("Failed to decode string from utf8")]
    InvalidString(#[from] std::string::FromUtf8Error),
    /// The decoder was unable to decode a base64 string.
    #[error("Failed to decode base64")]
    InvalidBase64(#[from] base64::DecodeError),
    /// The decoder was unable to read a string from gzip.
    #[error("Failed to read string from gzip")]
    InvalidGzip(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use crate::saveloading::raw_loader::decode_save_bytes;

    #[test]
    fn test_load_save_bytes() {
        let bytes = include_bytes!("../../res/CCLocalLevels.dat").to_vec();
        let decoded = decode_save_bytes(bytes);
        assert!(
            decoded.is_ok(),
            "Failed to decode save bytes {}",
            decoded.err().unwrap()
        );
    }
}
