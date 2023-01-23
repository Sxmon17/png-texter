#[allow(dead_code)]
pub mod chunk_type {
use std::convert::TryFrom;
use std::fmt;
    use std::str::FromStr;
    //use std::str::FromStr;
use crate::{Error, Result};


    #[derive(Clone, Eq, PartialEq, Debug)]
    struct ChunkType {
        bytes: [u8; 4],
    }

    impl ChunkType {
        /// Returns the raw bytes contained in this chunk
        pub fn bytes(&self) -> [u8; 4] {
            self.bytes
        }

        /// Returns the property state of the first byte as described in the PNG spec
        pub fn is_critical(&self) -> bool {
            todo!()
        }

        /// Returns the property state of the second byte as described in the PNG spec
        pub fn is_public(&self) -> bool {
            todo!()
        }

        /// Returns the property state of the third byte as described in the PNG spec
        pub fn is_reserved_bit_valid(&self) -> bool {
            todo!()
        }

        /// Returns the property state of the fourth byte as described in the PNG spec
        pub fn is_safe_to_copy(&self) -> bool {
            todo!()
        }

        /// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
        /// Note that this chunk type should always be valid as it is validated during construction.
        pub fn is_valid(&self) -> bool {
            todo!()
        }

        /// Valid bytes are represented by the characters A-Z or a-z
        fn is_valid_byte(byte: u8) -> bool {
            byte.is_ascii_lowercase() || byte.is_ascii_uppercase()
        }
    }

    impl TryFrom<[u8; 4]> for ChunkType {
        type Error = Error;

        fn try_from(bytes: [u8; 4]) -> Result<Self> {
            if bytes.iter().all(|&b| Self::is_valid_byte(b)) {
                Ok(Self { bytes })
            } else {
                Err(format!("Invalid chunk type: {:?}", bytes).into())
            }
        }
    }

    impl fmt::Display for ChunkType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            todo!()
        }
    }

    impl FromStr for ChunkType {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            todo!()
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        pub use std::{assert_eq, format};

        #[test]
        pub fn test_chunk_type_from_bytes() {
            let expected = [82, 117, 83, 116];
            let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();
            assert_eq!(expected, actual.bytes());
        }

        #[test]
        #[ignore]
        pub fn test_chunk_type_from_str() {
            let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
            let actual = ChunkType::from_str("RuSt").unwrap();
            assert_eq!(expected, actual);
        }
    }
}
