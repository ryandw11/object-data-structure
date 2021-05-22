use crate::io::streams::{ReadStream, WriteStream};
use std::any::{Any, TypeId};
use std::fmt::Debug;
use crate::internal::tag_builder::TagBuilder;
use std::ops::Deref;
use crate::internal::internal_utils::get_list_data;
use crate::util::write_any_tag;
use dyn_clone::DynClone;
use downcast_rs::Downcast;

pub type StringTag = Tag<String>;
pub type IntTag = Tag<i32>;
pub type FloatTag = Tag<f32>;
pub type DoubleTag = Tag<f64>;
pub type ShortTag = Tag<i16>;
pub type LongTag = Tag<i64>;
pub type CharTag = Tag<char>;
pub type ByteTag = Tag<u8>;
pub type VecTag = Tag<Vec<AnyTag>>;
pub type AnyTag = Tag<Box<dyn Any>>;

/*

    Taggables

 */

pub trait Taggable<T> {
    fn as_tag(&self, name: String) -> Tag<T>;
    fn process(tag_builder: TagBuilder) -> Option<Tag<T>>;
    fn write_data(tag: Tag<T>, write_stream: &mut WriteStream);
    fn get_id() -> u8;
}

// trait CloneableAny: Any + DynClone + Downcast {}
// clone_trait_object!(CloneableAny);
// downcast!(CloneableAny);

#[derive(Debug, Clone)]
pub struct Tag<T> {
    name: String,
    value: T,
    id: u8
}

impl<T: Taggable<T>> Tag<T>{
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
            value,
            id: T::get_id()
        }
    }
}

/*
    =============================
            String Tag
    =============================
 */
impl Tag<String> {
    pub fn create_from_data(mut self, mut read_stream: ReadStream, size: i32) -> StringTag {
        let string = read_stream.read_string(size as u64);
        self.value = string;

        self
    }

    pub fn get_id(&self) -> u8 {
        1
    }
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

    fn write_data(tag: Tag<String>, write_stream: &mut WriteStream) {
        write_stream.write(tag.get_id());
        let mut temp_stream = WriteStream::new();
        temp_stream.write_i16(tag.name.len() as i16);
        temp_stream.write_string(tag.name);
        temp_stream.write_string(tag.value);

        write_stream.write_i32(temp_stream.size() as i32);
        write_stream.write_vec(temp_stream.bytes());
    }

    fn get_id() -> u8 {
        1
    }
}

/*
    =============================
              Int Tag
    =============================
 */
impl Tag<i32> {
    pub fn create_from_data(mut self, mut read_stream: ReadStream, size: i32) -> IntTag {
        let int = read_stream.read_i32();
        self.value = int;
        self
    }

    pub fn get_id(&self) -> u8 {
        2
    }
}

impl Taggable<i32> for i32 {
    fn as_tag(&self, name: String) -> IntTag {
        IntTag::new(name, self.clone())
    }

    fn process(tag_builder: TagBuilder) -> Option<IntTag> {
        let name = tag_builder.name;
        if tag_builder.data_type == 2 {
            return Some(IntTag::new(name, -1).create_from_data(tag_builder.value_bytes.clone().unwrap(), tag_builder.value_length));
        }
        Option::None
    }

    fn write_data(tag: Tag<i32>, write_stream: &mut WriteStream) {
        write_stream.write(tag.get_id());
        let mut temp_stream = WriteStream::new();
        temp_stream.write_i16(tag.name.len() as i16);
        temp_stream.write_string(tag.name);
        temp_stream.write_i32(tag.value);

        write_stream.write_i32(temp_stream.size() as i32);
        write_stream.write_vec(temp_stream.bytes());
    }

    fn get_id() -> u8 {
        2
    }
}

/*
    =============================
             Float Tag
    =============================
 */
impl Tag<f32> {
    pub fn create_from_data(mut self, mut read_stream: ReadStream, size: i32) -> FloatTag {
        let float = read_stream.read_f32();
        self.value = float;
        self
    }

    pub fn get_id(&self) -> u8 {
        3
    }
}

impl Taggable<f32> for f32 {
    fn as_tag(&self, name: String) -> FloatTag {
        FloatTag::new(name, self.clone())
    }

    fn process(tag_builder: TagBuilder) -> Option<FloatTag> {
        let name = tag_builder.name;
        if tag_builder.data_type == 3 {
            return Some(FloatTag::new(name, -1.0).create_from_data(tag_builder.value_bytes.clone().unwrap(), tag_builder.value_length));
        }
        Option::None
    }

    fn write_data(tag: Tag<f32>, write_stream: &mut WriteStream) {
        write_stream.write(tag.get_id());
        let mut temp_stream = WriteStream::new();
        temp_stream.write_i16(tag.name.len() as i16);
        temp_stream.write_string(tag.name);
        temp_stream.write_f32(tag.value);

        write_stream.write_i32(temp_stream.size() as i32);
        write_stream.write_vec(temp_stream.bytes());
    }

    fn get_id() -> u8 {
        4
    }
}

/*
    =============================
             Double Tag
    =============================
 */
impl Tag<f64> {
    pub fn create_from_data(mut self, mut read_stream: ReadStream, size: i32) -> DoubleTag {
        let double = read_stream.read_f64();
        self.value = double;
        self
    }

    pub fn get_id(&self) -> u8 {
        4
    }
}

impl Taggable<f64> for f64 {
    fn as_tag(&self, name: String) -> DoubleTag {
        DoubleTag::new(name, self.clone())
    }

    fn process(tag_builder: TagBuilder) -> Option<DoubleTag> {
        let name = tag_builder.name;
        if tag_builder.data_type == 4 {
            return Some(DoubleTag::new(name, -1.0).create_from_data(tag_builder.value_bytes.clone().unwrap(), tag_builder.value_length));
        }
        Option::None
    }

    fn write_data(tag: Tag<f64>, write_stream: &mut WriteStream) {
        write_stream.write(tag.get_id());
        let mut temp_stream = WriteStream::new();
        temp_stream.write_i16(tag.name.len() as i16);
        temp_stream.write_string(tag.name);
        temp_stream.write_f64(tag.value);

        write_stream.write_i32(temp_stream.size() as i32);
        write_stream.write_vec(temp_stream.bytes());
    }

    fn get_id() -> u8 {
        4
    }
}

/*
    =============================
             Short Tag
    =============================
 */
impl Tag<i16> {
    pub fn create_from_data(mut self, mut read_stream: ReadStream, size: i32) -> ShortTag {
        let short = read_stream.read_i16();
        self.value = short;
        self
    }

    pub fn get_id(&self) -> u8 {
        5
    }
}

impl Taggable<i16> for i16 {
    fn as_tag(&self, name: String) -> ShortTag {
        ShortTag::new(name, self.clone())
    }

    fn process(tag_builder: TagBuilder) -> Option<ShortTag> {
        let name = tag_builder.name;
        if tag_builder.data_type == 5 {
            return Some(ShortTag::new(name, -1).create_from_data(tag_builder.value_bytes.clone().unwrap(), tag_builder.value_length));
        }
        Option::None
    }

    fn write_data(tag: Tag<i16>, write_stream: &mut WriteStream) {
        write_stream.write(tag.get_id());
        let mut temp_stream = WriteStream::new();
        temp_stream.write_i16(tag.name.len() as i16);
        temp_stream.write_string(tag.name);
        temp_stream.write_i16(tag.value);

        write_stream.write_i32(temp_stream.size() as i32);
        write_stream.write_vec(temp_stream.bytes());
    }

    fn get_id() -> u8 {
        5
    }
}

/*
    =============================
             Long Tag
    =============================
 */
impl Tag<i64> {
    pub fn create_from_data(mut self, mut read_stream: ReadStream, size: i32) -> LongTag {
        let long = read_stream.read_i64();
        self.value = long;
        self
    }

    pub fn get_id(&self) -> u8 {
        6
    }
}

impl Taggable<i64> for i64 {
    fn as_tag(&self, name: String) -> LongTag {
        LongTag::new(name, self.clone())
    }

    fn process(tag_builder: TagBuilder) -> Option<LongTag> {
        let name = tag_builder.name;
        if tag_builder.data_type == 6 {
            return Some(LongTag::new(name, -1).create_from_data(tag_builder.value_bytes.clone().unwrap(), tag_builder.value_length));
        }
        Option::None
    }

    fn write_data(tag: Tag<i64>, write_stream: &mut WriteStream) {
        write_stream.write(tag.get_id());
        let mut temp_stream = WriteStream::new();
        temp_stream.write_i16(tag.name.len() as i16);
        temp_stream.write_string(tag.name);
        temp_stream.write_i64(tag.value);

        write_stream.write_i32(temp_stream.size() as i32);
        write_stream.write_vec(temp_stream.bytes());
    }

    fn get_id() -> u8 {
        6
    }
}

/*
    =============================
             Char Tag
    =============================
 */
impl Tag<char> {
    pub fn create_from_data(mut self, mut read_stream: ReadStream, size: i32) -> CharTag {
        let character = read_stream.read_char();
        self.value = character;
        self
    }

    pub fn get_id(&self) -> u8 {
        7
    }
}

impl Taggable<char> for char {
    fn as_tag(&self, name: String) -> CharTag {
        CharTag::new(name, self.clone())
    }

    fn process(tag_builder: TagBuilder) -> Option<CharTag> {
        let name = tag_builder.name;
        if tag_builder.data_type == 7 {
            return Some(CharTag::new(name, ' ').create_from_data(tag_builder.value_bytes.clone().unwrap(), tag_builder.value_length));
        }
        Option::None
    }

    fn write_data(tag: Tag<char>, write_stream: &mut WriteStream) {
        write_stream.write(tag.get_id());
        let mut temp_stream = WriteStream::new();
        temp_stream.write_i16(tag.name.len() as i16);
        temp_stream.write_string(tag.name);
        temp_stream.write_char(tag.value);

        write_stream.write_i32(temp_stream.size() as i32);
        write_stream.write_vec(temp_stream.bytes());
    }

    fn get_id() -> u8 {
        7
    }
}

/*
    =============================
             Byte Tag
    =============================
 */
impl Tag<u8> {
    pub fn create_from_data(mut self, mut read_stream: ReadStream, size: i32) -> ByteTag {
        let byte = read_stream.read();
        self.value = byte;
        self
    }

    pub fn get_id(&self) -> u8 {
        8
    }
}

impl Taggable<u8> for u8 {
    fn as_tag(&self, name: String) -> ByteTag {
        ByteTag::new(name, self.clone())
    }

    fn process(tag_builder: TagBuilder) -> Option<ByteTag> {
        let name = tag_builder.name;
        if tag_builder.data_type == 8 {
            return Some(ByteTag::new(name, 0).create_from_data(tag_builder.value_bytes.clone().unwrap(), tag_builder.value_length));
        }
        Option::None
    }

    fn write_data(tag: Tag<u8>, write_stream: &mut WriteStream) {
        write_stream.write(tag.get_id());
        let mut temp_stream = WriteStream::new();
        temp_stream.write_i16(tag.name.len() as i16);
        temp_stream.write_string(tag.name);
        temp_stream.write(tag.value);

        write_stream.write_i32(temp_stream.size() as i32);
        write_stream.write_vec(temp_stream.bytes());
    }

    fn get_id() -> u8 {
        8
    }
}

/*
    =============================
             Vec Tag
    =============================
 */
impl Tag<Vec<AnyTag>> {
    pub fn create_from_data(mut self, mut read_stream: ReadStream, size: i32) -> VecTag {
        let data = get_list_data(read_stream, size);

        self.value = data;
        self
    }

    // pub fn get<T: Taggable<T> + 'static>(&mut self, i: usize) -> &Tag<T> {
    //     // let tag: AnyTag = self.value[i];
    //
    //     self.value[i].value.downcast_ref::<T>().unwrap()
    // }

    pub fn get_id(&self) -> u8 {
        9
    }
}

impl Taggable<Vec<AnyTag>> for Vec<AnyTag> {
    fn as_tag(&self, name: String) -> VecTag {
        // VecTag::new(name, self.clone())
        panic!("Cannot create a VecTag.");
    }

    fn process(tag_builder: TagBuilder) -> Option<VecTag> {
        let name = tag_builder.name;
        if tag_builder.data_type == 9 {
            return Some(VecTag::new(name, Vec::new()).create_from_data(tag_builder.value_bytes.clone().unwrap(), tag_builder.value_length));
        }
        Option::None
    }

    fn write_data(tag: Tag<Vec<AnyTag>>, write_stream: &mut WriteStream) {
        write_stream.write(tag.get_id());
        let mut temp_stream = WriteStream::new();
        temp_stream.write_i16(tag.name.len() as i16);
        temp_stream.write_string(tag.name.clone());

        for t in tag.get_value().iter() {
            write_any_tag(t, &mut temp_stream);
        }

        write_stream.write_i32(temp_stream.size() as i32);
        write_stream.write_vec(temp_stream.bytes());
    }

    fn get_id() -> u8 {
        9
    }
}


/*
    =============================
               Any Tag
    =============================
 */
impl Tag<Box<dyn Any>> {
    pub fn create_from_data(mut self, mut read_stream: ReadStream, size: i32) -> StringTag {
        panic!("This tag cannot be created.");
    }

    pub fn from_tag<T: 'static + Taggable<T>>(tag: Tag<T>) -> AnyTag {
        Self {
            name: tag.name,
            value: Box::new(tag.value),
            id: T::get_id()
        }
    }

    pub fn downcast_any_tag<T: 'static + Clone + Taggable<T>>(&self) -> Tag<T> {
        Tag::<T>::new(self.name.clone(), (*self.value.downcast_ref::<T>().unwrap()).clone())
    }

    // pub fn downcast_any_tag_noclone<T: 'static + Taggable<T>>(&self) -> Tag<T> {
    //     Tag::<T>::new(self.name.clone(), self.value.downcast::<T>().unwrap().to_owned())
    // }

    pub fn is_tag<T: 'static + Taggable<T>>(&self) -> bool {
        self.value.is::<T>()
    }

    pub fn get_id(&self) -> u8 {
        self.id
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
        else if tag_builder.data_type == 2 {
            return Some(AnyTag::from_tag::<i32>(i32::process(tag_builder.clone()).unwrap()));
        }
        else if tag_builder.data_type == 3 {
            return Some(AnyTag::from_tag::<f32>(f32::process(tag_builder.clone()).unwrap()));
        }
        else if tag_builder.data_type == 4 {
            return Some(AnyTag::from_tag::<f64>(f64::process(tag_builder.clone()).unwrap()));
        }
        else if tag_builder.data_type == 5 {
            return Some(AnyTag::from_tag::<i16>(i16::process(tag_builder.clone()).unwrap()));
        }
        else if tag_builder.data_type == 6 {
            return Some(AnyTag::from_tag::<i64>(i64::process(tag_builder.clone()).unwrap()));
        }
        else if tag_builder.data_type == 7 {
            return Some(AnyTag::from_tag::<char>(char::process(tag_builder.clone()).unwrap()));
        }
        else if tag_builder.data_type == 8 {
            return Some(AnyTag::from_tag::<u8>(u8::process(tag_builder.clone()).unwrap()));
        }
        else if tag_builder.data_type == 9 {
            type t = Vec<AnyTag>;
            return Some(AnyTag::from_tag::<Vec<AnyTag>>(t::process(tag_builder.clone()).unwrap()));
        }
        Option::None
    }

    fn write_data(tag: Tag<Box<dyn Any>>, write_stream: &mut WriteStream) {
        write_any_tag(&tag, write_stream)
    }

    fn get_id() -> u8 {
        0
    }
}