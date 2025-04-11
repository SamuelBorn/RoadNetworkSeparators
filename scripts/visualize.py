import argparse
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy.stats import binned_statistic

markers = ["x", "^", "o", "+"]
colors = ["#009682", "#df9b1b", "#4664aa", "#a3107c"]


def get_max_x(args):
    max_x = 0
    for file in args.files:
        x_values, _ = get_values(file, args)
        max_x = max(max_x, max(x_values))
    return max_x


def get_values(filename, args):
    data = [tuple(map(float, line.split())) for line in open(filename)]
    if not args.keep_outliers:
        data = [x for x in data if x[0] < 10_000_000]
    x, y = zip(*data)
    return x, y


def bin_data(x, y, args):
    if args.loglog:
        bin_edges = np.logspace(0, np.log10(max(x)), num=args.bins)
        means, bin_edges, _ = binned_statistic(x, y, statistic="mean", bins=bin_edges)
        bin_centers = np.sqrt(bin_edges[:-1] * bin_edges[1:])
    else:
        bin_edges = np.linspace(0, max(x), num=args.bins)
        means, bin_edges, _ = binned_statistic(x, y, statistic="mean", bins=bin_edges)
        bin_centers = (bin_edges[:-1] + bin_edges[1:]) / 2

    valid = ~np.isnan(means)
    bin_centers = bin_centers[valid]
    means = means[valid]
    return bin_centers, means


def scatter(x_values, y_values, label, color, marker, alpha=1):
    plt.scatter(
        x_values,
        y_values,
        color=color,
        marker=marker,
        label=label,
        alpha=alpha,
    )


def plot_function(fn, max_x, label, linestyle):
    x = np.logspace(0, np.log10(max_x), num=500)
    y = fn(x)
    plt.plot(x, y, label=label, color="black", alpha=0.2, linestyle=linestyle)


def visualize(args):
    plt.figure(figsize=(8, 6))

    if args.loglog:
        plt.xscale("log", base=2)
        plt.yscale("log", base=2)

    max_x = get_max_x(args)
    if args.sqrt:
        plot_function(np.sqrt, max_x, r"$x^{1/2}$", linestyle=":")
    if args.cbrt:
        plot_function(np.cbrt, max_x, r"$x^{1/3}$", linestyle="-.")

    for i, filename in enumerate(args.files):
        label = filename.stem
        if args.bins:
            label = f"{label} (binned average)"
        x_values, y_values = get_values(filename, args)
        if args.bins:
            x_values, y_values = bin_data(x_values, y_values, args)
        scatter(x_values, y_values, label, colors[i], markers[i])

    plt.grid(True, linestyle="--", alpha=0.2)
    plt.xlabel(args.x_label)
    plt.ylabel(args.y_label)
    plt.legend()
    plt.savefig(args.output, format=args.type, dpi=600)
    plt.show()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", type=str)
    parser.add_argument("--x-label", type=str, default="Number of nodes")
    parser.add_argument("--y-label", type=str, default="Size of separator")
    parser.add_argument("--loglog", action="store_true")
    parser.add_argument("--cbrt", action="store_true")
    parser.add_argument("--sqrt", action="store_true")
    parser.add_argument("--keep-outliers", action="store_true")
    parser.add_argument("--type", type=str, default="pdf")
    parser.add_argument("--bins", type=int)
    parser.add_argument("files", type=Path, nargs="+")

    args = parser.parse_args()

    if not args.name:
        args.output = Path("output") / f"{args.files[0].stem}.{args.type}"
    else:
        args.output = Path("output") / f"{args.name}.{args.type}"

    visualize(args)


if __name__ == "__main__":
    main()
