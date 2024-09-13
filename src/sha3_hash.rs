use sha3::{Digest, Sha3_256};
use std::{
    fs::{self},
    io::Read,
    path::Path,
};

pub const OFFSET_ONE: [u8; 4] = 1u32.to_le_bytes();
//pub const TEST_DIR: &str = "./testfiles";

pub fn run_sha3() {
    let file_list: Vec<String> = get_file_list("testfiles_good");
    let (file_hash_list, file_content_list) = get_file_hashes(file_list);
    println!(
        "sha3 file contents :{:?} and hashes : {:?}",
        file_content_list, file_hash_list
    );
}

pub fn get_file_list(upload_dir: &str) -> Vec<String> {
    let mut file_list: Vec<String> = Vec::new(); //replace with more concrete type
    let dir_path = format!("./{}", upload_dir);
    let dir = Path::new(&dir_path);
    let entries = fs::read_dir(dir).unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        file_list.push(path.display().to_string())
    }
    file_list.sort();
    file_list
}

pub fn get_file_hashes(file_list: Vec<String>) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    //read files and return vec of file hashes
    let mut file_hash_list: Vec<Vec<u8>> = Vec::new();
    let mut file_content_list: Vec<Vec<u8>> = Vec::new();
    for file in file_list.clone() {
        let mut file = fs::File::open(Path::new(&file)).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        //remove windows carraige return from file \r
        // content = content.replace("\r", ""); //testing with removing carriage return
        let file_content = content.as_bytes().to_vec();
        file_content_list.push(file_content.clone());
        let mut hash = Sha3_256::new();
        hash.update(&OFFSET_ONE);
        hash.update(&file_content);
        let hash = hash.finalize().to_vec();
        file_hash_list.push(hash);
    }
    (file_hash_list, file_content_list)
}

#[cfg(test)] // This annotation ensures that the following code is only compiled when testing
mod tests {
    use crate::sha3_hash::{get_file_hashes, get_file_list};

    #[test]
    fn test_run_good() {
        //load then file contents
        let known_value: Vec<Vec<u8>> = vec![
            vec![
                186, 72, 143, 56, 16, 195, 144, 68, 190, 201, 188, 101, 189, 42, 111, 254, 19, 36,
                255, 177, 14, 138, 130, 100, 119, 120, 209, 193, 253, 3, 247, 240,
            ],
            vec![
                41, 45, 100, 215, 124, 230, 200, 22, 70, 215, 237, 66, 231, 175, 214, 205, 155,
                186, 19, 235, 207, 179, 77, 211, 189, 224, 96, 243, 227, 179, 91, 177,
            ],
        ];
        let file_list: Vec<String> = get_file_list("testfiles_good");
        let (computed_value, _) = get_file_hashes(file_list);
        assert_eq!(known_value, computed_value);
    }

    #[test]
    fn test_run_bad() {
        //load then file contents
        let known_file_content: Vec<Vec<u8>> = vec![
            vec![
                116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 115, 105, 109, 112, 108, 101, 32,
                116, 120, 116, 32, 102, 105, 108, 101, 32, 116, 111, 32, 99, 104, 101, 99, 107, 32,
                117, 112, 108, 111, 97, 100, 47, 100, 111, 119, 110, 108, 111, 97, 100, 10, 10,
                110, 101, 119, 32, 108, 105, 110, 101,
            ],
            vec![
                116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 115, 105, 109, 112, 108, 101, 32,
                116, 120, 116, 32, 102, 105, 108, 101, 32, 116, 111, 32, 99, 104, 101, 99, 107, 32,
                117, 112, 108, 111, 97, 100, 47, 100, 111, 119, 110, 108, 111, 97, 100, 32, 50, 10,
                10, 110, 101, 119, 32, 108, 105, 110, 101, 32, 50,
            ],
        ];

        let known_value: Vec<Vec<u8>> = vec![
            vec![
                179, 241, 121, 6, 162, 151, 7, 77, 8, 163, 250, 59, 196, 190, 94, 25, 203, 74, 185,
                59, 168, 248, 105, 93, 58, 167, 124, 40, 228, 237, 22, 51,
            ],
            vec![
                223, 143, 202, 202, 94, 78, 218, 167, 102, 219, 197, 216, 211, 181, 56, 135, 40,
                59, 187, 245, 61, 99, 44, 190, 51, 79, 112, 167, 133, 67, 36, 209,
            ],
        ];

        let file_list: Vec<String> = get_file_list("testfiles_bad");
        let (computed_value, computed_contents) = get_file_hashes(file_list);
        assert_eq!(known_file_content, computed_contents);
        assert_eq!(known_value, computed_value);
        //assert_eq!(known_value, computed_value);
    }
}
