#include <algorithm>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <set>
#include <unordered_set>
#include <vector>

#include "kaHIP_interface.h"

void make_bidirectional(std::vector<int> &xadj, std::vector<int> &adjncy) {
    std::vector<std::set<int>> adjncy_new(xadj.size() - 1);

    for (std::size_t i = 0; i < xadj.size() - 1; i++) {
        for (int j = xadj[i]; j < xadj[i + 1]; j++) {
            adjncy_new[i].insert(adjncy[j]);
            adjncy_new[adjncy[j]].insert(i);
        }
    }

    auto n = xadj.size() - 1;
    xadj.clear();
    adjncy.clear();

    for (size_t i = 0; i < n; i++) {
        xadj.push_back(adjncy.size());
        for (auto x : adjncy_new[i]) {
            adjncy.push_back(x);
        }
    }
    xadj.push_back(adjncy.size());
}

void expand_component(int init_node, std::vector<int> &xadj,
                      std::vector<int> &adjncy, std::unordered_set<int> &sep,
                      std::vector<int> &part) {
    std::vector<int> stack;
    stack.push_back(init_node);

    while (!stack.empty()) {
        auto current_node = stack.back();
        stack.pop_back();

        part[current_node] = init_node;

        for (int i = xadj[current_node]; i < xadj[current_node + 1]; i++) {
            auto other_node = adjncy[i];
            if (part[other_node] == -1 && sep.find(other_node) == sep.end()) {
                stack.push_back(other_node);
            }
        }
    }
}

std::vector<int> partition_from_separator(std::vector<int> &xadj,
                                          std::vector<int> &adjncy,
                                          std::unordered_set<int> &sep) {
    std::vector<int> part(xadj.size() - 1, -1);

    for (std::size_t i = 0; i < xadj.size() - 1; i++) {
        if (part[i] == -1 && sep.find(i) == sep.end()) {
            expand_component(i, xadj, adjncy, sep, part);
        }
    }

    return part;
}

std::pair<std::vector<int>, std::vector<int>>
get_adjacency_array(std::map<int, std::vector<int>> &g) {
    std::map<int, int> mapping;
    for (auto &[from_node, _] : g) {
        mapping.insert({from_node, mapping.size()});
    }

    std::vector<int> xadj;
    std::vector<int> adjncy;

    for (auto &[from_node, to_nodes] : g) {
        xadj.push_back(adjncy.size());
        for (auto n : to_nodes) {
            adjncy.push_back(mapping[n]);
        }
    }

    xadj.push_back(adjncy.size());

    return {xadj, adjncy};
}

std::pair<std::vector<int>, std::vector<int>>
get_adjacency_array(std::vector<std::vector<int>> &g) {
    auto xadj = std::vector<int>(g.size() + 1);
    auto adjncy = std::vector<int>();

    for (std::size_t i = 0; i < g.size(); i++) {
        xadj[i] = adjncy.size();
        for (auto n : g[i]) {
            adjncy.push_back(n);
        }
    }
    xadj[g.size()] = adjncy.size();

    return {xadj, adjncy};
}

std::vector<std::pair<std::vector<int>, std::vector<int>>>
get_subgraphs(std::vector<int> &xadj, std::vector<int> &adjncy,
              std::unordered_set<int> &sep) {
    std::vector<int> part = partition_from_separator(xadj, adjncy, sep);

    std::map<int, std::map<int, std::vector<int>>> subgraphs;

    for (std::size_t i = 0; i < xadj.size() - 1; i++) {
        if (part[i] == -1)
            continue;
        subgraphs.insert({part[i], {}}); // does not overwrite

        for (int j = xadj[i]; j < xadj[i + 1]; j++) {
            if (part[i] == part[adjncy[j]]) {
                subgraphs[part[i]][i].push_back(adjncy[j]);
            }
        }
    }

    std::vector<std::pair<std::vector<int>, std::vector<int>>> result;
    for (auto &subgraph : subgraphs) {
        result.push_back(get_adjacency_array(subgraph.second));
    }

    return result;
}

void recurse_seperators(std::vector<int> &xadj, std::vector<int> &adjncy) {
    auto n = (int)xadj.size() - 1;
    auto nparts = 2;
    auto imbalance = 1.0 / 3.0;
    auto num_separator_vertices = 0;
    auto separator_raw = new int[n];

    node_separator(&n, nullptr, xadj.data(), nullptr, adjncy.data(), &nparts,
                   &imbalance, false, 0, FAST, &num_separator_vertices,
                   &separator_raw);

    std::cout << n << "\t" << num_separator_vertices << std::endl;
    // std::ofstream("output/same_degree.txt", std::ios::app)
    //     << n << " " << num_separator_vertices << std::endl;

    auto separator = std::unordered_set<int>(
        separator_raw, separator_raw + num_separator_vertices);
    auto subgraphs = get_subgraphs(xadj, adjncy, separator);

    for (auto &[s_xadj, s_adjncy] : subgraphs) {
        if (s_xadj.size() > 200) {
            recurse_seperators(s_xadj, s_adjncy);
        }
    }
}

void print_degree_distribution(std::vector<int> &xadj,
                               std::vector<int> &adjncy) {
    auto degrees = std::vector<int>(20, 0);
    for (size_t i = 0; i < xadj.size() - 1; i++) {
        auto degree = xadj[i + 1] - xadj[i];
        if (degree < 20)
            degrees[degree]++;
    }

    for (auto e : degrees) {
        std::cout << std::fixed << std::setprecision(2)
                  << double(e) / (xadj.size() - 1) << std::endl;
    }

    // results
    // 0.00
    // 0.22
    // 0.15
    // 0.55
    // 0.08
}

bool has_edge(std::vector<std::vector<int>> &g, int from, int to) {
    return std::find(g[from].begin(), g[from].end(), to) != g[from].end();
}
