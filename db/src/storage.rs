use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::slice::SliceIndex;

use derive_getters::{Dissolve, Getters};

use crate::common::{convert_u8_array_to_usize, convert_usize_to_u8_array, Key, KeyImp, ValueImp};
use std::cmp::Ordering;

const BUFFER_SIZE: usize = 8 * 4 * 1024;
pub trait EncoderWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<()>;
    fn write_size(&mut self, size: usize) -> std::io::Result<()>;
    fn flush(&mut self) -> std::io::Result<()>;
}
#[derive(Getters)]
pub struct BlockMeta<K: Key> {
    start: K,
    end: K,
    offset: usize,
}
impl<K: Key> BlockMeta<K> {
    pub fn range(&self) -> (&K, &K) {
        (&self.start, &self.end)
    }
}

impl<K: Key> PartialEq for BlockMeta<K> {
    fn eq(&self, other: &Self) -> bool {
        self.start().eq(&other.start()) && (self.end().eq(&other.end()))
    }
}

impl<K: Key> PartialOrd for BlockMeta<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start().partial_cmp(&other.start())
    }
}

pub trait DecoderReader {
    fn read_size(&mut self) -> usize;
    fn read(&mut self, size: usize) -> &[u8];
}
pub struct EncoderWriterMemoryImp {
    v: Vec<u8>,
}

impl EncoderWriterMemoryImp {
    pub fn new() -> Self {
        EncoderWriterMemoryImp { v: Vec::new() }
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.v
    }
}

impl EncoderWriter for EncoderWriterMemoryImp {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<()> {
        for i in buf {
            self.v.push(*i);
        }
        Ok(())
    }
    fn write_size(&mut self, size: usize) -> std::io::Result<()> {
        let u8_size = convert_usize_to_u8_array(size);
        for i in u8_size {
            self.v.push(i);
        }
        Ok(())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct DecoderReaderMemoryImp {
    v: Vec<u8>,
    index: usize,
}

impl DecoderReaderMemoryImp {
    pub fn new(v: Vec<u8>) -> Self {
        DecoderReaderMemoryImp { v, index: 0 }
    }
}

impl DecoderReader for DecoderReaderMemoryImp {
    fn read_size(&mut self) -> usize {
        let a = &self.v.as_slice()[self.index..self.index + 4];
        self.index += 4;
        convert_u8_array_to_usize(a)
    }

    fn read(&mut self, size: usize) -> &[u8] {
        let res = &self.v.as_slice()[self.index..self.index + size];
        self.index += size;
        res
    }
}
pub struct DecoderReaderImp {}

impl DecoderReader for DecoderReaderImp {
    fn read_size(&mut self) -> usize {
        todo!()
    }

    fn read(&mut self, size: usize) -> &[u8] {
        todo!()
    }
}

// abstract file
pub trait StorageReader {
    fn read(&mut self) -> (&Vec<u8>, usize);
    fn read_at(&mut self, offset: usize, size: usize) -> (&Vec<u8>, usize);
}

// abstract file
pub trait StorageWriter {
    fn append(&mut self, data: &[u8]);
    fn write(&mut self, offset: usize, data: &[u8]);
    fn flush(&mut self);
}

struct StorageReaderImp {
    r: BufReader<File>,
    buffer: Vec<u8>,
}

impl StorageReaderImp {
    pub fn new(path: &str) -> Self {
        let res = File::open(path).unwrap();
        let br = BufReader::with_capacity(BUFFER_SIZE, res);
        StorageReaderImp {
            r: br,
            buffer: vec![0; BUFFER_SIZE],
        }
    }
}

impl StorageReader for StorageReaderImp {
    fn read(&mut self) -> (&Vec<u8>, usize) {
        let size = self.r.read(&mut self.buffer).unwrap();
        (&self.buffer, size)
    }

    fn read_at(&mut self, offset: usize, size: usize) -> (&Vec<u8>, usize) {
        todo!()
    }
}

struct StorageWriterImp {
    w: BufWriter<File>,
}

impl StorageWriterImp {
    pub fn new(path: &str) -> Self {
        let res = File::create(path).unwrap();
        let bf = BufWriter::with_capacity(BUFFER_SIZE, res);
        StorageWriterImp { w: bf }
    }
}

impl StorageWriter for StorageWriterImp {
    fn append(&mut self, data: &[u8]) {
        self.w.write(data).unwrap();
    }

    fn write(&mut self, offset: usize, data: &[u8]) {
        todo!()
    }

    fn flush(&mut self) {
        self.w.flush().unwrap();
    }
}

pub fn read<SR: StorageReader>(storage: SR, block_offset: usize) -> Vec<(KeyImp, ValueImp)> {
    todo!()
}
pub fn read_meta<SR: StorageReader>(storage: &SR) -> Vec<BlockMeta<KeyImp>> {
    todo!()
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use crate::storage::{
        convert_u8_array_to_usize, convert_usize_to_u8_array, DecoderReader,
        DecoderReaderMemoryImp, EncoderWriter, EncoderWriterMemoryImp, StorageReader,
        StorageReaderImp, StorageWriter, StorageWriterImp,
    };

    #[test]
    #[ignore]
    fn test_write_perf() {
        let mut s = StorageWriterImp::new("test");
        let mut tmp: Vec<u8> = Vec::new();

        for i in 1..500000 * 3 {
            let mut v: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
            tmp.append(&mut v);
        }
        let start = Instant::now();
        s.append(tmp.as_slice());
        s.flush();
        let elapsed = start.elapsed().as_millis();
        println!("{}", elapsed);
    }

    #[test]
    #[ignore]
    fn test_read_perf() {
        let mut s = StorageReaderImp::new("test");
        let res = s.read();
        println!("{:?}", res);
    }

    #[test]
    fn test_convert_u8_and_usize() {
        test_convert(0);
        test_convert(1);
        test_convert(2);
        test_convert(128);
        test_convert(2345);
        test_convert(23423);
        test_convert(9999999);
    }

    fn test_convert(s: usize) {
        let a: [u8; 4] = convert_usize_to_u8_array(s);
        let res = convert_u8_array_to_usize(&a);
        assert_eq!(res, s);
    }
    #[test]
    fn test_encode_decode() {
        let a: [u8; 8] = [8, 1, 2, 3, 4, 5, 6, 7];
        let mut encode = EncoderWriterMemoryImp::new();
        encode.write(&a).unwrap();

        let mut decode = DecoderReaderMemoryImp::new(encode.to_vec());
        let h = decode.read(8);
        println!("{:?}", h);
    }
}
