#!/bin/python
from pathlib import Path
import sys
import igraph as ig
import numpy as np


def visualize() -> None:
    base_dir = Path(sys.argv[1])
    first_out = np.fromfile(base_dir / "first_out", dtype=np.int32)
    head = np.fromfile(base_dir / "head", dtype=np.int32)
    lat = np.fromfile(base_dir / "latitude", dtype=np.float32)
    lon = np.fromfile(base_dir / "longitude", dtype=np.float32)
    assert len(lat) == len(lon) == len(first_out) - 1
    assert len(head) == first_out[-1]

    coordinaes = [(lat[i], -lon[i]) for i in range(len(lat))]
    edges = [
        (v, h) for v in range(len(lat)) for h in head[first_out[v] : first_out[v + 1]]
    ]
    g = ig.Graph(edges=edges)
    layout = ig.Layout(coordinaes)

    colors = np.full(len(lat), "#000000")
    sizes = np.full(len(lat), 0)
    if len(sys.argv) == 3:
        highlight = np.loadtxt(sys.argv[2], dtype=int)
        for i in highlight:
            colors[i] = "#009682"
            sizes[i] = 32


    ig.plot(
        g,
        target=f"./output/{base_dir.name}.png",
        layout=layout,
        vertex_color=colors,
        vertex_size=sizes,
        edge_width=0.5,
        bbox=(2048, 2048),  
    )


if __name__ == "__main__":
    visualize()
