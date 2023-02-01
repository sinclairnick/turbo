use std::vec::IntoIter;

pub trait TreeIterExt<L>: Iterator {
    type Output: Iterator<Item = L>;
    /// Flattens a tree structure into a single iterator of leaf nodes.
    /// Supported iterated types are: LeafFirstThenChildren.
    fn flat_map_tree(self) -> Self::Output;
}

/// Visits leafs first, and recursed into children afterwards.
pub struct LeafFirstThenChildren<L>(pub Option<L>, pub Vec<LeafFirstThenChildren<L>>);

impl<L, It> TreeIterExt<L> for It
where
    It: Iterator<Item = LeafFirstThenChildren<L>>,
{
    type Output = FlatMapTree<L, It>;
    fn flat_map_tree(self) -> Self::Output {
        FlatMapTree {
            it: self,
            stack: Vec::new(),
        }
    }
}

pub struct FlatMapTree<L, It>
where
    It: Iterator<Item = LeafFirstThenChildren<L>>,
{
    it: It,
    stack: Vec<IntoIter<LeafFirstThenChildren<L>>>,
}

impl<L, It> Iterator for FlatMapTree<L, It>
where
    It: Iterator<Item = LeafFirstThenChildren<L>>,
{
    type Item = L;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(mut current) = self.stack.pop() {
                if let Some(LeafFirstThenChildren(leaf, children)) = current.next() {
                    self.stack.push(children.into_iter());
                    if let Some(l) = leaf {
                        return Some(l);
                    }
                }
            } else if let Some(LeafFirstThenChildren(leaf, children)) = self.it.next() {
                self.stack.push(children.into_iter());
                if let Some(l) = leaf {
                    return Some(l);
                }
            } else {
                return None;
            }
        }
    }
}
