import { Component, OnInit } from '@angular/core';
import { GraphService } from './graph.service';
import { graphFromBack } from './graph-from-back';
import * as shape from 'd3-shape';
import { Graph } from '@swimlane/ngx-graph';

@Component({
	selector: 'og-graph',
	templateUrl: './graph.component.html',
	styleUrls: ['./graph.component.scss']
})
export class GraphComponent implements OnInit {

	graphsFromBack: graphFromBack[] = [];
	graphs: Graph = {nodes: [], edges: []};
	curve = shape.curveLinear;  

	constructor(private graphService: GraphService) { }

	ngOnInit() {
		this.getGraph();
	}
	
	getGraph(){
		this.graphService.getGraph().subscribe(graphs =>{
			this.graphsFromBack = graphs;
			this.buildGraph();
		})
	}
	
	buildGraph() {
		this.graphsFromBack.forEach(graph=>{
			this.graphs.nodes = graph.vertices.map((vertice)=>{
				return {
					id: vertice,
					label: vertice
				}
			})
			this.graphs.edges = graph.arestas.map((aresta)=>{
				return {
					source: aresta[0],
					target: aresta[1]
				}
			})
		})
	}

}


  