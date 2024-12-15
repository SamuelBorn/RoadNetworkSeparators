#pragma once

#include <algorithm>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <queue>
#include <unordered_set>
#include <vector>

#include "global.hpp"
#include "kaHIP_interface.h"

Graph get_graph(std::vector<int> &xadj, std::vector<int> &adjncy);

void make_bidirectional(Graph &g);

void expand_component(int init_node, Graph &g, std::unordered_set<int> &sep,
                      std::vector<int> &part);

std::vector<int> partition_from_separator(Graph &g,
                                          std::unordered_set<int> &sep);

Graph get_graph(std::map<int, std::vector<int>> &g_map);

std::pair<std::vector<int>, std::vector<int>> get_adjacency_array(Graph &g);

std::vector<Graph> get_subgraphs(Graph &g, std::unordered_set<int> &sep);

void recurse_seperators(Graph &g);

void print_degree_distribution(Graph &g);

bool has_edge(Graph &g, int from, int to);

void graph_to_file(Graph &g, std::string filename);

std::vector<double> bfs(Graph &g, int start);

std::pair<int, int> farthest_node(int start, Graph &tree);

int diameter(Graph &tree);
