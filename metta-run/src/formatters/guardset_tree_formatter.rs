use crate::runners;

pub fn format(_metta_output: String) {
    //check if there are tree in metta output if there is tree format them
    for line in _metta_output.lines() {
        if line.starts_with("[(TreeNode") {
            format_tree(&line);
        } else {
            println!("{}", line);
        }
    }
}

fn format_tree(tree: &str) {
    fn simplify_tree_node(input: &str) -> String {
        let simplified_string = input.split(")").collect::<Vec<&str>>()[0].to_string() + ")";
        simplified_string
    }

    fn remove_brackets(input: &str) -> String {
        input.replace("[", "").replace("]", "").trim().to_string()
    }

    let main_metta_functions = format!(
        "
            (:getGuardSet (-> Tree (List Tree)))
            (= (getGuardSet (TreeNode $nodeVal $guardSet $children)) $guardSet)
            (= (head (Cons $x $xs)) $x)
            (= (tail (Cons $x $xs)) $xs)
        "
    );

    let get_guardset = |input: &str| -> String {
        let tree = remove_brackets(&input);
        let getter_code = format!("{}\n !(getGuardSet {}) ", main_metta_functions, tree);
        let result = runners::python::run(None, &getter_code);
        remove_brackets(&result)
    };

    let get_tree_head = |input: &str| -> String {
        let tree = remove_brackets(&input);
        let getter_code = format!("{}\n !(head {}) ", main_metta_functions, tree);
        let result = runners::python::run(None, &getter_code);
        remove_brackets(&result)
    };

    let get_tree_tail = |input: &str| -> String {
        let tree = remove_brackets(&input);
        let getter_code = format!("{}\n !(tail {}) ", main_metta_functions, tree);
        let result = runners::python::run(None, &getter_code);
        remove_brackets(&result)
    };

    fn print_tree(
        tree: &str,
        indent: u32,
        get_guardset: &dyn Fn(&str) -> String,
        get_tree_head: &dyn Fn(&str) -> String,
        get_tree_tail: &dyn Fn(&str) -> String,
    ) {
        if tree == "Nil" {
            return;
        }

        let current_node = simplify_tree_node(&tree);
        let mut children = get_guardset(&tree);

        if indent == 0 {
            println!("{}{}", " ".repeat(indent as usize), current_node);
        } else {
            println!("{}{}{}", " ".repeat(indent as usize), "├─", current_node);
        }

        if children != "Nil" {
            println!("{}{}", " ".repeat(indent as usize), "└──GuardSets's");
        }

        while children != "Nil" {
            let child = get_tree_head(&children);
            if child != "Nil" {
                print_tree(
                    &child,
                    indent + 4,
                    &get_guardset,
                    &get_tree_head,
                    &get_tree_tail,
                );
            }
            children = get_tree_tail(&children);
        }
    }

    print_tree(tree, 0, &get_guardset, &get_tree_head, &get_tree_tail);
}
