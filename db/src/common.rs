use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::io::Write;
use std::marker::PhantomData;

use crate::storage::{DecoderReader, EncoderWriter};

struct LRU_Cache<K, E: Clone> {
    k: PhantomData<K>,
    e: PhantomData<E>,
}

impl<K, E: Clone> LRU_Cache<K, E> {
    fn put(&mut self, k: K, e: E) {}

    fn get(&self, k: K) -> &E {
        todo!()
    }
}

const KEY_SIZE: usize = 32;

#[derive(Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Key {
    bytes: Vec<u8>,
}

impl Clone for Key {
    fn clone(&self) -> Self {
        Key {
            bytes: Vec::from(self.bytes.as_slice()),
        }
    }
}

impl Key {
    pub fn new(bytes: &[u8]) -> Self {
        let mut b = Vec::from(bytes);
        b.copy_from_slice(bytes);
        Key { bytes: b }
    }
    pub fn to_str(&self) -> String {
        String::from_utf8(Vec::from(&self.bytes[0..self.bytes.len()])).unwrap()
    }

    pub fn from_str(s: &str) -> Key {
        let bytes = Vec::from(s.as_bytes());

        Key { bytes: bytes }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = String::from_utf8(Vec::from(&self.bytes[0..self.bytes.len()]));
        f.write_str(res.unwrap().as_str())
    }
}

#[derive(Clone, Debug)]
pub struct Value {
    v: Vec<u8>,
}

impl Value {
    pub fn new(value: &[u8]) -> Self {
        Value {
            v: Vec::from(value),
        }
    }

    pub fn from_str(s: &str) -> Value {
        let mut v = Vec::new();
        for (_, b) in s.as_bytes().iter().enumerate() {
            v.push(*b);
        }
        Value { v }
    }
    pub fn to_str(&self) -> String {
        String::from_utf8(self.v.clone()).unwrap()
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        if self.v.len() != other.v.len() {
            return false;
        }
        for (i, v) in self.v.iter().enumerate() {
            if v != other.v.get(i).unwrap() {
                return false;
            }
        }
        true
    }
}

impl Eq for Value {}

pub fn encode_kv<EW: EncoderWriter>(k: Key, v: Value, w: &mut EW) {
    w.write(&convert_usize_to_u8_array(k.bytes.len())).unwrap();
    w.write(&k.bytes).unwrap();
    w.write(&convert_usize_to_u8_array(v.v.len())).unwrap();
    w.write(&v.v).unwrap();
}

pub fn decode_kv<DR: DecoderReader>(r: &mut DR) -> (Key, Value) {
    let key_size = r.read_size();
    let key_content = r.read(key_size);
    let key = Key::new(key_content);
    let value_size = r.read_size();
    let value_content = r.read(value_size);
    let value = Value::new(value_content);
    (key, value)
}
pub fn convert_u8_array_to_usize(s: &[u8]) -> usize {
    let mut res: usize = 0;
    let mut m: usize = 1;
    for i in s {
        let h = *i as usize;
        res += h * m;
        m *= 256;
    }
    res
}
pub fn convert_usize_to_u8_array(mut s: usize) -> [u8; 4] {
    let mut res: [u8; 4] = [0, 0, 0, 0];
    for i in 0..4 {
        res[i] = (s % 256) as u8;
        s = s / 256;
    }
    res
}

#[cfg(test)]
mod test {
    use crate::common::{decode_kv, encode_kv, Key, Value};
    use crate::storage::{DecoderReaderMemoryImp, EncoderWriterMemoryImp};

    #[test]
    pub fn test_key() {
        let a = Key::from_str("123");
        let b = Key::from_str("123");
        let c = Key::from_str("12d");
        let d = Key::from_str("125");
        let e = Key::from_str("12");
        assert_eq!(a, b);
        assert!(a < c);
        assert!(a < d);
        assert!(a > e);
    }

    #[test]
    pub fn test_value() {
        let a = Value::from_str("abc");
        assert_eq!("abc", a.to_str());
    }
    #[test]
    pub fn test_encode_decode() {
        let a = Key::from_str("key");
        let b = Value::from_str("value");
        let mut encode = EncoderWriterMemoryImp::new();
        encode_kv(a.clone(), b.clone(), &mut encode);
        let vec = encode.to_vec();
        let mut decoder = DecoderReaderMemoryImp::new(vec);
        let (c, d) = decode_kv(&mut decoder);
        assert_eq!(a, c);
        assert_eq!(b, d);
    }
}
