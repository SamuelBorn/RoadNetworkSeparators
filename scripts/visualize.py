import argparse
import os

import matplotlib.pyplot as plt
import numpy as np


def scatter(filename, color, marker):
    x_values, y_values = zip(*[map(int, line.split()) for line in open(filename)])
    plt.scatter(
        x_values,
        y_values,
        color=color,
        marker=marker,
        label=os.path.splitext(os.path.basename(filename))[0],
    )


def find_max_x(files):
    max_x = 0
    for filename in files:
        x_values, _ = zip(*[map(int, line.split()) for line in open(filename)])
        max_x = max(max_x, max(x_values))
    return max_x


def plot_function(fn, max_x, label, color="black", alpha=0.2):
    x_lin = np.linspace(0, max_x, 500)
    x_log = np.logspace(0, np.log10(max_x), 500)
    x = np.sort(np.concatenate((x_lin, x_log)))
    y = fn(x)
    plt.plot(x, y, label=label, color=color, alpha=alpha)


def visualize(args):
    plt.figure(figsize=(8, 6))

    if args.loglog:
        plt.xscale("log")
        plt.yscale("log")

    if args.cbrt:
        plot_function(np.cbrt, find_max_x(args.files), "$\sqrt[3]{x}$")
    if args.sqrt:
        plot_function(np.sqrt, find_max_x(args.files), "$\sqrt{x}$")

    markers = ["^", "x", "v", "+", "*", "o", "s"]
    colors = [
        "#56B4E9",
        "#E69F00",
        "#009E73",
        "#F0E442",
        "#0072B2",
        "#D55E00",
        "#CC79A7",
        "#000000",
    ]

    for i, filename in enumerate(args.files):
        scatter(filename, colors[i], markers[i])

    plt.title(args.title)
    plt.xlabel(args.x_label)
    plt.ylabel(args.y_label)
    plt.legend()
    plt.grid(True, linestyle="--", alpha=0.6)
    plt.savefig(args.output, format="pdf")
    plt.show()


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--title")
    parser.add_argument("--output")
    parser.add_argument("--x-label", default="Number of nodes")
    parser.add_argument("--y-label", default="Size of separator")
    parser.add_argument("--loglog", action="store_true")
    parser.add_argument("--cbrt", action="store_true")
    parser.add_argument("--sqrt", action="store_true")
    parser.add_argument("files", nargs="*")

    args = parser.parse_args()

    if not args.title:
        file_name_without_extension, _ = os.path.splitext(
            os.path.basename(args.files[0])
        )
        args.title = file_name_without_extension

    if not args.output:
        args.output = f"output/{args.title}.pdf"

    print(args)

    return args


def main():
    args = parse_args()
    visualize(args)


if __name__ == "__main__":
    main()
