#![allow(dead_code)]

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct BaseNode<K: Copy + PartialOrd, V> {
    node: Rc<RefCell<BaseNodeInner<K, V>>>,
}

// lowest node  level=0
pub struct BaseNodeInner<K: Copy + PartialOrd, V> {
    pub key: K,
    pub value: V,
    right: Option<BaseNode<K, V>>,
}

impl<K: Copy + PartialOrd, V> Clone for BaseNode<K, V> {
    fn clone(&self) -> Self {
        BaseNode {
            node: self.node.clone(),
        }
    }
}
impl<K: Copy + PartialOrd, V: Copy> BaseNode<K, V> {
    pub fn get_value(&self) -> V {
        self.get_ref().value.clone()
    }
}

impl<K: Copy + PartialOrd, V> BaseNode<K, V> {
    pub fn new(key: K, value: V, right: Option<BaseNode<K, V>>) -> BaseNode<K, V> {
        BaseNode {
            node: Rc::new(RefCell::new(BaseNodeInner::new(key, value, right))),
        }
    }

    pub fn get_key(&self) -> K {
        self.get_ref().key
    }
    pub fn set_value(&mut self, value: V) {
        let mut v = self.get_mut_ref();
        v.value = value;
    }

    pub fn map_value<T>(&self, f: fn(&V) -> T) -> T {
        f(&self.get_ref().value)
    }

    pub fn get_node(&self) -> Rc<RefCell<BaseNodeInner<K, V>>> {
        self.node.clone()
    }
    pub fn get_right_node(&self) -> Option<BaseNode<K, V>> {
        let n = self.get_ref();
        let res = n.right.clone();
        res
    }
    pub fn set_right_node(&mut self, right: Option<Self>) {
        let mut n = self.get_mut_ref();
        n.right = right;
    }
    fn get_ref(&self) -> Ref<BaseNodeInner<K, V>> {
        let n = self.node.borrow();
        n
    }
    fn get_mut_ref(&self) -> RefMut<BaseNodeInner<K, V>> {
        let n = self.node.borrow_mut();
        n
    }
}

impl<K: Copy + PartialOrd, V> BaseNodeInner<K, V> {
    fn new(key: K, value: V, right: Option<BaseNode<K, V>>) -> BaseNodeInner<K, V> {
        BaseNodeInner { key, value, right }
    }
}

pub enum IndexNodeChild<K: Copy + PartialOrd, V> {
    Base(BaseNode<K, V>),
    Index(IndexNode<K, V>),
}

impl<K: Copy + PartialOrd, V> Clone for IndexNodeChild<K, V> {
    fn clone(&self) -> Self {
        match self {
            IndexNodeChild::Base(b) => IndexNodeChild::Base(b.clone()),
            IndexNodeChild::Index(i) => IndexNodeChild::Index(i.clone()),
        }
    }
}

struct IndexNodeInner<K: Copy + PartialOrd, V> {
    key: K,
    right: Option<IndexNode<K, V>>,
    child: IndexNodeChild<K, V>,
}

pub struct IndexNode<K: Copy + PartialOrd, V> {
    node: Rc<RefCell<IndexNodeInner<K, V>>>,
}

pub struct IndexNodeIterator<K: Copy + PartialOrd, V> {
    pub node: Option<IndexNode<K, V>>,
}

impl<K: Copy + PartialOrd, V> Iterator for IndexNodeIterator<K, V> {
    type Item = IndexNode<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.node.clone();
        self.node = self.node.as_ref().and_then(|n| n.get_right_node());
        res
    }
}

impl<K: Copy + PartialOrd, V> Clone for IndexNode<K, V> {
    fn clone(&self) -> Self {
        IndexNode {
            node: self.node.clone(),
        }
    }
}

impl<K: Copy + PartialOrd, V> IndexNode<K, V> {
    pub fn new(key: K, right: Option<IndexNode<K, V>>, child: IndexNodeChild<K, V>) -> Self {
        IndexNode {
            node: Rc::new(RefCell::new(IndexNodeInner { key, right, child })),
        }
    }

    pub fn to_iter(&self) -> IndexNodeIterator<K, V> {
        IndexNodeIterator {
            node: Some(self.clone()),
        }
    }

    pub fn get_key(&self) -> K {
        self.get_ref().key
    }
    pub fn get_right_node(&self) -> Option<Self> {
        self.get_ref().right.clone()
    }
    pub fn get_child(&self) -> IndexNodeChild<K, V> {
        self.get_ref().child.clone()
    }
    pub fn set_right(&mut self, right: Option<IndexNode<K, V>>) {
        self.get_mut_ref().right = right;
    }
    fn get_ref(&self) -> Ref<IndexNodeInner<K, V>> {
        let n = self.node.borrow();
        n
    }
    fn get_mut_ref(&self) -> RefMut<IndexNodeInner<K, V>> {
        self.node.borrow_mut()
    }
}

pub struct BaseNodeIterator<K: Copy + PartialOrd, V> {
    node: Option<BaseNode<K, V>>,
}

impl<K: Copy + PartialOrd, V> BaseNodeIterator<K, V> {
    pub fn new(node: Option<BaseNode<K, V>>) -> Self {
        BaseNodeIterator { node }
    }
}

impl<K: Copy + PartialOrd, V> Iterator for BaseNodeIterator<K, V> {
    type Item = BaseNode<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.node.take();
        match current {
            Some(n) => {
                self.node = n.get_right_node();
                Some(n)
            }
            None => None,
        }
    }
}
