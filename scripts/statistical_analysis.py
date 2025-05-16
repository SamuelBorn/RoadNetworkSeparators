#!/bin/python3
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import sys
from scipy.optimize import curve_fit
from scipy.stats import binned_statistic, linregress
from sklearn.metrics import r2_score


def load_data(file: Path):
    data = np.loadtxt(file)
    np.sort(data, axis=0)
    # data = data[(data[:, 0] > 0) & (data[:, 0] < 10_000_000)]
    # data = data[(data[:, 0] > 2**8) & (data[:, 0] < 2**18)]
    data = data[(data[:, 0] > 2**8)]
    return data[:, 0], data[:, 1]


def bin_data(x, y, num_bins):
    bin_edges = np.linspace(0, max(x), num=num_bins)
    means, bin_edges, _ = binned_statistic(x, y, statistic="mean", bins=bin_edges)
    bin_centers = (bin_edges[:-1] + bin_edges[1:]) / 2
    valid = ~np.isnan(means)
    bin_centers = bin_centers[valid]
    means = means[valid]
    return bin_centers, means


def sqrt_model(x, a, b):
    return a * np.sqrt(x)


def sqrt_model_no_intercept(x, a):
    return a * np.sqrt(x)


def cbrt_model(x, a, b):
    return a * np.cbrt(x) + b


def cbrt_model_no_intercept(x, a):
    return a * np.cbrt(x)


def poly_model(x, a, b, c):
    return a * np.power(x, b) + c


def poly_model_no_intercept(x, a, b):
    return a * np.power(x, b)


def apply_log_log_transformation(x, y):
    return np.log2(x), np.log2(y)


def apply_cbrt_transformation(x, y):
    return x, np.power(y, 3)


def apply_sqrt_transformation(x, y):
    return x, np.power(y, 2)


def fit_line(x, y):
    slope, intercept, r_value, p_value, std_err = linregress(x, y)
    print(f"Fitted Line: y = {slope:.4f}x + {intercept:.4f}")
    print(f"R² Score: {r_value:.4f} (closer to 1 is better)")
    print(f"P-value: {p_value} (for slope, closer to 0 is better)")


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

    params, _ = curve_fit(cbrt_model_no_intercept, x, y, p0=[0.5])
    y_pred = cbrt_model_no_intercept(x, *params)
    r2 = r2_score(y, y_pred)
    print(f"{params[0]:.4f} * cbrt(x)   R²: {r2:.4f}")

    params, _ = curve_fit(sqrt_model_no_intercept, x, y, p0=[0.5])
    y_pred = sqrt_model_no_intercept(x, *params)
    r2 = r2_score(y, y_pred)
    print(f"{params[0]:.4f} * sqrt(x)   R²: {r2:.4f}")


def analyze_data(x, y):
    print("\nLog-Log Transformation")
    fit_line(*apply_log_log_transformation(x, y))

    print("\nNormal Fit")
    fit_curve(x, y)

    # print("\nAdjusted pow 3")
    # fit_line(x, np.power(y, 3))
    #
    # print("\nAdjusted pow 2")
    # fit_line(x, np.power(y, 2))


def main():
    # x, y = load_data(Path("./output/sep/Europe"))
    x,y = load_data(sys.argv[1])
    analyze_data(x, y)

    # x, y = apply_log_log_transformation(x, y)
    # x, y = bin_data(x, y, 30)
    # fit_line(x, y)





if __name__ == "__main__":
    main()
