use std::marker::PhantomData;
use rc4::{Rc4, Key, KeyInit, StreamCipher};
use rc4::{consts::*};
/*
 * 
 * Using RC4 here ...
 * * Fast, bidirectional stream cipher
 * * Doesn't require a key exchange/any other overhead
 * 
 * [Now obsolete]
 */

pub struct Payload<'a> {
    data: Vec<u8>,
    size: usize,
    rc4Pack: Rc4<U6>,
    _marker: PhantomData<&'a ()>,
}

impl Payload<'static> {
    pub fn new(data: Vec<u16>, key: String) -> Payload<'static> {
        let datalen = data.len();
        let rc4key = Key::<U6>::from_slice(key.as_bytes());
        let mut rc4 = Rc4::new(rc4key);
        let mut new_data: Vec<u8> = Vec::new();
        for (i, twobyte) in data.iter().enumerate() {
            new_data.push((twobyte & 0xFF) as u8);
            new_data.push((twobyte >> 8) as u8);
        }
        return Payload {
            data: new_data,
            size: datalen,
            rc4Pack: rc4,
            _marker: PhantomData,
        };
    }

    pub fn encrypt(&mut self) -> Vec<u16> {
        let mut new_data: Vec<u16> = Vec::new();
        self.rc4Pack.apply_keystream(&mut self.data);
        for i in 0..self.data.len() {
            new_data.push((self.data[i] as u16) << 8 | (self.data[i + 1] as u16));
        }
        return new_data;
    }

    pub fn decrypt(&mut self) -> Vec<u16> {
        let mut new_data: Vec<u16> = Vec::new();
        let mut data = self.data.clone();
        self.rc4Pack.apply_keystream(&mut data);
        for i in 0..data.len() {
            new_data.push((data[i] as u16) << 8 | (data[i + 1] as u16));
        }
        return new_data;
    }
}



pub fn encrypt(data: Vec<u16>, key: String) -> Vec<u16> {
    let rc4key = Key::<U6>::from_slice(key.as_bytes());
    let mut rc4 = Rc4::new(rc4key);

    let mut new_data: Vec<u16> = Vec::new();
    let mut byte_data: Vec<u8> = Vec::new();
    for twobyte in data.iter() {
        byte_data.push((twobyte & 0xFF) as u8);
        byte_data.push((twobyte >> 8) as u8);
    }
    rc4.apply_keystream(&mut byte_data);
    for i in 0..byte_data.len() {
        new_data.push((byte_data[i] as u16) << 8 | (byte_data[i + 1] as u16));
    }
    return new_data;
}

pub fn decrypt(data: Vec<u16>, key: String) -> Vec<u16> {
    let rc4key = Key::<U6>::from_slice(key.as_bytes());
    let mut rc4 = Rc4::new(rc4key);

    let mut new_data: Vec<u16> = Vec::new();
    let mut byte_data: Vec<u8> = Vec::new();
    for mut twobyte in data.iter() {
        byte_data.push((twobyte & 0xFF) as u8);
        byte_data.push((twobyte >> 8) as u8);
    }
    rc4.apply_keystream(&mut byte_data);
    for i in 0..byte_data.len() {
        new_data.push((byte_data[i] as u16) << 8 | (byte_data[i + 1] as u16));
    }
    return new_data;
}