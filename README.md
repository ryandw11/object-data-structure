**NOTE: This version is still under active development and is not ready for use. Please check back later.**
# Object Data Structure (ODS) for Rust
Object Data Structure is a file format inspired by NBT. Everything in this file format is made of tags. ODS is not human-readable, data is stored in bytes.
This repository is the Rust port of ODS. [The main version of ODS can be found here.](https://github.com/ryandw11/ODS)

# Usage
Due to the nature of Rust the Rust version of ODS functions differently than the Java and C# versions. In this version there is simply a generic Tag struct. That Tag struct
a generic parameter that implements the `Taggable` trait. For example, the String struct implements Taggable. The normal StringTag, IntTag, etc, are defined types
that make it easier to use.
  
Unlike other version of ODS, you must supply the expected data type that you want to retreive. So to obtain a tag in a file you must use:
```rust
use std::path::PathBuf;
use object_data_structure::ods::ObjectDataStructure;
use object_data_structure::tags::general::{StringTag, Tag};

fn main() {
    let mut ods: ObjectDataStructure = ObjectDataStructure::new_file(PathBuf::from("./test.ods"));
    let example_tag: StringTag = ods.get::<String>("ExampleTag".to_string()).unwrap();
    
    println!("{}", example_tag.get_value());
}
```

### AnyTag
AnyTag is a tag type specific to the rust version of ODS. An AnyTag represents a tag of any type. The AnyTag is defined internally by:
```rust
type AnyTag = Tag<Box<dyn Any>>;
```
Since wildcards do not exist in Rust an AnyTag is used as a replacement. The AnyTag allows you to group together tags of any type. You can convert a normal tag to an AnyTag by doing the following:
```rust
let string_tag = StringTag::new("TestTag".to_string(), "My example value.".to_string());
let any_tag = AnyTag::from_tag::<String>(string_tag);
```
You can then convert it back to a string tag by doing the following:
```rust
let string_tag: StringTag = any_tag.downcast_any_tag::<String>();
```
*Note: You currently need to know the type of the anytag in order to convert it back to a normal tag.*  
  
It is important to note that AnyTag is a special tag type that cannot be stored in a file. Attempt to save an AnyTag to a file directly will result in ODS panicing.

# ODS Visualizer
This tool allows you inspect ods files. The tool is coded in Java so you will need to install Java to use it.
![An image of the visualizer.](https://img.ryandw11.com/raw/oxoijtnok.png)  
[Click here to go to the visualizer repository.](https://github.com/ryandw11/ODS_Visualizer)  

# Offical Language Ports
- [ODS (Java)](https://github.com/ryandw11/ODS)
- [ODSSharp (C#)](https://github.com/ryandw11/ODSSharp)

# Contributing to the project
Feel free to contribute any bug fixes or performance optimizations to this repository.  
Any changes to the API (not specific to the rust version) must be suggested [on the main repository](https://github.com/ryandw11/ODS).
