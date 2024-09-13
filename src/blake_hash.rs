use blake3::Hash;
use std::{
    fs::{self},
    io::Read,
    path::Path,
};

pub const _OFFSET_ONE: [u8; 4] = 1u32.to_le_bytes();
//pub const TEST_DIR: &str = "./testfiles";

pub fn run_blake3() {
    let file_list: Vec<String> = get_file_list("testfiles_good");
    let (file_hash_list, file_content_list) = get_file_hashes(file_list);
    let file_hashes = file_hash_list
        .iter()
        .map(|h| h.as_bytes().to_vec())
        .collect::<Vec<_>>();
    println!(
        "file contents :{:?} and hashes : {:?}",
        file_content_list, file_hashes
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

pub fn get_file_hashes(file_list: Vec<String>) -> (Vec<Hash>, Vec<Vec<u8>>) {
    //read files and return vec of file hashes
    let mut file_hash_list: Vec<Hash> = Vec::new();
    let mut _file_content_list: Vec<Vec<u8>> = Vec::new();
    for file in file_list.clone() {
        let mut file = fs::File::open(Path::new(&file)).unwrap();
        // let mut content = String::new();
        // file.read_to_string(&mut content).unwrap();
        // //remove windows carraige return from file \r
        // content = content.replace("\r", "");
        // let file_content = content.as_bytes().to_vec();
        // file_content_list.push(file_content.clone());
        // let mut hash = blake3::Hasher::new();
        // hash.update(&OFFSET_ONE);
        // hash.update(&file_content);
        // let hash = hash.finalize();
        // file_hash_list.push(hash);
        // Buffer to read chunks of file
        let mut buffer = [0u8; 4096];
        let mut hasher = blake3::Hasher::new();

        // Read file in chunks
        loop {
            let bytes_read = file.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            // Feed the read bytes into the hash function
            hasher.update(&buffer[..bytes_read]);
        }
        let hash = hasher.finalize();
        file_hash_list.push(hash);
    }
    (file_hash_list, _file_content_list)
}

#[cfg(test)] // This annotation ensures that the following code is only compiled when testing
mod tests {
    use crate::blake_hash::{get_file_hashes, get_file_list};

    #[test]
    fn test_run_good() {
        //load then file contents
        let known_value: Vec<Vec<u8>> = vec![
            vec![
                76, 207, 203, 212, 211, 104, 6, 239, 192, 57, 64, 219, 157, 67, 159, 203, 159, 178,
                250, 169, 181, 207, 174, 107, 150, 188, 58, 119, 147, 41, 180, 121,
            ],
            vec![
                182, 126, 198, 177, 40, 24, 145, 166, 36, 85, 0, 104, 226, 13, 9, 255, 115, 78,
                146, 59, 182, 117, 134, 108, 80, 177, 39, 169, 188, 130, 58, 254,
            ],
        ];
        let file_list: Vec<String> = get_file_list("testfiles_good");
        let (computed_hashes, _) = get_file_hashes(file_list);
        let computed_value = computed_hashes
            .iter()
            .map(|h| h.as_bytes().to_vec())
            .collect::<Vec<_>>();
        assert_eq!(known_value, computed_value);
    }

    #[test]
    fn test_run_bad() {
        //load then file contents
        let _known_file_content: Vec<Vec<u8>> = vec![
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
                127, 12, 85, 134, 164, 42, 41, 124, 45, 137, 239, 42, 97, 32, 203, 121, 119, 19,
                185, 65, 125, 137, 10, 88, 195, 111, 80, 5, 71, 76, 63, 153,
            ],
            vec![
                66, 25, 178, 204, 202, 140, 244, 92, 254, 240, 121, 154, 226, 221, 118, 66, 223,
                59, 92, 210, 3, 27, 156, 225, 48, 86, 123, 162, 27, 229, 183, 90,
            ],
        ];
        // let known_value: Vec<Vec<u8>> = vec![
        //     vec![
        //         228, 101, 232, 48, 200, 194, 4, 2, 194, 119, 62, 73, 137, 5, 214, 56, 179, 176,
        //         223, 102, 137, 218, 201, 138, 51, 1, 49, 214, 207, 70, 84, 40,
        //     ],
        //     vec![
        //         8, 210, 243, 221, 251, 130, 213, 48, 131, 91, 51, 207, 141, 104, 211, 125, 97, 74,
        //         253, 57, 125, 181, 32, 17, 110, 75, 171, 130, 22, 42, 27, 235,
        //     ],
        // ];

        let file_list: Vec<String> = get_file_list("testfiles_bad");
        let (computed_hashes, _computed_contents) = get_file_hashes(file_list);
        let computed_value = computed_hashes
            .iter()
            .map(|h| h.as_bytes().to_vec())
            .collect::<Vec<_>>();
        //  assert_eq!(known_file_content, computed_contents);
        assert_eq!(known_value, computed_value);
        //assert_eq!(known_value, computed_value);
    }
}
