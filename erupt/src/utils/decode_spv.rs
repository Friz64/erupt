use std::{convert::TryInto, io};

// https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_magic_a_magic_number
const SPV_MAGIC_NUMBER_LE: u32 = 0x07230203;
const SPV_MAGIC_NUMBER_BE: u32 = SPV_MAGIC_NUMBER_LE.swap_bytes();

// inspired by https://docs.rs/ash/0.30.0/ash/util/fn.read_spv.html
/// Safely decode arbitrary SPIR-V data to it's correct word representation.
///
/// SPIR-V data consists of words (32-bit) not bytes (8-bit) and therefore
/// requires an alignment of 4 bytes. The byte data will be rejected if it is
/// not aligned. Additionally, it is possible that the bytes have wrong
/// endianness, which is also accounted for with this function.
pub fn decode_spv(bytes: &[u8]) -> io::Result<Vec<u32>> {
    if bytes.len() % 4 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "`bytes` is not 4-byte aligned",
        ));
    }

    let mut words: Vec<_> = bytes
        .chunks(4)
        .map(|word| u32::from_le_bytes(word.try_into().unwrap()))
        .collect();

    match words.get(0) {
        Some(&SPV_MAGIC_NUMBER_LE) => (),
        Some(&SPV_MAGIC_NUMBER_BE) => {
            for word in &mut words {
                *word = word.swap_bytes();
            }
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "`bytes` is not valid SPIR-V data",
            ))
        }
    }

    Ok(words)
}
