mod utils;

use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_json::{Result, Value};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: String) {
    alert(&format!("Hello, {}!", s));
}

#[derive(Clone, Debug)]
struct Person {
    name: String,
    dont: Vec<String>,
}

#[wasm_bindgen]
pub fn gift_exchange(s: String) -> String {
    let mut users = get_users(s).unwrap();

    users.shuffle(&mut thread_rng());

    let graph = create_graph(users.clone());

    let resp = hamiltonian_cycle(graph, NodeIndex::new(1), Vec::new());
    print_list(resp, users)
}

fn get_users(data: String) -> Result<Vec<Person>> {
    let mut users = Vec::new();
    let v: Value = serde_json::from_str(&data)?;
    for user in v["users"].as_array().unwrap() {
        let mut dont = Vec::new();
        let arr = user["dont"].as_array().unwrap();
        for u in arr {
            dont.push(String::from(u.as_str().unwrap()));
        }
        users.push(Person {
            name: String::from(user["name"].as_str().unwrap()),
            dont: dont,
        })
    }
    Ok(users)
}

fn create_graph(people: Vec<Person>) -> UnGraph<String, ()> {
    let mut graph = UnGraph::<String, ()>::new_undirected();
    graph = add_nodes(people.clone(), graph);
    add_edges(people.clone(), graph)
}

fn add_nodes(people: Vec<Person>, mut g: UnGraph<String, ()>) -> UnGraph<String, ()> {
    for person in people.iter() {
        g.add_node(person.name.clone());
    }
    g
}

fn add_edges(people: Vec<Person>, mut g: UnGraph<String, ()>) -> UnGraph<String, ()> {
    for (i, person) in people.iter().enumerate() {
        for (j, person2) in people.iter().enumerate() {
            if person.name == person2.name {
                continue;
            }
            if person.dont.iter().any(|i| *i == person2.name)
                || person2.dont.iter().any(|i| *i == person.name)
                || g.find_edge_undirected(NodeIndex::new(i), NodeIndex::new(j)) != None
            {
                continue;
            }
            g.add_edge(NodeIndex::new(i), NodeIndex::new(j), ());
        }
    }
    g
}

fn hamiltonian_cycle(
    g: UnGraph<String, ()>,
    n: NodeIndex,
    mut v: Vec<NodeIndex>,
) -> Vec<NodeIndex> {
    let mut nodes = g.neighbors(n).detach();
    while let Some(node) = nodes.next_node(&g) {
        let ng = g.clone();
        if v.contains(&n) {
            continue;
        }
        v.push(n);
        let resp = hamiltonian_cycle(ng, node, v.clone());
        if g.node_count() == resp.len() {
            return resp;
        }
        v.pop();
    }
    v
}

fn print_list(list: Vec<NodeIndex>, people: Vec<Person>) -> String {
    let mut s = String::new();
    let first_user = &people[list[0].index()];
    for ni in list.iter() {
        s = s + &format!("{:?} -> ", people[ni.index()].name);
    }
    s = s + &format!("{:?}", first_user.name);

    s
}
