import argparse
import os

import matplotlib.pyplot as plt


def visualize(args):
    plt.figure(figsize=(10, 6))
    plt.title(args.title)
    plt.xlabel(args.x_label)
    plt.ylabel(args.y_label)
    plt.grid(True, alpha=0.6, linestyle="--")
    plt.yscale("log")

    for path in args.files:
        values = read_file(path)
        plot_histogram(values)

    plt.savefig(args.output, format="pdf")
    plt.show()


def plot_histogram(values, bins=30):
    plt.hist(values, bins=bins, alpha=0.5, edgecolor="black")


def read_file(file_path):
    with open(file_path, "r") as f:
        return [float(line) for line in f.readlines()]


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--title")
    parser.add_argument("--output")
    parser.add_argument("--x-label", default="Value")
    parser.add_argument("--y-label", default="Frequency")
    parser.add_argument("files", nargs="*")

    args = parser.parse_args()

    if not args.title:
        file_name_without_extension, _ = os.path.splitext(
            os.path.basename(args.files[0])
        )
        args.title = file_name_without_extension

    if not args.output:
        args.output = f"output/{args.title}.pdf"

    return args


if __name__ == "__main__":
    args = parse_args()
    visualize(args)

