import argparse
import json

import matplotlib.pyplot as plt
import networkx as nx


def visualize(args):
    with open(args.filename) as f:
        graph_data = json.load(f)

    G = nx.Graph()
    for node, neighbors in graph_data.items():
        for neighbor in neighbors:
            G.add_edge(node, str(neighbor))

    # pos = nx.spring_layout(G)
    nx.draw(G, with_labels=True)
    plt.show()

    if args.output:
        plt.savefig(args.output)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("filename", help="JSON file containing graph data")
    parser.add_argument("--output", help="Output file for the visualization")
    args = parser.parse_args()

    visualize(args)


if __name__ == "__main__":
    main()
