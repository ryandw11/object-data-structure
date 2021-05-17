use crate::internal::ODSInternal;
use crate::tags::general::{Tag, ITag};
use std::path::PathBuf;
use crate::internal::file::ODSFile;
use std::fmt::Debug;

pub struct ObjectDataStruct<T: ODSInternal> {
    internal: T
}

impl ObjectDataStruct<ODSFile> {
    pub fn new_file(file: PathBuf) -> Box<ObjectDataStruct<ODSFile>> {
        Box::new(ObjectDataStruct {
            internal: ODSFile::new(file)
        })
    }

    pub fn get<T: 'static>(&mut self, key: String) -> Option<Tag<T>> {
        let tag = self.internal.get::<T>(key);
        if tag.is_none() {
            return Option::None;
        }

        tag
    }
}