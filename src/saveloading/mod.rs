use self::raw_loader::{decode_save_bytes, SaveDecodeError};

/// This module contains the code for loading a save from a raw byte array.
/// It is not recommended to use this module directly, as it doesnt account for a mac save or a unencoded save.
mod raw_loader;

const XOR_SAVE_KEY: u8 = 0x0B;

/// This function takes a save file as a byte array and returns a string. It will attempt to decode the save file if it is encoded.
pub(crate) fn load_save_str(save_data: Vec<u8>) -> Result<String, SaveDecodeError> {
    // First we check if the save is encoded or not.
    if let Ok(save_as_string) = String::from_utf8(save_data.clone()) {
        return Ok(save_as_string);
    }
    // Next, we attempt to decode the save with the window decryption.
    // TODO: add mac save decryption
    decode_save_bytes(save_data)
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::saveloading::load_save_str;
    /// Tests loading from an actual save file.
    #[test]
    fn test_load_save_str() {
        let bytes = include_bytes!("../../res/CCLocalLevels.dat").to_vec();
        let decoded = load_save_str(bytes);
        assert!(
            decoded.is_ok(),
            "Failed to decode save bytes {}",
            decoded.err().unwrap()
        );
    }
}