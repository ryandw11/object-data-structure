use std::path::PathBuf;
use crate::internal::ODSInternal;
use crate::tags::general::{Tag, ITag};
use crate::io::streams::ReadStream;
use crate::internal::internal_utils::get_sub_object_data;

pub struct ODSFile {
    file: PathBuf
}

impl ODSInternal for ODSFile {
    fn get<T: 'static>(&mut self, key: String) -> Option<Tag<T>> {
        if !self.file.exists() {
            return Option::None;
        }
        let mut read_stream = ReadStream::new(&self.file);

        get_sub_object_data::<T>(read_stream, key)
    }
}

impl ODSFile {
    pub fn new(file: PathBuf) -> Self {
        ODSFile {
            file
        }
    }
}