import networkx as nx
import matplotlib.pyplot as plt

# G = nx.read_adjlist("fragments/graph.txt", nodetype=int)
G = nx.read_edgelist("fragments/graph.txt", nodetype=int)
pos = nx.spring_layout(G)  
nx.draw(G, pos, with_labels=True)
# print(nx.is_connected(G))
plt.show()
