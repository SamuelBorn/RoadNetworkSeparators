import argparse
import struct
from pathlib import Path

from graph_tool.all import Graph, graph_draw


def visualize(args):
    first_out = read_binary_vec(args.dirname / "first_out", "i")
    head = read_binary_vec(args.dirname / "head", "i")
    latitude = read_binary_vec(args.dirname / "latitude", "f")
    longitude = read_binary_vec(args.dirname / "longitude", "f")
    assert len(first_out) - 1 == len(latitude) == len(longitude)

    g = Graph()
    g.set_directed(False)
    g.add_vertex(len(first_out) - 1)

    for v in range(len(first_out) - 1):
        for h in head[first_out[v] : first_out[v + 1]]:
            g.add_edge(v, h)

    pos = g.new_vertex_property("vector<double>")
    for v in g.vertices():
        pos[v] = [longitude[int(v)], latitude[int(v)]]

    graph_draw(
        g,
        pos=pos,
        output=str(args.output),
        output_size=(8000, 8000),
        vertex_size=0,
        edge_pen_width=1,
    )


def read_binary_vec(filename: Path, format_char: str):  # format_char: "i", "f"
    with open(filename, "rb") as f:
        data = f.read()
    return list(
        struct.unpack(f"{len(data) // struct.calcsize(format_char)}{format_char}", data)
    )


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("dirname")
    parser.add_argument("--output")
    parser.add_argument("--without-embedding", action="store_true")
    args = parser.parse_args()

    args.dirname = Path(args.dirname)

    if args.output:
        args.output = Path(args.output)
    else:
        args.output = Path("output") / args.dirname.name
    args.output = args.output.with_suffix(".png")

    visualize(args)


if __name__ == "__main__":
    main()
