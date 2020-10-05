use std::fmt::Debug;

type Node<'a, T> = Option<Box<HuffmanTreeNode<'a, T>>>;
pub type TreeItem<'a, T> = HuffmanTreeItem<'a, T>;

#[derive(Debug)]
pub struct HuffmanTree<'a, T>
where
    T: Debug,
{
    pub tree: Node<'a, T>,
    pub data: &'a Vec<TreeItem<'a, T>>,
}

#[derive(Debug)]
pub struct HuffmanTreeItem<'a, T>
where
    T: Debug,
{
    pub frequency: u32,
    pub data: Option<&'a T>,
}

impl<'a, T> HuffmanTreeItem<'a, T>
where
    T: Debug,
{
    pub fn new(data: &'a T, frequency: u32) -> Self {
        Self {
            data: Some(data),
            frequency,
        }
    }

    pub fn create_node(&'a self) -> Option<Box<HuffmanTreeNode<'a, T>>> {
        Some(Box::new(HuffmanTreeNode {
            left: None,
            right: None,
            item: Some(self),
            weight: self.frequency,
        }))
    }
}

#[derive(Debug)]
pub struct HuffmanTreeNode<'a, T>
where
    T: Debug,
{
    pub left: Node<'a, T>,
    pub right: Node<'a, T>,
    pub item: Option<&'a TreeItem<'a, T>>,
    pub weight: u32,
}

impl<'a, T> HuffmanTree<'a, T>
where
    T: Debug,
{
    pub fn new(data: &'a Vec<TreeItem<'a, T>>) -> Self {
        Self { tree: None, data }
    }

    pub fn build(&mut self) {
        match self.data.len() {
            0 => return,
            1 => {
                let tree = &mut self.tree;
                let item = &self.data[0];
                *tree = Some(Box::new(HuffmanTreeNode {
                    left: None,
                    right: None,
                    item: Some(item),
                    weight: item.frequency,
                }));
            }
            _ => {
                let mut list: Vec<Node<'a, T>> = self
                    .data
                    .into_iter()
                    .map(|each| each.create_node())
                    .collect();
                while list.len() > 1 {
                    let (left_index, right_index) = self.get_min_weight_index(&list);
                    let mut left = list.remove(left_index);
                    let mut right = list.remove(right_index - 1); //-1 becase the first remove changes the size
                    list.insert(0, self.merge_nodes(&mut left, &mut right));
                }
                let tree = &mut self.tree;
                *tree = list.pop().unwrap();
            }
        }
    }

    fn merge_nodes(&self, left: &mut Node<'a, T>, right: &mut Node<'a, T>) -> Node<'a, T> {
        let size = left.as_ref().unwrap().weight + right.as_ref().unwrap().weight;
        Some(Box::new(HuffmanTreeNode {
            left: left.take(),
            right: right.take(),
            item: None,
            weight: size,
        }))
    }

    fn get_min_weight_index(&self, list: &Vec<Node<'a, T>>) -> (usize, usize) {
        let mut min = None;
        let mut indicies: (usize, usize) = (0, 0);
        let list_len = list.len();
        for index in 0..list_len {
            if list[index].as_ref().is_none() {
                break;
            }
            for inner_index in 0..list_len {
                if index == inner_index {
                    continue;
                }
                if list[inner_index].as_ref().is_none() {
                    break;
                }
                let weight = list[index].as_ref().unwrap().weight
                    + list[inner_index].as_ref().unwrap().weight;
                if min.is_none() || weight < min.unwrap() {
                    min = Some(weight);
                    indicies = (index, inner_index);
                }
            }
        }
        indicies
    }

    pub fn original_data_size(&self) -> usize {
        self.data
            .into_iter()
            .fold(0, |acc, each| each.frequency as usize + acc)
            * 8
    }

    pub fn encoded_data_size(&self) -> usize {
        let mut count: isize = 0;
        let mut size = 0;
        fn traverse<'a, T: Debug>(
            node: &Box<HuffmanTreeNode<'a, T>>,
            count: &mut isize,
            size: &mut usize,
        ) {
            if let Some(next_node) = &node.left {
                *count += 1;
                traverse(next_node, count, size);
            }
            if let Some(data) = node.item {
                *size += (data.frequency as isize * (*count)) as usize;
            }
            if let Some(next_node) = &node.right {
                *count += 1;
                traverse(next_node, count, size);
            }
            *count -= 1;
        }
        traverse(&self.tree.as_ref().unwrap(), &mut count, &mut size);
        size
    }
}
