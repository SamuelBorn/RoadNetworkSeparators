import argparse
from pathlib import Path
import networkx as nx
import numpy as np
import matplotlib.pyplot as plt


def read_metis(filename: Path) -> nx.Graph:
    with open(filename, "r") as f:
        lines = f.readlines()
    g = nx.Graph()
    for v, line in list(enumerate(lines))[1:]:
        for u in line.split():
            g.add_edge(int(v), int(u))
    return g


def visualize(args: argparse.Namespace) -> None:
    g = read_metis(args.input)
    pos = nx.kamada_kawai_layout(g)
    nx.draw(g, pos=pos, with_labels=True)
    plt.show()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=Path)
    args = parser.parse_args()
    visualize(args)


if __name__ == "__main__":
    main()
