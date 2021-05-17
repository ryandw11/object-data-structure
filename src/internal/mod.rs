use crate::tags::general::{Tag, ITag};

pub mod file;
pub mod internal_utils;
pub mod tag_builder;

pub trait ODSInternal {
    fn get<T: 'static>(&mut self, key: String) -> Option<Tag<T>>;
}