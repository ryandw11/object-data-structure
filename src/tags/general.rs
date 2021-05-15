use crate::io::streams::{ReadStream, WriteStream};
use std::any::Any;
use std::fmt::Debug;

pub trait ITag: Debug {
    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: String);
    fn write_data(self, write_stream: &mut WriteStream);
    fn create_from_data(self, read_stream: ReadStream, size: i32) -> Self where Self: Sized;
    fn get_id(&self) -> u8;

    fn as_any(self: Box<Self>) -> Box<dyn Any + 'static>;
}

pub trait Tag<T> : ITag{
    fn new(name: String, value: T) -> Self where Self: Sized;
    fn get_value(&self) -> &T;
    fn set_value(&mut self, t: T);
}

#[derive(Debug)]
pub struct StringTag {
    name: String,
    value: String
}

impl Tag<String> for StringTag {
    fn new(name: String, value: String) -> StringTag {
        StringTag {
            name,
            value
        }
    }

    fn get_value(&self) -> &String {
        &self.value
    }

    fn set_value(&mut self, t: String) {
        self.value = t;
    }
}

impl ITag for StringTag {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn write_data(self, write_stream: &mut WriteStream) {
        write_stream.write(self.get_id());
        let mut temp_stream = WriteStream::new();
        temp_stream.write_i16(self.name.len() as i16);
        temp_stream.write_string(self.name);
        temp_stream.write_string(self.value);

        write_stream.write_i32(temp_stream.size() as i32);
        write_stream.write_vec(temp_stream.bytes());
    }

    fn create_from_data(mut self, mut read_stream: ReadStream, size: i32) -> Self {
        let string = read_stream.read_string(size as u64);
        self.value = string;

        self
    }

    fn get_id(&self) -> u8 {
        1
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any + 'static> {
        Box::new(self)
    }
}