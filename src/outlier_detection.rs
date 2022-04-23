use crate::{Graph, Trajectory};
use petgraph::graph::NodeIndex;
use petgraph::visit::depth_first_search;
use petgraph::visit::{Control, DfsEvent};
use petgraph::EdgeDirection::{Incoming, Outgoing};

pub struct Detector {
    graph: Graph,
    current_nodes: Vec<NodeIndex>,
}

impl Detector {
    pub fn new(graph: Graph) -> Self {
        let mut detector = Detector {
            graph,
            current_nodes: vec![],
        };
        detector.set_current_nodes_to_roots();
        detector
    }

    fn set_current_nodes_to_roots(&mut self) {
        self.current_nodes = self
            .graph
            .node_indices()
            .filter(|nx| self.graph.edges_directed(*nx, Incoming).count() == 0)
            .collect()
    }

    pub fn is_outlier(&self, point: [f64; 3]) -> bool {
        if self.current_nodes.is_empty() {
            println!("No current nodes.");
            return true;
        }
        let result: Control<NodeIndex> = depth_first_search(
            &self.graph,
            self.current_nodes.clone(),
            |event| match event {
                DfsEvent::Discover(nx, _) => {
                    if self.graph[nx].1.spatially_contains(point) {
                        Control::Break(nx)
                    } else {
                        let on_edge = self
                            .graph
                            .edges_directed(nx, Outgoing)
                            .any(|edge| edge.weight().1.on_trj(point));
                        if on_edge {
                            Control::Break(nx)
                        } else {
                            Control::Continue
                        }
                    }
                }
                _ => Control::Continue,
            },
        );

        if result.break_value().is_some() {
            return false;
        }
        true
    }
}
