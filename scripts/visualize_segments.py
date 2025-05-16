#!/bin/python3
# this file is used to visualize line segments of a file that look like this:
# (Point(Coord { x: 85.26012714888755, y: 18.56196412476775 }), Point(Coord { x: 83.20492860368525, y: 20.928480933643275 }))

import re
import sys

import matplotlib.pyplot as plt


def main():
    filename = sys.argv[1]
    with open(filename, "r") as file:
        for line in file:
            points = re.findall(r"x: (\d+\.\d+), y: (\d+\.\d+)", line)
            if len(points) == 2:
                p1, p2 = points
                x1, y1 = map(float, p1)
                x2, y2 = map(float, p2)
                plt.plot([x1, x2], [y1, y2])
    plt.axis("equal")
    plt.show()


if __name__ == "__main__":
    main()
