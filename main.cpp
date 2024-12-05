#include <iostream>
#include <vector>

#include "kaHIP_interface.h"
#include "my_graph_library.hpp"
#include "vector_io.h"

int main(int argn, char **argv) {

    auto xadj = load_vector<int>(
        "/home/born/Nextcloud/ws2425/Master/Graphs/karlsruhe/first_out");
    auto adjncy = load_vector<int>(
        "/home/born/Nextcloud/ws2425/Master/Graphs/karlsruhe/head");

    // auto xadj = std::vector<int>({0, 2, 5, 7, 9, 12});
    // auto adjncy = std::vector<int>({1, 4, 0, 2, 4, 1, 3, 2, 4, 0, 1, 3})  ;
    // auto xadj = std::vector<int>({0, 2, 4, 4, 5});
    // auto adjncy = std::vector<int>({1, 2, 0, 2, 0});

    make_bidirectional(xadj, adjncy);

    auto n = (int)xadj.size() - 1;
    auto nparts = 2;
    auto imbalance = 1.0 / 3.0;
    auto num_separator_vertices = 0;
    auto separator_raw = new int[n];

    node_separator(&n, nullptr, xadj.data(), nullptr, adjncy.data(), &nparts,
                   &imbalance, false, 0, STRONG, &num_separator_vertices,
                   &separator_raw);

    auto separator = std::unordered_set<int>(
        separator_raw, separator_raw + num_separator_vertices);

    auto subgraphs = get_subgraphs(xadj, adjncy, separator);

    for (auto &[s_xadj, s_adjncy] : subgraphs) {
        // print s_xadj and s_adjncy
        // std::cout << "xadj: ";
        // for (auto x : s_xadj) {
        //     std::cout << x << " ";
        // }
        // std::cout << std::endl;
        //
        // std::cout << "adjncy: ";
        // for (auto x : s_adjncy) {
        //     std::cout << x << " ";
        // }
        // std::cout << std::endl;
        //
        std::cout << "xadj " << s_xadj.size() << " adjncy " << s_adjncy.size()
                  << std::endl;
    }
}

void recurse_seperators(std::vector<int> &xadj, std::vector<int> &adjncy) {
    auto n = (int)xadj.size() - 1;
    auto nparts = 2;
    auto imbalance = 1.0 / 3.0;
    auto num_separator_vertices = 0;
    auto separator_raw = new int[n];

    node_separator(&n, nullptr, xadj.data(), nullptr, adjncy.data(), &nparts,
                   &imbalance, false, 0, STRONG, &num_separator_vertices,
                   &separator_raw);

    auto separator = std::unordered_set<int>(
        separator_raw, separator_raw + num_separator_vertices);

    auto subgraphs = get_subgraphs(xadj, adjncy, separator);

    for (auto &[s_xadj, s_adjncy] : subgraphs) {
        if (s_xadj.size() > 5) {
            recurse_seperators(s_xadj, s_adjncy);
        }
    }
}
