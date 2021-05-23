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

#[cfg(test)]
mod tests {
    use crate::tags::general::{StringTag, DoubleTag, VecTag, AnyTag, Container, VectorContainer, FloatTag, ShortTag, ObjectTag};
    use crate::io::streams::WriteStream;
    use std::fs;
    use std::path::PathBuf;
    use crate::ods::ObjectDataStructure;
    use std::fmt::Debug;
    use crate::tags::container_types::Object;

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

        let mut test = ods.get::<Container<VectorContainer>>("My_Double".to_string()).unwrap();
        println!("Is Value: {:?}", test.get::<String>(1).unwrap().get_value());


        println!("Is Double: {:?}", test.is_type::<f64>(3));

        println!("Size: {}", test.len());

        // let mut example_tag = VecTag::from_vec("My_Double".to_string(), &mut vec_tag!(
        //     StringTag::new("My_Cool_Tag".to_string(), "My Next Value!".to_string()),
        //     StringTag::new("3".to_string(), " greig erjiog eorjg oe!".to_string()),
        //     StringTag::new("f".to_string(), " few fw efwe!".to_string()),
        //     DoubleTag::new("db".to_string(), 20.5436)
        // ));
        // let mut object_tag = ObjectTag::from_vec("OOF".to_string(), vec_tag!(
        //     StringTag::new("My_Cool_Tag".to_string(), "My Next Value!".to_string()),
        //     StringTag::new("Name".to_string(), " greig erjiog eorjg oe!".to_string()),
        //     StringTag::new("Fun".to_string(), " few fw efwe!".to_string()),
        //     DoubleTag::new("db".to_string(), 20.5436)
        // ));
        //
        // ods.append(object_tag);

        let mut obj = ods.get::<Object>("OOF".to_string()).unwrap();

        println!("OBJ DATA: {}", obj.get::<String>("Fun".to_string()).unwrap().get_value());
        println!("OBJ DATA DIRECT: {}", ods.get::<String>("OOF.Fun".to_string()).unwrap().get_value());

        println!("{}", ods.find("Test_Tag".to_string()));

        ods.append(tag![StringTag, "BOB", "Test".to_string()]);

        // ods.delete("Test_Tag".to_string());

        ods.replace_data("My_Cool_Tag".to_string(), StringTag::new("My_Cool_Tag".to_string(), "My Other Value!".to_string()));

    }
}
