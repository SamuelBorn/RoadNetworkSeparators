#include <iostream>
#include <map>
#include <set>
#include <unordered_set>
#include <vector>

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
