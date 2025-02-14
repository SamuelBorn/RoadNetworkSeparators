import argparse
import os
from scipy.stats import linregress

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns


def get_values(filename):
    return zip(*[map(float, line.split()) for line in open(filename)])


def bin_data(x, y, num_bins):
    data = np.column_stack((x, y))
    bins = np.linspace(np.min(x), np.max(x) + 1, num_bins + 1)
    bin_indices = np.digitize(x, bins) - 1
    binned_means = []
    for i in range(num_bins):
        mask = bin_indices == i
        if np.any(mask):
            mean_values = data[mask].mean(axis=0)
            # mean_values = np.median(data[mask], axis=0)
            binned_means.append(mean_values)

    binned_means = np.array(binned_means)
    return binned_means[:, 0], binned_means[:, 1]


def scatter(x_values, y_values, label, color, marker, alpha=1):
    plt.scatter(
        x_values,
        y_values,
        color=color,
        marker=marker,
        label=label,
        alpha=alpha,
    )


def plot_heatmap(x_values, y_values, bins=50):
    # scale log log
    x_values = np.log10(x_values)
    y_values = np.log10(y_values)
    x_min, x_max = 0, max(x_values)
    y_min, y_max = 0, max(y_values)

    heatmap, xlabels, ylabels = np.histogram2d(
        x_values, y_values, bins=bins, range=[[x_min, x_max], [y_min, y_max]]
    )
    heatmap = np.power(heatmap, 0.01)
    mask = heatmap == 0
    sns.heatmap(
        heatmap.T,
        cmap="Blues",
        xticklabels=list(np.round(xlabels, 2)),
        yticklabels=list(np.round(ylabels, 2)),
        square=True,
        mask=mask.T,
    )
    plt.gca().invert_yaxis()


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
    plt.title(args.title)
    plt.xlabel(args.x_label)
    plt.ylabel(args.y_label)

    if args.loglog:
        plt.xscale("log")
        plt.yscale("log")

    # if args.sqrt:
    #     plot_function(np.sqrt, find_max_x(args.files), "$\sqrt{x}$")
    # if args.cbrt:
    #     plot_function(np.cbrt, find_max_x(args.files), "$\sqrt[3]{x}$", linestyle="-.")

    markers = ["^", "v", "x", "+"]
    colors = ["#009682", "#df9b1b", "#4664aa", "#a3107c"]

    plt.plot([0, np.log(1_000_000)], [0, np.log(np.sqrt(1_000_000))])
    plt.plot([0, np.log(1_000_000)], [0, np.log(np.cbrt(1_000_000))])
    for i, filename in enumerate(args.files):
        x_values, y_values = get_values(filename)
        x_values = np.log(x_values)
        y_values = np.log(y_values)

        # filter out europe outliers
        x_values, y_values = zip(
            *[(x, y) for x, y in zip(x_values, y_values) if x < 13]
        )
        label = os.path.splitext(os.path.basename(filename))[0]

        if args.bins:
            x_values, y_values = bin_data(x_values, y_values, args.bins)
            # fit line 
            slope, intercept, r_value, p_value, std_err = linregress(x_values, y_values)
            print(f"Fitted Line: y = {slope:.4f}x + {intercept:.4f}")
            print(f"R² Score: {r_value:.4f} (closer to 1 is better)")
            print(f"P-value: {p_value:.4e} (for slope, closer to 0 is better)")
            print(f"Standard Error: {std_err:.4f}\n")


        if args.heatmap:
            plot_heatmap(x_values, y_values)
            break
        else:
            scatter(x_values, y_values, label, colors[i], markers[i])

    plt.grid(True, linestyle="--", alpha=0.6)
    plt.legend()
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
    parser.add_argument("--heatmap", action="store_true")
    parser.add_argument("--bins", type=int)
    parser.add_argument("files", nargs="*")

    args = parser.parse_args()

    if not args.title:
        file_name_without_extension, _ = os.path.splitext(
            os.path.basename(args.files[0])
        )
        args.title = file_name_without_extension

    if not args.output:
        args.output = f"output/{args.title}.pdf"

    if args.loglog:
        args.x_label = args.x_label + " (log scale)"
        args.y_label = args.y_label + " (log scale)"

    return args


def main():
    args = parse_args()
    visualize(args)


if __name__ == "__main__":
    main()
