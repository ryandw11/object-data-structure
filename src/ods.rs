use crate::internal::ODSInternal;
use crate::tags::general::{Tag, Taggable, AnyTag};
use std::path::PathBuf;
use crate::internal::file::ODSFile;

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

    pub fn save(&mut self, tags: Vec<AnyTag>) {
        unimplemented!("This method is not implemented at this time.");
    }

    pub fn append<T: Taggable<T>>(&mut self, tag: Tag<T>) {
        self.internal.append(tag);
    }

    pub fn append_all(&mut self, tags: Vec<AnyTag>){
        unimplemented!("This method is not implemented at this time.");
    }

    pub fn find(&mut self, key: String) -> bool {
        self.internal.find(key)
    }

    pub fn delete(&mut self, key: String) -> bool {
        self.internal.delete(key)
    }

    pub fn replace_data<T: Taggable<T>>(&mut self, key: String, replacement: Tag<T>) -> bool {
        self.internal.replace_data(key, replacement)
    }

    pub fn set<T: Taggable<T>>(&mut self, key: String, value: Option<Tag<T>>) {

    }
}