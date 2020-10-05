pub struct LinkedList<'a, T> {
    next_node: Option<Box<LinkedList<'a, T>>>,
    value: &'a T,
}

impl<'a, T> LinkedList<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Self {
            next_node: None,
            value,
        }
    }

    pub fn insert_node(&mut self, value: &'a T) {
        let node = &mut self.next_node;
        match node {
            Some(next_node) => {
                next_node.insert_node(value);
            }
            None => {
                *node = Some(Box::new(LinkedList {
                    next_node: None,
                    value,
                }))
            }
        }
    }
}
