#!/bin/python3
import numpy as np
import matplotlib.pyplot as plt
import argparse
from pathlib import Path


def histogram_data(args: argparse.Namespace):
    if args.aggregated:
        with open(args.file, "r") as f:
            bin_edges = np.array([float(x) for x in f.readline().strip().split()])
            hist_values = np.array([float(x) for x in f.readline().strip().split()])
            return hist_values, bin_edges
    else:
        data = np.loadtxt(args.file)
        return np.histogram(data, bins=args.bins)


def main(args: argparse.Namespace):
    hist_values, bin_edges = histogram_data(args)

    plt.figure(figsize=(8, 6))
    plt.grid(True, alpha=0.2, linestyle="--")
    plt.xlabel(args.x_label)
    plt.ylabel(args.y_label)
    plt.bar(
        bin_edges[:-1],
        hist_values,
        width=np.diff(bin_edges),
        color="#009682",
        align="edge",
    )
    plt.savefig(f"./output/histogram/{args.output}.png", dpi=600)
    plt.show()


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("file", type=Path)
    parser.add_argument("--bins", type=int, default=64)
    parser.add_argument("--x-label", default="Value")
    parser.add_argument("--y-label", default="Frequency")
    parser.add_argument("--aggregated", action="store_true")
    parser.add_argument("--output")
    args = parser.parse_args()

    if not args.output:
        args.name = args.file.stem

    main(args)


if __name__ == "__main__":
    parse_args()
