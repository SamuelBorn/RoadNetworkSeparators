#pragma once

#include <algorithm>
#include <unordered_set>
#include <vector>

#include "global.hpp"
#include "my_graph_library.hpp"
#include "tree.hpp"

int last_non_fulfilled_degree(std::vector<std::vector<int>> &actual_degrees,
                              std::vector<int> &expected_degrees) {
    for (int i = expected_degrees.size() - 1; i >= 0; i--) {
        if (actual_degrees[i].size() < expected_degrees[i]) {
            return i;
        }
    }
    return -1;
}

// returns: (element, (index vector, index in vector))
std::pair<int, std::pair<int, int>>
random_node(std::vector<std::vector<int>> &actual_degrees, int last_idx) {
    auto total = 0;
    for (size_t i = 0; i < last_idx; i++) {
        total += actual_degrees[i].size();
    }

    std::uniform_int_distribution<int> int_dist(0, total - 1);
    auto random = int_dist(rng);

    auto idx = 0;
    for (auto e : actual_degrees) {
        if (random < e.size()) {
            return {e[random], {idx, random}};
        }
        random -= e.size();
        idx++;
    }
    return {-1, {-1, -1}};
}

std::pair<std::vector<int>, std::vector<int>>
random_same_degree_graph(int n, std::vector<double> degree_percentages) {
    auto g = generate_random_tree(n);

    auto expected_degrees = std::vector<int>();
    for (auto e : degree_percentages) {
        expected_degrees.push_back(std::floor(0.9 * e * n));
    }

    auto actual_degrees =
        std::vector<std::vector<int>>(expected_degrees.size());
    for (int i = 0; i < n; i++) {
        auto degree = g[i].size();
        if (degree < expected_degrees.size())
            actual_degrees[degree].push_back(i);
    }

    std::cout << "acutal" << std::endl;
    for (auto e : actual_degrees) {
        std::cout << e.size() << " ";
    }
    std::cout << std::endl;
    std::cout << "expected" << std::endl;
    for (auto e : expected_degrees) {
        std::cout << e << " ";
    }
    std::cout << std::endl;

    auto last_idx = last_non_fulfilled_degree(actual_degrees, expected_degrees);

    std::cout << last_idx << std::endl;

    while (last_idx != -1) {
        std::cout << "before random" << std::endl;
        std::cout << last_idx << std::endl;
        std::cout << "acutal ";
        for (auto e : actual_degrees) {
            std::cout << e.size() << " ";
        }
        std::cout  << std::endl;
        auto [n1, idx1] = random_node(actual_degrees, last_idx);
        auto [n2, idx2] = random_node(actual_degrees, last_idx);
        if (n1 != n2 && !has_edge(g, n1, n2)) {
            g[n1].push_back(n2);
            g[n2].push_back(n1);
            actual_degrees[idx1.first].erase(
                actual_degrees[idx1.first].begin() + idx1.second);
            actual_degrees[idx2.first].erase(
                actual_degrees[idx2.first].begin() + idx2.second);
            actual_degrees[idx1.first + 1].push_back(n1);
            actual_degrees[idx2.first + 1].push_back(n2);
            last_idx =
                last_non_fulfilled_degree(actual_degrees, expected_degrees);
        }
        // break;
    }

    std::cout << "acutal" << std::endl;
    for (auto e : actual_degrees) {
        std::cout << e.size() << " ";
    }
    std::cout << std::endl;
    std::cout << "expected" << std::endl;
    for (auto e : expected_degrees) {
        std::cout << e << " ";
    }
    std::cout << std::endl;

    return get_adjacency_array(g);
}
