#pragma once

#include <iostream>
#include <map>
#include <set>
#include <unordered_set>
#include <vector>

typedef std::map<int, std::vector<int>> Graph;

void make_bidirectional(std::vector<int> &xadj, std::vector<int> &adjncy);

void expand_component_recurse(int node, int current_part, std::vector<int> &xadj,
                      std::vector<int> &adjncy, std::unordered_set<int> &sep,
                      std::vector<int> &part);

std::vector<int> partition_from_separator(std::vector<int> &xadj, std::vector<int> &adjncy,
                               std::unordered_set<int> &sep);

std::pair<std::vector<int>, std::vector<int>> get_adjacency_array(Graph g);

std::vector<std::pair<std::vector<int>, std::vector<int>>>
get_connected_components(std::vector<int> &xadj, std::vector<int> &adjncy,
              std::unordered_set<int> &sep);

