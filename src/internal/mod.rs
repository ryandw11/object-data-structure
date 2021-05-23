use crate::tags::general::{Tag, Taggable, AnyTag};

pub mod file;
pub mod internal_utils;
pub mod tag_builder;
pub mod keyscout;

pub trait ODSInternal {
    fn get<T: Taggable<T>>(&mut self, key: String) -> Option<Tag<T>>;
    fn get_all(&mut self) -> Option<Vec<AnyTag>>;
    fn append<T: Taggable<T>>(&mut self, tag: Tag<T>);
    fn find(&mut self, key: String) -> bool;
    fn delete(&mut self, key: String) -> bool;
    fn replace_data<T: Taggable<T>>(&mut self, key: String, replacement: Tag<T>) -> bool;
    fn set<T: Taggable<T>>(&mut self, key: String, value: Option<Tag<T>>);
}