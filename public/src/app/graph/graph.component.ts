import { Component, OnInit } from '@angular/core';
import { GraphService } from './graph.service';
import { graphFromBack } from './graph-from-back';
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

	graphsFromBack: graphFromBack;
	graphs: Graph = {nodes: [], edges: []};
	
	center$: Subject<boolean> = new Subject();
	update$: Subject<boolean> = new Subject();

	draggingEnabled = true;
	enableZoom = true;
	panningEnabled = true;

	curveType: string = 'Bundle';
	curve: any = shape.curveLinear;
	interpolations = [
		'Bundle',
		'Cardinal',
		'Catmull Rom',
		'Linear',
		'Monotone X',
		'Monotone Y',
		'Natural',
		'Step',
		'Step After',
		'Step Before'
	]; 

	layout: String | Layout = 'dagre';
	layouts: any[] = [
	  {
		label: 'Dagre',
		value: 'dagre',
	  },
	  {
		label: 'Cola Force Directed',
		value: 'colaForceDirected'
	  },
	  {
		label: 'D3 Force Directed',
		value: 'd3ForceDirected',
	  },
	];

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

	toggleDraggable(){
		this.draggingEnabled = !this.draggingEnabled;
	}

	toggleZoom(){
		this.enableZoom = !this.enableZoom;
	}

	togglePanning(){
		this.panningEnabled = !this.panningEnabled;
	}

	setLayout(layoutName: string): void {
		this.layout = layoutName;
		this.centerGraph();
	}

	setInterpolationType(curveType) {
	this.curveType = curveType;
	if (curveType === 'Bundle') {
		this.curve = shape.curveBundle.beta(1);
	}
	if (curveType === 'Cardinal') {
		this.curve = shape.curveCardinal;
	}
	if (curveType === 'Catmull Rom') {
		this.curve = shape.curveCatmullRom;
	}
	if (curveType === 'Linear') {
		this.curve = shape.curveLinear;
	}
	if (curveType === 'Monotone X') {
		this.curve = shape.curveMonotoneX;
	}
	if (curveType === 'Monotone Y') {
		this.curve = shape.curveMonotoneY;
	}
	if (curveType === 'Natural') {
		this.curve = shape.curveNatural;
	}
	if (curveType === 'Step') {
		this.curve = shape.curveStep;
	}
	if (curveType === 'Step After') {
		this.curve = shape.curveStepAfter;
	}
	if (curveType === 'Step Before') {
		this.curve = shape.curveStepBefore;
	}
	}

}


  