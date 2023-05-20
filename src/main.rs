pub mod models;
pub mod utils;
pub mod graphy_search;
pub mod controllers;
pub mod cmd;


use std::io;
use std::io::Write;

use models::graph::Representation;
use controllers::handlers::*;

use crate::cmd::use_cmd::clear_console;


fn exibir_menu() {
    println!("Selecione uma opção:");
    println!("1 - Carregar arquivo");
    println!("2 - Exportar svg");
    println!("3 - Verificar grau");
    println!("4 - Procurar vizinhos");
    println!("5 - Verificar adjacência");
    println!("6 - Detectar árvore");
    println!("7 - Visitar arestas");
    println!("8 - Sair");
}
fn main() {
    let mut graph = Representation::new();
    loop {
        clear_console();
        exibir_menu();

        let mut opcao = String::new();
        print!("Opção: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler a entrada.");

        match opcao.trim().parse::<u32>() {
            Ok(1) => handle_load_file(&mut graph),
            Ok(2) => handle_export_svg(&mut graph),
            Ok(3) => handle_vertex_degree(&mut graph),
            Ok(4) => handle_vertex_neighbors(&mut graph),
            Ok(5) => handle_vertex_adjacency(&mut graph),
            Ok(6) => handle_tree_detection(&mut graph),
            Ok(7) => handle_visit_edges(&mut graph),
            Ok(8) => {
                println!("Você selecionou a opção 'Sair'.");
                break;
            }
            _ => println!("Opção inválida. Tente novamente."),
        }
        //graph.print_graph();
    }
}
