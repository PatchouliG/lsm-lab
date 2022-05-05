use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, LinkedList};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

struct List<K: Hash + Eq + Copy, E: Clone> {
    head: Option<Rc<RefCell<ListNode<K, E>>>>,
    tail: Option<Rc<RefCell<ListNode<K, E>>>>,
}
#[derive(Default)]
struct ListNode<K: Hash + Eq + Copy, E: Clone> {
    k: K,
    e: Rc<E>,
    next: Option<Rc<RefCell<ListNode<K, E>>>>,
    before: Option<Rc<RefCell<ListNode<K, E>>>>,
}

pub struct PopResult<K: Hash + Eq + Copy, E: Clone> {
    k: K,
    e: Rc<E>,
}

impl<K: Hash + Eq + Copy, E: Clone> Deref for PopResult<K, E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        self.e.as_ref()
    }
}

impl<K: Hash + Eq + Copy, E: Clone> PopResult<K, E> {
    fn new(node: Rc<RefCell<ListNode<K, E>>>) -> Self {
        let a = (node.borrow() as &RefCell<ListNode<K, E>>).borrow_mut();
        PopResult {
            k: a.k,
            e: a.e.clone(),
        }
    }
}

impl<K: Hash + Eq + Copy, E: Clone> List<K, E> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }
    fn push_back(&mut self, key: K, e: E) {
        let mut node = ListNode {
            k: key,
            e: Rc::new(e),
            next: None,
            before: None,
        };
        let node_rc = Rc::new(RefCell::new(node));
        self.push_node_back(node_rc);
    }
    fn push_node_back(&mut self, mut node_rc: Rc<RefCell<ListNode<K, E>>>) {
        match &self.tail {
            None => {
                self.head = Some(node_rc);
                self.tail = self.head.clone();
            }
            Some(n) => {
                (node_rc.borrow() as &RefCell<ListNode<K, E>>)
                    .borrow_mut()
                    .before = Some(n.clone());
                // let node_rc = Rc::new(RefCell::new(node));
                (n.borrow() as &RefCell<ListNode<K, E>>).borrow_mut().next = Some(node_rc.clone());
                self.tail = Some(node_rc);
            }
        }
    }
    fn remove(&mut self, node: Rc<RefCell<ListNode<K, E>>>) {
        let mut current = (node.borrow() as &RefCell<ListNode<K, E>>).borrow_mut();
        match &current.before {
            Some(n) => {
                let mut a = ((n.borrow() as &Rc<RefCell<ListNode<K, E>>>).borrow()
                    as &RefCell<ListNode<K, E>>)
                    .borrow_mut();
                a.next = current.next.clone()
            }
            None => {}
        }
        match &current.next {
            Some(n) => {
                let mut a = ((n.borrow() as &Rc<RefCell<ListNode<K, E>>>).borrow()
                    as &RefCell<ListNode<K, E>>)
                    .borrow_mut();
                a.before = current.before.clone()
            }
            None => {}
        }
        if Rc::ptr_eq(&self.head.as_ref().unwrap(), &node) {
            self.head = current.borrow().next.clone();
        }
        if Rc::ptr_eq(&self.tail.as_ref().unwrap(), &node) {
            self.tail = current.borrow().before.clone();
        }
        // let s = current.borrow_mut().next.clone();
        // current_option = s;
    }

    fn pop_front(&mut self) -> Option<PopResult<K, E>> {
        let head = self.head.take();
        match head {
            None => None,
            Some(n) => {
                let res = n.clone();
                let mut head_node = (n.borrow() as &RefCell<ListNode<K, E>>).borrow_mut();

                let next_node = &head_node.next;
                // clean next before
                next_node.as_ref().map(|n| {
                    (n.borrow() as &RefCell<ListNode<K, E>>)
                        .borrow_mut()
                        .before
                        .take()
                });
                if Rc::ptr_eq(&n, self.tail.as_ref().unwrap()) {
                    self.tail.take();
                }

                // set head
                self.head = next_node.clone();
                // clean head next
                head_node.next.take();
                drop(head_node);
                drop(n);

                Some(PopResult::new(res))
            }
        }
    }
}

pub struct LRUBLockCache<K: Hash + Eq + Copy, E: Clone> {
    map: HashMap<K, Rc<RefCell<ListNode<K, E>>>>,
    list: List<K, E>,
    capacity: usize,
}

impl<K: Hash + Eq + Copy, E: Clone> LRUBLockCache<K, E> {
    pub fn new(capacity: usize) -> Self {
        LRUBLockCache {
            map: HashMap::new(),
            list: List::new(),
            capacity,
        }
    }
    pub fn put(&mut self, key: K, e: E) {
        if self.map.len() == self.capacity {
            let node = self.list.pop_front();
            if node.is_some() {
                let k = node.unwrap().k;
                self.map.remove(&k);
            }
        }
        let mut node = ListNode {
            k: key,
            e: Rc::new(e),
            next: None,
            before: None,
        };
        let node_rc = Rc::new(RefCell::new(node));
        self.map.insert(key, node_rc.clone());
        self.list.push_node_back(node_rc);
    }

    pub fn get(&mut self, k: K) -> Option<PopResult<K, E>> {
        let index_option = self.map.get(&k);
        if index_option.is_some() {
            self.list.remove(index_option.unwrap().clone());
            self.list.push_node_back(index_option.unwrap().clone());
            Some(PopResult::new(index_option.unwrap().clone()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cache::{LRUBLockCache, List};

    #[test]
    fn test_list() {
        let mut list = List::new();
        list.push_back(1, 1);
        list.push_back(2, 2);
        list.push_back(3, 3);

        let mut m = list.pop_front().unwrap();
        assert_eq!(1, *m);
        m = list.pop_front().unwrap();
        assert_eq!(2, *m);
        m = list.pop_front().unwrap();
        assert_eq!(3, *m);

        // just 1
        list.push_back(1, 1);
        m = list.pop_front().unwrap();
        assert_eq!(1, *m);

        // pop none
        let m = list.pop_front();
        assert!(m.is_none());

        list.push_back(1, 1);
        list.push_back(2, 2);
        list.push_back(3, 3);
    }
    #[test]
    fn test_cache_get_put() {
        let mut cache = LRUBLockCache::new(2);
        let a_k = 1;
        let a_v = 1;
        let b_k = 2;
        let b_v = 2;
        let c_k = 3;
        let c_v = 3;
        cache.put(a_k, a_v);
        cache.put(b_k, b_v);
        assert_eq!(a_v, *cache.get(a_k).unwrap());
        assert_eq!(b_v, *cache.get(b_k).unwrap());
        cache.put(c_k, c_v);
        assert_eq!(c_v, *cache.get(c_k).unwrap());
        assert!(cache.get(a_k).is_none());
    }
}
