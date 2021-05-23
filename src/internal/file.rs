use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::internal::internal_utils::{delete_sub_object_data, find_sub_object_data, get_list_data, get_sub_object_data, replace_sub_object_data, scout_object_data};
use crate::internal::keyscout::KeyScout;
use crate::internal::ODSInternal;
use crate::io::streams::{ReadStream, WriteStream, IOWrite, StandardIO, Stream};
use crate::tags::general::{AnyTag, Tag, Taggable};

pub struct ODSFile {
    file: PathBuf
}

impl ODSInternal for ODSFile {
    fn get<T: Taggable<T>>(&mut self, key: String) -> Option<Tag<T>> {
        if !self.file.exists() {
            return Option::None;
        }
        let mut read_stream = Stream::new_from_file(&self.file);

        get_sub_object_data::<T>(read_stream, key)
    }

    fn get_all(&mut self) -> Option<Vec<AnyTag>> {
        if !self.file.exists() {
            return Option::None;
        }

        let mut read_stream = Stream::new_from_file(&self.file);

        Some(get_list_data(read_stream.clone(), read_stream.size() as i32))
    }

    fn append<T: Taggable<T>>(&mut self, tag: Tag<T>) {
        if !self.file.exists() {
            File::create(self.file.clone());
        }
        let mut file = File::open(self.file.clone()).unwrap();

        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data);
        let mut stream = Stream::new_with_data(data);
        T::write_data(tag, &mut stream);
        stream.export_to_file(self.file.clone());
    }

    fn find(&mut self, key: String) -> bool {
        if !self.file.exists() {
            return false;
        }
        let mut read_stream = Stream::new_from_file(&self.file);

        find_sub_object_data(read_stream, key)
    }

    fn delete(&mut self, key: String) -> bool {
        if !self.file.exists() {
            return false;
        }
        let mut read_stream = Stream::new_from_file(&self.file);
        let mut my_counter = KeyScout::new();
        scout_object_data(&mut read_stream, key, &mut my_counter);

        if my_counter.get_end().is_none() {
            return false;
        }
        let mut data = read_stream.bytes();
        delete_sub_object_data(&mut data, &mut my_counter);

        let mut write_stream = WriteStream::new();
        write_stream.write_vec(data);
        write_stream.export_to_file(self.file.clone());
        true
    }

    fn replace_data<T: Taggable<T>>(&mut self, key: String, replacement: Tag<T>) -> bool {
        if !self.file.exists() {
            return false;
        }
        let mut read_stream = Stream::new_from_file(&self.file);
        let mut my_counter = KeyScout::new();
        scout_object_data(&mut read_stream, key, &mut my_counter);

        if my_counter.get_end().is_none() {
            return false;
        }

        let mut write_tag = Stream::new_empty();
        T::write_data(replacement, &mut write_tag);

        let mut data = read_stream.bytes();
        let replacement_data = write_tag.bytes();

        replace_sub_object_data(&mut data, &mut my_counter, &replacement_data);

        let mut write_stream = WriteStream::new();
        write_stream.write_vec(data);

        write_stream.export_to_file(self.file.clone())
    }

    fn set<T: Taggable<T>>(&mut self, key: String, value: Option<Tag<T>>) {
        if value.is_none() {
            let output = self.delete(key);
            if !output {
                // TODO handle errors
            }
            return;
        }
        if key == "" {
            //save(vec![value]);
            // Hacky method until save works.
            let mut stream = WriteStream::new();
            stream.export_to_file(self.file.clone());

            self.append(value.unwrap());
            return;
        }
        let mut read_stream = Stream::new_from_file(&self.file);
        let mut counter = KeyScout::new();
        scout_object_data(&mut read_stream, key, &mut counter);

        if counter.get_end().is_none() {
            if counter.get_children().len() < 1 {
                self.append(value.unwrap());
                return;
            }
            let mut existing_key = String::new();
            for child in counter.get_children().iter() {
                if existing_key.len() != 0 {

                }
            }
        }
    }
}

impl ODSFile {
    pub fn new(file: PathBuf) -> Self {
        ODSFile {
            file
        }
    }
}