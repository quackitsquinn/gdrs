use self::raw_loader::{decode_save_bytes, SaveDecodeError};

/// This module contains the code for loading a save from a raw byte array.
/// It is not recommended to use this module directly, as it doesnt account for a mac save or a unencoded save.
mod raw_loader;
mod xml;

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
#[macro_export]
#[cfg(debug_assertions)]
macro_rules! dump_file {
    ($file_name: expr, $file_data: expr) => {
        #[cfg(debug_assertions)]
        {
            use std::fs::File;
            use std::io::Write;

            let mut file = File::create($file_name).unwrap();
            file.write_all($file_data.as_bytes()).unwrap();
        }
    };
}
/// A macro that sets up logging for tests.
#[cfg(test)]
#[macro_export]
macro_rules! setup_logging {
    ($test_name: expr) => {
        simplelog::CombinedLogger::init(vec![
            simplelog::TermLogger::new(
                simplelog::LevelFilter::Debug,
                simplelog::Config::default(),
                simplelog::TerminalMode::Mixed,
                simplelog::ColorChoice::Auto,
            ),
            simplelog::WriteLogger::new(
                simplelog::LevelFilter::Trace,
                simplelog::Config::default(),
                std::fs::File::create(concat!($test_name, ".log")).unwrap(),
            ),
        ])
        .unwrap();
    };
}

#[cfg(test)]
mod test {

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
