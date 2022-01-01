#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

pub struct BaseNode<K: Copy + PartialOrd, V> {
    node: Rc<RefCell<BaseNodeInner<K, V>>>,
}
// lowest node  level=0
struct BaseNodeInner<K: Copy + PartialOrd, V> {
    pub key: K,
    pub value: V,
    right: Option<BaseNode<K, V>>,
}

impl<K: Copy + PartialOrd, V> Clone for BaseNode<K, V> {
    fn clone(&self) -> Self {
        BaseNode { node: self.node.clone() }
    }
}

impl<K: Copy + PartialOrd, V> BaseNode<K, V> {
    pub fn new(key: K, value: V, right: Option<BaseNode<K, V>>) -> BaseNode<K, V> {
        BaseNode { node: Rc::new(RefCell::new(BaseNodeInner::new(key, value, right))) }
    }

    pub fn get_key(&self) -> K {
        self.get_ref().key
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

pub struct SkipListIter<K: Copy + PartialOrd, V> {
    node: Option<BaseNode<K, V>>,
}

impl <K: Copy + PartialOrd, V> SkipListIter<K,V>{
    pub fn new(node:Option<BaseNode<K,V>>) ->Self{
        SkipListIter{node}
    }
}

impl<K: Copy + PartialOrd, V> Iterator for SkipListIter<K, V> {
    type Item = BaseNode<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.node.take();
        match current {
            Some(n) => {
                self.node = n.get_right_node();
                Some(n)
            }
            None => {
                None
            }
        }
    }
}

// }