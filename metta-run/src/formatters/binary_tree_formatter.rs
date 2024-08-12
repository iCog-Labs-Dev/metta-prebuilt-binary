use std::env;

use crate::runners;

pub fn format(metta_output: String) {
    //check if there are tree in metta output if there is tree format them
    for line in metta_output.lines() {
        if line.starts_with("[(TreeNode") {
            format_tree(&line);
        } else {
            println!("{}", line);
        }
    }
}

fn format_tree(tree: &str) {
    let metta_run = format!(
        "{}/metta-bin/tools/utils/metta_runner.py",
        env::var("HOME").unwrap()
    );

    fn simplify_tree_node(input: &str) -> String {
        let simplified_string = input.split(")").collect::<Vec<&str>>()[0].to_string() + ")";
        simplified_string
    }

    fn remove_brackets(input: &str) -> String {
        input.replace("[", "").replace("]", "").trim().to_string()
    }

    fn get_node_type(node: &str) -> &str {
        if node.contains("ROOT") {
            return "ROOT";
        } else if node.contains("OR") {
            return "OR";
        } else {
            return "AND";
        }
    }

    let get_child = format!(
        "
            ;; -------- a function to get the right or left child

            (: getChild (-> Tree Location Tree))
            (= (getChild (TreeNode $nodeValue $guardSet Nil) $opt) Nil)
            (= (getChild (TreeNode $nodeValue $guardSet (Cons $l $xs)) L) $l)
            (= (getChild (TreeNode $nodeValue $guardSet (Cons $l Nil)) R) NilNode)
            (= (getChild (TreeNode $nodeValue $guardSet (Cons $l (Cons $r $xs))) R) $r)
        "
    );

    let get_left_child = |input: &str| -> String {
        let tree = remove_brackets(&input);
        let getter_code = format!("{}\n !(getChild {} L) ", get_child, tree);
        let result = runners::python::run(&metta_run, &getter_code);
        remove_brackets(&result)
    };

    let get_right_child = |input: &str| -> String {
        let tree = remove_brackets(&input);
        let getter_code = format!("{}\n !(getChild {} R) ", get_child, tree);
        let result = runners::python::run(&metta_run, &getter_code);
        remove_brackets(&result)
    };

    fn print_tree(
        tree: &str,
        indent: u32,
        get_left_child: &dyn Fn(&str) -> String,
        get_right_child: &dyn Fn(&str) -> String,
    ) {
        if tree == "Nil" {
            return;
        }

        let current_node = simplify_tree_node(&tree);

        let left_child = get_left_child(tree);
        let right_child = get_right_child(tree);

        if indent == 0 {
            let node_type = get_node_type(&current_node);
            println!("TreeNode {}", node_type);
        } else if tree == "NilNode" {
            println!("{}  {}", "─".repeat(indent as usize), "(NilNode)");
            return;
        } else {
            println!("{}  {}", "─".repeat(indent as usize), current_node);
        }

        if left_child != "Nil" || right_child != "Nil" {
            print!("{}  ├──Left  ", " ".repeat(indent as usize + 2));
            if left_child != "Nil" {
                print_tree(&left_child, indent + 4, get_left_child, get_right_child);
            }

            print!("{}  └──Right ", " ".repeat(indent as usize + 2));
            if right_child != "Nil" {
                print_tree(&right_child, indent + 4, get_left_child, get_right_child);
            }
        }
    }

    print_tree(tree, 0, &get_left_child, &get_right_child);
}
