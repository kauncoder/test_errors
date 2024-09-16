use blake3::Hash;
use std::{
    fs::{self},
    io::Read,
    path::Path,
};

pub const _OFFSET_ONE: [u8; 4] = 1u32.to_le_bytes();
//pub const TEST_DIR: &str = "./testfiles";

pub fn run_blake3() {
    let file_list: Vec<String> = get_file_list("testfiles_bad");
    let (file_hash_list, file_content_list) = get_file_hashes(file_list);
    let file_hashes_as_hex = file_hash_list
        .iter()
        .map(|h| h.to_hex())
        .collect::<Vec<_>>();
    println!(
        "file contents :{:?} and hashes : {:?}",
        file_content_list, file_hashes_as_hex
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

#[allow(dead_code)]
pub fn get_file_hashes_with_cleaning(file_list: Vec<String>) -> (Vec<Hash>, Vec<Vec<u8>>) {
    //read files and return vec of file hashes
    let mut file_hash_list: Vec<Hash> = Vec::new();
    let mut _file_content_list: Vec<Vec<u8>> = Vec::new();
    for file in file_list.clone() {
        let mut file = fs::File::open(Path::new(&file)).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        //remove windows carraige return from file \r
        content = content.replace("\r", "");
        let file_content = content.as_bytes().to_vec();
        let mut hash = blake3::Hasher::new();
        hash.update(&file_content);
        let hash = hash.finalize();
        file_hash_list.push(hash);
    }
    (file_hash_list, _file_content_list)
}

#[cfg(test)] // This annotation ensures that the following code is only compiled when testing
mod tests {
    use crate::blake_hash::{get_file_hashes, get_file_hashes_with_cleaning, get_file_list};

    #[test]
    fn test_run_good_after_cleaning() {
        let known_value: Vec<String> = vec![
            "929c621dbb080958e9f619d35365a1dd700cd9ae1493a42bc911b62c20af78f9".to_string(),
            "7f0c5586a42a297c2d89ef2a6120cb797713b9417d890a58c36f5005474c3f99".to_string(),
            "0f50752d0560740475b9e619c5ca9ead1e6a50717f5f00ebca57743bb118a1c6".to_string(),
            "4219b2ccca8cf45cfef0799ae2dd7642df3b5cd2031b9ce130567ba21be5b75a".to_string(),
        ];

        let file_list: Vec<String> = get_file_list("testfiles_bad");
        let (computed_hashes, _computed_contents) = get_file_hashes_with_cleaning(file_list);
        let computed_value = computed_hashes
            .iter()
            .map(|h| h.to_hex().to_string())
            .collect::<Vec<_>>();
        assert_eq!(known_value, computed_value);
    }

    #[test]
    fn test_run_good_with_good_files() {
        let known_value: Vec<String> = vec![
            "4ccfcbd4d36806efc03940db9d439fcb9fb2faa9b5cfae6b96bc3a779329b479".to_string(),
            "b67ec6b1281891a624550068e20d09ff734e923bb675866c50b127a9bc823afe".to_string(),
        ];
        let file_list: Vec<String> = get_file_list("testfiles_good");
        let (computed_hashes, _computed_contents) = get_file_hashes_with_cleaning(file_list);
        let computed_value = computed_hashes
            .iter()
            .map(|h| h.to_hex().to_string())
            .collect::<Vec<_>>();
        assert_eq!(known_value, computed_value);
    }

    #[test]
    fn test_run_bad() {
        let known_value: Vec<String> = vec![
            "929c621dbb080958e9f619d35365a1dd700cd9ae1493a42bc911b62c20af78f9".to_string(),
            "7f0c5586a42a297c2d89ef2a6120cb797713b9417d890a58c36f5005474c3f99".to_string(),
            "0f50752d0560740475b9e619c5ca9ead1e6a50717f5f00ebca57743bb118a1c6".to_string(),
            "4219b2ccca8cf45cfef0799ae2dd7642df3b5cd2031b9ce130567ba21be5b75a".to_string(),
        ];

        let file_list: Vec<String> = get_file_list("testfiles_bad");
        let (computed_hashes, _computed_contents) = get_file_hashes(file_list);
        let computed_value = computed_hashes
            .iter()
            .map(|h| h.to_hex().to_string())
            .collect::<Vec<_>>();
        assert_eq!(known_value, computed_value);
    }
}
