use crate::tags::general::{AnyTag, VecTag, Container, VectorContainer};
use crate::io::streams::{WriteStream, Stream};
use crate::tags::general::Taggable;

pub(crate) fn write_any_tag<'a>(tag: &'a AnyTag, write_stream: &'a mut Stream) {
    println!("{:?}", tag);
    match tag.get_id() {
        1 => String::write_data(tag.downcast_any_tag::<String>(), write_stream),
        2 => i32::write_data(tag.downcast_any_tag::<i32>(), write_stream),
        3 => f32::write_data(tag.downcast_any_tag::<f32>(), write_stream),
        4 => f64::write_data(tag.downcast_any_tag::<f64>(), write_stream),
        5 => i16::write_data(tag.downcast_any_tag::<i16>(), write_stream),
        6 => i64::write_data(tag.downcast_any_tag::<i64>(), write_stream),
        7 => char::write_data(tag.downcast_any_tag::<char>(), write_stream),
        8 => u8::write_data(tag.downcast_any_tag::<u8>(), write_stream),
        9 => {
            type T = Container<VectorContainer>;
            T::write_data(tag.downcast_any_tag::<Container<VectorContainer>>(), write_stream);
        },
        _ => {
            panic!("Unknown type!");
        }
    }
}

macro_rules! vec_tag {
    ($($x:expr),*) => {
        vec![$(as_any_tag![$x]),*]
    }
}

macro_rules! as_any_tag {
    ($x:expr) => {
        crate::tags::general::AnyTag::from_tag($x)
    }
}