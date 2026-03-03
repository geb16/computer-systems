// we import Vec for dynamic list of nodes
use std::collections::VecDeque;

// first we define a struct representing one machine in the network
struct Node {
    id: usize,
    vulnerable: bool,
    infected: bool,
}

// implementing Node behavior
impl Node {
    fn new(id: usize, vulnerable: bool) -> Self {
        Self {
            id,
            vulnerable,
            infected: false,
        }
    }
}

// function to simulate worm spread
fn simulate_worm(nodes: &mut Vec<Node>) {
    // queue used for BFS-style propagation
    let mut queue = VecDeque::new();

    // find initially infected nodes
    for node in nodes.iter() {
        if node.infected {
            queue.push_back(node.id);
        }
    }

    while let Some(current_id) = queue.pop_front() {
        for neighbor in nodes.iter_mut() {
            if !neighbor.infected && neighbor.vulnerable {
                println!(
                    "Node {} infected node {}",
                    current_id, neighbor.id
                );
                neighbor.infected = true;
                queue.push_back(neighbor.id);
            }
        }
    }
}

fn main() {
    // create network
    let mut nodes = vec![
        Node::new(0, false),
        Node::new(1, true),
        Node::new(2, false),
        Node::new(3, false),
        Node::new(4, true),
    ];

    // initial infection
    nodes[1].infected = true;

    println!("Initial infection: Node 1");

    simulate_worm(&mut nodes);

    println!("\nFinal infection state:");
    for node in nodes.iter() {
        println!(
            "Node {} => infected: {}",
            node.id, node.infected
        );
    }
}

//  A node can only be infected if it is vulnerable and not already infected.
// What real-world control stops worm propagation at network level?
//  Firewalls and network segmentation can help stop worm propagation 
// by blocking unauthorized traffic and isolating infected machines from the rest of the network.