#!/bin/python3
import sys

import matplotlib.pyplot as plt


def read_edges(filename):
    edges = []
    with open(filename, "r") as file:
        for line in file:
            try:
                x1, y1, x2, y2 = map(int, line.split())
                edges.append(((x1, y1), (x2, y2)))
            except:
                pass
    return edges


def plot_edges(edges):
    plt.figure(figsize=(8, 8))
    for (x1, y1), (x2, y2) in edges:
        plt.plot(
            [x1, x2], [y1, y2], 'o-'
        )  # 'bo-' means blue circles at points with a solid line

    plt.xlabel("X-axis")
    plt.ylabel("Y-axis")
    plt.title("Edge Visualization")
    plt.grid(True)
    plt.show()


def main():
    filename = sys.argv[1]
    edges = read_edges(filename)
    plot_edges(edges)


if __name__ == "__main__":
    main()
