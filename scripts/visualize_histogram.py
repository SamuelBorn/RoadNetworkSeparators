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

    if args.log_x:
        plt.xscale("log")
    if args.log_y and not args.normalize: # Log scale for y-axis doesn't make sense with normalization to percentage
        plt.yscale("log")
    elif args.log_y and args.normalize:
        print("Warning: --log-y is ignored when --normalize is used.")


    values = read_file(args.file)
    
    if args.normalize:
        weights = np.ones_like(values) / len(values)
        plt.hist(values, bins=args.bins, alpha=1, color="#009682", weights=weights)
        plt.gca().yaxis.set_major_formatter(mtick.PercentFormatter(xmax=1.0))
    else:
        plt.hist(values, bins=args.bins, alpha=1, color="#009682")

    # Ensure output directory exists
    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)

    plt.savefig(output_path, format="png", dpi=600)
    plt.show()


def read_file(file_path):
    with open(file_path, "r") as f:
        return [float(line) for line in f.readlines()]


def parse_args():
    parser = argparse.ArgumentParser(description="Generate a histogram from a file of numbers.")
    parser.add_argument("--output", help="Output file path for the histogram image.")
    parser.add_argument("--bins", type=int, default=30, help="Number of bins for the histogram.")
    parser.add_argument("--log-x", action="store_true", help="Use a logarithmic scale for the x-axis.")
    parser.add_argument("--log-y", action="store_true", help="Use a logarithmic scale for the y-axis (ignored if --normalize is used).")
    parser.add_argument("--x-label", default="Value", help="Label for the x-axis.")
    parser.add_argument("--y-label", default="Frequency", help="Label for the y-axis (used if --normalize is not active).")
    parser.add_argument("--normalize", action="store_true", help="Show percentages on the y-axis instead of absolute counts.")
    parser.add_argument("file", type=Path, help="Input file containing one number per line.")

    args = parser.parse_args()

    if not args.output:
        # Create a default output directory if it doesn't exist
        default_output_dir = Path("output/histogram")
        default_output_dir.mkdir(parents=True, exist_ok=True)
        args.output = default_output_dir / f"{args.file.stem}.png"
    
    return args


if __name__ == "__main__":
    args = parse_args()
    visualize(args)
