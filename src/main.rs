use blake3::Hash;
use std::{
    fs::File,
    io::{BufReader, Read},
};

pub const OFFSET_ONE: [u8; 4] = 1u32.to_le_bytes();
pub const TEST_FILE_PATH: &str = "./testfiles/test.txt";

fn main() {
    //load then file contents
    let file = File::open(TEST_FILE_PATH).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = Vec::new();
    reader.read_to_end(&mut contents).unwrap();

    let hasher_blake3 = blake3::Hasher::new();
    let hash = run_blake3(hasher_blake3, contents);
    //println!("hash value is : {:?}", hash.as_bytes().to_vec());
}

fn run_blake3(mut hasher: blake3::Hasher, contents: Vec<u8>) -> Hash {
    hasher.update(&OFFSET_ONE);
    hasher.update(&contents);
    hasher.finalize()
}
#[cfg(test)] // This annotation ensures that the following code is only compiled when testing
mod tests {
    #[test]
    fn test_run_blaake3() {
        use std::{
            fs::File,
            io::{BufReader, Read},
        };

        use crate::{run_blake3, TEST_FILE_PATH};
        //load then file contents
        let known_value: Vec<u8> = vec![193, 147, 145, 5, 39, 26, 61, 181, 134, 92, 230, 178, 35, 170, 228, 50, 243, 230, 64, 67, 68, 239, 45, 57, 144, 224, 228, 243, 242, 180, 45, 51];
        let file = File::open(TEST_FILE_PATH).unwrap();
        let mut reader = BufReader::new(file);
        let mut contents = Vec::new();
        reader.read_to_end(&mut contents).unwrap();

        let hasher_blake3 = blake3::Hasher::new();
        let hash = run_blake3(hasher_blake3, contents);
        let computed_value = hash.as_bytes().to_vec();
        assert_eq!(known_value, computed_value);
    }
}
