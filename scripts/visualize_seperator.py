import matplotlib.pyplot as plt
import numpy as np

experiment = "karlsruhe"

# Scatter plot data
x_values = []
y_values = []
with open(f"./fragments/{experiment}.txt", 'r') as f:
    for line in f:
        parts = line.strip().split()
        x_values.append(int(parts[0]))
        y_values.append(int(parts[1]))
plt.scatter(x_values, y_values, color='blue')

# Plot cubic root function
x_fit = np.linspace(0, max(x_values), 500)
y_fit = np.cbrt(x_fit)
plt.plot(x_fit, y_fit, color='red', label='cbrt(x)')
# y_fit = 3 * np.sqrt(x_fit)
# plt.plot(x_fit, y_fit, color='red', label='3 * sqrt(x)')

plt.title(experiment)
plt.legend()
plt.grid(True, linestyle='--', alpha=0.6)
plt.savefig(f"fragments/{experiment}.svg", format='svg')
plt.show()
