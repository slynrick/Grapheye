import { Component, OnInit } from '@angular/core';
import { GraphService } from './graph.service';
import { graphFromBack } from '../dominio/graph-from-back';
import * as shape from 'd3-shape';
import { Graph, Node, Layout } from '@swimlane/ngx-graph';
import { Subject } from 'rxjs';

@Component({
	selector: 'og-graph',
	templateUrl: './graph.component.html',
	styleUrls: ['./graph.component.scss']
})
export class GraphComponent implements OnInit {

    id = '';
	graphFromBack: graphFromBack = new graphFromBack();
	graph: Graph = {nodes: [], edges: []};
	
	center$: Subject<boolean> = new Subject();
	update$: Subject<boolean> = new Subject();
	curveType: string = 'Bundle';
    curve: any = shape.curveLinear;
    layout: String | Layout = 'dagre';
    
    backAnswer: boolean = false;

	constructor(private graphService: GraphService) { }

	ngOnInit() {
        const randomNumber = (Math.floor(Math.random() * 6) + 1).toString();
        this.getGraph(randomNumber);
	}

	getGraph(node: string){
		this.graphService.generateGraph(node).subscribe(graph =>{
			this.graphFromBack = graph;
			this.buildGraph();
            this.update$.next(true);
            this.id = '';
            this.backAnswer = true;
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
        this.buildGraph();
    }
	
	private buildGraph() {
		this.graph.nodes = this.graphFromBack.vertices.map((vertice)=>{
			return {
				id: vertice,
				label: vertice
			}
		})
		this.graph.edges = this.graphFromBack.arestas.map((aresta)=>{
			return {
				source: aresta[0],
				target: aresta[1]
			}
        })
	}

	layoutUpdate(layoutName: string): void {
		this.layout = layoutName;
		this.centerGraph();
    }
    

}


  