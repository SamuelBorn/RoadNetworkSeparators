#!/bin/python3
import matplotlib.pyplot as plt
import sys
import re

def read_line_segments_from_file(filename):
    line_segments = []
    with open(filename, 'r') as file:
        for line in file:
            # Regular expression to extract two sets of coordinates
            match = re.findall(r"Coord\s*\{\s*x:\s*([-+]?\d*\.?\d+),\s*y:\s*([-+]?\d*\.?\d+)\s*\}", line)
            if len(match) == 2:  # Ensure we have two sets of coordinates
                point1 = (float(match[0][0]), float(match[0][1]))
                point2 = (float(match[1][0]), float(match[1][1]))
                line_segments.append((point1, point2))
    return line_segments

def visualize_line_segments(line_segments):
    plt.figure(figsize=(8, 6))
    for segment in line_segments:
        x_coords = [segment[0][0], segment[1][0]]
        y_coords = [segment[0][1], segment[1][1]]
        plt.plot(x_coords, y_coords, marker='o', label=f"({segment[0]}, {segment[1]})")

    plt.grid(True)
    plt.show()

if __name__ == "__main__":
    filename = sys.argv[1]
    line_segments = read_line_segments_from_file(filename)
    if line_segments:
        visualize_line_segments(line_segments)
    else:
        print("No valid line segments found in the file.")
