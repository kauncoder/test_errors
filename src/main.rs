use blake3::Hash;
use std::{
    fs::{self},
    path::Path,
};

pub const OFFSET_ONE: [u8; 4] = 1u32.to_le_bytes();
pub const TEST_DIR: &str = "./testfiles";

fn main() {
    let file_list: Vec<String> = get_file_list(TEST_DIR);
    let file_hashes = get_file_hashes(file_list)
        .iter()
        .map(|h| h.as_bytes().to_vec())
        .collect::<Vec<_>>();
    println!("file hashes : {:?}", file_hashes);
}

fn get_file_list(upload_dir: &str) -> Vec<String> {
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

fn get_file_hashes(file_list: Vec<String>) -> Vec<Hash> {
    //read files and return vec of file hashes
    let mut file_hash_list: Vec<Hash> = Vec::new();
    for file in file_list.clone() {
        let file_content = std::fs::read(file.clone()).unwrap();
        let mut hash = blake3::Hasher::new();
        hash.update(&OFFSET_ONE);
        hash.update(&file_content);
        let hash = hash.finalize();
        file_hash_list.push(hash);
    }
    file_hash_list
}

#[cfg(test)] // This annotation ensures that the following code is only compiled when testing
mod tests {
    use crate::{get_file_hashes, get_file_list};

    #[test]
    fn test_run_good() {
        //load then file contents
        let known_value: Vec<Vec<u8>> = vec![
            vec![
                193, 147, 145, 5, 39, 26, 61, 181, 134, 92, 230, 178, 35, 170, 228, 50, 243, 230,
                64, 67, 68, 239, 45, 57, 144, 224, 228, 243, 242, 180, 45, 51,
            ],
            vec![
                64, 2, 61, 59, 61, 22, 48, 75, 149, 110, 221, 51, 232, 2, 29, 237, 201, 217, 81,
                200, 73, 11, 159, 208, 189, 167, 148, 161, 13, 30, 60, 68,
            ],
        ];
        let file_list: Vec<String> = get_file_list("testfiles_good");
        let computed_value = get_file_hashes(file_list)
            .iter()
            .map(|h| h.as_bytes().to_vec())
            .collect::<Vec<_>>();
        assert_eq!(known_value, computed_value);
    }

    #[test]
    fn test_run_bad() {
        //load then file contents
        let known_value: Vec<Vec<u8>> = vec![
            vec![
                246, 207, 76, 200, 105, 48, 50, 111, 16, 109, 151, 176, 250, 147, 234, 30, 41, 56,
                90, 215, 237, 134, 65, 202, 250, 61, 222, 125, 47, 59, 26, 55,
            ],
            vec![
                180, 33, 151, 17, 193, 222, 26, 14, 239, 90, 125, 170, 80, 242, 72, 30, 250, 79,
                115, 150, 216, 20, 229, 54, 97, 218, 159, 176, 158, 93, 95, 221,
            ],
        ];

        let file_list: Vec<String> = get_file_list("testfiles_bad");
        let computed_value = get_file_hashes(file_list)
            .iter()
            .map(|h| h.as_bytes().to_vec())
            .collect::<Vec<_>>();
        assert_eq!(known_value, computed_value);
    }
}
