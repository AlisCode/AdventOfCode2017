pub struct Process<'a> {
    name: &'a str,
    weight: i32,
    children_names: Vec<&'a str>,
    children_process: Vec<&'a Process<'a>>,
    parent: Option<&'a Process<'a>>,
}

impl<'a> Process<'a> {
    pub fn new(input: &'a str) -> Self {

        Process {
            name: "test",
            weight: 1,
            children_names: Vec::new(),
            children_process: Vec::new(),
            parent: None,
        }
    }

    pub fn get_root_process(&self) -> &'a str {
        match self.parent {
            Some(parent) => parent.get_root_process(),
            _ => self.name,
        }
    }
}
