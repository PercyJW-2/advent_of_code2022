use std::cell::RefCell;
use std::rc::Rc;
use crate::day_seven::TreeElemType::{File, Folder};

pub fn day_seven(riddle_input: &String) {
    let root_node = Rc::new(RefCell::new(TreeNode {
        value: TreeElem {
            elem_type: Folder,
            size: 0,
            name: "/".to_string(),
        },
        children: vec![],
        parent: None,
    }));
    let mut current_node = root_node.clone();
    for cmd_output in riddle_input.split("\n") {
        if cmd_output.starts_with("$ ") {
            if !cmd_output.starts_with("$ ls") {
                let folder_name = cmd_output.replace("$ cd ", "");
                if folder_name == ".." {
                    let found_parent: Option<Rc<RefCell<TreeNode>>>;
                    {
                        let parent = current_node.borrow().parent.as_ref().expect("No Parent").clone();
                        found_parent = Some(parent);
                    }
                    current_node = found_parent.expect("No Parent");
                } else if folder_name != "/" {
                    let mut found_child: Option<Rc<RefCell<TreeNode>>> = None;
                    for tree_elem in &current_node.borrow().children {
                        if tree_elem.borrow().value.name == folder_name {
                            found_child = Some(tree_elem.clone());
                            break;
                        }
                    }
                    current_node = found_child.expect("Folder not found");
                }
            }
        } else {
            if cmd_output.starts_with("dir ") {
                current_node.borrow_mut().children.push(Rc::new(RefCell::new(TreeNode {
                    value: TreeElem {
                        elem_type: Folder,
                        size: 0,
                        name: cmd_output.split_once(" ").expect("Invalid Format").1.to_string(),
                    },
                    children: vec![],
                    parent: Some(current_node.clone()),
                })));
            } else if cmd_output.len() > 0 {
                let (size, name) = cmd_output.split_once(" ").expect("Invalid Format");
                current_node.borrow_mut().children.push(Rc::new(RefCell::new(TreeNode {
                    value: TreeElem {
                        elem_type: File,
                        size: size.parse().expect("Invalidly Formatted Number"),
                        name: name.to_string(),
                    },
                    children: vec![],
                    parent: Some(current_node.clone()),
                })));
            }
        }
    }
    update_folder_sizes(root_node.clone());
    let total_size = sum_folders_below_equal_limit(root_node.clone(), 100000);
    println!("Size of folders that are below 100.000 in size: {total_size}");
    let unused_space: i32 = 70_000_000 - root_node.borrow().value.size;
    let needed_space: i32 = 30_000_000 - unused_space;
    println!("Size that needs to be freed: {needed_space}");
    let deleted_folder_size: i32 = get_closest_larger_folder_size(root_node.clone(), needed_space);
    println!("Size of the to be deleted folder: {deleted_folder_size}");
}

fn update_folder_sizes(tree_root: Rc<RefCell<TreeNode>>) -> i32 {
    let mut size: i32 = 0;
    for child in &tree_root.borrow().children {
        let child_type: TreeElemType = child.borrow().value.elem_type;
        size += match child_type {
            File => child.borrow().value.size,
            Folder => update_folder_sizes(child.clone())
        }
    }
    let mut current_folder = tree_root.borrow_mut();
    current_folder.value.size = size;
    if size <= 100000 {
    }
    size
}

fn sum_folders_below_equal_limit(tree_root: Rc<RefCell<TreeNode>>, size_lim: i32) -> i32 {
    let mut amount = 0;
    if tree_root.borrow().value.size <= size_lim {
        amount += tree_root.borrow().value.size;
    }
    for child in &tree_root.borrow().children {
        let child_type: TreeElemType = child.borrow().value.elem_type;
        if child_type == Folder {
            amount += sum_folders_below_equal_limit(child.clone(), size_lim);
        }
    }
    amount
}

fn get_closest_larger_folder_size(tree_root: Rc<RefCell<TreeNode>>, limit: i32) -> i32 {
    return if tree_root.borrow().value.size >= limit {
        let mut currently_best = 70_000_000;
        for child in &tree_root.borrow().children {
            if child.borrow().value.elem_type == Folder {
                let best = get_closest_larger_folder_size(child.clone(), limit);
                if best != -1 && best < currently_best {
                    currently_best = best;
                }
            }
        }
        if currently_best < tree_root.borrow().value.size {
            currently_best
        } else {
            tree_root.borrow().value.size
        }
    } else {
        -1
    }
}

struct TreeElem {
    elem_type: TreeElemType,
    size: i32,
    name: String,
}

#[derive(Copy, Clone, PartialEq)]
enum TreeElemType {
    File,
    Folder
}

struct TreeNode {
    pub value: TreeElem,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>
}
