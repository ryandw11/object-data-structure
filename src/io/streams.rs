use std::path::{PathBuf};
use std::fs::File;
use std::io::{Read, Cursor, Seek, SeekFrom};

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use std::fs;

pub struct ReadStream {
    buffer: Cursor<Vec<u8>>,
    size: usize
}

impl ReadStream {
    pub fn new(file: &PathBuf) -> ReadStream {
        let mut f = File::open(file).unwrap();
        let mut buffer = Vec::new();
        let size = f.read_to_end(&mut buffer).unwrap();

        ReadStream {
            buffer: Cursor::new(buffer),
            size
        }
    }

    pub fn read(&mut self) -> u8 {
        self.buffer.read_u8().unwrap()
    }

    pub fn read_vec(&mut self, buf: &mut [u8]) -> usize {
        self.buffer.read(buf).unwrap()
    }

    pub fn read_string(&mut self, size: u64) -> String {
        let old_pos = self.buffer.position();
        let mut string_buf = Vec::new();
        while self.buffer.position() < old_pos + size {
            string_buf.push(self.buffer.read_u8().unwrap());
        }

        String::from_utf8(string_buf).unwrap()
    }

    pub fn read_i16(&mut self) -> i16 {
        self.buffer.read_i16::<BigEndian>().unwrap()
    }

    pub fn read_i32(&mut self) -> i32 {
        self.buffer.read_i32::<BigEndian>().unwrap()
    }

    pub fn read_i64(&mut self) -> i64 {
        self.buffer.read_i64::<BigEndian>().unwrap()
    }

    pub fn set_position(&mut self, position: u64){
        self.buffer.set_position(position);
    }

    pub fn skip(&mut self, skip_amount: u64){
        self.buffer.set_position(self.buffer.position() + skip_amount);
    }

    pub fn position(&mut self) -> u64 {
        self.buffer.position()
    }

    pub fn can_read_more(&mut self) -> bool {
        self.buffer.position() < self.size as u64
    }
}

pub struct WriteStream {
    buffer: Cursor<Vec<u8>>
}

impl WriteStream {
    pub fn new() -> WriteStream {
        let buffer : Vec<u8> = Vec::new();
        let cursor = Cursor::new(buffer);
        WriteStream{
            buffer: cursor
        }
    }

    pub fn write(&mut self, byte: u8){
        self.buffer.write_u8(byte);
    }

    pub fn write_vec(&mut self, bytes: Vec<u8>) {
        for byte in bytes {
            self.buffer.write_u8(byte);
        }
    }

    pub fn write_i16 (&mut self, short: i16) {
        self.buffer.write_i16::<BigEndian>(short);
    }

    pub fn write_i32 (&mut self, int: i32) {
        self.buffer.write_i32::<BigEndian>(int);
    }

    pub fn write_i64(&mut self, long: i64){
        self.buffer.write_i64::<BigEndian>(long);
    }

    pub fn write_string(&mut self, string: String){
        let data = string.as_bytes();
        for byte in data.iter() {
            self.buffer.write_u8(*byte);
        }
    }

    pub fn position(&mut self) -> u64 {
        self.buffer.position()
    }

    pub fn bytes(&mut self) -> Vec<u8> {
        let pos = self.position();
        // Go to front
        self.buffer.set_position(0);
        let mut end_vec : Vec<u8> = Vec::new();
        // Read entire buffer.
        self.buffer.read_to_end(&mut end_vec);
        // Go back to the previous position
        self.buffer.set_position(pos);
        end_vec
    }

    pub fn size(&mut self) -> u64 {
        // Store the current position.
        let pos = self.position();
        // Go to the end of the buffer.
        self.buffer.seek(SeekFrom::End(0));
        // The length of the buffer is the end position. (Add 1 to count position 0).
        let output = self.buffer.position();
        self.buffer.set_position(pos);

        output
    }

    pub fn export_to_file(&mut self, file: PathBuf) {
        fs::write("test.ods", self.bytes());
    }
}