use geo::Point;
use hashbrown::{HashMap, HashSet};

use ordered_float::OrderedFloat;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rstar::{RTree, AABB};
use serde_json::Map;

use super::geometric_graph::GeometricGraph;

const EPS: f32 = 0.0000001;
