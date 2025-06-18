import numpy as np
import matplotlib.pyplot as plt

def sample_points_in_circle(n_points, center, radius):
    angles = 2 * np.pi * np.random.rand(n_points)
    radii = radius * np.sqrt(np.random.rand(n_points))
    x = center[0] + radii * np.cos(angles)
    y = center[1] + radii * np.sin(angles)
    return np.column_stack((x, y))

def generate_hierarchical_points(levels, points_per_level, initial_center, radii):
    all_points = []
    centers = np.array([initial_center])
    for i in range(levels):
        points = np.vstack([sample_points_in_circle(points_per_level, c, radii[i]) for c in centers])
        all_points.append(points)
        centers = points
    return all_points

def plot_points(points_by_level):
    colors = ["#009682",  "#E69F00", "#56B4E9" ]

    sizes = [100, 25, 6]
    alphas = [1, 0.666, 0.333]
    plt.figure(figsize=(5, 5))
    for i, points in reversed(list(enumerate(points_by_level))):
        plt.scatter(
            points[:, 0],
            points[:, 1],
            s=sizes[i % len(sizes)],
            c=colors[i % len(colors)],
            alpha=alphas[i % len(alphas)],
            label=f'Level {i + 1}'
        )
    plt.legend(loc='upper right')
    plt.axis("off")
    plt.grid(True, linestyle='--', alpha=0.5)
    plt.show()


if __name__ == "__main__":
    all_points = generate_hierarchical_points(
        levels=3,
        points_per_level=15,
        initial_center=(0, 0),
        # radii=[10000, 600, 50]
        radii=[10000, 10000, 10000]
        # radii=[10000, 5000, 1000]
    )
    plot_points(all_points)
