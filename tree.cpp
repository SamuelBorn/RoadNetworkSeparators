#include <cstdlib>
#include <fstream>
#include <iostream>
#include <queue>
#include <random>
#include <unordered_set>
#include <vector>

#include "global.hpp"
#include "random_local.hpp"

std::vector<std::vector<int>> generate_local_tree(int n) {
    auto cumulative_weights = cumulative_distance_weights(n, distance_poly);

    std::vector<std::vector<int>> g(n);
    auto visited = std::unordered_set<int>(n);

    auto n1 = 0;
    visited.insert(n1);

    while (visited.size() < n) {
        int n2 = sample_local_neighbor(n1, cumulative_weights);
        if (visited.find(n2) == visited.end()) { // n2 not visited
            visited.insert(n2);
            g[n1].push_back(n2);
            g[n2].push_back(n1);
            std::cout << visited.size() << std::endl;
        }
        n1 = n2;
    }

    return g;
}

std::vector<std::vector<int>> generate_random_tree(int n) {
    std::uniform_int_distribution<int> dist(0, n - 1);

    std::vector<std::vector<int>> g(n);

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

// only works for trees - return (farthest node, distance)
std::pair<int, int> find_farthest_node(int start,
                                       std::vector<std::vector<int>> &tree) {
    std::vector<int> distances(tree.size() - 1, -1);
    distances[start] = 0;

    std::queue<int> q;
    q.push(start);

    auto farthest = start;
    auto max_distance = 0;

    while (!q.empty()) {
        auto u = q.front();
        q.pop();
        for (auto v : tree[u]) {
            if (distances[v] == -1) { // Unvisited
                distances[v] = distances[u] + 1;
                q.push(v);

                // Update farthest node
                if (distances[v] > max_distance) {
                    max_distance = distances[v];
                    farthest = v;
                }
            }
        }
    }

    return {farthest, max_distance};
}

// only works for trees
int get_diameter(std::vector<std::vector<int>> &tree) {
    auto [farthest, _] = find_farthest_node(0, tree);
    auto [farthest2, diameter] = find_farthest_node(farthest, tree);
    return diameter;
}

void diameter_overview(int max_size, int step_size, int runs) {
    std::ofstream("fragments/diameter_overview.txt") << "";
    for (int n = step_size; n <= max_size; n += step_size) {
        for (int i = 0; i < runs; i++) {
            auto tree = generate_random_tree(n);
            auto diameter = get_diameter(tree);
            std::cout << n << "," << diameter << std::endl;
            std::ofstream("fragments/diameter_overview.txt", std::ios::app)
                << n << " " << diameter << std::endl;
        }
    }
}
