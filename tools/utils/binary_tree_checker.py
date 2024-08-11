import argparse
from hyperon import MeTTa

class BinaryTreeChecker:
    def __init__(self):
        self.metta = MeTTa()
        self.metta.run(
            """
            ; check the length of the list
            (: length (-> (List $t) Number))
            (= (length Nil) 0)
            (= (length (Cons $x $xs))
                (+ 1 (length $xs)))

            ; function to check if the tree is binary
            (: check_binary_tree (-> Tree Bool))
            (= (check_binary_tree (TreeNode $value $guard_set $children))
                (== 2 (length $children)))
            """
        )

    def is_binary_tree(self, tree_node):
        result = self.metta.run(
            f"""
            !(check_binary_tree {tree_node})
            """
        )
        return result

class CLIChecker:
    def __init__(self):
        self.checker = BinaryTreeChecker()

    def main(self):
        parser = argparse.ArgumentParser(
            description="Check if a tree structure is binary"
        )
        parser.add_argument("tree_node", help="The input tree to be checked")
        args = parser.parse_args()
        is_binary = self.checker.is_binary_tree(args.tree_node)
        print(is_binary)


if __name__ == "__main__":
    app = CLIChecker()
    app.main()