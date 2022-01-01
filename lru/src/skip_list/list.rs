#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
// todo add log debug, trace

mod skip_list {
    use std::cell::{RefCell, Ref};
    use std::rc::Rc;
    use std::borrow::{Borrow};
    use std::ops::{Deref, DerefMut};
    use std::alloc::handle_alloc_error;
    use std::fs::read_to_string;
    use std::fmt::Display;

    type BaseNodeInList<K, V> = Rc<RefCell<BaseNode<K, V>>>;
    type IndexNodeInList<K, V> = Rc<RefCell<IndexNode<K, V>>>;

    use crate::rand::simple_rand::*;
    use std::thread::sleep;
    use std::iter::Map;
    use std::collections::HashMap;

    fn get_key<K: Copy + PartialOrd, V>(node: &BaseNodeInList<K, V>) -> K {
        (node as &RefCell<BaseNode<K, V>>).borrow().key
    }


    // lowest node  level=0
    struct BaseNode<K: Copy + PartialOrd, V> {
        pub key: K,
        pub value: V,
        right: Option<BaseNodeInList<K, V>>,
    }

    // level >1
    struct IndexNode<K: Copy + PartialOrd, V> {
        key: K,
        right: Option<IndexNodeInList<K, V>>,
        child: IndexNodeChild<K, V>,
    }


    enum IndexNodeChild<K: Copy + PartialOrd, V> {
        Base(BaseNodeInList<K, V>),
        Index(IndexNodeInList<K, V>),
    }

    struct SkipList<K: Copy + PartialOrd, V> {
        // head_bass_node: Option<BaseNode<K, V>>,
        indexes: Vec<IndexNodeInList<K, V>>,
        base_head: Option<BaseNodeInList<K, V>>,
        len: usize,
        r: RefCell<SimpleRand>,
    }

    struct Context<K: Copy + PartialOrd, V> {
        op: Operation,
        key: K,
        value: Option<V>,
        // some if overwrite
        old_value: Option<V>,
        index_nodes: Vec<IndexNodeInList<K, V>>,
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
            Context { op: Operation::Add, key, value: Some(value), old_value: None, index_nodes: vec![] }
        }
        fn visit(&mut self, node: IndexNodeInList<K, V>) {
            self.index_nodes.push(node);
        }
        fn handle_operation(&mut self, node: BaseNodeInList<K, V>) {
            // match self.op {
            //     Operation::Add => {
            //         let mut n = (node.borrow() as &RefCell<BaseNode<K, V>>).borrow_mut();
            //         let node_key = n.key;
            //         assert!(self.key >= node_key);
            //         // add
            //         if self.key > node_key {
            //             let right = n.right.clone();
            //             let new_node = SkipList::new_base_node(self.key, self.value.take().unwrap(), right);
            //             n.right = Some(new_node.clone());
            //
            //             let level = self.list.random_level();
            //             // build index node
            //
            //             for _ in 0..level {
            //                 // get left
            //                 // 1. from self.index
            //                 // 2. from list.index
            //                 let left;
            //                 let pop = self.index_nodes.pop();
            //                 if pop.is_some() {
            //                     left = pop.unwrap();
            //                 } else {
            //                     left=unimplemented!()
            //                 }
            //
            //                 let index_node = SkipList::new_index_node(self.key,
            //                                                           (left.borrow() as &RefCell<IndexNode<K, V>>).borrow_mut().right.take(), IndexNodeChild::Base(new_node.clone()));
            //             }
            //             // fix index node right  on the search path
            //         } else {
            //             //     override exit node value
            //         }
            //     }
            //     Operation::Set => {}
            //     Operation::Remove => {}
            // }
        }
    }

    #[derive(Copy, Clone)]
    enum Operation {
        Add,
        Set,
        Remove,
    }

    struct SkipListIter<K: Copy + PartialOrd, V> {
        node: Option<BaseNodeInList<K, V>>,
    }


    impl<K: Copy + PartialOrd, V> Iterator for SkipListIter<K, V> {
        type Item = BaseNodeInList<K, V>;

        fn next(&mut self) -> Option<Self::Item> {
            let current = self.node.take();
            match current {
                Some(n) => {
                    self.node = (n.borrow() as &RefCell<BaseNode<K, V>>).borrow().right.clone();
                    Some(n)
                }
                None => {
                    None
                }
            }
        }
    }

    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    struct Graph<K: Display + Copy> {
        base: Vec<K>,
        index: Vec< Vec<K>>,
    }

    impl<K: Display + Copy> Graph<K> {
        pub fn new() -> Graph<K> {
            Graph { base: vec![], index: Vec::new() }
        }
        pub fn add_base_key(&mut self, k: K) {
            self.base.push(k);
        }
    }


    impl<K: Copy + PartialOrd + Display + Serialize, V: Display> SkipList<K, V> {
        pub fn to_graph(&self) -> Graph<K> {
            // let mut s = String::new();
            let mut g = Graph::new();
            let iterator = self.to_iter();
            for i in iterator {
                let key = get_key(&i);
                g.add_base_key(key);
            }
            // todo add index
            // self.indexes.into_iter().map(|head|{
            //     g.index.insert()
            // })
            g
        }
        pub fn to_string(&self) -> String {
            String::from(serde_json::to_string(&self.to_graph()).unwrap())
        }
    }

    impl<K: Copy + PartialOrd, V> SkipList<K, V> {
        pub fn new() -> SkipList<K, V> {
            SkipList { indexes: Vec::new(), base_head: None, len: 0, r: RefCell::new(SimpleRand::new()) }
        }
        pub fn to_iter(&self) -> SkipListIter<K, V> {
            SkipListIter { node: self.base_head.clone() }
        }
        // need handle overrite todo
        pub fn add(&mut self, key: K, value: V) {
            if self.is_empty() {
                let base_node = SkipList::new_base_node(key, value, None);
                self.base_head = Some(base_node);
                self.len += 1;
                return;
            }
            let head = self.get_head_base();
            // insert to head
            if (head.borrow() as &RefCell<BaseNode<K, V>>).borrow().key.gt(&key) {
                let base_node = SkipList::new_base_node(key, value, Some(head));
                self.base_head = Some(base_node.clone());
                self.len += 1;

                // get random level
                let level = self.random_level();
                // build index node
                // add index node to indexs
                let mut child = IndexNodeChild::Base(base_node);
                for n in 0..level {
                    let index_node = SkipList::new_index_node(key, None, child);
                    self.indexes.push(index_node.clone());
                    child = IndexNodeChild::Index(index_node);
                }

                return;
            }

            let context = Context::with_add_op(key, value);
            if self.is_one_level() {
                self.visit_base(key, head, context);
                // if context.old_value.is_none() { self.len += 1; }
                return;
            }
            self.visit_level(key, self.get_head_index(), context);
            // if context.old_value.is_none() { self.len += 1; }
            return;
        }
        pub fn get(&self, key: K) -> Option<&V> {
            // handle empty
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


        fn visit_base(&mut self, key: K, base_node: BaseNodeInList<K, V>, context: Context<K, V>) {
            let mut node: BaseNodeInList<K, V> = base_node.clone();
            let mut last = node.clone();

            loop {
                let n = (node.borrow() as &RefCell<BaseNode<K, V>>).borrow();
                let current_key = n.key;
                if current_key.lt(&key) {
                    last = node.clone();
                    if n.right.is_some() {
                        let t = n.right.as_ref().unwrap().clone();
                        drop(n);
                        node = t;
                        //     current node is last node
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.handle_operation(last, context);
        }


        fn visit_level(&mut self, key: K, index_node: IndexNodeInList<K, V>, mut context: Context<K, V>) {
            context.visit(index_node.clone());
            let node = <SkipList<K, V>>::find_less_node(&key, index_node);
            let c = &(node.borrow() as &RefCell<IndexNode<K, V>>).borrow().child;
            match c {
                IndexNodeChild::Base(t) => { self.visit_base(key, t.clone(), context) }
                IndexNodeChild::Index(t) => { self.visit_level(key, t.clone(), context) }
            }
        }

        fn handle_operation(&mut self, base_node: BaseNodeInList<K, V>, mut context: Context<K, V>) {
            match context.op {
                Operation::Add => {
                    let mut n = (base_node.borrow() as &RefCell<BaseNode<K, V>>).borrow_mut();
                    let node_key = n.key;
                    assert!(context.key >= node_key);
                    // add
                    if context.key > node_key {
                        let right = n.right.clone();
                        let new_node = SkipList::new_base_node(context.key, context.value.take().unwrap(), right);
                        n.right = Some(new_node.clone());

                        let level = self.random_level();
                        // build index node

                        for i in 0..level {
                            // get left
                            // 1. from self.index
                            // 2. from list.index
                            let left;
                            let pop = context.index_nodes.pop();
                            if pop.is_some() {
                                left = pop.unwrap();

                                let index_node = SkipList::new_index_node(context.key,
                                                                          (left.borrow() as &RefCell<IndexNode<K, V>>).borrow_mut().right.take(),
                                                                          IndexNodeChild::Base(new_node.clone()));
                                (left.borrow() as &RefCell<IndexNode<K, V>>).borrow_mut().right = Some(index_node);
                                //     no left
                            } else {
                                assert!(self.indexes.get(i).is_none());
                                let index_node = SkipList::new_index_node(context.key,
                                                                          None,
                                                                          IndexNodeChild::Base(new_node.clone()));
                                self.indexes.insert(i, index_node);
                            }
                        }
                        // fix index node right  on the search path

                        self.len += 1;
                    } else {
                        //     override exit node value
                    }
                }
                Operation::Set => {}
                Operation::Remove => {}
            }
        }


        fn find_less_node(key: &K, index_node: IndexNodeInList<K, V>) -> IndexNodeInList<K, V> {
            let mut node: IndexNodeInList<K, V> = index_node.clone();
            loop {
                let n = (node.borrow() as &RefCell<IndexNode<K, V>>).borrow();
                let current_key = n.key;
                if current_key.lt(&key) && n.right.is_some() {
                    let t = n.right.as_ref().unwrap().clone();
                    drop(n);
                    node = t;
                } else {
                    break;
                }
            }
            node
        }
        fn is_empty(&self) -> bool {
            self.len == 0
        }

        fn is_one_level(&self) -> bool {
            self.indexes.len() == 0
        }

        fn get_head_base(&self) -> BaseNodeInList<K, V> {
            self.base_head.as_ref().unwrap().clone()
        }

        fn get_head_index(&self) -> IndexNodeInList<K, V> {
            (*self.indexes.get(self.indexes.len() - 1).unwrap()).clone()
        }

        // fn get_or_create_index(&self,level:usize,index_node:IndexNodeInList<K,V>){
        //     if self.indexes.get(level).is_some(){
        //         return self.indexes.get(level).unwrap();
        //     }
        //     SkipList::new_index_node(key,None,)
        //     unimplemented!()
        // }

        fn new_index_node(key: K, right: Option<IndexNodeInList<K, V>>, child: IndexNodeChild<K, V>) -> IndexNodeInList<K, V> {
            Rc::new(RefCell::new(IndexNode { key, right, child }))
        }
        fn new_base_node(key: K, value: V, right: Option<BaseNodeInList<K, V>>) -> BaseNodeInList<K, V> {
            Rc::new(RefCell::new(BaseNode { key, value, right }))
        }

        fn add_base(&mut self, node: &BaseNode<K, V>) {
            unimplemented!()
        }

        fn random_level(&self) -> usize {
            let max_level = max_level(self.len);
            if max_level == 0 { return 0; }
            let n = self.r.borrow_mut().next() as usize % max_level;
            n as usize
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

        #[test]
        fn test_new_list() {
            let list: SkipList<i32, i32> = SkipList::new();
            assert_eq!(list.len, 0);
            assert_eq!(list.indexes.len(), 0);
            assert_eq!(list.base_head.is_none(), true);
            //     create new skip list
            //     check fielda lens ,indexs etc
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
            assert_eq!((&iter.next().unwrap().borrow().key), &-2);
            assert_eq!((&iter.next().unwrap().borrow().key), &-1);
            assert_eq!((&iter.next().unwrap().borrow().key), &0);
            assert_eq!((&iter.next().unwrap().borrow().key), &1);
            assert_eq!((&iter.next().unwrap().borrow().key), &2);
            assert_eq!((&iter.next().unwrap().borrow().key), &3);
            assert_eq!((&iter.next().unwrap().borrow().key), &4);
            assert_eq!(iter.next().is_none(), true);
            assert_eq!(list.len, 7)
        }

        #[test]
        #[ignore]
        fn test_add_list() {
            let mut list: SkipList<i32, i32> = SkipList::new();
            list.add(1, 1);
            assert_eq!(1, list.len);
            assert_eq!(0, list.indexes.len());
            assert_eq!(true, list.base_head.is_some());
            assert_eq!(true, list.is_one_level());

            list.add(2, 2);
            assert_eq!(2, list.len);
            assert_eq!(list.get(2).unwrap(), &2);
            assert_eq!(list.get(1).unwrap(), &1);

            list.add(-1, 2);

            //     add k,v to list
            //     check field
        }

        #[test]
        #[ignore]
        fn test_overwrite_list() {
            let mut list: SkipList<i32, i32> = SkipList::new();
            list.add(1, 1);
            list.add(2, 2);
            list.add(1, 2);
            assert_eq!(list.get(1).unwrap(), &2);
            assert_eq!(list.get(2).unwrap(), &2);
        }


        fn test_remove_list() {
            //     remove from list
            //     check field
        }

        // #[test]
        // fn test_max_level() {
        //     assert_eq!(max_level(0), 0);
        //     assert_eq!(max_level(1), 0);
        //     assert_eq!(max_level(2), 1);
        //     assert_eq!(max_level(64), 6);
        // }

        #[test]
        fn test_print() {
            let mut list: SkipList<i32, i32> = SkipList::new();
            list.add(1, 3);
            list.add(5, 3);
            list.add(2, 3);
            list.add(8, 3);
            list.add(0, 3);
            let g = list.to_graph();
            let s = serde_json::to_string(&g).unwrap();
            println!("{}", s);
        }
    }
}




