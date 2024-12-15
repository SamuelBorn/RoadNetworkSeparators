#pragma once

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

Graph generate_local_tree(int n);
Graph generate_random_tree(int n);
void print_diameter_overview(int max_size = 1000, int step_size = 100,
                             int runs = 5);
