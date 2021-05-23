use std::fs;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::PathBuf;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

/// The IORead is a trait that is meant for streams that Read data.
/// See the `ReadStream` and `Stream`
pub trait IORead {
    /// Reads a single byte.
    fn read(&mut self) -> u8;

    /// Reads a vector from the Stream.
    /// # Example
    /// ```rust
    /// // Read 10 bytes into the vector named vec.
    /// let mut vec = vec![0;10];
    /// read_stream.read_vec(&mut vec);
    /// ```
    fn read_vec(&mut self, buf: &mut [u8]);

    /// Read a string from the Stream.
    ///
    /// # Params
    /// size: `u64` -> Denotes the length of the string.
    fn read_string(&mut self, size: u64) -> String;

    /// Read a Short (i16) from the Stream.
    fn read_i16(&mut self) -> i16;

    /// Read an Int (i32) from the Stream.
    fn read_i32(&mut self) -> i32;

    /// Read a Long (i64) from the Stream.
    fn read_i64(&mut self) -> i64;

    /// Read a Float (f32) from the Stream.
    fn read_f32(&mut self) -> f32;

    /// Read a Double (f64) from the Stream.
    fn read_f64(&mut self) -> f64;

    /// Read a Character (char) from the Stream.
    fn read_char(&mut self) -> char;
}

/// This trait has the methods for Standard IO operations for the Streams.
pub trait StandardIO {
    /// Set the position of the stream.
    ///
    /// # Params
    /// position: u64 -> The position to set the stream to.
    ///
    /// # Example
    /// ```rust
    /// read_stream.set_position(0);
    /// ```
    fn set_position(&mut self, position: u64);

    /// Move the cursor of the stream up by the specified amount.
    ///
    /// # Params
    /// skip_amount: u64 -> The amount to skip by.
    fn skip(&mut self, skip_amount: u64);

    /// Get the size of the Stream
    fn size(&mut self) -> usize;

    /// Get the current position of the cursor.
    fn position(&mut self) -> u64;

    /// Check if the stream can read more.
    /// (Aka) if the cursor is not at the end of the stream.
    fn can_read_more(&mut self) -> bool;

    /// Move the cursor to the end of the Stream.
    fn goto_end(&mut self);

    /// Convert the Stream into a Vector of bytes.
    fn bytes(&mut self) -> Vec<u8>;
}

/// The IOWrite trait has methods for writing to a Stream.
/// **Note:** The bytes are written where the cursor is. Make sure to move the
/// cursor to the end to write there.
pub trait IOWrite {
    /// Write a single byte to the Stream.
    fn write(&mut self, byte: u8);

    /// Write a vector of bytes to the Stream.
    fn write_vec(&mut self, bytes: Vec<u8>);

    /// Write a Short (i16) to the Stream.
    fn write_i16(&mut self, short: i16);

    /// Write an Int (i32) to the Stream.
    fn write_i32(&mut self, int: i32);

    /// Write an Long (i64) to the Stream.
    fn write_i64(&mut self, long: i64);

    /// Write a Float (f32) to the Stream.
    fn write_f32(&mut self, float: f32);

    /// Write a Double (f64) to the Stream.
    fn write_f64(&mut self, double: f64);

    /// Write a Character (char) to the Stream.
    fn write_char(&mut self, character: char);

    /// Write a String to the Stream.
    fn write_string(&mut self, string: String);

    /// Export all of the bytes to a file.
    /// **Note:** This will overwrite the existing data in that file.
    fn export_to_file(&mut self, file: PathBuf) -> bool;
}

/// A Stream is a Stream that can both Read and Write. This
/// Struct implements `IOWrite`, `IORead`, and `StandardIO`
///
/// # Examples
/// ```rust
/// use object_data_structure::io::streams::{Stream, IOWrite, StandardIO, IORead};
///
/// let mut empty_stream = Stream::new_empty();
/// empty_stream.write_string("This is a test string!".to_string());
/// empty_stream.set_position(0);
/// let string = empty_stream.read_string(22); // This is a test string!
/// ```
/// Create from a file:
/// ```rust
///  use object_data_structure::io::streams::{Stream, IOWrite, StandardIO, IORead};
///  use std::path::PathBuf;
///
///  let mut empty_stream = Stream::new_from_file(&PathBuf::from("./test.ods"));
/// ```
#[derive(Clone, Debug)]
pub struct Stream {
    buffer: Cursor<Vec<u8>>
}

impl Stream {
    /// Create a new Stream that is populated with data from a file.
    /// **Note:** The position starts at 0.
    ///
    /// # Params
    /// file: `&PathBuf` -> The PathBuf of the file to read.
    pub fn new_from_file(file: &PathBuf) -> Stream {
        let mut f = File::open(file).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        Stream {
            buffer: Cursor::new(buffer)
        }
    }

    /// Create a Stream with no existing data.
    pub fn new_empty() -> Stream {
        let buffer: Vec<u8> = Vec::new();
        let cursor = Cursor::new(buffer);
        Stream {
            buffer: cursor
        }
    }

    /// Create a stream with pre-existing data.
    /// **Note:** The position is set to the end of the stream by default.
    /// # Params
    /// data: Vec<u8> -> The Vector of bytes to populate the Stream with.
    pub fn new_with_data(data: Vec<u8>) -> Stream {
        let len = data.len();
        let mut cursor = Cursor::new(data);
        cursor.set_position(len as u64);
        Stream {
            buffer: cursor
        }
    }
}

impl IORead for Stream {
    fn read(&mut self) -> u8 {
        self.buffer.read_u8().unwrap()
    }

    fn read_vec(&mut self, buf: &mut [u8]) {
        self.buffer.read_exact(buf);
    }

    fn read_string(&mut self, size: u64) -> String {
        let old_pos = self.buffer.position();
        let mut string_buf = Vec::new();
        while self.buffer.position() < old_pos + size {
            string_buf.push(self.buffer.read_u8().unwrap());
        }

        String::from_utf8(string_buf).unwrap()
    }

    fn read_i16(&mut self) -> i16 {
        self.buffer.read_i16::<BigEndian>().unwrap()
    }

    fn read_i32(&mut self) -> i32 {
        self.buffer.read_i32::<BigEndian>().unwrap()
    }

    fn read_i64(&mut self) -> i64 {
        self.buffer.read_i64::<BigEndian>().unwrap()
    }

    fn read_f32(&mut self) -> f32 {
        self.buffer.read_f32::<BigEndian>().unwrap()
    }

    fn read_f64(&mut self) -> f64 {
        self.buffer.read_f64::<BigEndian>().unwrap()
    }

    fn read_char(&mut self) -> char {
        self.buffer.read_u8().unwrap() as char
    }
}

impl IOWrite for Stream {
    fn write(&mut self, byte: u8) {
        self.buffer.write_u8(byte);
    }

    fn write_vec(&mut self, bytes: Vec<u8>) {
        for byte in bytes {
            self.buffer.write_u8(byte);
        }
    }

    fn write_i16(&mut self, short: i16) {
        self.buffer.write_i16::<BigEndian>(short);
    }

    fn write_i32(&mut self, int: i32) {
        self.buffer.write_i32::<BigEndian>(int);
    }

    fn write_i64(&mut self, long: i64) {
        self.buffer.write_i64::<BigEndian>(long);
    }

    fn write_f32(&mut self, float: f32) {
        self.buffer.write_f32::<BigEndian>(float);
    }

    fn write_f64(&mut self, double: f64) {
        self.buffer.write_f64::<BigEndian>(double);
    }

    fn write_char(&mut self, character: char) {
        self.buffer.write_u8(character as u8);
    }

    fn write_string(&mut self, string: String) {
        let data = string.as_bytes();
        for byte in data.iter() {
            self.buffer.write_u8(*byte);
        }
    }

    fn export_to_file(&mut self, file: PathBuf) -> bool {
        // TODO Add error handling.
        fs::write(file, self.bytes()).is_ok()
    }
}

impl StandardIO for Stream {
    fn set_position(&mut self, position: u64) {
        self.buffer.set_position(position);
    }

    fn skip(&mut self, skip_amount: u64) {
        self.buffer.set_position(self.buffer.position() + skip_amount);
    }

    fn size(&mut self) -> usize {
        // Store the current position.
        let pos = self.position();
        // Go to the end of the buffer.
        self.buffer.seek(SeekFrom::End(0));
        // The length of the buffer is the end position. (Add 1 to count position 0).
        let output = self.buffer.position();
        self.buffer.set_position(pos);

        output as usize
    }

    fn position(&mut self) -> u64 {
        self.buffer.position()
    }

    fn can_read_more(&mut self) -> bool {
        self.buffer.position() < self.size() as u64
    }

    fn goto_end(&mut self) {
        self.buffer.seek(SeekFrom::End(0));
    }

    fn bytes(&mut self) -> Vec<u8> {
        let pos = self.position();
        // Go to front
        self.buffer.set_position(0);
        let mut end_vec: Vec<u8> = Vec::new();
        // Read entire buffer.
        self.buffer.read_to_end(&mut end_vec);
        // Go back to the previous position
        self.buffer.set_position(pos);
        end_vec
    }
}

/// A ReadStream is a Stream that can only read.
/// In most cases you want to just use a normal `Stream` instead.
#[derive(Clone, Debug)]
pub struct ReadStream {
    buffer: Cursor<Vec<u8>>,
    size: usize,
}

impl ReadStream {
    pub fn new(file: &PathBuf) -> ReadStream {
        let mut f = File::open(file).unwrap();
        let mut buffer = Vec::new();
        let size = f.read_to_end(&mut buffer).unwrap();

        ReadStream {
            buffer: Cursor::new(buffer),
            size,
        }
    }
}

impl IORead for ReadStream {
    fn read(&mut self) -> u8 {
        self.buffer.read_u8().unwrap()
    }

    fn read_vec(&mut self, buf: &mut [u8]) {
        self.buffer.read_exact(buf);
    }

    fn read_string(&mut self, size: u64) -> String {
        let old_pos = self.buffer.position();
        let mut string_buf = Vec::new();
        while self.buffer.position() < old_pos + size {
            string_buf.push(self.buffer.read_u8().unwrap());
        }

        String::from_utf8(string_buf).unwrap()
    }

    fn read_i16(&mut self) -> i16 {
        self.buffer.read_i16::<BigEndian>().unwrap()
    }

    fn read_i32(&mut self) -> i32 {
        self.buffer.read_i32::<BigEndian>().unwrap()
    }

    fn read_i64(&mut self) -> i64 {
        self.buffer.read_i64::<BigEndian>().unwrap()
    }

    fn read_f32(&mut self) -> f32 {
        self.buffer.read_f32::<BigEndian>().unwrap()
    }

    fn read_f64(&mut self) -> f64 {
        self.buffer.read_f64::<BigEndian>().unwrap()
    }

    fn read_char(&mut self) -> char {
        self.buffer.read_u8().unwrap() as char
    }
}

impl StandardIO for ReadStream {
    fn set_position(&mut self, position: u64) {
        self.buffer.set_position(position);
    }

    fn skip(&mut self, skip_amount: u64) {
        self.buffer.set_position(self.buffer.position() + skip_amount);
    }

    fn size(&mut self) -> usize {
        self.size
    }

    fn position(&mut self) -> u64 {
        self.buffer.position()
    }

    fn can_read_more(&mut self) -> bool {
        self.buffer.position() < self.size as u64
    }

    fn goto_end(&mut self) {
        self.buffer.seek(SeekFrom::End(0));
    }

    fn bytes(&mut self) -> Vec<u8> {
        let pos = self.position();
        // Go to front
        self.buffer.set_position(0);
        let mut end_vec: Vec<u8> = Vec::new();
        // Read entire buffer.
        self.buffer.read_to_end(&mut end_vec);
        // Go back to the previous position
        self.buffer.set_position(pos);
        end_vec
    }
}

/// A WriteStream is a Stream that can only write.
/// In most cases you want to use a normal `Stream` instead.
pub struct WriteStream {
    buffer: Cursor<Vec<u8>>
}

impl WriteStream {
    pub fn new() -> WriteStream {
        let buffer: Vec<u8> = Vec::new();
        let cursor = Cursor::new(buffer);
        WriteStream {
            buffer: cursor
        }
    }

    pub fn new_with_data(data: Vec<u8>) -> WriteStream {
        let len = data.len();
        let mut cursor = Cursor::new(data);
        cursor.set_position(len as u64);
        WriteStream {
            buffer: cursor
        }
    }
}

impl IOWrite for WriteStream {
    fn write(&mut self, byte: u8) {
        self.buffer.write_u8(byte);
    }

    fn write_vec(&mut self, bytes: Vec<u8>) {
        for byte in bytes {
            self.buffer.write_u8(byte);
        }
    }

    fn write_i16(&mut self, short: i16) {
        self.buffer.write_i16::<BigEndian>(short);
    }

    fn write_i32(&mut self, int: i32) {
        self.buffer.write_i32::<BigEndian>(int);
    }

    fn write_i64(&mut self, long: i64) {
        self.buffer.write_i64::<BigEndian>(long);
    }

    fn write_f32(&mut self, float: f32) {
        self.buffer.write_f32::<BigEndian>(float);
    }

    fn write_f64(&mut self, double: f64) {
        self.buffer.write_f64::<BigEndian>(double);
    }

    fn write_char(&mut self, character: char) {
        self.buffer.write_u8(character as u8);
    }

    fn write_string(&mut self, string: String) {
        let data = string.as_bytes();
        for byte in data.iter() {
            self.buffer.write_u8(*byte);
        }
    }

    fn export_to_file(&mut self, file: PathBuf) -> bool {
        // TODO Add error handling.
        fs::write(file, self.bytes()).is_ok()
    }
}

impl StandardIO for WriteStream {
    fn set_position(&mut self, position: u64) {
        self.buffer.set_position(position);
    }

    fn skip(&mut self, skip_amount: u64) {
        self.buffer.set_position(self.buffer.position() + skip_amount);
    }

    fn size(&mut self) -> usize {
        // Store the current position.
        let pos = self.position();
        // Go to the end of the buffer.
        self.buffer.seek(SeekFrom::End(0));
        // The length of the buffer is the end position. (Add 1 to count position 0).
        let output = self.buffer.position();
        self.buffer.set_position(pos);

        output as usize
    }

    fn position(&mut self) -> u64 {
        self.buffer.position()
    }

    fn can_read_more(&mut self) -> bool {
        self.buffer.position() < self.size() as u64
    }

    fn goto_end(&mut self) {
        self.buffer.seek(SeekFrom::End(0));
    }

    fn bytes(&mut self) -> Vec<u8> {
        let pos = self.position();
        // Go to front
        self.buffer.set_position(0);
        let mut end_vec: Vec<u8> = Vec::new();
        // Read entire buffer.
        self.buffer.read_to_end(&mut end_vec);
        // Go back to the previous position
        self.buffer.set_position(pos);
        end_vec
    }
}