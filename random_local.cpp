#include <algorithm>
#include <cmath>
#include <cstdlib>
#include <random>
#include <vector>

#include "global.hpp"
#include "graph_library.hpp"
#include "tree.hpp"

int dist_linear(int u, int v, int n) {
    auto direct = std::abs(u - v);
    auto wrapped = n - direct;
    return std::min(direct, wrapped);
}

int dist_quadratic(int u, int v, int n) {
    return std::pow(dist_linear(u, v, n), 2);
}

int dist_exp(int u, int v, int n) { return std::pow(2, dist_linear(u, v, n)); }

std::pair<int, int> get_random_edge(int n,
                                    std::vector<double> &cumulative_weights) {
    std::uniform_int_distribution<int> int_dist(0, n - 1);
    std::uniform_real_distribution<double> double_dist(0, 1);
    auto u = int_dist(rng);
    auto random = double_dist(rng);
    auto it = std::lower_bound(cumulative_weights.begin(),
                               cumulative_weights.end(), random);
    auto offset = std::distance(cumulative_weights.begin(), it);
    return {u, (u + offset) % n};
}

std::vector<double> get_cumulative_weights(int n,
                                           int (*distance)(int, int, int)) {
    auto weights = std::vector<double>(n);
    weights[0] = 0;
    for (size_t i = 1; i < n; i++) {
        auto dist = distance(0, i, n);
        weights[i] = weights[i - 1] + 1.0 / dist;
    }
    for (size_t i = 0; i < n; i++) {
        weights[i] = weights[i] / weights[n - 1];
    }
    return weights;
}

std::pair<std::vector<int>, std::vector<int>>
random_local_graph(int n, int m, int (*dist)(int, int, int)) {
    auto g = generate_random_tree(n);
    auto weights = get_cumulative_weights(n, dist);

    auto remaining = m - n + 1;
    while (remaining > 0) {
        auto [u, v] = get_random_edge(n, weights);

        if (std::find(g[u].begin(), g[u].end(), v) == g[u].end()) {
            g[u].push_back(v);
            g[v].push_back(u);
            remaining--;
        }
    }

    return get_adjacency_array(g);
}
