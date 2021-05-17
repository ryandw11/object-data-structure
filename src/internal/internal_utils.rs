use crate::io::streams::ReadStream;
use crate::tags::general::{Tag, ITag, StringTag};
use crate::internal::tag_builder::TagBuilder;
use std::any::Any;

pub fn get_sub_object_data<T: 'static>(mut read_stream: ReadStream, key: String) -> Option<Tag<T>> {
    let name_list : Vec<&str> = key.as_str().split('.').collect();
    let name = name_list[0].to_string();
    let other_key = get_key(key.as_str().split('.').collect());

    let mut current_builder = TagBuilder::new();
    while read_stream.can_read_more() {
        current_builder.set_data_type(read_stream.read() as i32);
        current_builder.set_data_size(read_stream.read_i32());
        current_builder.set_starting_index(read_stream.position() as i64);
        current_builder.set_name_size(read_stream.read_i16() as i32);

        if current_builder.get_name_size() != name.len() as i32 {
            read_stream.skip(current_builder.get_data_size() as u64);
            current_builder = TagBuilder::new();
            continue;
        }

        let tag_name = read_stream.read_string(current_builder.get_name_size() as u64);
        current_builder.set_name(tag_name.clone());

        if name != tag_name {
            read_stream.set_position((current_builder.get_starting_index() as i64 + current_builder.get_data_size() as i64) as u64);
            current_builder = TagBuilder::new();
            continue;
        }

        let starting_index = current_builder.get_starting_index();
        let data_size = current_builder.get_data_size();

        current_builder.set_value_length(((starting_index - read_stream.position() as i64) + data_size as i64) as i32);
        current_builder.set_value_bytes(read_stream);
        if other_key.is_some() {
            // TODO Validate not compressed
            return get_sub_object_data(current_builder.get_value_bytes().unwrap(), other_key.unwrap());
        }

        return current_builder.process::<T>();
    }

    Option::None
}

fn get_key(s: Vec<&str>) -> Option<String> {
    let mut list = Vec::from(s);
    list.remove(0);
    if list.len() == 1 {
        return Some(String::from(list[0]));
    }
    if list.len() < 1 {
        return Option::None;
    }

    Some(list.join("."))
}