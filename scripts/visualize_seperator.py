import matplotlib.pyplot as plt
import numpy as np

file_path = "./output/random_exp.txt"
x_values = []
y_values = []
with open(file_path, 'r') as file:
    for line in file:
        parts = line.strip().split()
        x_values.append(int(parts[0]))
        y_values.append(int(parts[1]))
plt.scatter(x_values, y_values, color='blue', label='Data Points', zorder=5)

# Plot cubic root function
x_fit = np.linspace(0, max(x_values), 500)
y_fit = np.cbrt(x_fit)
plt.plot(x_fit, y_fit, color='red')

# Plot settings
plt.grid(True, linestyle='--', alpha=0.6)
plt.show()
