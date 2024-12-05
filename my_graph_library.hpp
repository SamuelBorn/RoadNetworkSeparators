#pragma once

#include <unordered_set>
#include <vector>

void expand_partition(int node, int current_part, std::vector<int> &xadj,
                      std::vector<int> &adjncy, std::unordered_set<int> &sep,
                      std::vector<int> &part);

std::vector<int> get_partition(std::vector<int> &xadj, std::vector<int> &adjncy,
                               std::unordered_set<int> &sep);
