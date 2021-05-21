use crate::internal::ODSInternal;
use crate::tags::general::{Tag, Taggable, AnyTag};
use std::path::PathBuf;
use crate::internal::file::ODSFile;
use std::fmt::Debug;

pub struct ObjectDataStructure<T: ODSInternal> {
    internal: T
}

impl ObjectDataStructure<ODSFile> {
    pub fn new_file(file: PathBuf) -> Box<ObjectDataStructure<ODSFile>> {
        Box::new(ObjectDataStructure {
            internal: ODSFile::new(file)
        })
    }

    pub fn get<T: Taggable<T>>(&mut self, key: String) -> Option<Tag<T>> {
        self.internal.get::<T>(key)
    }

    pub fn get_all(&mut self) -> Option<Vec<AnyTag>> {
        self.internal.get_all()
    }
}