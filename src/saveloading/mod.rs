/// This module contains the code for loading a save from a raw byte array.
/// It is not recommended to use this module directly, as it doesnt account for a mac save or a unencoded save.
mod raw_loader;

const XOR_SAVE_KEY: u8 = 0x0B;
