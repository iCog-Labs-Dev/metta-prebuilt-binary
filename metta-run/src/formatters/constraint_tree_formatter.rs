use crate::runners;

pub fn format(metta_output: String) {
    //check if there are tree in metta output if there is tree format them
    for line in metta_output.lines() {
        if line.starts_with("[(TreeNode") {
            let is_binary_tree = is_binary_tree(&line);
            if !is_binary_tree {
                format_tree(&line);
                continue;
            }
            println!("{}", line);
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

    let main_children_functions = format!(
        "
            ;; -------- a function to get the right or left child

            (:getChildren (-> Tree (List Tree)))
            (= (getChildren (TreeNode $nodeVal $guardSet $children)) $children)
            (= (getTreeHead (Cons $x $xs)) $x)
            (= (getTreeTail (Cons $x $xs)) $xs)
        "
    );

    let get_children = |input: &str| -> String {
        let tree = remove_brackets(&input);
        let getter_code = format!("{}\n !(getChildren {}) ", main_children_functions, tree);
        let result = runners::python::run(None, &getter_code);
        remove_brackets(&result)
    };

    let main_guardset_functions = format!(
        "
            (:getGuardSet (-> Tree (List Tree)))
            (= (getGuardSet (TreeNode $nodeVal $guardSet $children)) $guardSet)
            (= (head (Cons $x $xs)) $x)
            (= (tail (Cons $x $xs)) $xs)
        "
    );

    let get_guardset = |input: &str| -> String {
        let tree = remove_brackets(&input);
        let getter_code = format!("{}\n !(getGuardSet {}) ", main_guardset_functions, tree);
        let result = runners::python::run(None, &getter_code);
        remove_brackets(&result)
    };

    let get_tree_head = |input: &str| -> String {
        let tree = remove_brackets(&input);
        let getter_code = format!("{}\n !(getTreeHead {}) ", main_children_functions, tree);
        let result = runners::python::run(None, &getter_code);
        remove_brackets(&result)
    };

    let get_tree_tail = |input: &str| -> String {
        let tree = remove_brackets(&input);
        let getter_code = format!("{}\n !(getTreeTail {}) ", main_children_functions, tree);
        let result = runners::python::run(None, &getter_code);
        remove_brackets(&result)
    };

    fn print_tree(
        tree: &str,
        indent: u32,
        get_children: &dyn Fn(&str) -> String,
        get_guardset: &dyn Fn(&str) -> String,
        get_tree_head: &dyn Fn(&str) -> String,
        get_tree_tail: &dyn Fn(&str) -> String,
    ) {
        if tree == "Nil" {
            return;
        }

        let current_node = simplify_tree_node(&tree);
        let mut children = get_children(&tree);
        let mut guardset = get_guardset(&tree);

        if indent == 0 {
            println!("{}{}", " ".repeat(indent as usize), current_node);
        } else {
            println!("{}{}{}", " ".repeat(indent as usize), "├─", current_node);
        }

        if guardset != "Nil" {
            println!("{}{}", " ".repeat(indent as usize), "└──GuardSet's");
            while guardset != "Nil" {
                let guard = get_tree_head(&guardset);
                if guard != "Nil" {
                    println!("{}{}{}", " ".repeat(indent as usize), "   ├─", guard);
                }
                guardset = get_tree_tail(&guardset);
            }
        }

        if children != "Nil" {
            println!("{}{}", " ".repeat(indent as usize), "└──Children's");

            while children != "Nil" {
                let child = get_tree_head(&children);
                if child != "Nil" {
                    print_tree(
                        &child,
                        indent + 4,
                        &get_children,
                        &get_guardset,
                        &get_tree_head,
                        &get_tree_tail,
                    );
                }
                children = get_tree_tail(&children);
            }
        }
    }

    print_tree(
        tree,
        0,
        &get_children,
        &get_guardset,
        &get_tree_head,
        &get_tree_tail,
    );
}

fn is_binary_tree(tree: &str) -> bool {
    let main_metta_functions = format!(
        "
            ; check the length of the list
            (: length (-> (List $t) Number))
            (= (length Nil) 0)
            (= (length (Cons $x $xs))
                (+ 1 (length $xs)))

            ; function to check if the tree is binary
            (: check_binary_tree (-> Tree Bool))
            (= (check_binary_tree (TreeNode $value $guard_set $children))
                (== 2 (length $children)))
        "
    );

    let tree = &tree.to_string().replace("[", "").replace("]", "");
    let getter_code = format!("{}\n !(check_binary_tree {}) ", main_metta_functions, tree);
    let result = runners::python::run(None, &getter_code);

    if result == "[[False]]" {
        return false;
    } else {
        return true;
    }
}
