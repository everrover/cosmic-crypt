pub struct EncryptionDecryption {
    key: String,
    algorithm: String,
    otherParams: HashMap<String, String>,
}

impl EncryptionDecryption {
    pub fn new(key: String, algorithm: String) -> EncryptionDecryption {
        EncryptionDecryption {
            key,
            algorithm,
            otherParams: HashMap::new(),
        }
    }

    pub fn encrypt(&self, data: Vec<u8>) -> Vec<u8> {
        let mut encrypted_data: Vec<u8> = Vec::new();
        match self.algorithm.as_str() {
            "xor" => {
                for byte in data {
                    encrypted_data.push(byte ^ self.key.as_bytes()[0]);
                }
            },
            "aes256" => {
                match self.otherParams.get("cipher") {
                    Some(cipher) => {
                        // AES 256 encryption
                        // encrypted_data = aes256_encrypt(data, self.key, cipher);
                    },
                    None => {
                        println!("Cipher not provided");
                        panic!();
                    }
                }
                // AES 256 encryption
                // encrypted_data = aes256_encrypt(data, self.key);
            },
            _ => {
                println!("Algorithm not supported");
            }
        }
        encrypted_data
    }

    pub fn decrypt(&self, data: Vec<u8>) -> Vec<u8> {
        let mut decrypted_data: Vec<u8> = Vec::new();
        match self.algorithm.as_str() {
            "xor" => {
                for byte in data {
                    decrypted_data.push(byte ^ self.key.as_bytes()[0]);
                }
            }
            _ => {
                println!("Algorithm not supported");
            }
        }
        decrypted_data
    }
}