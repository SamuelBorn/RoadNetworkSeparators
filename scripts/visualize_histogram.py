#!/bin/python3
import argparse
import os
from pathlib import Path

import matplotlib.pyplot as plt


def visualize(args):
    plt.figure(figsize=(8, 6))
    plt.xlabel(args.x_label)
    plt.ylabel(args.y_label)
    plt.grid(True, alpha=0.2, linestyle="--")

    if args.log_x:
        plt.xscale("log")
    if args.log_y:
        plt.yscale("log")

    values = read_file(args.file)
    plt.hist(values, bins=args.bins, alpha=1, color="#009682")

    plt.savefig(args.output, format="png", dpi=600)
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
    parser.add_argument("file", type=Path)

    args = parser.parse_args()

    if not args.output:
        args.output = f"output/histogram/{args.file.stem}.png"

    return args


if __name__ == "__main__":
    args = parse_args()
    visualize(args)

