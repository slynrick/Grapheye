import { Component, OnInit } from '@angular/core';
import { GraphService } from './graph.service';
import { graphFromBack } from './graph-from-back';
import * as shape from 'd3-shape';
import { Graph, Node } from '@swimlane/ngx-graph';
import { Subject } from 'rxjs';

@Component({
	selector: 'og-graph',
	templateUrl: './graph.component.html',
	styleUrls: ['./graph.component.scss']
})
export class GraphComponent implements OnInit {

	id = '';

	graphsFromBack: graphFromBack;
	graphs: Graph = {nodes: [], edges: []};
	curve = shape.curveLinear;

	center$: Subject<boolean> = new Subject();
	update$: Subject<boolean> = new Subject();

	constructor(private graphService: GraphService) { }

	ngOnInit() {
		this.getGraph();
	}

	addNode(){
		let node: Node = {id: this.id, label: this.id}
		this.graphs.nodes.push(node);
		this.updateGraph();
		this.id = '';
	}

	setId(id: string){
		this.id = id;
	}

	centerGraph() {
        this.center$.next(true);
	}

	updateGraph() {
        this.update$.next(true);
    }
	
	getGraph(){
		this.graphService.getGraph().subscribe(graph =>{
			this.graphsFromBack = graph;
			this.buildGraph();
		})
		this.update$.next(true);
	}
	
	buildGraph() {
			this.graphs.nodes = this.graphsFromBack.vertices.map((vertice)=>{
				return {
					id: vertice,
					label: vertice
				}
				
			})
			this.graphs.edges = this.graphsFromBack.arestas.map((aresta)=>{
				return {
					source: aresta[0],
					target: aresta[1]
				}
			})
	}

}


  