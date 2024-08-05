import argparse

from hyperon import MeTTa


class TreeFormatter:
    def __init__(self):
        self.metta = MeTTa()
        self.metta.run(
            """
            ;; -----------------------------------
            ;; -------- a function to get the right or left child
            ;; -----------------------------------

            (: getChild (-> Tree Location Tree))
            (= (getChild (TreeNode $nodeValue $guardSet Nil) $opt) Nil)
            (= (getChild (TreeNode $nodeValue $guardSet (Cons $l $xs)) L) $l)
            (= (getChild (TreeNode $nodeValue $guardSet (Cons $l (Cons $r $xs))) R) $r)
            """
        )

    def remove_brackets(self, input):
        return str(input).replace("[", "").replace("]", "")

    def get_right_child(self, input):
        input = self.remove_brackets(str(input))
        return self.remove_brackets(
            self.metta.run(
                f"""
        !(getChild {input} R)
        """
            )
        )

    def get_left_child(self, input):
        input = self.remove_brackets(str(input))
        return self.remove_brackets(
            self.metta.run(
                f"""
        !(getChild {input} L)
        """
            )
        )

    def simplify_tree_node(self, input):
        simplified_string = str(input).split(")")[0] + ")"
        return simplified_string

    def get_node_type(self, node):
        if "ROOT" in node:
            return "ROOT"
        elif "OR" in node:
            return "OR"
        else:
            return "AND"

    def print_tree(self, input, indent=0):
        if input == "Nil":
            return

        current_node = self.simplify_tree_node(input)

        left_child = self.get_left_child(input)
        right_child = self.get_right_child(input)

        if indent == 0:
            node_type = self.get_node_type(current_node)
            print(f"TreeNode {node_type}")
        else:
            print("─" * indent + " " + str(current_node))

        if left_child != "Nil" or right_child != "Nil":
            print(" " * (indent + 2) + "├──Left  ", end="")
            if left_child != "Nil":
                self.print_tree(left_child, indent + 4)

            print(" " * (indent + 2) + "└──Right ", end="")
            if right_child != "Nil":
                self.print_tree(right_child, indent + 4)


class CLIFormatter:
    def __init__(self):
        self.formatter = TreeFormatter()

    def main(self):
        parser = argparse.ArgumentParser(
            description="Format and print a tree structure"
        )
        parser.add_argument("input", help="The input tree to be formatted")
        args = parser.parse_args()
        self.formatter.print_tree(self.formatter.remove_brackets(args.input))


if __name__ == "__main__":
    app = CLIFormatter()
    app.main()
