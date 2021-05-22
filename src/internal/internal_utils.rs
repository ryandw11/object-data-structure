use std::any::Any;

use crate::internal::keyscout::{KeyScout, KeyScoutChild};
use crate::internal::tag_builder::TagBuilder;
use crate::io::streams::ReadStream;
use crate::tags::general::{AnyTag, StringTag, Tag, Taggable};

pub fn get_sub_object_data<T: Taggable<T>>(mut read_stream: ReadStream, key: String) -> Option<Tag<T>> {
    let name_list: Vec<&str> = key.as_str().split('.').collect();
    let name = name_list[0].to_string();
    let other_key = get_key(key.as_str().split('.').collect());

    let mut current_builder = TagBuilder::new();
    while read_stream.can_read_more() {
        current_builder.set_data_type(read_stream.read() as i32);
        current_builder.set_data_size(read_stream.read_i32());
        current_builder.set_starting_index(read_stream.position() as i64);
        current_builder.set_name_size(read_stream.read_i16() as i32);

        if current_builder.get_name_size() != name.len() as i32 {
            read_stream.set_position((current_builder.get_starting_index() as i64 + current_builder.get_data_size() as i64) as u64);
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

pub fn get_list_data(mut read_stream: ReadStream, limit: i32) -> Vec<AnyTag> {
    let mut output: Vec<AnyTag> = Vec::new();


    let initial_pos: i32 = read_stream.position() as i32;

    while (read_stream.position() as i32) < initial_pos + limit {
        let mut current_builder = TagBuilder::new();
        current_builder.set_data_type(read_stream.read() as i32);
        current_builder.set_data_size(read_stream.read_i32());
        current_builder.set_starting_index(read_stream.position() as i64);
        current_builder.set_name_size(read_stream.read_i16() as i32);
        let tag_name = read_stream.read_string(current_builder.get_name_size() as u64);
        current_builder.set_name(tag_name);
        current_builder.set_value_length(((current_builder.starting_index as i32) - (read_stream.position() as i32)) + current_builder.data_size);
        current_builder.set_value_bytes(read_stream.clone());
        read_stream.set_position((current_builder.get_starting_index() as i64 + current_builder.get_data_size() as i64) as u64);
        output.push(current_builder.process::<Box<dyn Any>>().unwrap());
    }

    output
}

pub fn find_sub_object_data(mut read_stream: ReadStream, key: String) -> bool {
    let name_list: Vec<&str> = key.as_str().split('.').collect();
    let name = name_list[0].to_string();
    let other_key = get_key(key.as_str().split('.').collect());

    let mut current_builder = TagBuilder::new();
    while read_stream.can_read_more() {
        current_builder.set_data_type(read_stream.read() as i32);
        current_builder.set_data_size(read_stream.read_i32());
        current_builder.set_starting_index(read_stream.position() as i64);
        current_builder.set_name_size(read_stream.read_i16() as i32);
        if current_builder.get_name_size() != name.len() as i32 {
            read_stream.set_position((current_builder.get_starting_index() as i64 + current_builder.get_data_size() as i64) as u64);
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
            return find_sub_object_data(current_builder.get_value_bytes().unwrap(), other_key.unwrap());
        }

        return true;
    }

    false
}

pub fn delete_sub_object_data<'a>(data: &'a mut Vec<u8>, counter: &'a mut KeyScout) -> &'a mut Vec<u8> {
    let end_amount = counter.get_end().as_ref().unwrap().get_size() + 5;
    counter.remove_amount(end_amount);
    let mut end = counter.get_end().as_ref().unwrap();

    // Remove the data
    data.splice(((end.get_starting_index() - 1) as usize)..((end.get_starting_index() + 4 + end.get_size()) as usize), Vec::new().iter().cloned());

    for child in counter.get_children().iter() {
        let index = child.get_starting_index() as usize;
        let size : usize = child.get_size() as usize;
        data[index] = (size >> 24) as u8;
        data[index + 1] = (size >> 16) as u8;
        data[index + 2] = (size >> 8) as u8;
        data[index + 3] = (size) as u8;
    }

    data
}

pub fn replace_sub_object_data<'a, 'b>(data: &'a mut Vec<u8>, counter: &'a mut KeyScout, data_to_replace: &'b Vec<u8>) -> &'a mut Vec<u8> {
    let end_amount = counter.get_end().as_ref().unwrap().get_size() + 5;
    counter.remove_amount(end_amount);
    counter.add_amount(data_to_replace.len() as i32);

    let mut end = counter.get_end().as_ref().unwrap();

    // Remove the data
    data.splice(((end.get_starting_index() - 1) as usize)..((end.get_starting_index() + 4 + end.get_size()) as usize), data_to_replace.iter().cloned());

    for child in counter.get_children().iter() {
        let index = child.get_starting_index() as usize;
        let size : usize = child.get_size() as usize;
        data[index] = (size >> 24) as u8;
        data[index + 1] = (size >> 16) as u8;
        data[index + 2] = (size >> 8) as u8;
        data[index + 3] = (size) as u8;
    }

    data
}

pub fn scout_object_data<'a, 'b>(read_stream: &'a mut ReadStream, key: String, counter: &'b mut KeyScout) -> &'b mut KeyScout {
    let name_list: Vec<&str> = key.as_str().split('.').collect();
    let name = name_list[0].to_string();
    let other_key = get_key(key.as_str().split('.').collect());

    let mut current_builder = TagBuilder::new();
    while read_stream.can_read_more() {
        let mut child = KeyScoutChild::new();
        current_builder.set_data_type(read_stream.read() as i32);

        child.set_starting_index(read_stream.position() as i32);

        current_builder.set_data_size(read_stream.read_i32());
        current_builder.set_starting_index(read_stream.position() as i64);
        current_builder.set_name_size(read_stream.read_i16() as i32);

        if current_builder.get_name_size() != name.len() as i32 {
            read_stream.set_position((current_builder.get_starting_index() as i64 + current_builder.get_data_size() as i64) as u64);
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
        if other_key.is_some() {
            // TODO Validate not compressed

            child.set_size(current_builder.get_data_size());
            child.set_name(current_builder.get_name());
            counter.add_child(child);
            return scout_object_data(read_stream, other_key.unwrap(), counter);
        }

        child.set_name(current_builder.get_name());
        child.set_size(current_builder.get_data_size());
        counter.set_end(child);

        return counter;
    }

    counter
}