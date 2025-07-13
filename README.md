## Disclaimer

This is experimental code written during my Masters thesis.
The code is provided as-is.
It was never intended to be released and is thus poorly documented and will likely contain bugs.
However, it might be of use to other researchers.

## Abstract

Empirical observations suggest that road networks possess small graph separators, scaling approximately as O(n^{1/3}).
This scaling is considerably smaller than the worst-case bounds established for major graph classes, such as planar graphs, which have separators scaling as O(n^{1/2}).
However, the underlying structural properties responsible for this phenomenon remain poorly understood.
This thesis systematically investigates which network characteristics explain the presence of these small separators.

Our analysis of real-world network data indicates a slightly larger separator scaling of approximately O(n^{0.37}).
We evaluate the impact of several graph properties by attempting to replicate the scaling behavior with synthetic graph models.
The analysis reveals that simpler properties, such as sparsity or the existence of an embedding, are insufficient on their own to explain the separator sizes observed in road networks.
Models based on such characteristics consistently yield separators that scale as O(n^{1/2}) or worse.

Instead, our results indicate that small separators are an emergent property of a hierarchical structure.
This conclusion is substantiated by our generative models.
We find that two conceptually different approaches, one based on explicit hierarchical construction and another simulating physical barriers with multi-scale noise, both produce graphs whose separators scale as O(n^{0.37}), closely matching our empirical findings.
The shared success of diverse models that enforce a hierarchical organization suggests that this is a critical property responsible for the small separators in road networks.
