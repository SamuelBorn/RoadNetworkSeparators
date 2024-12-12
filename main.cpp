#include <iostream>
#include <random>
#include <vector>

#include "global.hpp"
#include "kaHIP_interface.h"
#include "graph_library.hpp"
#include "random_local.hpp"
#include "same_degree.hpp"
#include "tree.hpp"
#include "vector_io.h"

std::mt19937 rng;

int main(int argn, char **argv) {
    // rng.seed(42);
    std::random_device rd;
    rng.seed(rd());

    // germany: n=5763064 m=13984846
    // karlsruhe: n=120413 m=302605 mbi=309736
    auto xadj = load_vector<int>(
        "/home/born/Nextcloud/ws2425/Master/Graphs/karlsruhe/first_out");
    auto adjncy = load_vector<int>(
        "/home/born/Nextcloud/ws2425/Master/Graphs/karlsruhe/head");
    make_bidirectional(xadj, adjncy);

    // auto [xadj, adjncy] = same_degree_graph(120000, {0, 0.22, 0.15, 0.55, 0.08});
    
    auto g = generate_local_tree(100000);
    // graph_to_file(g, "fragments/graph.txt");


    // recurse_seperators(xadj, adjncy);
}
