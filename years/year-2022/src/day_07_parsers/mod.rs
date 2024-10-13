use std::collections::{HashMap, HashSet};
use std::path::Path;

use log::debug;

pub(crate) mod cmd_parser;

pub type FileSize = u32;

pub struct FileSystem {
    nodes: HashMap<String, Node>,
}

enum Node {
    File(FileSize),
    Directory(HashSet<String>),
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn iterate_dir_sizes(&self) -> Vec<(String, FileSize)> {
        let mut sizes = Vec::with_capacity(self.nodes.len());

        for path in self.nodes.keys() {
            // Register only directories
            if matches!(self.nodes.get(path), Some(Node::Directory(_))) {
                sizes.push((path.to_owned(), self.get_size(path).unwrap()));
            }
        }

        sizes
    }

    pub fn new_dir(&mut self, dir_path: &str) {
        let dir_node = Node::Directory(HashSet::new());
        self.register_node(dir_path, dir_node);
    }

    pub fn new_file(&mut self, file_path: &str, size: FileSize) {
        let file_node = Node::File(size);
        self.register_node(file_path, file_node);
    }

    fn register_node(&mut self, node_path: &str, node: Node) {
        let formal_path = Path::new(node_path);
        let mut path_ancestors = formal_path
            .ancestors()
            .map(|x| x.to_string_lossy().to_string());

        // Register node
        let self_path = path_ancestors.next().unwrap();
        self.nodes.insert(self_path.clone(), node);

        let mut to_register = self_path;

        // Build ancestor directories
        for ancestor_path in path_ancestors {
            if let Node::Directory(ancestor_children) = self
                .nodes
                .entry(ancestor_path.clone())
                .or_insert_with(|| Node::Directory(HashSet::new()))
            {
                ancestor_children.insert(to_register);
                to_register = ancestor_path;
            }
        }
    }

    pub fn get_size(&self, path: &str) -> Option<FileSize> {
        match self.nodes.get(path) {
            None => None,
            Some(Node::File(size)) => Some(*size),
            Some(Node::Directory(paths)) => {
                let mut total_size = 0;

                debug!(
                    "Paths of {}: {:?}",
                    path,
                    paths.iter().collect::<Vec<&String>>()
                );

                for sub_dir in paths {
                    total_size += self.get_size(sub_dir).unwrap_or(0);
                }

                Some(total_size)
            }
        }
    }
}
