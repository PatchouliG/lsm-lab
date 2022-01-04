use crate::skip_list::node::{BaseNode, IndexNode};
use std::fmt::{Display, Write};

pub trait Context<K: Copy + PartialOrd, V> {
    fn get_key(&self) -> K;
    fn is_index_match(&self, node: &IndexNode<K, V>) -> bool {
        match node.get_right_node() {
            Some(right) => right.get_key() > self.get_key(),
            None => true,
        }
    }
    fn is_base_node_match(&self, node: &BaseNode<K, V>) -> bool {
        self.get_key() == node.get_key()
    }
    fn visit_index(&mut self, node: IndexNode<K, V>);
    fn visit_matched_base(&mut self, node: BaseNode<K, V>);
}

// just print
pub struct DebugContext<K: Copy + PartialOrd + Display> {
    key: K,
    output: String,
}

impl<K: Copy + PartialOrd + Display> DebugContext<K> {
    pub fn new(key: K) -> DebugContext<K> {
        DebugContext {
            key,
            output: String::new(),
        }
    }
}
impl<K: Copy + PartialOrd + Display, V: Copy + Display> Context<K, V> for DebugContext<K> {
    fn get_key(&self) -> K {
        self.key
    }

    fn visit_index(&mut self, node: IndexNode<K, V>) {
        self.output
            .write_fmt(format_args!("visitor index key:{}\n", node.get_key()))
            .unwrap();
    }

    fn visit_matched_base(&mut self, node: BaseNode<K, V>) {
        self.output
            .write_fmt(format_args!(
                "visitor base key:{},value:{}\n",
                node.get_key(),
                node.get_value()
            ))
            .unwrap();
    }
}

#[cfg(test)]
mod test {
    use crate::skip_list::list::{Graph, SkipList};

    fn test_helper(list_str: &str, key: i32, expect: &str) {
        let g: Graph<i32, i32> = serde_json::from_str(list_str).unwrap();
        let list = SkipList::from_graph(g);
        let context = list.debug_visitor(key);
        assert_eq!(context.output, expect);
    }

    #[test]
    fn test_search_in_mid_list() {
        let s = r#"{"base":[[0,3],[1,3],[2,3],[5,3],[8,3]],"index":{"0":[0,2,5,8],"1":[0,2,8],"2":[0,8],"3":[0]}}"#;
        test_helper(
            s,
            2,
            "visitor index key:0
visitor index key:0
visitor index key:2
visitor index key:2
visitor base key:2,value:3
",
        )
    }

    #[test]
    fn test_empty_list() {
        let s = r#"{"base":[],"index":{}}"#;
        test_helper(s, 1, "");
    }

    #[test]
    fn test_head_with_level_list() {
        let s = r#"{"base":[[0,3],[1,3],[2,3],[5,3],[8,3]],"index":{"0":[0,2,5,8],"1":[0,2,8],"2":[0,8],"3":[0]}}"#;
        test_helper(
            s,
            0,
            "visitor index key:0
visitor index key:0
visitor index key:0
visitor index key:0
visitor base key:0,value:3
",
        );
    }

    #[test]
    fn test_mid_node_without_level_list() {
        let s = r#"{"base":[[0,3],[1,3],[2,3],[5,3],[8,3]],"index":{}}"#;
        test_helper(
            s,
            5,
            "visitor base key:5,value:3
",
        );
    }

    #[test]
    fn test_not_found_in_mid() {
        let s = r#"{"base":[[0,3],[1,3],[2,3],[5,3],[8,3]],"index":{}}"#;
        test_helper(s, 4, "");
    }

    #[test]
    fn test_not_found_in_head() {
        let s = r#"{"base":[[0,3],[1,3],[2,3],[5,3],[8,3]],"index":{}}"#;
        test_helper(s, -1, "");
    }
}
