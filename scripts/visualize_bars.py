#!/bin/python3
import argparse
from pathlib import Path
import matplotlib.pyplot as plt


color = "#009682"


def visualize(args: argparse.Namespace):
    with open(args.file) as file:
        data = [line.split(" ") for line in file]
        name = [d[0] for d in data]
        data = [float(d[1]) for d in data]
    plt.figure(figsize=(8, 6))
    plt.grid(True, alpha=0.2)
    plt.bar(name, data, color=color)
    plt.xlabel(args.x_label)
    plt.ylabel(args.y_label)
    plt.savefig(args.output, format=args.type, dpi=600)
    plt.show()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=str)
    parser.add_argument("--type", type=str, default="pdf")
    parser.add_argument("--x-label", type=str, required=True)
    parser.add_argument("--y-label", type=str, required=True)
    parser.add_argument("file", type=Path)

    args = parser.parse_args()

    if not args.output:
        args.output = Path("output") / f"{args.file.stem}.{args.type}"

    visualize(args)


if __name__ == "__main__":
    main()
