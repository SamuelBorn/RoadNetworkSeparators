import argparse
from scipy.optimize import curve_fit
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy.stats import binned_statistic

markers = ["x", "^", "o", "+", "v", "s", "D", "P", "*"]
markers = markers * 10
colors = list(plt.get_cmap("tab10").colors)
colors = colors * 10
colors[0] = "#009682"


def get_max_x(args):
    max_x = 0
    for file in args.files:
        x_values, _ = get_values(file, args)
        max_x = max(max_x, max(x_values))
    return max_x


def get_values(filename, args):
    x, y = np.loadtxt(filename, unpack=True)
    if not args.keep_outliers:
        y = y[x < 10_000_000]
        x = x[x < 10_000_000]
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


def scatter(x_values, y_values, label, color, marker, alpha=1.0):
    plt.scatter(
        x_values,
        y_values,
        color=color,
        marker=marker,
        label=label,
        alpha=alpha,
    )


def plot_function(fn, max_x, label, linestyle, color="black", alpha=0.2):
    x = np.logspace(0, np.log10(max_x), num=500)
    y = fn(x)
    plt.plot(x, y, label=label, color=color, alpha=alpha, linestyle=linestyle)


def visualize(args):
    plt.figure(figsize=(8, 6))

    if args.loglog:
        plt.xscale("log", base=2)
        plt.yscale("log", base=2)

    max_x = get_max_x(args)
    if args.sqrt:
        plot_function(np.sqrt, max_x, r"$x^{1/2}$", linestyle="-")
    if args.cbrt:
        plot_function(np.cbrt, max_x, r"$x^{1/3}$", linestyle="-.")
    if args.europe:
        plot_function(
            lambda x: 0.3411 * x**0.3702,
            max_x,
            r"Observed Europe Fit $(\approx 0.34\cdot x^{0.37}$)",
            linestyle=":",
        )

    # plot_function(lambda x: 0.546995 * x**(0.2676), max_x, r"$\log_2 y = 0.2676 \cdot \log_2 x - 0.8704$", linestyle=":", color="purple", alpha=0.25)
    # plot_function(lambda x: 0.300993 * x**(0.365), max_x, r"$\log_2 y = 0.3650 \cdot \log_2 x - 1.7322$", linestyle="-", color="brown", alpha=0.25)
    # plot_function(lambda x: 0.4599 * x**(1/3) - 0.0341, max_x, r"$0.4599 \cdot x^{1/3} - 0.0341$", linestyle="-.", color="blue", alpha=0.25)
    # plot_function(lambda x: 0.1087 * x**(1/2) + 0.5000, max_x, r"$0.1087 \cdot x^{1/2} + 0.5000$", linestyle="-", color="green", alpha=0.25)
    # plot_function(lambda x: 0.2346 * x**(0.3980) + 0.6140, max_x, r"$0.2346 \cdot x^{0.3980} + 0.6140$", linestyle="--", color="orange", alpha=0.25)

    for i, filename in enumerate(args.files):
        label = args.labels[i] if args.labels else filename.stem
        if args.bins:
            label = f"{label} (binned average)"
        x_values, y_values = get_values(filename, args)
        if args.bins:
            x_values, y_values = bin_data(x_values, y_values, args)

        if args.cutoff:
            condition_left = x_values < args.cutoff
            condition_right = x_values >= args.cutoff
            x_values_left = x_values[condition_left]
            y_values_left = y_values[condition_left]
            x_values = x_values[condition_right]
            y_values = y_values[condition_right]
            scatter(x_values_left, y_values_left, "", colors[i], markers[i], alpha=0.3)
        scatter(x_values, y_values, label, colors[i], markers[i])

        if args.fit_line:
            assert args.loglog
            tmp_x = np.log2(x_values)
            tmp_y = np.log2(y_values)
            m, _ = np.polyfit(tmp_x, tmp_y, 1)
            p, _ = curve_fit(lambda x, a: a * np.power(x, m), x_values, y_values)
            plot_function(
                lambda x: p[0] * np.power(x, m),
                max_x,
                f"${p[0]:.4f} \\cdot x^{{ {m:.4f} }}$ (fitted)",
                linestyle=":",
                color=colors[i],
                alpha=0.5,
            )

    plt.grid(True, linestyle="--", alpha=0.2)
    plt.xlabel(args.x_label)
    plt.ylabel(args.y_label)
    plt.legend(loc="upper left")
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
    parser.add_argument("--fit-line", action="store_true")
    parser.add_argument("--europe", action="store_true")
    parser.add_argument("--labels", type=str, nargs="*")
    parser.add_argument("--cutoff", type=int, default=512)
    parser.add_argument("files", type=Path, nargs="+")

    args = parser.parse_args()

    if not args.name:
        args.output = Path("output") / f"{args.files[0].stem}.{args.type}"
    else:
        args.output = Path("output") / f"{args.name}.{args.type}"

    visualize(args)


if __name__ == "__main__":
    main()
