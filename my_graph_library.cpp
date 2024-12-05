#include <unordered_set>
#include <vector>

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

    return part;
}
