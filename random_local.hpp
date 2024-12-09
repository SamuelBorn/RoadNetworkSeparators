#pragma once

#include <algorithm>
#include <cmath>
#include <cstdlib>
#include <random>
#include <utility>
#include <vector>

#include "my_graph_library.hpp"
#include "tree.hpp"

int dist_linear(int u, int v, int n);
int dist_quadratic(int u, int v, int n);
int dist_exp(int u, int v, int n);

std::pair<int, int> get_random_edge(int n,
                                    std::vector<double> &cumulative_weights);

std::vector<double> get_cumulative_weights(int n,
                                           int (*distance)(int, int, int));

std::pair<std::vector<int>, std::vector<int>>
random_local_graph(int n, int m, int (*dist)(int, int, int));
