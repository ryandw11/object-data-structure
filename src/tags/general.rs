use crate::io::streams::{ReadStream, WriteStream};
use std::any::Any;
use std::fmt::Debug;
use crate::internal::tag_builder::TagBuilder;
use std::ops::Deref;

pub type StringTag = Tag<String>;
pub type AnyTag = Tag<Box<dyn Any>>;

pub trait Taggable<T> {
    fn as_tag(&self, name: String) -> Tag<T>;
    fn process(tag_builder: TagBuilder) -> Option<Tag<T>>;
}

impl Taggable<String> for String {
    fn as_tag(&self, name: String) -> StringTag {
        StringTag::new(name, self.clone())
    }

    fn process(tag_builder: TagBuilder) -> Option<StringTag> {
        let name = tag_builder.name;
        if tag_builder.data_type == 1 {
            return Some(StringTag::new(name, String::new()).create_from_data(tag_builder.value_bytes.clone().unwrap(), tag_builder.value_length));
        }
        Option::None
    }
}

impl Taggable<Box<dyn Any>> for Box<dyn Any> {
    fn as_tag(&self, name: String) -> AnyTag {
        panic!("This operation is not supported for AnyTags!");
    }

    fn process(tag_builder: TagBuilder) -> Option<AnyTag> {
        if tag_builder.data_type == 1 {
            return Some(AnyTag::from_tag::<String>(String::process(tag_builder.clone()).unwrap()));
        }
        Option::None
    }
}

#[derive(Debug, Clone)]
pub struct Tag<T> {
    name: String,
    value: T
}

impl<T> Tag<T>{
    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn set_value(&mut self, t: T) {
        self.value = t;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
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

impl Tag<Box<dyn Any>> {
    pub fn write_data(self, write_stream: &mut WriteStream) {
        panic!("This tag is not able to be written.");
    }

    pub fn create_from_data(mut self, mut read_stream: ReadStream, size: i32) -> StringTag {
        panic!("This tag cannot be created.");
    }

    pub fn from_tag<T: 'static + Taggable<T>>(tag: Tag<T>) -> AnyTag {
        Self {
            name: tag.name,
            value: Box::new(tag.value),
        }
    }

    pub fn downcast_any_tag<T: 'static + Clone + Taggable<T>>(&self) -> Tag<T> {
        let data = self.value.downcast_ref::<T>();
        Tag::<T>::new(self.name.clone(), (*self.value.downcast_ref::<T>().unwrap()).clone())
    }

    pub fn get_id(&self) -> u8 {
        0
    }
}