#!/bin/python3
import argparse
import os
from pathlib import Path

import matplotlib.pyplot as plt
import matplotlib.ticker as mtick
import numpy as np


def visualize(args):
    plt.figure(figsize=(8, 6))
    plt.xlabel(args.x_label)

    if args.normalize:
        plt.ylabel("Percentage")
    else:
        plt.ylabel(args.y_label)

    plt.grid(True, alpha=0.2, linestyle="--")

    values = read_file(args.file)

    actual_bins = args.bins

    if args.log_x:
        plt.xscale("log")

        positive_values = [v for v in values if v > 0]

        if len(positive_values) >= 2:
            min_val = min(positive_values)
            max_val = max(positive_values)

            if min_val < max_val:
                log_min = np.log10(min_val)
                log_max = np.log10(max_val)
                actual_bins = np.logspace(log_min, log_max, num=args.bins + 1)
            else:
                print(
                    f"Warning: --log-x: All positive data values are identical ({min_val}). Custom logarithmic bins cannot be created. Using default binning strategy on log scale."
                )
        else:
            unique_positive_count = len(set(positive_values))
            print(
                f"Warning: --log-x: Insufficient distinct positive data ({unique_positive_count} unique positive values) to create custom logarithmic bins. Using default binning strategy on log scale."
            )
            if not positive_values and values:
                print(
                    "Warning: --log-x: No positive data values found. Log scale applied, but plot may be empty or show warnings."
                )

    if args.log_y and not args.normalize:
        plt.yscale("log")
    elif args.log_y and args.normalize:
        print("Warning: --log-y is ignored when --normalize is used.")

    if args.normalize:
        if not values:
            print("Warning: No data to normalize. Plot will be empty.")
            weights = []
            # To avoid error with plt.hist if values is empty and actual_bins is an int
            # plt.hist([], bins=3) is fine, plt.hist([], bins=[1,2,3]) is fine
        else:
            weights = np.ones_like(values) / len(values) if values else []
        plt.hist(values, bins=actual_bins, alpha=1, color="#009682", weights=weights)
        if values:  # Only apply formatter if there's data that might be plotted
            plt.gca().yaxis.set_major_formatter(mtick.PercentFormatter(xmax=1.0))
    else:
        plt.hist(values, bins=actual_bins, alpha=1, color="#009682")

    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)

    plt.savefig(output_path, format="png", dpi=600)
    plt.show()


def read_file(file_path):
    with open(file_path, "r") as f:
        return [float(line) for line in f.readlines()]


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--output")
    parser.add_argument("--bins", type=int, default=30)
    parser.add_argument("--log-x", action="store_true")
    parser.add_argument("--log-y", action="store_true")
    parser.add_argument("--x-label", default="Value")
    parser.add_argument("--y-label", default="Frequency")
    parser.add_argument("--normalize", action="store_true")
    parser.add_argument("file", type=Path)
    args = parser.parse_args()

    if not args.output:
        default_output_dir = Path("output/histogram")
        default_output_dir.mkdir(parents=True, exist_ok=True)
        args.output = default_output_dir / f"{args.file.stem}.png"

    return args


if __name__ == "__main__":
    args = parse_args()
    visualize(args)
