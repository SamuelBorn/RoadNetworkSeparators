#pragma once

#include <iostream>
#include <map>
#include <set>
#include <unordered_set>
#include <utility>
#include <vector>

void make_bidirectional(std::vector<int> &xadj, std::vector<int> &adjncy);

void expand_component(int init_node, std::vector<int> &xadj,
                      std::vector<int> &adjncy, std::unordered_set<int> &sep,
                      std::vector<int> &part);

std::vector<int> partition_from_separator(std::vector<int> &xadj,
                                          std::vector<int> &adjncy,
                                          std::unordered_set<int> &sep);

std::pair<std::vector<int>, std::vector<int>>
get_adjacency_array(std::map<int, std::vector<int>> &g);

std::pair<std::vector<int>, std::vector<int>>
get_adjacency_array(std::vector<std::vector<int>> &g);

std::vector<std::pair<std::vector<int>, std::vector<int>>>
get_subgraphs(std::vector<int> &xadj, std::vector<int> &adjncy,
              std::unordered_set<int> &sep);

void print_degree_distribution(std::vector<int> &xadj,
                               std::vector<int> &adjncy);

bool has_edge(std::vector<std::vector<int>> &g, int from, int to);

void recurse_seperators(std::vector<int> &xadj, std::vector<int> &adjncy);