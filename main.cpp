#include <iostream>
#include <vector>

#include "kaHIP_interface.h"
#include "my_graph_library.hpp"
#include "vector_io.h"

int main(int argn, char **argv) {

    std::vector<int> xadj = load_vector<int>(
        "/home/born/Nextcloud/ws2425/Master/Graphs/karlsruhe/first_out");
    std::vector<int> adjncy = load_vector<int>(
        "/home/born/Nextcloud/ws2425/Master/Graphs/karlsruhe/head");

    // auto xadj = std::vector<int>({0, 2, 5, 7, 9, 12});
    // auto adjncy = std::vector<int>({1, 4, 0, 2, 4, 1, 3, 2, 4, 0, 1, 3});

    auto n = (int)xadj.size() - 1;
    auto nparts = 2;
    auto imbalance = 1.0 / 3.0;
    auto num_separator_vertices = 0;
    auto separator_raw = new int[n];

    node_separator(&n, nullptr, xadj.data(), nullptr, adjncy.data(), &nparts,
                   &imbalance, false, 0, ECO, &num_separator_vertices,
                   &separator_raw);

    auto separator = std::unordered_set<int>(
        separator_raw, separator_raw + num_separator_vertices);

    auto part = get_partition(xadj, adjncy, separator);
}
