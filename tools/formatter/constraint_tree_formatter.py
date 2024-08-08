import argparse

from hyperon import MeTTa


class TreeFormatter:
    def __init__(self):
        self.metta = MeTTa()
        self.metta.run(
            """
            ;; -----------------------------------
            ;; -----------------------------------
            ;; -------- a function to get node children
            ;; -----------------------------------
            ;; -----------------------------------

            (:getChildren (-> Tree (List Tree)))
            (= (getChildren (TreeNode $nodeVal $guardSet $children)) $children)

            (= (getTreeHead (Cons $x $xs)) $x)
            (= (getTreeTail (Cons $x $xs)) $xs)
            """
        )

    def remove_brackets(self, input):
        return str(input).replace("[", "").replace("]", "")

    def get_children(self, input):
        input = self.remove_brackets(str(input))
        return self.remove_brackets(
            self.metta.run(
                f"""
        !(getChildren {input})
        """
            )
        )

    def get_tree_head(self, input):
        input = self.remove_brackets(str(input))
        return self.remove_brackets(
            self.metta.run(
                f"""
        !(getTreeHead {input})
        """
            )
        )

    def get_tree_tail(self, input):
        input = self.remove_brackets(str(input))
        return self.remove_brackets(
            self.metta.run(
                f"""
        !(getTreeTail {input})
        """
            )
        )

    def simplify_tree_node(self, input):
        simplified_string = str(input).split(")")[0] + ")"
        return simplified_string


    def print_tree(self, input, indent=0):
        if input == "Nil":
            return

        current_node = self.simplify_tree_node(input)
        children = self.get_children(input)

        if indent == 0:
            print(" " * indent + str(current_node), end="\n")
        else:
            print(" " * indent + "├─" + str(current_node), end="\n")

        if children != "Nil":
            print(" " * (indent + 2) + "└──Children's", end="\n")

        while children != "Nil":
            child = self.get_tree_head(children)
            if child != "Nil":
                self.print_tree(child, indent + 4)
            children = self.get_tree_tail(children)


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
