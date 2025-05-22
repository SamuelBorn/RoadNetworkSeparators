#!/bin/python3
import argparse
from pathlib import Path
import numpy as np
import matplotlib.pyplot as plt


def load_points(args: argparse.Namespace):
    x, y = np.loadtxt(args.file, unpack=True)
    return x, y


def visualize_points(args: argparse.Namespace):
    x, y = load_points(args)
    plt.gca().set_aspect("equal", adjustable="box")
    plt.scatter(x, y, s=1)
    plt.grid(True)
    plt.show()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("file", type=Path)
    args = parser.parse_args()

    visualize_points(args)


if __name__ == "__main__":
    main()
