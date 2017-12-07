pub struct Process<'a> {
    name: &'a str,
    weight: i32,
    children_names: Vec<&'a str>,
    children_process: Vec<&'a Process<'a>>,
    parent: Option<&'a Process<'a>>,
}

impl<'a> Process<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut iter_input = input.split_whitespace();
        let name: &str = iter_input.next().unwrap();
        let weight: i32 = iter_input.next().unwrap().parse::<i32>().unwrap();

        println!("got a new process : name {}, weight: {}", name, weight);

        Process {
            name,
            weight,
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
