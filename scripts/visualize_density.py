import argparse
from pathlib import Path

import matplotlib.colors as mcolors
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.colors import LogNorm

kit_cmap = mcolors.LinearSegmentedColormap.from_list("custom_map", ["white", "#009682"])


def get_data(args):
    x, y = np.loadtxt(args.file, unpack=True)
    if args.loglog:
        x = np.log2(x)
        y = np.log2(y)
    return x, y


def visualize(args):
    x, y = get_data(args)
    plt.figure(figsize=(8, 6))
    plt.hist2d(x, y, bins=[args.bins, 20], cmap=kit_cmap, norm=LogNorm())
    plt.xlabel(args.x_label)
    plt.ylabel(args.y_label)
    plt.colorbar()
    plt.savefig(args.output, dpi=600)
    plt.show()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("file", type=Path)
    parser.add_argument("--name", type=str)
    parser.add_argument("--x-label", type=str, default="Number of nodes")
    parser.add_argument("--y-label", type=str, default="Size of separator")
    parser.add_argument("--loglog", action="store_true")
    parser.add_argument("--bins", type=int, default=45)
    args = parser.parse_args()

    if not args.name:
        args.output = Path("output") / f"{args.file.stem}-hist.pdf"
    else:
        args.output = Path("output") / f"{args.name}-hist.pdf"

    if args.loglog:
        args.x_label = f"$\\log_2(\\text{{ {args.x_label} }})$"
        args.y_label = f"$\\log_2(\\text{{ {args.y_label} }})$"

    visualize(args)


if __name__ == "__main__":
    main()
