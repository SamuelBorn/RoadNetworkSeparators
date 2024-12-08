#include <iostream>
#include <vector>

#include "kaHIP_interface.h"
#include "my_graph_library.hpp"
#include "tree.hpp"
#include "vector_io.h"

void recurse_seperators(std::vector<int> &xadj, std::vector<int> &adjncy) {
    auto n = (int)xadj.size() - 1;
    auto nparts = 2;
    auto imbalance = 1.0 / 3.0;
    auto num_separator_vertices = 0;
    auto separator_raw = new int[n];

    node_separator(&n, nullptr, xadj.data(), nullptr, adjncy.data(), &nparts,
                   &imbalance, false, 0, FAST, &num_separator_vertices,
                   &separator_raw);

    std::cout << n << " " << num_separator_vertices << std::endl;
    // std::ofstream("output.txt", std::ios::app) << n << " " <<
    // num_separator_vertices << std::endl;
    
    auto separator = std::unordered_set<int>(
        separator_raw, separator_raw + num_separator_vertices);
    auto subgraphs = get_subgraphs(xadj, adjncy, separator);


    for (auto &[s_xadj, s_adjncy] : subgraphs) {
        if (s_xadj.size() > 200) {
            recurse_seperators(s_xadj, s_adjncy);
        }
    }
}

int main(int argn, char **argv) {

    // nodes = 5763064, edges = 13984846 
    auto xadj = load_vector<int>(
        "/home/born/Nextcloud/ws2425/Master/Graphs/germany/first_out");
    auto adjncy = load_vector<int>(
        "/home/born/Nextcloud/ws2425/Master/Graphs/germany/head");
    make_bidirectional(xadj, adjncy);


    // auto xadj = std::vector<int>({0, 2, 5, 7, 9, 12});
    // auto adjncy = std::vector<int>({1, 4, 0, 2, 4, 1, 3, 2, 4, 0, 1, 3})  ;
    // auto xadj = std::vector<int>({0, 2, 4, 4, 5});
    // auto adjncy = std::vector<int>({1, 2, 0, 2, 0});

    recurse_seperators(xadj, adjncy);

    auto g = generate_random_tree(1000000);
}
