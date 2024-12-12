#pragma once

#include <cstdlib>
#include <iostream>
#include <queue>
#include <random>
#include <unordered_set>
#include <vector>

// Function to generate a random tree with n nodes
std::vector<std::vector<int>> generate_random_tree(int n);

// Function to find the farthest node and its distance from the given starting
// node in a tree
std::pair<int, int> find_farthest_node(int start,
                                       std::vector<std::vector<int>> &tree);

// Function to calculate the diameter of a tree
int get_diameter(std::vector<std::vector<int>> &tree);

// Function to analyze and display the diameter of trees for different sizes
void diameter_overview(int max_size = 100000, int step_size = 1000, int runs = 5);


std::vector<std::vector<int>> generate_local_tree(int n);
