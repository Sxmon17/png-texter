#[allow(dead_code)]
pub mod chunk_type {
    #[derive(Clone, Eq, PartialEq, Debug)]
    struct ChunkType {
        value: [i32; 4],
    }

    impl ChunkType {
        pub fn bytes(&self) -> [i32; 4] {
            self.value
        }

        pub fn try_from(value: [i32; 4]) -> Result<ChunkType, String> {
            return match value.iter().all(|&x| x >= 0 && x <= 255){
                true => Ok(ChunkType { value }),
                false => Err("Value needs to be between 0 and 255".to_string())
            };
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
    }
}
