#!/bin/python3
import numpy as np
import matplotlib.pyplot as plt
import argparse
from pathlib import Path


colors = list(plt.get_cmap("tab10").colors)
colors = colors * 50
colors[0] = "#009682"
colors[1] = "#ff7f0e"
colors[2] = "#7f7f7f"


def histogram_data(file_path: Path, aggregated: bool):
    if aggregated:
        with open(file_path, "r") as f:
            bin_edges = np.fromstring(f.readline(), sep=" ")
            hist_values = np.fromstring(f.readline(), sep=" ")
            return hist_values, bin_edges
    else:
        return np.loadtxt(file_path)


def main(args: argparse.Namespace):
    plt.figure(figsize=(8, 6))
    plt.grid(True, alpha=0.2, linestyle="--")
    plt.xlabel(args.x_label)
    plt.ylabel(args.y_label)

    for i, file in enumerate(args.files):
        hist_values, bin_edges = histogram_data(file, args.aggregated)

        bin_widths = np.diff(bin_edges)
        total_area = np.sum(hist_values * bin_widths)
        if total_area <= 0:
            continue

        normalized_values = hist_values / total_area
        color = colors[i % len(colors)]

        plt.bar(
            bin_edges[:-1],
            normalized_values,
            width=bin_widths,
            color=color,
            align="edge",
            label=args.labels[i] if args.labels else file.stem,
            alpha=0.6,
        )
        if args.prefixsum:
            cdf_values = np.cumsum(normalized_values * bin_widths)
            plt.bar(
                bin_edges[:-1],
                cdf_values,
                width=bin_widths,
                color=color,
                alpha=0.2,
                align="edge",
            )

    plt.legend(loc="upper right")
    output_dir = Path.cwd() / "output" / "histogram"
    output_dir.mkdir(parents=True, exist_ok=True)
    plt.savefig(output_dir / f"{args.output}.pdf")
    plt.show()


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("files", type=Path, nargs="+")
    parser.add_argument("--bins", type=int, default=64)
    parser.add_argument("--x-label", default="Value")
    parser.add_argument("--y-label", default="Probability Density")
    parser.add_argument("--labels", type=str, nargs="*")
    parser.add_argument("--aggregated", action="store_true")
    parser.add_argument(
        "--prefixsum",
        action="store_true",
    )
    parser.add_argument("--output")
    args = parser.parse_args()

    if not args.output:
        args.output = args.files[0].stem

    main(args)


if __name__ == "__main__":
    parse_args()
