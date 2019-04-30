pub mod adjacency_list;
pub mod adjacency_matrix;
pub mod graph;

pub enum GraphMethod {
    AdjacencyListMethod(adjacency_list::AdjacencyList),
    AdjacencyMatrixMethod(adjacency_matrix::AdjacencyMatrix),
}
