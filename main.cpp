#include <iostream>
#include <random>
#include <vector>

#include "global.hpp"
#include "graph_library.hpp"
#include "random_local.hpp"
#include "same_degree.hpp"
#include "tree.hpp"
#include "vector_io.h"

std::mt19937 rng;

int main(int argn, char **argv) {
    std::random_device rd;
    rng.seed(rd());

    // // germany: n=5763064 m=13984846
    // // karlsruhe: n=120413 m=302605 mbi=309736
    // auto xadj = load_vector<int>(
    //     "/home/born/Nextcloud/ws2425/Master/Graphs/karlsruhe/first_out");
    // auto adjncy = load_vector<int>(
    //     "/home/born/Nextcloud/ws2425/Master/Graphs/karlsruhe/head");
    // auto g = get_graph(xadj, adjncy);
    // make_bidirectional(g);
    // recurse_seperators(g);

    // auto g = random_local_graph_tree_distance(12000, 31000 / 2);
    auto g = random_local_graph_tree_distance(12000, 31000 / 2);
    recurse_seperators(g);
    
    // graph_to_file(g, "fragments/graph.txt");
}
