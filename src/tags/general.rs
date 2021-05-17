use crate::io::streams::{ReadStream, WriteStream};
use std::any::Any;
use std::fmt::Debug;

pub type StringTag = Tag<String>;

pub trait Taggable<T> {
    fn as_tag(&self, name: String) -> Tag<T>;
    fn get_default() -> T;
}

impl Taggable<String> for String {
    fn as_tag(&self, name: String) -> Tag<String> {
        StringTag::new(name, self.clone())
    }

    fn get_default() -> String {
        String::new()
    }
}

pub trait ITag {
    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: String);
}

#[derive(Debug, Clone)]
pub struct Tag<T> {
    name: String,
    value: T
}

impl<T> ITag for Tag<T> {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl<T> Tag<T>{
    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn set_value(&mut self, t: T) {
        self.value = t;
    }

    pub fn new(name: String, value: T) -> Self {
        Self {
            name,
            value
        }
    }
}

impl Tag<String> {
    // pub fn new(name: String, value: String) -> StringTag {
    //     StringTag {
    //         name,
    //         value
    //     }
    // }

    pub fn write_data(self, write_stream: &mut WriteStream) {
        write_stream.write(self.get_id());
        let mut temp_stream = WriteStream::new();
        temp_stream.write_i16(self.name.len() as i16);
        temp_stream.write_string(self.name);
        temp_stream.write_string(self.value);

        write_stream.write_i32(temp_stream.size() as i32);
        write_stream.write_vec(temp_stream.bytes());
    }

    pub fn create_from_data(mut self, mut read_stream: ReadStream, size: i32) -> StringTag {
        let string = read_stream.read_string(size as u64);
        self.value = string;

        self
    }

    pub fn get_id(&self) -> u8 {
        1
    }

    pub fn as_any(self: &'static Self) -> Box<dyn Any + '_> {
        Box::new(self)
    }
}