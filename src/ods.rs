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

    pub fn get<T: 'static + ITag + Debug>(&mut self, key: String) -> Option<T> {
        let tag = self.internal.get(key);
        if tag.is_none() {
            return Option::None;
        }
        let new_tag = tag.unwrap();
        println!("{:?}", new_tag.as_any().is::<T>());
        // Issue here: I have no clue how to fix this.
        // let output = new_tag.as_any().downcast::<T>().unwrap();
        // Some(*output)
        Option::None
    }
}