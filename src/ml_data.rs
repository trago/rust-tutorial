use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MLData {
    pub nodes: Vec<Node>,
    pub tree: Vec<TreeNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub i: String,
    #[serde(default = "default_fnz_id")]
    fnz_id: String,
    pub a: HashMap<String, String>,
}

fn default_fnz_id() -> String {
    String::from("-1")
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TreeNode {
    pub i: String,
    pub c: Option<Vec<TreeNode>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MLDataContainer {
    pub element_statistics: MLData,
}

pub fn read_ml_json(path: &Path) -> MLDataContainer {
    let json_str = fs::read_to_string(path).unwrap();

    let mut deserializer = serde_json::Deserializer::from_str(&json_str);
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);

    MLDataContainer::deserialize(deserializer).unwrap()
}

// filters a vector of nodes based on the `XX` key value of `node.a` attribute
pub fn search_xx(nodes: &Vec<Node>) -> Node {
    let mut nodexx: Node = nodes[1].clone();

    for node in nodes.iter() {
        if node.a.contains_key("XX") {
            nodexx = node.clone()
        }
    }
    nodexx
}

// computes the correlation between a given node and a vector of nodes
fn correlate(base_node: &Node, nodes: &Vec<Node>) -> Vec<f64> {
    let mut corr = Vec::new();
    let dont_compare = vec![
        String::from("WH"),
        String::from("LT"),
        String::from("TP"),
        String::from("HT"),
    ];

    let size_base_node = base_node
        .a
        .iter()
        .filter(|(gk, _)| !dont_compare.contains(&*gk))
        .count() as f64;

    for node in nodes.iter() {
        let mut sum = 0.0;
        for (k, v) in base_node.a.iter() {
            sum += node
                .a
                .iter()
                .filter(|(gk, gv)| *gk == k && *gv == v && !dont_compare.contains(&*gk))
                .count() as f64;
        }
        corr.push(sum / size_base_node);
    }
    corr
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[cfg(test)]
mod test {
    use crate::ml_data::{correlate, read_ml_json, search_xx, Person};
    use std::path::Path;

    #[test]
    fn json_test() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let p: Person = serde_json::from_str(data).unwrap();

        // Do things just like with any other Rust data structure.
        println!("Please call {} at the number {}", p.name, p.phones[0]);
    }

    #[test]
    fn load_json_test() {
        let path = Path::new("resources/1645511997141_M8INRNFV6O_curr.json");
        let data = read_ml_json(&path);

        println!("number of nodes {}", data.element_statistics.nodes.len());
    }

    #[test]
    fn search_xx_test() {
        let path = Path::new("resources/1645511997141_M8INRNFV6O_curr.json");
        let data = read_ml_json(&path);
        let node = search_xx(&data.element_statistics.nodes);
        println!("node {:?}", node.a);
    }

    #[test]
    fn correlate_test() {
        let path = Path::new("resources/1645511997141_M8INRNFV6O_curr.json");
        let data = read_ml_json(&path);
        let node = search_xx(&data.element_statistics.nodes);
        let corr = correlate(&node, &data.element_statistics.nodes);
        println!("correlation with Node XX {:?}", corr);
    }
}
