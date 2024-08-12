import argparse

from hyperon import MeTTa


class Metta_Runner_CLI:
    def __init__(self):
        self.metta = MeTTa()

    def main(self):
        parser = argparse.ArgumentParser(description="run metta code")
        parser.add_argument("input", help=" The input code to be run")
        args = parser.parse_args()
        print(self.metta.run(args.input))


if __name__ == "__main__":
    app = Metta_Runner_CLI()
    app.main()
