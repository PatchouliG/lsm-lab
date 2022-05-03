use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

const BUFFER_SIZE: usize = 8 * 4 * 1024;
pub struct EncoderWriter {
    vec: Vec<u8>,
}

impl EncoderWriter {
    pub fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }
    pub fn write_size(&mut self, size: usize) -> std::io::Result<usize> {
        todo!()
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

pub trait DecoderReader {
    fn read_size(&self) -> usize;
    fn read(&self, size: usize) -> &[u8];
}

pub struct DecoderReaderImp {}

impl DecoderReader for DecoderReaderImp {
    fn read_size(&self) -> usize {
        todo!()
    }

    fn read(&self, size: usize) -> &[u8] {
        todo!()
    }
}

// abstract file
pub trait StorageReader {
    fn read(&mut self) -> (&Vec<u8>, usize);
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

#[cfg(test)]
mod test {
    use std::time::Instant;

    use crate::storage::{StorageReader, StorageReaderImp, StorageWriter, StorageWriterImp};

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
}
