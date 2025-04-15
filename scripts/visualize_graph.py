import argparse
import struct
from pathlib import Path

from graph_tool.all import Graph, graph_draw, sfdp_layout
import graph_tool


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
    first_out = read_binary_vec(args.graphdir / "first_out", "i")
    head = read_binary_vec(args.graphdir / "head", "i")
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
        latitude: list[float] = read_binary_vec(args.graphdir / "latitude", "f")
        longitude: list[float] = read_binary_vec(args.graphdir / "longitude", "f")
        latitude = [-1 * x for x in latitude]
        # longitude = [-1 * x for x in longitude]
        assert len(latitude) == len(longitude) == len(g.get_vertices())
        pos = g.new_vertex_property("vector<double>")
        for v in g.vertices():
            pos[v] = [longitude[int(v)], latitude[int(v)]]

    highlight_colors = [
        [0, 0.5882352941176471, 0.5098039215686274, 1],
        [0.8745098039215686, 0.6078431372549019, 0.10588235294117647, 1],
        [0.27450980392156865, 0.39215686274509803, 0.6666666666666666, 1],
        [0.6392156862745098, 0.06274509803921569, 0.48627450980392156, 1],
    ]
    vertex_color = g.new_vertex_property("vector<double>")
    vertex_size = g.new_vertex_property("double")
    for v in g.vertices():
        vertex_color[v] = [0, 0, 0, 1]
        vertex_size[v] = 0
    if args.highlight_nodes:
        for file, color in zip(args.highlight_nodes, highlight_colors):
            highlight_indices: list[int] = read_node_list(Path(file))
            for idx in highlight_indices:
                vertex_size[idx] = args.size / 50
                if vertex_color[idx] != [0, 0, 0, 1]:
                    vertex_color[idx] = [0, 0, 0, 1]
                else:
                    vertex_color[idx] = color

    graph_draw(
        g,
        pos=pos,
        output=str(args.output),
        output_size=(args.size, args.size),
        vertex_size=vertex_size,
        vertex_fill_color=vertex_color,
        edge_pen_width=1,
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("graphdir", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--highlight-nodes", type=Path, nargs="*")
    parser.add_argument("--auto-layout", action="store_true")
    parser.add_argument("--size", type=int, default=2**10)
    args: argparse.Namespace = parser.parse_args()

    if not args.output:
        args.output = Path("output") / args.graphdir.name
    args.output = args.output.with_suffix(".png")

    visualize(args)


if __name__ == "__main__":
    main()
