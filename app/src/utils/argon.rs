use argon2rs::argon2i_simple;
use rand::{self, Rng};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

const PASSWORD_LEN: usize = 128;

pub fn make_salt() -> String {
    let mut rng = rand::thread_rng();
    (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn make_hash(password: &str, salt: &str) -> Vec<u8> {
    argon2i_simple(password, salt).to_vec()
}
