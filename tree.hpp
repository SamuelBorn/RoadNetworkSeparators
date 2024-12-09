#pragma once

#include <cstdlib>
#include <iostream>
#include <random>
#include <unordered_set>
#include <vector>

std::vector<std::vector<int>> generate_random_tree(int n, int seed) {
    std::mt19937 rng(seed);
    std::uniform_int_distribution<int> dist(0, n - 1);

    std::vector<std::vector<int>> g(n);

    auto not_visited = std::unordered_set<int>(n);
    auto visited = std::unordered_set<int>(n);
    for (size_t i = 1; i < n; i++)
        not_visited.insert(i);
    auto n1 = 0;
    visited.insert(n1);

    while (!not_visited.empty()) {
        int n2 = dist(rng);
        if (not_visited.find(n2) != not_visited.end()) {
            not_visited.erase(n2);
            visited.insert(n2);

            g[n1].push_back(n2);
            g[n2].push_back(n1);
        }
        n1 = n2;
    }

    // // print graph
    // for (size_t i = 0; i < n; i++) {
    //     std::cout << i << ": ";
    //     for (auto x : g[i]) {
    //         std::cout << x << " ";
    //     }
    //     std::cout << std::endl;
    // }

    return g;
}
