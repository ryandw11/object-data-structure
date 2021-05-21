/**
    Object Data Structure (ODS) for Rust.
*/
pub mod io;
pub mod tags;
pub mod ods;
pub mod internal;

extern crate byteorder;

#[cfg(test)]
mod tests {
    use crate::tags::general::{StringTag, Tag};
    use crate::io::streams::WriteStream;
    use std::fs;
    use std::path::PathBuf;
    use crate::ods::ObjectDataStruct;
    use std::fmt::Debug;

    pub trait ITestTag {
        fn new() -> Self;
    }

    pub trait TestTag <T> : ITestTag {
        fn my_test(self) -> T;
    }

    pub struct TagIMPL {
        value: String,
        name: String,
    }

    impl TestTag<String> for TagIMPL {
        fn my_test(self) -> String {
            "This is a test!".to_string()
        }
    }

    impl ITestTag for TagIMPL {
        fn new() -> Self {
            TagIMPL {
                value: "Test".to_string(),
                name: "Test".to_string(),
            }
        }
    }

    pub fn print_data<T: TestTag<U>, U: Debug>(tag: T) {
        println!("{:?}", tag.my_test());
    }



    #[test]
    fn it_works() {
        // let tag = TagIMPL::new();
        // print_data(tag);


        // let tag = StringTag::new("Test".to_string(), "Test String from Rust".to_string());
        // let mut stream = WriteStream::new();
        // tag.write_data(&mut stream);
        // stream.export_to_file(PathBuf::from("./test.ods"));

        let mut ods = ObjectDataStruct::new_file(PathBuf::from("./test.ods"));
        let data = ods.get::<String>("Test".to_string());

        println!("Value From Test Tag: {}", data.unwrap().get_value());

        let tags = ods.get_all().unwrap();
        println!("{:?}", tags[0].downcast_any_tag::<String>());

    }
}
