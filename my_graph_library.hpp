#pragma once

#include <iostream>
#include <map>
#include <unordered_set>
#include <vector>

typedef std::map<int, std::vector<int>> Graph;

void expand_partition(int node, int current_part, std::vector<int> &xadj,
                      std::vector<int> &adjncy, std::unordered_set<int> &sep,
                      std::vector<int> &part);

std::vector<int> get_partition(std::vector<int> &xadj, std::vector<int> &adjncy,
                               std::unordered_set<int> &sep);

std::pair<std::vector<int>, std::vector<int>> get_adjacency_array(Graph g);

std::vector<std::pair<std::vector<int>, std::vector<int>>>
get_subgraphs(std::vector<int> &xadj, std::vector<int> &adjncy,
              std::vector<int> &part);

void make_bidirectional(std::vector<int> &xadj, std::vector<int> &adjncy);

