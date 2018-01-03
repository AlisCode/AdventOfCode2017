use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Component {
    start: u32,
    end: u32,
    pub strength: u32,
}

impl Component {
    pub fn new(start: u32, end: u32) -> Self {
        Component {
            start,
            end,
            strength: start + end,
        }
    }

    pub fn from_input(input: &str) -> Self {
        let parts: Vec<&str> = input.split('/').collect();

        let one: u32 = parts[0].parse::<u32>().unwrap();
        let two: u32 = parts[1].parse::<u32>().unwrap();

        Component::new(one, two)
    }
}

/// REDO THIS SOMEDAY. PART 2 NOT WORKING.
pub fn get_max_component(
    list: &[(Component)],
    used_index: &HashSet<usize>,
    last: u32,
    p2: bool,
) -> Component {
    if list.len() == used_index.len() {
        return Component::new(0, 0);
    }

    let mut u = used_index.clone();
    list
        .iter()
        .enumerate()
        .filter(|&(i, p)| !used_index.contains(&i) && (p.start == last || p.end == last))
        .map(|(i, p)| {
            u.insert(i);
            let comp = get_max_component(list, &u, p.start + p.end - last, p2);
            u.remove(&i);
            Component::new(comp.start + (p2 as u32), comp.end + p.start + p.end)
        })
        .max()
        .unwrap_or(Component::new(0, 0))
}

pub fn resolve_part_one(input: &str) -> u32 {
    let components: Vec<Component> =
        input
            .lines()
            .map(|a| Component::from_input(a))
            .collect();

    let max_comp = get_max_component(&components, &HashSet::new(), 0, false);
    max_comp.strength
}

pub fn resolve_part_two(input: &str) -> u32 {
    let components: Vec<Component> =
        input
            .lines()
            .map(|a| Component::from_input(a))
            .collect();

    let max_comp = get_max_component(&components, &HashSet::new(), 0, true);
    max_comp.strength
}
