use std::process::Child;

#[derive(Debug, Clone)]
pub struct KeyScoutChild {
    name: String,
    size: i32,
    starting_index: i32
}

impl KeyScoutChild {
    pub fn new() -> KeyScoutChild {
        KeyScoutChild {
            name: "null".to_string(),
            size: 0,
            starting_index: -1
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_size(&mut self, size: i32){
        self.size = size;
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn set_starting_index(&mut self, starting_index: i32) {
        self.starting_index = starting_index;
    }

    pub fn get_starting_index(&self) -> i32 {
        self.starting_index
    }

    pub fn remove_size(&mut self, amount: i32) {
        self.size -= amount;
    }

    pub fn add_size(&mut self, amount: i32) {
        self.size += amount;
    }
}

#[derive(Clone)]
pub struct KeyScout {
    children: Vec<KeyScoutChild>,
    end: Option<KeyScoutChild>
}

impl KeyScout {
    pub fn new() -> KeyScout {
        KeyScout {
            children: Vec::new(),
            // Null keyscoutchild.
            end: Option::None
        }
    }

    pub fn add_child(&mut self, child: KeyScoutChild) {
        self.children.push(child);
    }

    pub fn get_children(&mut self) -> &Vec<KeyScoutChild> {
        &self.children
    }

    pub fn get_child_by_name(&mut self, name: String) -> Option<&KeyScoutChild> {
        for child in self.children.iter() {
            if child.get_name() == name {
                return Some(child);
            }
        }

        Option::None
    }

    pub fn set_end(&mut self, end: KeyScoutChild) {
        self.end = Some(end);
    }

    pub fn get_end(&mut self) -> &Option<KeyScoutChild> {
        &self.end
    }

    pub fn remove_amount(&mut self, size: i32) {
        for mut child in self.children.iter_mut() {
            child.remove_size(size);
        }
    }

    pub fn add_amount(&mut self, size: i32) {
        for mut child in self.children.iter_mut() {
            child.add_size(size);
        }
    }
}