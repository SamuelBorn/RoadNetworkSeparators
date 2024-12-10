#pragma once

#include <algorithm>
#include <unordered_set>
#include <vector>
#include <cmath>
#include <random>
#include <utility>

#include "global.hpp"
#include "graph_library.hpp"
#include "tree.hpp"

// Checks if the actual degrees match the expected degrees
bool is_as_expected(std::vector<std::vector<int>> &actual_degrees,
                    std::vector<int> &expected_degrees);

// Returns: (element, (index vector, index in vector))
// Small bias towards nodes that have a degree that does not occur often
std::pair<int, std::pair<int, int>>
random_node(std::vector<std::vector<int>> &actual_degrees,
            std::vector<int> &expected_degrees);

// Generates a graph with the same degree distribution as given percentages
std::pair<std::vector<int>, std::vector<int>>
same_degree_graph(int n, std::vector<double> degree_percentages);

