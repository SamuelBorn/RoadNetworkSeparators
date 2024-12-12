#pragma once

#include <cstdlib>
#include <iostream>
#include <queue>
#include <random>
#include <unordered_set>
#include <vector>

#include "global.hpp"

Graph generate_local_tree(int n);
Graph generate_random_tree(int n);
std::pair<int, int> find_farthest_node(int start, Graph &tree);
int get_diameter(Graph &tree);
void print_diameter_overview(int max_size = 100000, int step_size = 1000,
                             int runs = 5);
