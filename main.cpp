#include <fstream>
#include <iostream>
#include <random>
#include <vector>

#include "kaHIP_interface.h"
#include "my_graph_library.hpp"
#include "random_local.hpp"
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
    // std::ofstream("output/random_exp.txt", std::ios::app)
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

int main(int argn, char **argv) {

    // germany: n=5763064 m=13984846
    // karlsruhe: n=120413 m=302605 mbi=309736
    auto xadj = load_vector<int>(
        "/home/born/Nextcloud/ws2425/Master/Graphs/germany/first_out");
    auto adjncy = load_vector<int>(
        "/home/born/Nextcloud/ws2425/Master/Graphs/germany/head");
    make_bidirectional(xadj, adjncy);

    print_degree_distribution(xadj, adjncy);

    // auto [xadj, adjncy] =
    //     random_local_graph(120413, 309736 / 2, dist_exp);
    // std::cout << xadj.size() << std::endl;
    // std::cout << adjncy.size() << std::endl;

    // recurse_seperators(xadj, adjncy);
}
