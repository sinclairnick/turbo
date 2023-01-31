use std::future::{Future, IntoFuture};

use anyhow::Result;
use futures::{
    future::{join_all, JoinAll},
    FutureExt,
};

pub trait TreeIterExt<L, It>: Iterator
where
    It: Iterator<Item = Tree<L>>,
{
    /// Flattens a tree structure into a single iterator of leaf nodes.
    /// Tree is represented as a tuple of (Option<L>, Vec<Tree<L>>).
    fn flat_map_tree_deep(self) -> FlatMapTreeDeep<L, It>;
}

type Tree<L> = (Option<L>, Vec<Tree<L>>);

impl<L, It> TreeIterExt for It
where
    It: Iterator<Item = Tree<L>>,
{
    fn flat_map_tree_deep(self) -> FlatMapTreeDeep {
        FlatMapTreeDeep {
            inner: self.flat_map(|(l, children)| {
                std::iter::once(l).chain(children.into_iter().flat_map_tree_deep())
            }),
        }
    }
}

struct FlatMapTreeDeep<L, It>
where
    It: Iterator<Item = Tree<L>>,
{
    stack: Vec<It>,
}

impl<L, It> Iterator for FlatMapTreeDeep<L, It>
where
    It: Iterator<Item = Tree<L>>,
{
    type Item = L;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current) = self.stack.pop() {
                if let Some((leaf, children)) = current.next() {
                    self.stack.extend(children.into_iter().rev());
                    if let Some(l) = leaf {
                        return Some(l);
                    }
                }
            } else {
                return None;
            }
        }
    }
}
