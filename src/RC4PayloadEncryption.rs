

/*
 * Payload Encryption :: As title suggests. Evades signature based detection.
 *
 * Entire malware may/may not be encrypted.
 *
 * Increases entropy
 * High entropy is flagged as suspicious at times
 *
 * XOR, AES, RC4, RSA, etc.
 * 
 * Using XOR here ... 
 * * Faster
 * * Doesn't require a key exchange/any other overhead
 * 
 *
 */

pub struct Payload<'a> {
    data: Vec<u16>,
    size: usize,
    key: String,
    key_bytes: &'a[u16],
    // functionPtr: fn(Vec<u8>) -> Vec<u8>,
    // encryption: EncryptionDecryption, // will use later on
}

impl Payload {
    pub fn new(data: Vec<u16>, key: String) -> Payload {
        let datalen = data.len();
        let keyBytes = key.encode_utf16().collect();
        Payload {
            data,
            size: datalen,
            key,
            key_bytes: keyBytes,
        }
    }

    pub fn encrypt(&self) -> Vec<u16> {
        let mut new_data:Vec<u16> = Vec::new();
        for (i, byte) in self.data.iter().enumerate() {
            new_data.push(byte ^ self.key_bytes[i % self.key_bytes.len()]);
        }
        return new_data;
    }

    pub fn decrypt(&self) -> Vec<u16> {
        let mut new_data:Vec<u16> = Vec::new();
        for (i, byte) in self.data.iter().enumerate() {
            new_data.push(byte ^ self.key_bytes[i % self.key_bytes.len()]);
        }
        return new_data;
    }
}



pub fn encrypt(data: Vec<u16>, key_bytes: &[u16]) -> Vec<u16> {
    // self.encryption.decrypt(data)
    let mut new_data:Vec<u16> = Vec::new();
    for (i, byte) in data.iter().enumerate() {
        new_data.push(byte ^ key_bytes[i%key_bytes.len()]);
    }
    return new_data;
}

pub fn decrypt(data: Vec<u16>, key_bytes: &[u16]) -> Vec<u16> {
    // self.encryption.decrypt(data)
    let mut new_data:Vec<u16> = Vec::new();
    for (i, byte) in data.iter().enumerate() {
        new_data.push(byte ^ key_bytes[i%key_bytes.len()]);
    }
    return new_data;
}