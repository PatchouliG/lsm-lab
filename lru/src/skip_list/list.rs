#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use super::node::{BaseNode, IndexNode, IndexNodeChild, SkipListIter};
use std::alloc::handle_alloc_error;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::fmt::Display;
use std::fs::read_to_string;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::rand::simple_rand::*;
use std::collections::{BTreeMap, HashMap};
use std::iter::Map;

type GetResult<V> = Option<Rc<RefCell<V>>>;

struct SkipList<K: Copy + PartialOrd, V> {
    indexes: BTreeMap<usize, IndexNode<K, V>>,
    base_head: Option<BaseNode<K, V>>,
    len: usize,
    r: RefCell<SimpleRand>,
    // for debug
    always_max: bool,
}

struct Context<K: Copy + PartialOrd, V> {
    op: Operation,
    key: K,
    // some if add op
    value: Option<V>,
    index_nodes_on_path: Vec<IndexNode<K, V>>,
}

fn max_level(len: usize) -> usize {
    match len {
        0 => 0,
        _ => {
            let num: f64 = len as f64;
            num.log2().ceil() as usize
        }
    }
}

impl<K: Copy + PartialOrd, V> Context<K, V> {
    fn with_add_op(key: K, value: V) -> Context<K, V> {
        Context {
            op: Operation::Add,
            key,
            value: Some(value),
            index_nodes_on_path: vec![],
        }
    }
    fn visit(&mut self, node: IndexNode<K, V>) {
        self.index_nodes_on_path.push(node);
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Operation {
    Add,
    Get,
    Remove,
}

use crate::skip_list::node::{BaseNodeInner, IndexNodeIterator};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Graph<K, V> {
    base: Vec<(K, V)>,
    index: BTreeMap<usize, Vec<K>>,
}

impl<K: Display + Copy, V: Display + Copy> Graph<K, V> {
    pub fn new() -> Graph<K, V> {
        Graph {
            base: vec![],
            index: BTreeMap::new(),
        }
    }
    pub fn add_base_key(&mut self, k: K, value: V) {
        self.base.push((k, value));
    }
}

impl<K: Copy + PartialOrd + Display + Serialize, V: Display + Copy + Serialize> SkipList<K, V> {
    pub fn to_graph(&self) -> Graph<K, V> {
        let mut graph = Graph::new();
        let iterator = self.to_iter();
        for node_iter in iterator {
            let key = node_iter.get_key();
            let value = node_iter.map_value(|v| v.clone());
            graph.add_base_key(key, value);
        }
        for (level_number, level_head) in self.indexes.iter() {
            let mut vec = vec![];
            for node in level_head.to_iter() {
                vec.push(node.get_key());
            }
            graph.index.insert(*level_number, vec);
        }
        graph
    }
    pub fn to_string(&self) -> String {
        String::from(serde_json::to_string(&self.to_graph()).unwrap())
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

impl<K: Copy + PartialOrd, V> SkipList<K, V> {
    pub fn new() -> SkipList<K, V> {
        SkipList {
            indexes: BTreeMap::new(),
            base_head: None,
            len: 0,
            r: RefCell::new(SimpleRand::new()),
            always_max: false,
        }
    }
    pub fn with_max_level() -> SkipList<K, V> {
        let mut res = SkipList::new();
        res.always_max = true;
        res
    }
    pub fn to_iter(&self) -> SkipListIter<K, V> {
        SkipListIter::new(self.base_head.clone())
    }
    pub fn level(&self) -> usize {
        self.indexes.len()
    }
    pub fn add(&mut self, key: K, value: V) {
        if self.is_empty() {
            let base_node = BaseNode::new(key, value, None);
            self.base_head = Some(base_node);
            self.inc_len();
            return;
        }
        // insert to head if key is less then head key
        let head = self.get_head_base();
        if head.get_key().gt(&key) {
            let base_node = BaseNode::new(key, value, Some(head));
            self.base_head = Some(base_node.clone());
            self.inc_len();
            self.fix_index_nodes(key, vec![], base_node);
            return;
        }

        // head index is more than key, visit base
        let (base_node, indexs) =
            if !self.has_index_level() || self.get_head_index().unwrap().get_key() > key {
                let base_node = self.search_in_base(key, self.get_head_base());
                (base_node, vec![])
            } else {
                self.search_by_index(key, Operation::Add)
            };
        self.handle_add(key, value, base_node, indexs);
        return;
    }
    pub fn map_value<T>(&self, key: K, f: fn(&V) -> T) -> Option<T> {
        // handle empty
        if self.len() == 0 {
            return None;
        }
        // check first base node
        let head = self.get_head_base();
        if head.get_key() > key {
            return None;
        }
        // handle one level case
        let node_founded =
            if !self.has_index_level() || self.get_head_index().unwrap().get_key() > key {
                let head = self.get_head_base();
                self.search_in_base(key, head)
            } else {
                let (base, _) = self.search_by_index(key, Operation::Get);
                self.search_in_base(key, base)
            };

        if node_founded.get_key() == key {
            return Some(node_founded.map_value(f));
        } else {
            return None;
        }
    }
    pub fn remove(&mut self, key: K) {
        // handle empty
        // handle one levels,len -=1
        // visit by visit handle
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    // ---------private-------------

    fn inc_len(&mut self) {
        self.len += 1;
    }

    fn search_in_base(&self, key: K, start_node: BaseNode<K, V>) -> BaseNode<K, V> {
        assert!(self.len() >= 1);
        assert!(key >= self.get_head_base().get_key());
        let mut base_node_iter = SkipListIter::new(Some(start_node));
        let node = base_node_iter
            .find(|n| {
                n.get_key() <= key
                    && (n.get_right_node().is_none() || n.get_right_node().unwrap().get_key() > key)
            })
            .unwrap();
        node
    }

    fn search_by_index(&self, key: K, op: Operation) -> (BaseNode<K, V>, Vec<IndexNode<K, V>>) {
        let mut current_index_node = self.get_head_index().unwrap();
        let mut res = vec![];
        assert!(current_index_node.get_key() <= (key));
        loop {
            // 1. find the fist node which key is less the search key
            let first_node = current_index_node
                .to_iter()
                .find(|n| {
                    n.get_key() <= key
                        && (n.get_right_node().is_none()
                            || n.get_right_node().unwrap().get_key() > key)
                })
                .unwrap();
            // record if is set/remove op
            if op == Operation::Add || op == Operation::Remove {
                res.push(first_node.clone());
            }
            let child_node = first_node.get_child();
            match child_node {
                //  continue loop
                IndexNodeChild::Index(n) => {
                    current_index_node = n;
                }
                // return if is base
                IndexNodeChild::Base(n) => {
                    return (n, res);
                }
            }
        }
    }

    fn handle_add(
        &mut self,
        key: K,
        value: V,
        mut base_node: BaseNode<K, V>,
        index_on_path: Vec<IndexNode<K, V>>,
    ) {
        let node_key = base_node.get_key();
        assert!(key >= node_key);
        // add new node
        if key > node_key {
            let right = base_node.get_right_node();
            let new_node = BaseNode::new(key, value, right);
            base_node.set_right_node(Some(new_node.clone()));

            // build index node
            self.fix_index_nodes(key, index_on_path, new_node);
            self.inc_len();
            //     override exit node value
        } else {
            base_node.set_value(value);
        }
    }

    fn fix_index_nodes(
        &mut self,
        key: K,
        mut index_nodes: Vec<IndexNode<K, V>>,
        new_node: BaseNode<K, V>,
    ) {
        let mut child = IndexNodeChild::Base(new_node.clone());
        let level = self.random_level();
        for i in 0..level + 1 {
            // get left
            // 1. from self.index
            // 2. from list.index
            let pop = index_nodes.pop();
            let new_index_node = match pop {
                Some(mut left) => {
                    let index_node = IndexNode::new(key, left.get_right_node(), child);
                    left.set_right(Some(index_node.clone()));
                    index_node
                }
                //     no left node
                None => {
                    // assert!(self.indexes.get(i).is_none());
                    let mut index_node = IndexNode::new(key, None, child);
                    if let Some(n) = self.indexes.get(&i) {
                        index_node.set_right(Some(n.clone()));
                    }
                    self.indexes.insert(i, index_node.clone());
                    index_node
                }
            };
            child = IndexNodeChild::Index(new_index_node);
        }
        // fix index node right  on the search path
    }

    fn find_less_node(key: &K, index_node: IndexNode<K, V>) -> IndexNode<K, V> {
        assert!(index_node.get_key() <= *key);
        index_node
            .to_iter()
            .find(|n| {
                n.get_key() <= *key
                    && (n.get_right_node().is_none()
                        || n.get_right_node().unwrap().get_key() > *key)
            })
            .unwrap()
    }
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn has_index_level(&self) -> bool {
        self.indexes.len() > 0
    }

    fn get_head_base(&self) -> BaseNode<K, V> {
        self.base_head.clone().unwrap()
    }

    fn get_head_index(&self) -> Option<IndexNode<K, V>> {
        self.indexes.get(&(self.indexes.len() - 1)).cloned()
    }

    fn random_level(&self) -> usize {
        let max_level = max_level(self.len);
        if max_level == 0 {
            return 0;
        }
        if self.always_max {
            max_level
        } else {
            let n = self.r.borrow_mut().next() as usize % max_level;
            n as usize
        }
    }
}

impl<K: Copy + PartialOrd, V: Copy> SkipList<K, V> {
    pub fn get_value(&self, key: K) -> Option<V> {
        self.map_value(key, |res| res.clone())
    }
}

// 1.  find nearest base node
// a. handle emtpy list
// b. find nearest index node in this level
// c. go to lower level ,if is base to 2, else to b
// 2. check base node one by one

#[cfg(test)]
// remember test head ,tail node and empty list
mod test {
    use super::SkipList;
    use std::borrow::Borrow;
    use std::cell::RefCell;

    #[test]
    fn test_new_list() {
        let list: SkipList<i32, i32> = SkipList::new();
        assert_eq!(list.len, 0);
        assert_eq!(list.indexes.len(), 0);
        assert_eq!(list.base_head.is_none(), true);
    }

    #[test]
    fn test_iter_list() {
        let mut list: SkipList<i32, i32> = SkipList::new();
        list.add(1, 1);
        list.add(2, 2);
        list.add(0, 0);
        list.add(3, 3);
        list.add(-1, -1);
        list.add(-2, -2);
        list.add(4, 4);
        list.to_graph();
        let mut iter = list.to_iter();
        assert_eq!((&iter.next().unwrap().borrow().get_key()), &-2);
        assert_eq!((&iter.next().unwrap().borrow().get_key()), &-1);
        assert_eq!((&iter.next().unwrap().borrow().get_key()), &0);
        assert_eq!((&iter.next().unwrap().borrow().get_key()), &1);
        assert_eq!((&iter.next().unwrap().borrow().get_key()), &2);
        assert_eq!((&iter.next().unwrap().borrow().get_key()), &3);
        assert_eq!((&iter.next().unwrap().borrow().get_key()), &4);
        assert_eq!(iter.next().is_none(), true);
        assert_eq!(list.len, 7)
    }

    #[test]
    fn test_add_list() {
        let mut list: SkipList<i32, i32> = SkipList::new();
        list.add(1, 1);
        assert_eq!(1, list.len);
        assert_eq!(0, list.indexes.len());
        assert_eq!(true, list.base_head.is_some());
        assert_eq!(false, list.has_index_level());

        list.add(2, 2);
        assert_eq!(2, list.len);

        list.add(-1, 2);
        assert_eq!(
            list.to_string(),
            r#"{"base":[[-1,2],[1,1],[2,2]],"index":{"0":[-1,2]}}"#
        );
    }

    #[test]
    fn test_overwrite_list() {
        let mut list: SkipList<i32, i32> = SkipList::new();
        list.add(1, 1);
        list.add(2, 2);
        list.add(0, 0);
        list.add(1, 3);
        list.add(0, 3);
        list.add(2, 3);
        list.print();
    }

    fn test_remove_list() {
        //     remove from list
        //     check field
    }

    #[test]
    fn test_get() {
        let mut list: SkipList<i32, i32> = SkipList::with_max_level();
        assert!(list.get_value(2).is_none());
        list.add(1, 0);
        assert_eq!(list.get_value(1).unwrap(), 0);
        assert!(list.get_value(2).is_none());
        list.add(2, 3);
        assert_eq!(list.get_value(2).unwrap(), 3);
        list.add(1, 1);
        assert_eq!(list.get_value(1).unwrap(), 1);
    }

    #[test]
    fn test_print() {
        let list = build_list();
        // let g = list.to_graph();
        let s = list.to_string();
        assert_eq!(
            s,
            r#"{"base":[[0,3],[1,3],[2,3],[5,3],[8,3]],"index":{"0":[0,2,5,8],"1":[0,2,8],"2":[0,8],"3":[0]}}"#
        );
        assert_eq!(list.len(), 5);
        assert_eq!(list.level(), 5 - 1);
    }

    fn build_list() -> SkipList<i32, i32> {
        let mut list: SkipList<i32, i32> = SkipList::with_max_level();
        list.add(1, 3);
        list.add(5, 3);
        list.add(2, 3);
        list.add(8, 3);
        list.add(0, 3);
        list
    }
}
