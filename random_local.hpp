#pragma once

#include <algorithm>
#include <cmath>
#include <cstdlib>
#include <random>
#include <vector>

#include "global.hpp"
#include "graph_library.hpp"
#include "tree.hpp"

double distance_linear(int u, int v, int n);
double distance_poly(int u, int v, int n);
double distance_exp(int u, int v, int n);
int sample_distribution(std::vector<double> &cumulative_weights);
int sample_local_neighbor(int u, std::vector<double> &cumulative_weights);
std::vector<double>
cumulative_distance_weights(int n, double (*distance)(int, int, int));
Graph random_local_graph(int n, int m, double (*distance)(int, int, int));
