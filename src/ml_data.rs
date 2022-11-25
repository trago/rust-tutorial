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

pub fn read_ml_json(path: &Path) -> MLDataContainer{

    let json_str = fs::read_to_string(path).unwrap();

    let mut deserializer = serde_json::Deserializer::from_str(&json_str);
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);

    MLDataContainer::deserialize(deserializer).unwrap()
}