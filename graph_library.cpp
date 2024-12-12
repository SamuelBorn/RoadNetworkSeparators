#include <algorithm>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <unordered_set>
#include <vector>

#include "global.hpp"
#include "kaHIP_interface.h"

Graph get_graph(std::vector<int> &xadj, std::vector<int> &adjncy) {
    Graph g(xadj.size() - 1);
    for (size_t i = 0; i < xadj.size() - 1; i++) {
        for (int j = xadj[i]; j < xadj[i + 1]; j++) {
            g[i].push_back(adjncy[j]);
        }
    }
    return g;
}

void make_bidirectional(Graph &g) {
    for (size_t i = 0; i < g.size(); i++) {
        for (size_t j = 0; j < g[i].size(); j++) {
            auto to = g[i][j];
            if (std::find(g[to].begin(), g[to].end(), i) == g[to].end()) {
                g[to].push_back(i);
            }
        }
    }
}

void expand_component(int init_node, Graph &g, std::unordered_set<int> &sep,
                      std::vector<int> &part) {
    std::vector<int> stack;
    stack.push_back(init_node);

    while (!stack.empty()) {
        auto current = stack.back();
        stack.pop_back();
        part[current] = init_node;

        for (auto n : g[current]) {
            if (part[n] == -1 && sep.find(n) == sep.end()) {
                stack.push_back(n);
            }
        }
    }
}

std::vector<int> partition_from_separator(Graph &g,
                                          std::unordered_set<int> &sep) {
    std::vector<int> part(g.size(), -1);

    for (std::size_t i = 0; i < g.size(); i++) {
        if (part[i] == -1 && sep.find(i) == sep.end()) {
            expand_component(i, g, sep, part);
        }
    }

    return part;
}

Graph get_graph(std::map<int, std::vector<int>> &g_map) {
    std::map<int, int> mapping;
    Graph g(g_map.size());
    for (auto &[from, to_nodes] : g_map) {
        mapping.insert({from, mapping.size()});
        for (auto to : to_nodes) {
            mapping.insert({to, mapping.size()});
            g[mapping[from]].push_back(mapping[to]);
        }
    }
    return g;
}

std::pair<std::vector<int>, std::vector<int>> get_adjacency_array(Graph &g) {
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

std::vector<Graph> get_subgraphs(Graph &g, std::unordered_set<int> &sep) {
    std::vector<int> part = partition_from_separator(g, sep);
    std::map<int, std::map<int, std::vector<int>>> subgraphs;

    for (std::size_t u = 0; u < g.size(); u++) {
        if (part[u] == -1) // skip separator nodes
            continue;
        subgraphs.insert({part[u], {}});

        for (auto v : g[u]) {
            if (part[u] == part[v]) {
                subgraphs[part[u]][u].push_back(v);
            }
        }
    }

    std::vector<Graph> result;
    for (auto &[_, subgraph] : subgraphs) {
        result.push_back(get_graph(subgraph));
    }

    return result;
}

void recurse_seperators(Graph &g) {
    auto n = (int)g.size();
    auto nparts = 2;
    auto imbalance = 1.0 / 3.0;
    auto num_separator_vertices = 0;
    auto separator_raw = new int[n];
    auto [xadj, adjncy] = get_adjacency_array(g);

    node_separator(&n, nullptr, xadj.data(), nullptr, adjncy.data(), &nparts,
                   &imbalance, false, 0, ECO, &num_separator_vertices,
                   &separator_raw);

    std::cout << n << "\t" << num_separator_vertices << std::endl;
    // std::ofstream("fragments/karlsruhe.txt", std::ios::app)
    //     << n << " " << num_separator_vertices << std::endl;

    auto separator = std::unordered_set<int>(
        separator_raw, separator_raw + num_separator_vertices);

    for (auto &g : get_subgraphs(g, separator)) {
        if (g.size() > 200) {
            recurse_seperators(g);
            break;
        }
    }
}

void print_degree_distribution(Graph &g) {
    auto max_degree = (size_t)20;
    auto degrees = std::vector<int>(max_degree, 0);
    for (auto edges : g) {
        degrees[std::min(edges.size(), max_degree - 1)]++;
    }

    for (auto e : degrees) {
        std::cout << double(e) / g.size() << std::endl;
    }

    // results
    // 0.00
    // 0.22
    // 0.15
    // 0.55
    // 0.08
}

bool has_edge(Graph &g, int from, int to) {
    return std::find(g[from].begin(), g[from].end(), to) != g[from].end();
}

void graph_to_file(Graph &g, std::string filename) {
    std::ofstream(filename) << "";
    for (size_t i = 0; i < g.size(); i++) {
        for (size_t j = 0; j < g[i].size(); j++) {
            std::ofstream(filename, std::ios::app)
                << i << " " << g[i][j] << std::endl;
        }
    }
}
