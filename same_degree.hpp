#pragma once

#include <algorithm>
#include <unordered_set>
#include <vector>

#include "global.hpp"
#include "graph_library.hpp"
#include "tree.hpp"

bool is_as_expected(Graph &actual_degrees, std::vector<int> &expected_degrees);

std::pair<int, std::pair<int, int>>
random_node(Graph &actual_degrees, std::vector<int> &expected_degrees);

std::pair<std::vector<int>, std::vector<int>>
same_degree_graph(int n, std::vector<double> degree_percentages);
