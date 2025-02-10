from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy.optimize import curve_fit
from scipy.stats import linregress
from sklearn.metrics import r2_score


def load_data(file: Path):
    data = np.loadtxt(file)
    np.sort(data, axis=0)
    return data[:, 0], data[:, 1]


def sliding_window(x, y, window_size):
    x_mean = []
    y_mean = []
    for i in range(len(x)):
        x_mean.append(np.mean(x[i : i + window_size]))
        y_mean.append(np.mean(y[i : i + window_size]))
    return x_mean, y_mean


def bin_data(x, y, num_bins):
    data = np.column_stack((x, y))
    bins = np.linspace(np.min(x), np.max(x) + 1, num_bins + 1)
    bin_indices = np.digitize(x, bins) - 1
    binned_means = []
    for i in range(num_bins):
        mask = bin_indices == i
        if np.any(mask):
            mean_values = data[mask].mean(axis=0)
            binned_means.append(mean_values)

    binned_means = np.array(binned_means)
    return binned_means[:, 0], binned_means[:, 1]


def sqrt_model(x, a, b):
    return a * np.sqrt(x)


def cbrt_model(x, a, b):
    return a * np.cbrt(x) + b


def poly_model(x, a, b, c):
    return a * np.power(x, b) + c


def apply_log_log_transformation(x, y):
    return np.log(x), np.log(y)


def fit_line(x, y):
    slope, intercept, r_value, p_value, std_err = linregress(x, y)
    print(f"Fitted Line: y = {slope:.4f}x + {intercept:.4f}")
    print(f"R² Score: {r_value:.4f} (closer to 1 is better)")
    print(f"P-value: {p_value:.4e} (for slope, closer to 0 is better)")
    print(f"Standard Error: {std_err:.4f}\n")


def fit_curve(x, y):
    params, _ = curve_fit(cbrt_model, x, y, p0=[0.5, 0.5])
    y_pred = cbrt_model(x, *params)
    r2 = r2_score(y, y_pred)
    print(f"{params[0]:.4f} * cbrt(x) + {params[1]:.4f}   R²: {r2:.4f}")

    params, _ = curve_fit(sqrt_model, x, y, p0=[0.5, 0.5])
    y_pred = sqrt_model(x, *params)
    r2 = r2_score(y, y_pred)
    print(f"{params[0]:.4f} * sqrt(x) + {params[1]:.4f}   R²: {r2:.4f}")

    params, _ = curve_fit(poly_model, x, y, p0=[0.5, 0.5, 0.5])
    y_pred = poly_model(x, *params)
    r2 = r2_score(y, y_pred)
    print(f"{params[0]:.4f} * x^{params[1]:.4f} + {params[2]:.4f}   R²: {r2:.4f}\n")


def main():
    x, y = load_data(Path("output/sep_germany.txt"))
    # fit_line(*apply_log_log_transformation(x, y))
    # fit_curve(x, y)
    fit_line(x, np.power(y, 3))
    fit_line(x, np.power(y, 2))
    plt.scatter(x, y**2, label="y^3")
    plt.show()

    # x,y = load_data(Path("output/sep_germany.txt"))
    # fit_line(*apply_log_log_transformation(x, y))
    # fit_curve(x, y)
    #
    # x,y = load_data(Path("output/sep_karlsruhe_ifc.txt"))
    # fit_line(*apply_log_log_transformation(x, y))
    # fit_curve(x, y)


if __name__ == "__main__":
    main()
