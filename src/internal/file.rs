use std::path::PathBuf;
use crate::internal::ODSInternal;
use crate::tags::general::{Tag, Taggable, AnyTag};
use crate::io::streams::ReadStream;
use crate::internal::internal_utils::{get_sub_object_data, get_list_data};

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
}

impl ODSFile {
    pub fn new(file: PathBuf) -> Self {
        ODSFile {
            file
        }
    }
}