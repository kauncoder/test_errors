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
    println!("hash value is : {}", hash.to_hex());
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
        let known_value =
            "c1939105271a3db5865ce6b223aae432f3e6404344ef2d3990e0e4f3f2b42d33".to_string();
        let file = File::open(TEST_FILE_PATH).unwrap();
        let mut reader = BufReader::new(file);
        let mut contents = Vec::new();
        reader.read_to_end(&mut contents).unwrap();

        let hasher_blake3 = blake3::Hasher::new();
        let hash = run_blake3(hasher_blake3, contents);
        let computed_value = format!("{}", hash.to_hex());
        assert_eq!(known_value, computed_value);
    }
}
