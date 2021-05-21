use std::path::PathBuf;
use crate::internal::ODSInternal;
use crate::tags::general::{Tag, Taggable, AnyTag};
use crate::io::streams::{ReadStream, WriteStream};
use crate::internal::internal_utils::{get_sub_object_data, get_list_data, find_sub_object_data, scout_object_data, delete_sub_object_data};
use std::fs::File;
use std::io::Read;
use crate::internal::keyscout::KeyScout;

pub struct ODSFile {
    file: PathBuf
}

impl ODSInternal for ODSFile {
    fn get<T: Taggable<T>>(&mut self, key: String) -> Option<Tag<T>> {
        if !self.file.exists() {
            return Option::None;
        }
        let mut read_stream = ReadStream::new(&self.file);

        get_sub_object_data::<T>(read_stream, key)
    }

    fn get_all(&mut self) -> Option<Vec<AnyTag>> {
        if !self.file.exists() {
            return Option::None;
        }

        let mut read_stream = ReadStream::new(&self.file);

        Some(get_list_data(read_stream.clone(), read_stream.size() as i32))
    }

    fn append<T: Taggable<T>>(&mut self, tag: Tag<T>) {
        if !self.file.exists() {
            File::create(self.file.clone());
        }
        let mut file = File::open(self.file.clone()).unwrap();

        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data);
        let mut stream = WriteStream::new_with_data(data);
        T::write_data(tag, &mut stream);
        stream.export_to_file(self.file.clone());
    }

    fn find(&mut self, key: String) -> bool {
        if !self.file.exists() {
            return false;
        }
        let mut read_stream = ReadStream::new(&self.file);

        find_sub_object_data(read_stream, key)
    }

    fn delete(&mut self, key: String) -> bool {
        if !self.file.exists() {
            return false;
        }
        let mut read_stream = ReadStream::new(&self.file);
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
}

impl ODSFile {
    pub fn new(file: PathBuf) -> Self {
        ODSFile {
            file
        }
    }
}