import { Component, OnInit } from '@angular/core';
import { GraphService } from '../graph/graph.service';

@Component({
  selector: 'og-graph-checkpoint2',
  templateUrl: './graph-checkpoint2.component.html',
  styleUrls: ['./graph-checkpoint2.component.scss']
})
export class GraphCheckpoint2Component implements OnInit {

  constructor(private graphService: GraphService) { }

  ngOnInit() {
  }

}
