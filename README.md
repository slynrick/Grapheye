# Rust Graph Theory 

This is a library and a binary program to create and change graphs using the rest API.
This is a software from a class work and the idea of this is to make an easy platform for students get used with graphs and their theories.

The backend was made on rust with Rocket framework and the frontend made on Angular. The pages are shown below:

[Rust Website](https://www.rust-lang.org) and 
[Angular Website](https://angular.io/)


## Installation

Install rust using the tutorial [here](https://doc.rust-lang.org/book/ch01-01-installation.html).

Clone the repository: 
```sh
git clone https://github.com/FabioRick/OG.git
```

After the instalation and the repository set use those commands:

```sh
rustup default nightly
```

**Alternative :**

```sh
rustup override set nightly # inside the project base directory
```
**And finally :**

```sh
rustup update && cargo update # every time that appers a compile error
```

## Compile

Use cargo to compile and run inside the base directory of the project. this software is installed on the default installation of rust.

```sh
cargo build # to compile only 

#or

cargo run # to compile and run
```
## API commands

### **Data format**

The **"par_vertices"** and ***"par_arestas"** keys are only used if the API request it.

```json
{
    "nome": "GRAFO_ALEATORIO_GRUPO1_N_3",
    "vertices": [
        "1",
        "2",
        "3"
    ],
    "arestas": [
        ["1", "2"],
        ["2", "3"]
    ],
    "par_vertices": [],
    "par_arestas": [[]]
}
```
### **Commands**

Remmember that the options fo the keys below:
```html
<method> : AdjacencyList or AdjacencyMatrix
<node> : Number of the target node being 1 the first
```

Below the API and a explanation of those commands:

```html
/api/read/<method> 
Read a graph from data inside the post. Using the data format shown above.

/api/exec/edge/add/<method> : Return a graph with the edge added from the graph sended by the post. Using the data format shown above.

/api/exec/edge/remove/<method> : Return a graph with the edge removed from the graph sended by the post. Using the data format shown above.

/api/exec/node/add/<method> : Return a graph with the node added from the graph sended by the post. Using the data format shown above.

/api/exec/node/remove/<method>: Return a graph with the node removed from the graph sended by the post. Using the data format shown above.

/api/exec/node/neighborhood/<method>/<node> : Return an array with the neighborhood of the requested node from the graph sended by the post. Using the data format shown above.
```