import { Component, OnInit } from '@angular/core';
import { GraphService } from './graph.service';
import { graphFromBack } from '../dominio/graph-from-back';
import * as shape from 'd3-shape';
import { Graph, Node, Layout } from '@swimlane/ngx-graph';
import { Subject } from 'rxjs';
import { graphSearch } from '../dominio/graph-search';

@Component({
	selector: 'og-graph',
	templateUrl: './graph.component.html',
	styleUrls: ['./graph.component.scss']
})
export class GraphComponent implements OnInit {

    id = '';
    graphFromBack: graphFromBack = new graphFromBack();
    graph: Graph = {nodes: [], edges: []};
    
    graphSearchBack: graphFromBack = new graphFromBack();
    graphSearch: Graph = {nodes: [], edges: []};
	
	center$: Subject<boolean> = new Subject();
    update$: Subject<boolean> = new Subject();
    
	curveType: string = 'Bundle';
    curve: any = shape.curveLinear;
    layout: String | Layout = 'dagre';
    
    backAnswer: boolean = false;
    hasDeepGraph: boolean = false;

	constructor(private graphService: GraphService) { }

	ngOnInit() {
        this.read();
	}

	getGraph(node: string){
		this.graphService.generateGraph(node).subscribe(graph =>{
			this.graphFromBack = graph;
			this.buildGraph(this.graph, this.graphFromBack);
            this.update$.next(true);
            this.id = '';
            this.backAnswer = true;
		})
    }

    read(){
        let graph = new graphFromBack();
        graph.vertices = ['1','2','3','4','5']
        graph.arestas = [['1','2'],['2','3'],['3','4'],['2','4'],['2','5']]
        graph.nome = 'GRAFO_ALEATORIO_GRUPO1_N_3';
		this.graphService.readGraph(graph).subscribe(res =>{
			this.graphFromBack = res.data;
            this.buildGraph(this.graph, this.graphFromBack);
            this.backAnswer = true;
            this.center$.next(true);
            this.update$.next(true);
		})
    }
    
	centerGraph() {
    	this.center$.next(true);
	}

	updateGraph() {
        this.update$.next(true);
    }

    atualizaGraph(graph: graphFromBack) {
        this.backAnswer = false;
        this.graphFromBack = graph;
        graph.vertices.length == 0 ? this.backAnswer = false : this.backAnswer = true;
        this.buildGraph(this.graph, this.graphFromBack);
    }

    openSecondGraph(graph: graphFromBack){
        this.graphSearchBack = graph;
        this.buildGraph(this.graphSearch, this.graphSearchBack);
        this.hasDeepGraph = true;
    }
    
    openSearchGraph(graphSearch: graphSearch){
        this.graphSearchBack.nome = this.graphFromBack.nome;
        this.graphSearchBack.vertices = this.graphFromBack.vertices;
        this.graphSearchBack.arestas = graphSearch.data.stages;
        this.buildGraph(this.graphSearch, this.graphSearchBack);

        this.hasDeepGraph = true;
    }
    
	layoutUpdate(layoutName: string): void {
        this.layout = layoutName;
		this.centerGraph();
    }
    
    closeSecond(){
        this.hasDeepGraph = false;
    }

    private buildGraph(graph, graphFromBack: graphFromBack) {
        graph.nodes = graphFromBack.vertices.map((vertice)=>{
            return {
                id: vertice,
                label: vertice
            }
        })
        graph.edges = graphFromBack.arestas.map((aresta, index)=>{
            return {
                source: aresta[0],
                target: aresta[1],
                label: (index+1).toString()
            }
        })
    }

}


  