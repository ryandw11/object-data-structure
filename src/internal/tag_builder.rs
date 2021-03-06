use crate::io::streams::{ReadStream, Stream};
use crate::tags::general::{Tag, StringTag, Taggable};
use std::any::{Any, TypeId};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct TagBuilder {
    pub data_type: i32,
    pub data_size: i32,
    pub starting_index: i64,
    pub name: String,
    pub name_size: i32,
    pub value_bytes: Option<Stream>,
    pub value_length: i32
}

impl TagBuilder {
    pub fn new() -> TagBuilder {
        TagBuilder {
            data_type: -1,
            data_size: -1,
            starting_index: -1,
            name: String::new(),
            name_size: -1,
            value_bytes: Option::None,
            value_length: -1
        }
    }

    pub fn set_data_type(&mut self, data_type: i32) {
        self.data_type = data_type;
    }

    pub fn get_data_type(self) -> i32{
        self.data_type
    }

    pub fn set_data_size(&mut self, size: i32){
        self.data_size = size;
    }

    pub fn get_data_size(&mut self) -> i32 {
        self.data_size
    }

    pub fn set_starting_index(&mut self, starting_index: i64) {
        self.starting_index = starting_index;
    }

    pub fn get_starting_index(&mut self) -> i64 {
        self.starting_index
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name_size(&mut self, size: i32) {
        self.name_size = size;
    }

    pub fn get_name_size(&mut self) -> i32 {
        self.name_size
    }

    pub fn set_value_bytes(&mut self, read_stream: Stream) {
        self.value_bytes = Some(read_stream.clone());
    }

    pub fn get_value_bytes(mut self) -> Option<Stream> {
        self.value_bytes.clone()
    }

    pub fn set_value_length(&mut self, length: i32) {
        self.value_length = length;
    }

    pub fn get_value_length(self) -> i32 {
        self.value_length
    }

    pub fn process<T: Taggable<T>>(self) -> Option<Tag<T>> {
        T::process(self)
    }
}