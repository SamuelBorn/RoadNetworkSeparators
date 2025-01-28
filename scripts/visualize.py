import argparse
import os

from matplotlib.colors import LogNorm
import matplotlib.pyplot as plt
import numpy as np


def get_values(filename):
    return zip(*[map(int, line.split()) for line in open(filename)])

def scatter(filename, color, marker, alpha=1):
    x_values, y_values = get_values(filename)
    plt.scatter(
        x_values,
        y_values,
        color=color,
        marker=marker,
        label=os.path.splitext(os.path.basename(filename))[0],
        alpha=alpha,
    )

def heatmap(filename, bins=50):
    x_values, y_values = get_values(filename)
    x_values = np.cbrt(x_values)
    
    # Create a 2D histogram
    heatmap_data, x_edges, y_edges = np.histogram2d(x_values, y_values, bins=bins)
    
    # Plot the heatmap
    plt.imshow(
        heatmap_data.T,  # Transpose to align x and y correctly
        origin="lower",  # Set the origin to lower-left
        extent=(x_edges[0], x_edges[-1], y_edges[0], y_edges[-1]),  # Define the bounds
        norm=LogNorm()
    )


def find_max_x(files):
    max_x = 0
    for filename in files:
        x_values, _ = get_values(filename)
        max_x = max(max_x, max(x_values))
    return max_x


def plot_function(fn, max_x, label, color="black", alpha=0.2, linestyle="-"):
    x = np.logspace(0, np.log10(max_x), 500)
    y = fn(x)
    plt.plot(x, y, label=label, color=color, alpha=alpha, linestyle=linestyle)


def visualize(args):
    plt.figure(figsize=(8, 6))

    if args.loglog:
        plt.xscale("log")
        plt.yscale("log")

    if args.sqrt:
        plot_function(np.sqrt, find_max_x(args.files), "$\sqrt{x}$")
    if args.cbrt:
        plot_function(np.cbrt, find_max_x(args.files), "$\sqrt[3]{x}$", linestyle="-.")

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
        # heatmap(filename)

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

    return args


def main():
    args = parse_args()
    visualize(args)


if __name__ == "__main__":
    main()
