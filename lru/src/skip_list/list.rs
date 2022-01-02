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

#[derive(Copy, Clone)]
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
            graph.add_base_key(key, node_iter.get_ref().value);
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

        let context = Context::with_add_op(key, value);
        // head index is more than key, visit base
        if !self.has_index_level() || self.get_head_index().unwrap().get_key() > key {
            self.visit_base(key, head, context);
            return;
        }
        self.visit_level(key, self.get_head_index().unwrap(), context);
        return;
    }
    pub fn get(&self, key: K) -> Option<&V> {
        // handle empty
        // if self.len()==0{
        //     None
        // }
        // if !self.has_index_level(){
        //     let head = self.get_head_base();
        //     self.visit_base(head,)
        //
        // }
        // handle one level
        // visit by visit handle
        unimplemented!()
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

    fn visit_base(&mut self, key: K, base_node: BaseNode<K, V>, context: Context<K, V>) {
        assert!(self.len() >= 1);
        assert!(key >= self.get_head_base().get_key());
        let node = self
            .to_iter()
            .find(|n| {
                n.get_key() <= key
                    && (n.get_right_node().is_none() || n.get_right_node().unwrap().get_key() > key)
            })
            .unwrap();
        self.handle_operation(node, context);
    }

    fn visit_level(&mut self, key: K, index_node: IndexNode<K, V>, mut context: Context<K, V>) {
        assert!(index_node.get_key().le(&key));
        context.visit(index_node.clone());
        let node = <SkipList<K, V>>::find_less_node(&key, index_node);
        let c = node.get_child();
        match c {
            IndexNodeChild::Base(t) => self.visit_base(key, t.clone(), context),
            IndexNodeChild::Index(t) => self.visit_level(key, t.clone(), context),
        }
    }

    fn handle_operation(&mut self, mut base_node: BaseNode<K, V>, context: Context<K, V>) {
        match context.op {
            Operation::Add => self.handle_add(&mut base_node, context),
            Operation::Get => {}
            Operation::Remove => {}
        }
    }

    fn handle_add(&mut self, base_node: &mut BaseNode<K, V>, context: Context<K, V>) {
        let node_key = base_node.get_key();
        assert!(context.key >= node_key);
        // add new node
        if context.key > node_key {
            let right = base_node.get_right_node();
            let new_node = BaseNode::new(context.key, context.value.unwrap(), right);
            base_node.set_right_node(Some(new_node.clone()));
            // n.right = Some(new_node.clone());

            // build index node

            self.fix_index_nodes(context.key, context.index_nodes_on_path, new_node);
            self.inc_len();
            //     override exit node value
        } else {
            base_node.set_value(context.value.unwrap());
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
    #[ignore]
    fn test_get() {
        let mut list: SkipList<i32, i32> = SkipList::with_max_level();
        assert!(list.get(2).is_none());
        list.add(1, 0);
        assert_eq!(list.get(1).unwrap(), &0);
        assert!(list.get(2).is_none());
        list.add(2, 3);
        assert_eq!(list.get(2).unwrap(), &3);
        list.add(1, 1);
        assert_eq!(list.get(1).unwrap(), &1);
    }

    #[test]
    fn test_print() {
        let list = build_list();
        // let g = list.to_graph();
        let s = list.to_string();
        assert_eq!(
            s,
            r#"{"base":[[0,3],[1,3],[2,3],[5,3],[8,3]],"index":{"0":[0,2,8,5],"1":[0,2,8],"2":[0,8],"3":[0]}}"#
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
