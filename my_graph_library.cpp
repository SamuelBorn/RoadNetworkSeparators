#include <iostream>
#include <map>
#include <set>
#include <unordered_set>
#include <vector>

typedef std::map<int, std::vector<int>> Graph;

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
}

void expand_partition(int node, int current_part, std::vector<int> &xadj,
                      std::vector<int> &adjncy, std::unordered_set<int> &sep,
                      std::vector<int> &part) {
    part[node] = current_part;

    for (int i = xadj[node]; i < xadj[node + 1]; i++) {
        auto other_node = adjncy[i];

        if (part[other_node] == -1 && sep.find(other_node) == sep.end()) {
            expand_partition(other_node, current_part, xadj, adjncy, sep, part);
        }
    }
}

std::vector<int> get_partition(std::vector<int> &xadj, std::vector<int> &adjncy,
                               std::unordered_set<int> &sep) {
    std::vector<int> part(xadj.size() - 1, -1);

    for (std::size_t i = 0; i < xadj.size() - 1; i++) {
        if (part[i] == -1 && sep.find(i) == sep.end()) {
            expand_partition(i, i, xadj, adjncy, sep, part);
        }
    }

    std::map<int, int> m = {};
    for (auto x : part)
        m[x] = 0;
    for (auto x : part)
        m[x] = m[x] + 1;
    for (auto [a, b] : m)
        std::cout << a << " " << b << std::endl;

    return part;
}

// std::pair<std::vector<int>, std::vector<int>> get_adjacency_array(Graph g) {}

std::vector<std::pair<std::vector<int>, std::vector<int>>>
get_subgraphs(std::vector<int> &xadj, std::vector<int> &adjncy,
              std::vector<int> &part) {

    std::map<int, Graph> subgraphs;

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

    for (auto [x, y] : subgraphs) {
        std::cout << x << ": ";
        std::cout << y.size() << std::endl;
    }

    return {};

    // std::vector<std::pair<std::vector<int>, std::vector<int>>> result;
    // for (auto &subgraph : subgraphs) {
    //     result.push_back(get_adjacency_array(subgraph.second));
    // }
    // return result;
}
