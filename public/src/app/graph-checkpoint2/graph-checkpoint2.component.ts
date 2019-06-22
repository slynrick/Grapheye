import { Component, OnInit, Output, Input, EventEmitter } from '@angular/core';
import { GraphService } from '../graph/graph.service';
import { graphFromBack } from '../dominio/graph-from-back';
import { graphSearch } from '../dominio/graph-search';

@Component({
    selector: 'og-graph-checkpoint2',
    templateUrl: './graph-checkpoint2.component.html',
    styleUrls: ['./graph-checkpoint2.component.scss']
})
export class GraphCheckpoint2Component implements OnInit {
    //AdjacencyList
    @Input() graph: graphFromBack;
    @Output() searchGraph = new EventEmitter<graphSearch>();
    @Output() secondGraph = new EventEmitter<graphFromBack>();

    connected = '';
    tree = '';
    cycle = '';
    forest = '';
    doneBack = false;
    nodeForDeepSearch = '';
    nodeForBreadthSearch = '';
    nodeDistance = '';
    distances: number[]=[];

    constructor(private graphService: GraphService) { }

    ngOnInit() {
    }

    getGraphBooleans() {
        this.isConnected();
        this.hasCycle();
        this.isForest();
        this.isTree();
    }

    isConnected() {
        this.graphService.isConnected('AdjacencyList', this.graph).subscribe(res => {
            this.connected = res.data.is_connected.toString();
            this.connected = this.connected.charAt(0).toUpperCase() + this.connected.slice(1);
        })
    }

    isTree() {
        this.graphService.isTree('AdjacencyList', this.graph).subscribe(res => {
            this.tree = res.data.is_tree.toString();
            this.tree = this.tree.charAt(0).toUpperCase() + this.tree.slice(1);
        })
    }

    isForest() {
        this.graphService.isForest('AdjacencyList', this.graph).subscribe(res => {
            this.forest = res.data.is_forest.toString();
            this.forest = this.forest.charAt(0).toUpperCase() + this.forest.slice(1);
        })
    }

    hasCycle() {
        this.graphService.hasCycle('AdjacencyList', this.graph).subscribe(res => {
            this.cycle = res.data.has_cycle.toString();
            this.cycle = this.cycle.charAt(0).toUpperCase() + this.cycle.slice(1);
        })
    }

    deepfirstSearch() {
        let inicio = ((+this.nodeForDeepSearch)-1).toString();
        this.graphService.deepfirstSearch(inicio, 'AdjacencyList', this.graph).subscribe(res => {
            this.searchGraph.emit(res);
        })
    }

    breadthfirstSearch() {
        let inicio = ((+this.nodeForBreadthSearch)-1).toString();
        this.graphService.breadthfirstSearch(inicio, 'AdjacencyList', this.graph).subscribe(res => {
            this.searchGraph.emit(res);
        })
    }

    defineDistances() {
        let inicio = ((+this.nodeDistance)-1).toString();
        this.graphService.defineDistances(inicio,'AdjacencyList', this.graph).subscribe(res => {
            this.distances = res.data.map(dist => {
                return dist === -1? 0 : dist;
            })
        })
    }

    getForestGenerator() {
        this.graphService.getForestGenerator('AdjacencyList', this.graph).subscribe(res => {
            let graph = new graphFromBack();
            graph = res.data;
            this.secondGraph.emit(graph);
        })
    }

}
