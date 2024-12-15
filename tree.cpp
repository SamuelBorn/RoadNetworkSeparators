#include <cstdlib>
#include <fstream>
#include <iostream>
#include <queue>
#include <random>
#include <unordered_set>
#include <vector>

#include "global.hpp"
#include "graph_library.hpp"
#include "random_local.hpp"

Graph generate_local_tree(int n) {
    auto cumulative_weights = cumulative_distance_weights(n, distance_poly);
    Graph g(n);
    auto visited = std::unordered_set<int>(n);
    auto n1 = 0;
    visited.insert(n1);

    while (visited.size() < n) {
        auto n2 = sample_local_neighbor(n1, cumulative_weights);
        if (visited.find(n2) == visited.end()) { // n2 not visited
            visited.insert(n2);

            g[n1].push_back(n2);
            g[n2].push_back(n1);
        }
        n1 = n2;
    }

    return g;
}

Graph generate_random_tree(int n) {
    std::uniform_int_distribution<int> dist(0, n - 1);
    Graph g(n);
    auto visited = std::unordered_set<int>(n);
    auto n1 = 0;
    visited.insert(n1);

    while (visited.size() < n) {
        int n2 = dist(rng);
        if (visited.find(n2) == visited.end()) { // n2 not visited
            visited.insert(n2);

            g[n1].push_back(n2);
            g[n2].push_back(n1);
        }
        n1 = n2;
    }

    return g;
}

void print_diameter_overview(int max_size, int step_size, int runs) {
    std::ofstream("fragments/diameter_overview_local.txt") << "";
    for (int n = step_size; n <= max_size; n += step_size) {
        for (int i = 0; i < runs; i++) {
            // auto tree = generate_local_tree(n);
            auto tree = generate_random_tree(n);
            auto d = diameter(tree);
            std::cout << n << " " << d << std::endl;
            std::ofstream("fragments/diameter_overview.txt", std::ios::app)
                << n << " " << diameter << std::endl;
        }
    }
}
