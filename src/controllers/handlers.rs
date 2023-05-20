

use crate::cmd::use_graphviz;
use crate::graphy_search::{deep_search, visit_all_edges};
use crate::models::graph::Representation;
use crate::utils::get_clean_selection_input;

pub fn handle_load_file(graph: &mut Representation){
    graph.load_from_file("input.txt");
    println!("Arquivo carregado com sucesso!");
}

pub fn handle_export_svg(graph: &mut Representation){
    graph.export_graphviz_file();

    let mut result = String::new();

    match use_graphviz::export_svg("graphviz.dot") {
        Ok(()) => result.push_str("Arquivo exportado em: ./graphviz.svg"),
        Err(e) => result.push_str(e.to_string().as_str()),
    }

    println!("{:?}", result);
}

pub fn handle_tree_detection(graph: &mut Representation){
    let path = deep_search(graph.get_graph());

    let result = path.iter()
        .map(|edge| format!("(v{},v{})", edge.0,edge.1))
        .collect::<Vec<String>>()
        .join("->");

    println!("{:?}",result.clone().as_str())
}

pub fn handle_visit_edges(graph: &mut Representation) {

    let path = visit_all_edges(graph.get_graph(), graph.get_graph_type());

    let result = path.iter()
        .map(|edge| format!("(v{},v{})", edge.0,edge.1))
        .collect::<Vec<String>>()
        .join("->");

        println!("{:?}",result.clone().as_str());
}

pub fn handle_vertex_adjacency(graph: &mut Representation) {
    let model  = graph.get_labels_list();

    let joined_string = model.join(", ");
    let result = format!("[{}]", joined_string);

    println!("Vértices disponíveis: {}", result);
    
    let source = get_clean_selection_input("Digite o primeiro vertice: ", &model);
    let destiny = get_clean_selection_input("Digite o segundo vertice: ", &model);

    let result = graph.is_adjacent(source, destiny);

    let message = if result {"São adjacentes"} else {"Não são adjacentes"};


    println!("{}", message)
}


pub fn handle_vertex_neighbors(graph: &mut Representation){
    let model  = graph.get_labels_list();

    let joined_string = model.join(", ");
    let result = format!("[{}]", joined_string);

    println!("Vértices disponíveis: {}", result);
    
    let source = get_clean_selection_input("Digite um vertice: ", &model);

    let neighbors = graph.get_neighbors_from(source);

    let result = neighbors.iter()
    .map(|x| format!("v{}", x))
    .collect::<Vec<String>>()
    .join(",");

    println!("{}", result);
}

pub fn handle_vertex_degree(graph: &mut Representation){
    let model  = graph.get_labels_list();

    let joined_string = model.join(", ");
    let result = format!("[{}]", joined_string);

    println!("Vértices disponíveis: {}", result);
    
    let source = get_clean_selection_input("Digite um vertice: ", &model);

    let result = graph.verify_vertex_degree(source);

    println!("{}",result);
}