use aes::cipher::{BlockEncryptMut, generic_array::GenericArray};
use cbc::cipher::KeyIvInit;
use openssl::pkcs5::pbkdf2_hmac;
use openssl::hash::MessageDigest;
use hex;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;

fn hex_string_to_u8_array(hex_string: &str, length: usize) -> Vec<u8> {
    let mut padded_hex_string = String::from(hex_string);
    if padded_hex_string.len() % 2 != 0 {
        // Add a leading zero if the string has an odd length
        padded_hex_string = format!("0{}", padded_hex_string);
    }
    while padded_hex_string.len() < length * 2 {
        // Pad with zeros on the left until the string reaches the desired length
        padded_hex_string = format!("{}{}", "0", padded_hex_string);
    }
    hex::decode(&padded_hex_string).unwrap()
}

fn encrypt_aes(data: &mut [u8], key: &[u8], iv: &[u8]) -> [u8; 16] {
    let mut output = [0u8; 16];
    let mut block = GenericArray::clone_from_slice(data);
    Aes128CbcEnc::new_from_slices(key, iv).unwrap().encrypt_block_mut(&mut block);
    for i in 0..16 {
        output[i] = block.as_slice()[i];
    }
    return output;
}

pub fn generate_key(title_id: &str) -> String {
    let keygen_pw: [u8; 6] = [0x6d, 0x79, 0x70, 0x61, 0x73, 0x73];
    let common_key: [u8; 16] = [0xd7, 0xb0, 0x04, 0x02, 0x65, 0x9b, 0xa2, 0xab, 0xd2, 0xcb, 0x0d, 0xb2, 0x7f, 0xa2, 0xb6, 0x56];

    let mut ret = String::with_capacity(32);
    let tmp = title_id.trim_start_matches("00");

    let mut h = String::from("fd040105060b111c2d49"); // Keygen Secret
    h.push_str(tmp);

    let bhl = h.len() / 2;
    let mut bh = vec![0; bhl];
    for (i, j) in (0..h.len()).step_by(2).zip(0..bhl) {
        bh[j] = (h.as_bytes()[i] % 32 + 9) % 25 * 16 + (h.as_bytes()[i + 1] % 32 + 9) % 25;
    }
    
    let md5sum = md5::compute(bh).0;

    let mut key = [0u8; 16];
    pbkdf2_hmac(&keygen_pw, &md5sum, 20, MessageDigest::sha1(), &mut key).unwrap();

    let mut iv = [0u8; 16];
    let tid_array = hex_string_to_u8_array(title_id, 8);
    for i in 0..tid_array.len() {
        iv[i] = tid_array[i];
    }
    iv[8..].fill(0);

    for b in iv.iter() {
        println!("{:0x}", b);
    }

    let encrypted_key = encrypt_aes(&mut key, &common_key, &iv);

    for b in encrypted_key.iter() {
        ret.push_str(&format!("{:02x}", b));
    }

    println!("{}", ret);

    return ret;
}