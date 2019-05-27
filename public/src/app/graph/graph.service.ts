import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import {graphFromBack } from '../dominio/graph-from-back';
import { environment } from 'src/environments/environment';
import { Observable } from 'rxjs';
import { take } from 'rxjs/operators';
import { graphResponse } from '../dominio/graph-response';

@Injectable()
export class GraphService {
	
	constructor(private http: HttpClient) { }

		generateGraph(node: string): Observable<graphFromBack> {
			return this.http.get<graphFromBack>(environment.url + 'exec/generate/' + node);
		}

		addEdge(aresta: string, metodo: string, graph: graphFromBack){
			return this.http.post<graphResponse>(environment.url + '/edge/add/'+ metodo, graph);
		}

		removeEdge(aresta: string, metodo: string, graph: graphFromBack){
			return this.http.post<graphResponse>(environment.url + '/edge/remove/' + metodo, graph);
		}

		addNode(metodo: string, graph: graphFromBack){
			return this.http.post<graphResponse>(environment.url + 'exec/node/add/' + metodo, graph);
		}

		removeNode(vertice: string, metodo: string, graph: graphFromBack){
			return this.http.post<graphResponse>(environment.url + '/node/remove/'+ metodo , graph);
		}

		getVizinhos(vertice: string, metodo: string, graph: graphFromBack){
			return this.http.post<graphResponse>(environment.url + '/node/neighborhood/' + metodo + '/' + vertice, graph);
		}
	
}
