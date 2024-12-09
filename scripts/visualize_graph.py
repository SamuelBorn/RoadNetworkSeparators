import networkx as nx
import matplotlib.pyplot as plt

G = nx.Graph()

edges = [
    (0, 13), (0, 6),
    (1, 2), (1, 15),
    (2, 7),
    (3, 21),
    (6, 22),
    (7, 20),
    (9, 1), (9, 3),
    (11, 8),
    (12, 9), (12, 18),
    (13, 16), (13, 17),
    (14, 5), (14, 28),
    (15, 10),
    (16, 27), (16, 12), (16, 11),
    (17, 24),
    (20, 19), (20, 26),
    (25, 23), (25, 29),
    (27, 25),
    (29, 14), (29, 4)
]

G.add_edges_from(edges)
pos = nx.spring_layout(G)  
nx.draw(G, pos, with_labels=True)
plt.show()

