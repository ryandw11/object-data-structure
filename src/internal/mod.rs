use crate::tags::general::{Tag, Taggable, AnyTag};

pub mod file;
pub mod internal_utils;
pub mod tag_builder;

pub trait ODSInternal {
    fn get<T: Taggable<T>>(&mut self, key: String) -> Option<Tag<T>>;
    fn get_all(&mut self) -> Option<Vec<AnyTag>>;
}