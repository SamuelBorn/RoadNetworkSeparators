#pragma once

#include <algorithm>
#include <unordered_set>
#include <vector>

#include "global.hpp"
#include "my_graph_library.hpp"
#include "tree.hpp"

bool is_as_expected(std::vector<std::vector<int>> &actual_degrees,
                    std::vector<int> &expected_degrees) {
    for (size_t i = 0; i < actual_degrees.size(); i++) {
        if (actual_degrees[i].size() < expected_degrees[i]) {
            return false;
        }
    }
    return true;
}

// returns: (element, (index vector, index in vector))
// small bias towards nodes that are have a degree that does not occur often
std::pair<int, std::pair<int, int>>
random_node(std::vector<std::vector<int>> &actual_degrees,
            std::vector<int> &expected_degrees) {
    auto tmp = std::vector<std::pair<int, int>>();
    for (size_t i = 0; i < actual_degrees.size() - 1; i++) {
        if (actual_degrees[i + 1].size() < expected_degrees[i + 1]) {
            std::uniform_int_distribution<int> dist(
                0, actual_degrees[i].size() - 1);
            tmp.push_back({i, dist(rng)});
        }
    }
    std::uniform_int_distribution<int> dist(0, tmp.size() - 1);
    auto idx = dist(rng);
    return {actual_degrees[tmp[idx].first][tmp[idx].second], tmp[idx]};
}

std::pair<std::vector<int>, std::vector<int>>
same_degree_graph(int n, std::vector<double> degree_percentages) {
    auto g = generate_random_tree(n);

    auto expected = std::vector<int>();
    for (auto e : degree_percentages)
        expected.push_back(std::floor(0.95 * e * n));

    auto actual = std::vector<std::vector<int>>(expected.size());
    for (int i = 0; i < n; i++) {
        auto degree = g[i].size();
        if (degree < expected.size()) {
            actual[degree].push_back(i);
        }
    }

    while (!is_as_expected(actual, expected)) {
        auto [n1, idx1] = random_node(actual, expected);
        auto [n2, idx2] = random_node(actual, expected);
        if (n1 != n2 && !has_edge(g, n1, n2)) {
            g[n1].push_back(n2);
            g[n2].push_back(n1);
            actual[idx1.first].erase(actual[idx1.first].begin() + idx1.second);
            actual[idx2.first].erase(actual[idx2.first].begin() + idx2.second);
            actual[idx1.first + 1].push_back(n1);
            actual[idx2.first + 1].push_back(n2);
        }
    }

    return get_adjacency_array(g);
}
