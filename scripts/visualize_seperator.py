import matplotlib.pyplot as plt
import numpy as np
import sys

kit_color = "#009682"

experiment = sys.argv[1]

# Scatter plot data
x_values = []
y_values = []
with open(f"./output/{experiment}.txt", "r") as f:
    for line in f:
        parts = line.strip().split()
        x_values.append(int(parts[0]))
        y_values.append(int(parts[1]))
plt.scatter(x_values, y_values, color=kit_color)

# Plot cubic root function
x_fit = np.linspace(0, max(x_values), 500)
y_fit = np.cbrt(x_fit)
plt.plot(x_fit, y_fit, color="purple", label="cbrt(x)")
y_fit = np.cbrt(x_fit)

plt.title(experiment)
plt.xlabel("Number of nodes")
plt.ylabel("Size of separator")
plt.legend()
plt.grid(True, linestyle="--", alpha=0.6)

plt.savefig(f"output/{experiment}.pdf", format="pdf")
plt.show()
