#pragma once

#include <iostream>
#include <map>
#include <set>
#include <unordered_set>
#include <vector>

void make_bidirectional(std::vector<int> &xadj, std::vector<int> &adjncy);

void expand_component_recursive(int node, int current_part,
                                std::vector<int> &xadj,
                                std::vector<int> &adjncy,
                                std::unordered_set<int> &sep,
                                std::vector<int> &part);

std::vector<int> partition_from_separator(std::vector<int> &xadj,
                                          std::vector<int> &adjncy,
                                          std::unordered_set<int> &sep);

std::pair<std::vector<int>, std::vector<int>>
get_adjacency_array(std::map<int, std::vector<int>> g);

std::vector<std::pair<std::vector<int>, std::vector<int>>>
get_subgraphs(std::vector<int> &xadj, std::vector<int> &adjncy,
              std::unordered_set<int> &sep);
