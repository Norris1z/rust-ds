use std::cmp::{Ord, Ordering};
use std::iter::IntoIterator;

#[derive(Debug)]
pub struct BinaryTree<'a, T: Ord> {
    value: &'a T,
    right: Option<Box<BinaryTree<'a, T>>>,
    left: Option<Box<BinaryTree<'a, T>>>,
    count: usize,
}

impl<'a, T> BinaryTree<'a, T>
where
    T: Ord,
{
    pub fn new(value: &'a T) -> Self {
        Self {
            value,
            right: None,
            left: None,
            count: 1,
        }
    }

    fn next_node_or_create(next_node: &mut Option<Box<BinaryTree<'a, T>>>, value: &'a T) {
        match next_node {
            Some(node) => {
                node.insert_node(value);
            }
            None => {
                *next_node = Some(Box::new(BinaryTree {
                    value,
                    right: None,
                    left: None,
                    count: 1,
                }))
            }
        }
    }

    pub fn insert_node(&mut self, value: &'a T) {
        match value.cmp(self.value) {
            Ordering::Equal => {
                self.count += 1;
            }
            Ordering::Greater => {
                BinaryTree::next_node_or_create(&mut self.right, value);
            }
            Ordering::Less => {
                BinaryTree::next_node_or_create(&mut self.left, value);
            }
        }
    }

    pub fn to_vec_ascending(&self) -> Vec<&'a T> {
        let mut output: Vec<&'a T> = vec![];
        traverse(self, &mut output, &TraverseMode::Ascending);
        output
    }

    pub fn to_vec_descending(&self) -> Vec<&'a T> {
        let mut output: Vec<&'a T> = vec![];
        traverse(self, &mut output, &TraverseMode::Descending);
        output
    }

    pub fn get_value_count(&self, value: &'a T) -> Option<usize> {
        fn tr<'x, U: Ord>(node: &BinaryTree<'x, U>, value: &'x U) -> Option<usize> {
            return match value.cmp(node.value) {
                Ordering::Equal => Some(node.count),
                Ordering::Greater => {
                    if let Some(ref next_node) = node.right {
                        return tr(next_node, value);
                    }
                    None
                }
                Ordering::Less => {
                    if let Some(ref next_node) = node.left {
                        return tr(next_node, value);
                    }
                    None
                }
            };
        }
        tr(self, value)
    }
}

impl<'a, T> IntoIterator for BinaryTree<'a, T>
where
    T: Ord,
{
    type Item = &'a T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        let mut output: Vec<&'a T> = vec![];
        traverse(&self, &mut output, &TraverseMode::Ascending);
        output.into_iter()
    }
}

pub enum TraverseMode {
    Ascending,
    Descending,
}

fn traverse<'x, U: Ord>(node: &BinaryTree<'x, U>, out: &mut Vec<&'x U>, mode: &TraverseMode) {
    if let Some(ref next_node) = match mode {
        TraverseMode::Ascending => node.left.as_ref(),
        TraverseMode::Descending => node.right.as_ref(),
    } {
        traverse(next_node, out, mode);
    }

    out.push(node.value);

    if let Some(ref next_node) = match mode {
        TraverseMode::Ascending => node.right.as_ref(),
        TraverseMode::Descending => node.left.as_ref(),
    } {
        traverse(next_node, out, mode);
    }
}
