use crate::tags::general::{Container, ObjectContainer, VectorContainer};
use std::any::Any;

pub type Object = Container<ObjectContainer>;
pub type Vector = Container<VectorContainer>;

pub type Short = i16;
pub type Integer = i32;
pub type Long = i64;
pub type Float = f32;
pub type Double = f64;

pub type AnyObject = Box<dyn Any>;