import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { GraphService } from '../graph/graph.service';
import { graphFromBack } from '../dominio/graph-from-back';

@Component({
    selector: 'og-graph-checkpoint1',
    templateUrl: './graph-checkpoint1.component.html',
    styleUrls: ['./graph-checkpoint1.component.scss']
})
export class GraphCheckpoint1Component implements OnInit {

    @Input() graph: graphFromBack;
    @Output() changeGraph = new EventEmitter<graphFromBack>()

    vizinhos: number[] = [];
    node = '';

    addSourceEdge = '';
    addTargetEdge = '';

    removeSourceEdge = '';
    removeTargetEdge = '';

    vizinhosNode = '';
    neighborhoods = '0';


    constructor(private graphService: GraphService) { }

    ngOnInit() {
    }

    addNode() {
        let graph = this.buildGraph();
        graph.par_vertices.push("1");
        this.graphService.addNode('AdjacencyList', graph).subscribe(res => {
            this.changeGraph.emit(res.data);
        })
    }

    removeNode(){
        let size = this.graph.vertices.length
        if (!isNaN(+this.node) && +this.node <= this.graph.vertices.length && +this.node > 0) {
            let graph = this.buildGraph();
            graph.par_vertices.push(this.node);
            this.graphService.removeNode('AdjacencyList', graph).subscribe(graphFromBack => {
                this.changeGraph.emit(graphFromBack.data)
            })
        }
    }

    addAresta() {
        if (!isNaN(+this.addSourceEdge) && !isNaN(+this.addTargetEdge) ) {
            let graph = this.buildGraph();
            let arestas = [this.addSourceEdge, this.addTargetEdge];
            graph.par_arestas.push(arestas);
            this.graphService.addEdge('AdjacencyList', graph).subscribe(graphFromBack => {
                this.changeGraph.emit(graphFromBack.data)
            })
        }
    }

    removeAresta() {
        if (!isNaN(+this.removeSourceEdge) && !isNaN(+this.removeTargetEdge) ) {
            let graph = this.buildGraph();
            let arestas = [this.removeSourceEdge, this.removeTargetEdge];
            graph.par_arestas.push(arestas);
            this.graphService.removeEdge('AdjacencyList', graph).subscribe(graphFromBack => {
                this.changeGraph.emit(graphFromBack.data)
            })
        }
    }

    getVizinhos() {
        this.graphService.getVizinhos(this.vizinhosNode, 'AdjacencyList', this.graph).subscribe(res => {
            console.log(res.data);
            this.vizinhos = res.data;
            res.data.length == 0 ? this.neighborhoods = '0' : this.getNeighborhoods(this.vizinhos);
        })
    }

    private buildGraph(): graphFromBack {
        let graph = new graphFromBack();  
        graph.arestas = this.graph.arestas;
        graph.nome = this.graph.nome;
        graph.vertices = this.graph.vertices;

        return graph
    }

    getNeighborhoods(vizinhos: number[]){
       this.neighborhoods = vizinhos.reduce((acc: string, vizinho)=> {
            return acc.concat(vizinho.toString() + ' ');
        }, '')
    }
}
