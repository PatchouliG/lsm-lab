use std::fmt::{Display, Formatter};
use std::hash::Hash;

use crate::storage::{DecoderReader, EncoderWriter};
use std::io::Write;
use std::marker::PhantomData;

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

pub trait Key: Clone + Ord + Hash {
    fn from_str(s: &str) -> Self;
}

pub trait Value: Clone {
    fn from_str(s: &str) -> Self;
    fn to_str(&self) -> String;
}

pub trait Entry<K: Key, V: Value>: Sized {
    fn get_key(&self) -> K;
    fn get_value(&self) -> V;
}

pub trait KV_iterator<K: Key, V: Value>: Iterator<Item = (K, V)> {}

const KEY_SIZE: usize = 32;

#[derive(Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct KeyImp {
    bytes: Vec<u8>,
}

impl Clone for KeyImp {
    fn clone(&self) -> Self {
        KeyImp {
            bytes: Vec::from(self.bytes.as_slice()),
        }
    }
}

impl KeyImp {
    pub fn new(bytes: &[u8]) -> Self {
        let mut b = Vec::from(bytes);
        b.copy_from_slice(bytes);
        KeyImp { bytes: b }
    }
    pub fn to_str(&self) -> String {
        String::from_utf8(Vec::from(&self.bytes[0..self.bytes.len()])).unwrap()
    }
}

impl Key for KeyImp {
    fn from_str(s: &str) -> KeyImp {
        let bytes = Vec::from(s.as_bytes());

        KeyImp { bytes: bytes }
    }
}

impl Display for KeyImp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = String::from_utf8(Vec::from(&self.bytes[0..self.bytes.len()]));
        f.write_str(res.unwrap().as_str())
    }
}

impl KeyImp {}

#[derive(Clone, Debug)]
pub struct ValueImp {
    v: Vec<u8>,
}

impl ValueImp {
    pub fn new(value: &[u8]) -> Self {
        ValueImp {
            v: Vec::from(value),
        }
    }
}

impl PartialEq for ValueImp {
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

impl Eq for ValueImp {}

impl Value for ValueImp {
    fn from_str(s: &str) -> ValueImp {
        let mut v = Vec::new();
        for (_, b) in s.as_bytes().iter().enumerate() {
            v.push(*b);
        }
        ValueImp { v }
    }

    fn to_str(&self) -> String {
        String::from_utf8(self.v.clone()).unwrap()
    }
}

pub fn encode_kv<EW: EncoderWriter>(k: KeyImp, v: ValueImp, w: &mut EW) {
    w.write(&convert_usize_to_u8_array(k.bytes.len())).unwrap();
    w.write(&k.bytes).unwrap();
    w.write(&convert_usize_to_u8_array(v.v.len())).unwrap();
    w.write(&v.v).unwrap();
}

pub fn decode_kv<DR: DecoderReader>(r: &mut DR) -> (KeyImp, ValueImp) {
    let key_size = r.read_size();
    let key_content = r.read(key_size);
    let key = KeyImp::new(key_content);
    let value_size = r.read_size();
    let value_content = r.read(value_size);
    let value = ValueImp::new(value_content);
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
    use crate::common::{decode_kv, encode_kv, Key, KeyImp, Value, ValueImp};
    use crate::storage::{DecoderReaderMemoryImp, EncoderWriterMemoryImp};

    #[test]
    pub fn test_key() {
        let a = KeyImp::from_str("123");
        let b = KeyImp::from_str("123");
        let c = KeyImp::from_str("12d");
        let d = KeyImp::from_str("125");
        let e = KeyImp::from_str("12");
        assert_eq!(a, b);
        assert!(a < c);
        assert!(a < d);
        assert!(a > e);
    }

    #[test]
    pub fn test_value() {
        let a = ValueImp::from_str("abc");
        assert_eq!("abc", a.to_str());
    }
    #[test]
    pub fn test_encode_decode() {
        let a = KeyImp::from_str("key");
        let b = ValueImp::from_str("value");
        let mut encode = EncoderWriterMemoryImp::new();
        encode_kv(a.clone(), b.clone(), &mut encode);
        let vec = encode.to_vec();
        let mut decoder = DecoderReaderMemoryImp::new(vec);
        let (c, d) = decode_kv(&mut decoder);
        assert_eq!(a, c);
        assert_eq!(b, d);
    }
}
