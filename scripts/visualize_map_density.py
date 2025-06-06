import argparse
import numpy as np
import matplotlib.pyplot as plt


def create_histogram_map(filepath, n):
    try:
        points = np.loadtxt(filepath)
    except FileNotFoundError:
        print(f"Error: The file '{filepath}' was not found.")
        return
    except Exception as e:
        print(f"An error occurred while reading the file: {e}")
        return

    if points.ndim != 2 or points.shape[1] != 2:
        print("Error: The input file must contain two columns of numbers (x y).")
        return

    x = points[:, 0]
    y = points[:, 1]

    grid, x_edges, y_edges = np.histogram2d(
        x, y, bins=n, range=[[np.min(x), np.max(x)], [np.min(y), np.max(y)]]
    )

    plt.figure(figsize=(8, 6))
    plt.imshow(grid.T, origin="lower", cmap="viridis", interpolation="nearest")
    plt.colorbar(label="Number of Points in Cell")
    plt.title(f"2D Histogram on a {n}x{n} Grid")
    plt.xlabel("X Coordinate Bins")
    plt.ylabel("Y Coordinate Bins")
    plt.show()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Visualize a 2D histogram of point data on an NxN grid."
    )
    parser.add_argument(
        "filepath",
        type=str,
        help="The path to the input file with x y coordinates per line.",
    )
    parser.add_argument(
        "-n",
        "--grid_size",
        type=int,
        required=True,
        help="The size of the grid (n x n).",
    )
    args = parser.parse_args()
    create_histogram_map(args.filepath, args.grid_size)
