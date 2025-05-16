#!/bin/python3
import networkx as nx
import matplotlib.pyplot as plt
import re


def visualize_graph_from_string(graph_string):
    g = nx.DiGraph()

    if not graph_string.strip():
        print("Input string is empty. Cannot draw graph.")
        return

    for line in graph_string.strip().split("\n"):
        line = line.strip()
        if not line:
            continue

        match = re.match(r"(\d+):\s*\{([^}]*)\}", line)
        if not match:
            print(f"Skipping malformed line: {line}")
            continue

        node_str, connections_str = match.groups()

        try:
            node = int(node_str)
            g.add_node(node)

            if connections_str.strip():
                neighbors = connections_str.split(",")
                for neighbor_str in neighbors:
                    neighbor_str = neighbor_str.strip()
                    if neighbor_str:
                        try:
                            neighbor = int(neighbor_str)
                            g.add_edge(node, neighbor)
                        except ValueError:
                            print(
                                f"Skipping invalid neighbor '{neighbor_str}' for node {node}"
                            )
        except ValueError:
            print(f"Skipping line with invalid node: {node_str}")
            continue

    if not g.nodes:
        print("No valid graph data processed. Cannot draw graph.")
        return

    pos = nx.spring_layout(g)
    nx.draw(
        g,
        pos,
        with_labels=True,
        node_size=700,
        node_color="skyblue",
        font_size=10,
        font_weight="bold",
        arrowsize=15,
        arrows=True,
    )
    plt.title("Graph Visualization")
    plt.show()


if __name__ == "__main__":
    graph_data_string = """
0: {6, 1}
1: {0}
2: {5, 8}
3: {9}
4: {5}
5: {9, 4, 2, 7}
6: {7, 0}
7: {5, 6}
8: {2}
9: {5, 3}
"""
    visualize_graph_from_string(graph_data_string)
