use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use derive_getters::Getters;

pub struct BaseNodeInList<K: Copy + PartialOrd, V> {
    node: Rc<RefCell<BaseNodeInner<K, V>>>,
}

impl<K: Copy + PartialOrd, V> Clone for BaseNodeInList<K, V> {
    fn clone(&self) -> Self {
        BaseNodeInList { node: self.node.clone() }
    }
}

impl<K: Copy + PartialOrd, V> BaseNodeInList<K, V> {
    pub fn new(key: K, value: V, right: Option<BaseNodeInList<K, V>>) -> BaseNodeInList<K, V> {
        BaseNodeInList { node: Rc::new(RefCell::new(BaseNodeInner::new(key, value, right))) }
    }

    pub fn get_key(&self) -> K {
        self.get_ref().key
    }
    pub fn get_right_node(&self) -> Option<BaseNodeInList<K, V>> {
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

// lowest node  level=0
#[derive(Getters)]
struct BaseNodeInner<K: Copy + PartialOrd, V> {
    pub key: K,
    pub value: V,
    right: Option<BaseNodeInList<K, V>>,
}


impl<K: Copy + PartialOrd, V> BaseNodeInner<K, V> {
    fn new(key: K, value: V, right: Option<BaseNodeInList<K, V>>) -> BaseNodeInner<K, V> {
        BaseNodeInner { key, value, right }
    }
}


mod test {
    use super::BaseNodeInner;

    fn test() {
        let n = BaseNodeInner::new(1, 2, None);
        n.key();
    }
}

// }