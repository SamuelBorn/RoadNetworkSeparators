#!/bin/python3
import argparse
from scipy.optimize import curve_fit
from pathlib import Path
import math
import matplotlib.pyplot as plt
import numpy as np
from scipy.stats import binned_statistic

colors = ["#009682", "#df9b1b", "#4664aa", "#a3107c"]


def get_max_x(files, keep_outliers):
    max_x = 0
    dummy_args = argparse.Namespace(keep_outliers=keep_outliers)
    for file in files:
        x_values, _ = get_values(file, dummy_args)
        if x_values.size > 0:
            max_x = max(max_x, np.max(x_values))
    return max_x


def get_values(filename, args):
    with open(filename, "r") as f:
        data = [tuple(map(float, line.split())) for line in f if line.strip()]
    if not args.keep_outliers:
        data = [p for p in data if p[0] < 10_000_000]
    if not data:
        return np.array([]), np.array([])
    # data = [p for p in data if p[0] < 500_000]
    x, y = zip(*data)
    return np.array(x), np.array(y)


def group_data_by_bins(x, y, n_bins, log_scale):
    max_val_x, min_val_x = np.max(x), np.min(x)
    if log_scale:
        min_pos_x = np.min(x[x > 0])
        bin_edges = np.logspace(
            np.log10(min_pos_x), np.log10(max_val_x), num=n_bins + 1
        )
        bin_centers = np.sqrt(bin_edges[:-1] * bin_edges[1:])
    else:
        bin_edges = np.linspace(min_val_x, max_val_x, num=n_bins + 1)
        bin_centers = (bin_edges[:-1] + bin_edges[1:]) / 2

    _, _, binnumber = binned_statistic(x, y, statistic="count", bins=bin_edges)
    y_values_per_bin = [[] for _ in range(n_bins)]
    for i in range(len(x)):
        bin_idx = binnumber[i] - 1
        if 0 <= bin_idx < n_bins:
            y_values_per_bin[bin_idx].append(y[i])

    valid_indices = [i for i, bin_data in enumerate(y_values_per_bin) if bin_data]
    valid_bin_centers = bin_centers[valid_indices]
    valid_bins_data = [y_values_per_bin[i] for i in valid_indices]
    medians_per_bin = [np.median(data) for data in valid_bins_data]
    return valid_bin_centers, valid_bins_data, medians_per_bin


def plot_function(fn, max_x, label, linestyle, color="black", alpha=0.2):
    ax = plt.gca()
    is_log_x = ax.get_xscale() == "log"
    is_log_y = ax.get_yscale() == "log"
    plot_min_x = max(ax.get_xlim()[0], 1e-9) if is_log_x else ax.get_xlim()[0]
    x = (
        np.logspace(np.log10(plot_min_x), np.log10(max_x), num=200)
        if is_log_x
        else np.linspace(plot_min_x, max_x, num=200)
    )
    y = fn(x)
    valid = (y > 0) if is_log_y else np.isfinite(y)
    ax.plot(
        x[valid], y[valid], label=label, color=color, alpha=alpha, linestyle=linestyle
    )


def visualize(args):
    plt.figure(figsize=(10, 7))
    if args.loglog:
        plt.xscale("log", base=2)
        plt.yscale("log", base=2)

    plot_max_x = get_max_x(args.files, args.keep_outliers)
    if args.sqrt:
        plot_function(np.sqrt, plot_max_x, r"$x^{1/2}$", ":")
    if args.cbrt:
        plot_function(np.cbrt, plot_max_x, r"$x^{1/3}$", "-.")

    all_bin_centers = []
    file_plot_data = []
    for i, filename in enumerate(args.files):
        x_values, y_values = get_values(filename, args)
        if x_values.size == 0:
            continue
        bin_centers, y_vals_bin, medians_bin = group_data_by_bins(
            x_values, y_values, args.bins, args.loglog
        )
        if not bin_centers.any():
            continue
        all_bin_centers.extend(bin_centers)
        file_plot_data.append(
            {
                "f": filename,
                "c": np.array(bin_centers),
                "d": y_vals_bin,
                "m": np.array(medians_bin),
                "col": colors[i % len(colors)],
            }
        )

    unique_centers = sorted(list(set(all_bin_centers)))
    min_diff = np.min(np.diff(unique_centers)) if len(unique_centers) > 1 else 0.5
    box_width = (
        min_diff * 0.6
        if len(unique_centers) > 1
        else (
            unique_centers[0] * 0.1
            if plt.gca().get_xscale() == "log" and len(unique_centers) == 1
            else 0.5
        )
    )

    plotted_labels = {}
    for plot_data in file_plot_data:
        label = plot_data["f"].stem
        bp = plt.boxplot(
            plot_data["d"],
            positions=plot_data["c"],
            widths=box_width,
            patch_artist=True,
            showfliers=False,
            medianprops=dict(color="black"),
            boxprops=dict(facecolor=plot_data["col"], alpha=0.6),
            whiskerprops=dict(color=plot_data["col"], alpha=0.8),
            capprops=dict(color=plot_data["col"], alpha=0.8),
        )
        if label not in plotted_labels:
            plotted_labels[label] = bp["boxes"][0]

        if args.fit_line:
            centers, medians = plot_data["c"], plot_data["m"]
            valid = np.isfinite(centers) & np.isfinite(medians)
            if args.loglog:
                valid &= (centers > 0) & (medians > 0)
            fit_centers, fit_medians = centers[valid], medians[valid]
            if len(fit_centers) < 2:
                continue

            if args.loglog:
                m, _ = np.polyfit(np.log2(fit_centers), np.log2(fit_medians), 1)
                a = curve_fit(lambda x, a: a * x**m, fit_centers, fit_medians)[0][0]
                fit_label, fit_func = (
                    f"${a:.2f} \\cdot x^{{ {m:.2f} }}$ (fit)",
                    lambda x: a * x**m,
                )
            else:
                m, c = np.polyfit(fit_centers, fit_medians, 1)
                fit_label, fit_func = f"${m:.2f}x + {c:.2f}$ (fit)", lambda x: m * x + c

            plot_function(fit_func, plot_max_x, fit_label, "--", plot_data["col"], 0.8)
            plotted_labels[fit_label] = plt.Line2D(
                [0], [0], color=plot_data["col"], linestyle="--", alpha=0.8
            )

    plt.grid(True, linestyle="--", alpha=0.3)
    plt.xlabel(args.x_label)
    plt.ylabel(args.y_label)
    handles, labels = plt.gca().get_legend_handles_labels()
    combined_labels = {**plotted_labels, **dict(zip(labels, handles))}
    plt.legend(combined_labels.values(), combined_labels.keys(), loc="best")
    plt.tight_layout()
    plt.savefig(args.output, format=args.type, dpi=300)
    print(f"Plot saved to {args.output}")
    plt.show()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", type=str)
    parser.add_argument("--x-label", type=str, default="X")
    parser.add_argument("--y-label", type=str, default="Y")
    parser.add_argument("--loglog", action="store_true")
    parser.add_argument("--cbrt", action="store_true")
    parser.add_argument("--sqrt", action="store_true")
    parser.add_argument("--keep-outliers", action="store_true")
    parser.add_argument("--type", type=str, default="pdf")
    parser.add_argument("--bins", type=int, required=True)
    parser.add_argument("--fit-line", action="store_true")
    parser.add_argument("files", type=Path, nargs="+")
    args = parser.parse_args()
    base_name = args.name if args.name else args.files[0].stem
    args.output = Path("output") / f"{base_name}_boxplot.{args.type}"
    visualize(args)


if __name__ == "__main__":
    main()
