use crate::io::streams::ReadStream;
use crate::tags::general::{Tag, StringTag, ITag, Taggable};
use std::any::{Any, TypeId};

#[derive(Debug)]
pub struct TagBuilder {
    data_type: i32,
    data_size: i32,
    starting_index: i64,
    name: String,
    name_size: i32,
    value_bytes: Option<ReadStream>,
    value_length: i32
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

    pub fn get_data_type(&mut self) -> i32{
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

    pub fn get_name(self) -> String {
        self.name
    }

    pub fn set_name_size(&mut self, size: i32) {
        self.name_size = size;
    }

    pub fn get_name_size(&mut self) -> i32 {
        self.name_size
    }

    pub fn set_value_bytes(&mut self, read_stream: ReadStream) {
        self.value_bytes = Some(read_stream.clone());
    }

    pub fn get_value_bytes(mut self) -> Option<ReadStream> {
        self.value_bytes.clone()
    }

    pub fn set_value_length(&mut self, length: i32) {
        self.value_length = length;
    }

    pub fn get_value_length(&mut self) -> i32 {
        self.value_length
    }

    pub fn process<T: /*Taggable<T> +*/ 'static>(&mut self) -> Option<Tag<T>> {
        println!("{:?}", TypeId::of::<Tag<T>>());
        println!("{:?}", TypeId::of::<StringTag>());
        let name = self.name.to_string();
        // type TagData = Tag<T>;
        // TagData::new(name, T::get_default());
        match self.get_data_type() {
            1 => Some(StringTag::new(name, String::new()).create_from_data(self.value_bytes.clone().unwrap(), self.value_length)),
            // TODO:: Custom Tags
            _ => Option::None
        }
        // Option::None
    }
}