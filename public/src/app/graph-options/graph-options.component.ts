import { Component, OnInit, EventEmitter, Output } from '@angular/core';
import { Layout } from '@swimlane/ngx-graph';
import { Subject } from 'rxjs';

@Component({
    selector: 'og-graph-options',
    templateUrl: './graph-options.component.html',
    styleUrls: ['./graph-options.component.scss']
})
export class GraphOptionsComponent implements OnInit {

    @Output() center = new EventEmitter<boolean>();
    @Output() update = new EventEmitter<boolean>();
    @Output() layoutChange = new EventEmitter<string>();

    layout: String | Layout = 'dagre';
    
    layouts = [
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

    draggingEnabled = true;
    enableZoom = true;
    panningEnabled = true;

    constructor() { }

    ngOnInit() {
    }

    setLayout(layoutName: string): void {
        this.layoutChange.emit(layoutName);
        this.centerGraph();
    }

    centerGraph() {
        this.center.emit(true)
    }

    updateGraph() {
        this.update.emit(true);
    }

}
