#include <boost/graph/adjacency_list.hpp>
#include <boost/graph/connected_components.hpp>
#include <boost/graph/detail/adjacency_list.hpp>
#include <boost/graph/graph_selectors.hpp>
#include <boost/graph/subgraph.hpp>
#include <iostream>
#include <unistd.h>
#include <vector>

#include "kaHIP_interface.h"
#include "vector_io.h"

typedef boost::adjacency_list<boost::vecS, boost::vecS, boost::directedS>
    Graph;

void print_graph(Graph &g) {
    for (auto e : make_iterator_range(edges(g))) {
        std::cout << source(e, g) << "→" << target(e, g) << " ";
    }
    std::cout << std::endl;
}

Graph get_boost_graph(std::vector<int> &xadj, std::vector<int> &adjncy) {
    Graph g(xadj.size() - 1);
    for (int i = 0; i < xadj.size() - 1; ++i) {
        for (int j = xadj[i]; j < xadj[i + 1]; ++j) {
            auto x = boost::out_edges(i, g);
            boost::add_edge(i, adjncy[j], g);
        }
    }
    return g;
}

bool is_bidirectional(Graph &g) {
    for (auto e : make_iterator_range(edges(g))) {
        if (boost::edge(target(e, g), source(e, g), g).second) {
            return true;
        }
    }
    return false;
}

int main(int argn, char **argv) {

    // std::vector<int> xadj = load_vector<int>(
    //     "/home/born/Nextcloud/ws2425/Master/Graphs/karlsruhe/first_out");
    // std::vector<int> adjncy = load_vector<int>(
    //     "/home/born/Nextcloud/ws2425/Master/Graphs/karlsruhe/head");

    auto xadj = std::vector<int>({0, 2, 5, 7, 9, 12});
    auto adjncy = std::vector<int>({1, 4, 0, 2, 4, 1, 3, 2, 4, 0, 1, 3});
    // auto xadj = std::vector<int>({0, 2, 5, 7, 9, 12});
    // auto adjncy = std::vector<int>({1, 4, 0, 2, 4, 1, 3, 2, 4, 0, 1, 3});
    // auto xadj = std::vector<int>({0, 1, 2, 3, 4});
    // auto adjncy = std::vector<int>({1, 2, 3, 0});
    auto g = get_boost_graph(xadj, adjncy);
    
    auto a = g.m_edges;
    // print g.m_edges

    std::cout << is_bidirectional(g) << std::endl;

    return 0;

    auto e = boost::out_edges(0, g);
    for (auto it = e.first; it != e.second; ++it) {
        std::cout << source(*it, g) << "→" << target(*it, g) << " ";
    }

    auto n = (int)xadj.size() - 1;
    auto nparts = 2;
    auto imbalance = 0.1;
    auto num_separator_vertices = 0;
    auto separator = new int[n];

    node_separator(&n, nullptr, xadj.data(), nullptr, adjncy.data(), &nparts,
                   &imbalance, false, 0, ECO, &num_separator_vertices,
                   &separator);

    for (size_t i = 0; i < num_separator_vertices; i++) {
        boost::clear_vertex(separator[i], g);
    }

    std::vector<int> component_map(boost::num_vertices(g));
    boost::connected_components(g, &component_map[0]);

    std::map<int, Graph> subgraphs;
    for (auto e : make_iterator_range(edges(g))) {
        auto component = component_map[source(e, g)];
        add_edge(source(e, g), target(e, g), subgraphs[component]);
    }

    auto count = 0;
    for (auto &subgraph : subgraphs) {
        if (boost::num_vertices(subgraph.second) > 1) {
            count++;
        }
    }
    std::cout << "count " << count << std::endl;
}
