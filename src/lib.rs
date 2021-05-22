/**
    Object Data Structure (ODS) for Rust.
*/
pub mod io;
pub mod tags;
pub mod ods;
#[macro_use]
pub mod util;
pub mod internal;

extern crate byteorder;
#[macro_use]
extern crate dyn_clone;
#[macro_use]
extern crate downcast_rs;

#[cfg(test)]
mod tests {
    use crate::tags::general::{StringTag, DoubleTag, VecTag, AnyTag};
    use crate::io::streams::WriteStream;
    use std::fs;
    use std::path::PathBuf;
    use crate::ods::ObjectDataStructure;
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

        let mut ods = ObjectDataStructure::new_file(PathBuf::from("./test.ods"));
        // let data = ods.get::<String>("Test".to_string());

        // println!("Value From Test Tag: {}", data.unwrap().get_value());

        // let tags = ods.get_all().unwrap();
        // println!("{:?}", tags[0].downcast_any_tag::<VecTag>().get_value());

        let test = ods.get::<Vec<AnyTag>>("My_Double".to_string());
        // println!("{:?}", test.unwrap().get::<String>(0));

        // let example_tag = VecTag::new("My_Double".to_string(), vec_tag!(
        //     StringTag::new("My_Cool_Tag".to_string(), "My Next Value!".to_string()),
        //     StringTag::new("3".to_string(), " greig erjiog eorjg oe!".to_string()),
        //     StringTag::new("f".to_string(), " few fw efwe!".to_string()),
        //     DoubleTag::new("db".to_string(), 20.5436)
        // ));
        // ods.append(example_tag);

        println!("{}", ods.find("Test_Tag".to_string()));

        // ods.delete("Test_Tag".to_string());

        ods.replace_data("My_Cool_Tag".to_string(), StringTag::new("My_Cool_Tag".to_string(), "My Other Value!".to_string()));

    }
}
