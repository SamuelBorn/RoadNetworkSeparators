#include <algorithm>
#include <cmath>
#include <cstdlib>
#include <random>
#include <vector>

#include "global.hpp"
#include "graph_library.hpp"
#include "tree.hpp"

double distance_linear(int u, int v, int n) {
    auto direct = std::abs(u - v);
    auto wrapped = n - direct;
    return std::min(direct, wrapped);
}

double distance_poly(int u, int v, int n) {
    return std::pow(distance_linear(u, v, n), 2);
}

double distance_exp(int u, int v, int n) {
    return std::pow(2, distance_linear(u, v, n));
}

int sample_distribution(std::vector<double> &cumulative_weights) {
    auto uniform = std::uniform_real_distribution<double>(0, 1);
    auto random = uniform(rng);
    auto it = std::lower_bound(cumulative_weights.begin(),
                               cumulative_weights.end(), random);
    return std::distance(cumulative_weights.begin(), it);
}

int sample_local_neighbor(int u, std::vector<double> &cumulative_weights) {
    auto offset = sample_distribution(cumulative_weights);
    return (u + offset) % cumulative_weights.size();
}

std::vector<double>
cumulative_distance_weights(int n, double (*distance)(int, int, int)) {
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

Graph random_local_graph(int n, int m, double (*distance)(int, int, int)) {
    auto g = generate_random_tree(n);
    auto weights = cumulative_distance_weights(n, distance);
    auto uniform = std::uniform_int_distribution<int>(0, n - 1);

    auto remaining = m - n + 1;
    while (remaining > 0) {
        auto u = uniform(rng);
        auto v = sample_local_neighbor(u, weights);

        if (std::find(g[u].begin(), g[u].end(), v) == g[u].end()) {
            g[u].push_back(v);
            g[v].push_back(u);
            remaining--;
        }
    }

    return g;
}

Graph random_local_graph_tree_distance_hard_limit(int n, int m) {
    auto g = generate_random_tree(n);

    auto remaining = m - n + 1;
    while (remaining > 0) {
        std::cout << remaining << std::endl;
        auto u = std::uniform_int_distribution<int>(0, n - 1)(rng);

        auto map = bfs_partial(g, u, 500);
        auto ids = std::vector<int>();
        auto weights = std::vector<double>();
        ids.reserve(map.size());
        weights.reserve(map.size());

        for (auto [node, d] : map) {
            ids.push_back(node);
            if (d == 0) {
                weights.push_back(0);
            } else {
                weights.push_back(1.0 / std::pow(d, 3));
            }
        }

        auto dist =
            std::discrete_distribution<int>(weights.begin(), weights.end());
        auto idx = dist(rng);
        auto v = ids[idx];

        if (!has_edge(g, u, v)) {
            g[u].push_back(v);
            g[v].push_back(u);
            remaining--;
        }
    }

    return g;
}

Graph random_local_graph_tree_distance(int n, int m) {
    auto g = generate_random_tree(n);

    auto remaining = m - n + 1;
    while (remaining > 0) {
        std::cout << remaining << std::endl;
        auto u = std::uniform_int_distribution<int>(0, n - 1)(rng);

        auto distances = bfs(g, u);
        distances[u] = 1;
        for (size_t i = 0; i < distances.size(); i++) {
            distances[i] = 1 / std::pow(distances[i], 3.6);
        }
        distances[u] = 0;

        auto dist =
            std::discrete_distribution<int>(distances.begin(), distances.end());
        auto v = dist(rng);

        if (!has_edge(g, u, v)) {
            g[u].push_back(v);
            g[v].push_back(u);
            remaining--;
        }
    }

    return g;
}
