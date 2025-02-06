import argparse
import struct
from pathlib import Path

from graph_tool.all import Graph, graph_draw, sfdp_layout


def read_node_list(filename: Path) -> list[int]:
    with open(filename, "r") as f:
        return [int(line.strip()) for line in f if line.strip()]


# format_char: "i", "f"
def read_binary_vec(filename: Path, format_char: str) -> list[int | float]:
    with open(filename, "rb") as f:
        data = f.read()
    return list(
        struct.unpack(f"{len(data) // struct.calcsize(format_char)}{format_char}", data)
    )


def visualize(args: argparse.Namespace) -> None:
    first_out = read_binary_vec(args.dirname / "first_out", "i")
    head = read_binary_vec(args.dirname / "head", "i")
    g: Graph = Graph()
    g.set_directed(False)
    g.add_vertex(len(first_out) - 1)

    for v in range(len(first_out) - 1):
        for h in head[first_out[v] : first_out[v + 1]]:
            if g.edge(v, h) is None:
                g.add_edge(v, h)

    if args.auto_layout:
        pos = sfdp_layout(g)
    else:
        latitude: list[float] = read_binary_vec(args.dirname / "latitude", "f")
        longitude: list[float] = read_binary_vec(args.dirname / "longitude", "f")
        assert len(latitude) == len(longitude) == len(g.get_vertices())
        pos = g.new_vertex_property("vector<double>")
        for v in g.vertices():
            pos[v] = [longitude[int(v)], latitude[int(v)]]

    vertex_color = g.new_vertex_property("vector<double>")
    for v in g.vertices():
        vertex_color[v] = [0, 0, 0, 1]
    if args.highlight_nodes:
        highlight_indices: list[int] = read_node_list(args.highlight_nodes)
        for idx in highlight_indices:
            vertex_color[idx] = [1, 0, 0, 1]

    graph_draw(
        g,
        pos=pos,
        output=str(args.output),
        output_size=(2**13, 2**13),
        vertex_size=4,
        vertex_fill_color=vertex_color,
        edge_pen_width=1,
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("dirname", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--highlight-nodes", type=Path)
    parser.add_argument("--auto-layout", action="store_true")
    args: argparse.Namespace = parser.parse_args()

    if not args.output:
        args.output = Path("output") / args.dirname.name
    args.output = args.output.with_suffix(".png")

    visualize(args)


if __name__ == "__main__":
    main()
