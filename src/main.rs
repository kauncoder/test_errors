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
    use crate::{get_file_hashes, get_file_list, TEST_DIR};

    #[test]
    fn test_run_blake3() {
        //load then file contents
        let known_value: Vec<Vec<u8>> = vec![
            //     vec![
            //         246, 207, 76, 200, 105, 48, 50, 111, 16, 109, 151, 176, 250, 147, 234, 30, 41, 56,
            //         90, 215, 237, 134, 65, 202, 250, 61, 222, 125, 47, 59, 26, 55,
            //     ],
            //     vec![
            //         180, 33, 151, 17, 193, 222, 26, 14, 239, 90, 125, 170, 80, 242, 72, 30, 250, 79,
            //         115, 150, 216, 20, 229, 54, 97, 218, 159, 176, 158, 93, 95, 221,
            //     ],
            // ];
            vec![
                193, 147, 145, 5, 39, 26, 61, 181, 134, 92, 230, 178, 35, 170, 228, 50, 243, 230,
                64, 67, 68, 239, 45, 57, 144, 224, 228, 243, 242, 180, 45, 51,
            ],
            vec![
                64, 2, 61, 59, 61, 22, 48, 75, 149, 110, 221, 51, 232, 2, 29, 237, 201, 217, 81,
                200, 73, 11, 159, 208, 189, 167, 148, 161, 13, 30, 60, 68,
            ],
        ];
        //     vec![
        //         155, 133, 41, 126, 76, 192, 241, 95, 144, 235, 9, 252, 31, 212, 120, 230, 179, 219,
        //         57, 93, 96, 62, 247, 190, 215, 248, 118, 214, 140, 226, 159, 187,
        //     ],
        //     vec![
        //         238, 238, 225, 160, 96, 127, 5, 59, 26, 200, 76, 3, 232, 137, 19, 188, 135, 48,
        //         153, 189, 233, 23, 12, 128, 54, 140, 152, 194, 132, 81, 229, 6,
        //     ],
        //     vec![
        //         117, 98, 196, 237, 200, 184, 238, 94, 54, 57, 169, 221, 163, 253, 208, 186, 55,
        //         100, 42, 87, 249, 179, 235, 181, 180, 170, 255, 104, 250, 78, 3, 228,
        //     ],
        //     vec![
        //         91, 43, 237, 104, 177, 76, 57, 213, 1, 53, 249, 162, 208, 201, 21, 175, 245, 235,
        //         210, 98, 138, 184, 84, 25, 181, 246, 141, 72, 51, 166, 9, 65,
        //     ],
        //     vec![
        //         122, 58, 70, 144, 73, 18, 223, 64, 204, 135, 28, 67, 28, 179, 154, 5, 126, 35, 139,
        //         215, 111, 225, 109, 161, 129, 151, 203, 87, 79, 8, 216, 180,
        //     ],
        //     vec![
        //         177, 65, 85, 64, 58, 125, 38, 7, 114, 131, 199, 70, 170, 99, 130, 141, 218, 25, 26,
        //         115, 239, 160, 179, 92, 39, 53, 3, 13, 92, 4, 194, 115,
        //     ],
        // ];

        let file_list: Vec<String> = get_file_list(TEST_DIR);
        let computed_value = get_file_hashes(file_list)
            .iter()
            .map(|h| h.as_bytes().to_vec())
            .collect::<Vec<_>>();
        assert_eq!(known_value, computed_value);
    }
}
