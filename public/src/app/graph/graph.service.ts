import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import {graphFromBack } from '../dominio/graph-from-back';
import { environment } from 'src/environments/environment';
import { Observable } from 'rxjs';
import { graphResponse } from '../dominio/graph-response';
import { graphSearch } from '../dominio/graph-search';

@Injectable()
export class GraphService {
	
	constructor(private http: HttpClient) { }

		generateGraph(node: string): Observable<graphFromBack> {
			return this.http.get<graphFromBack>(environment.url + 'exec/generate/' + node);
        }
        
        readGraph(graph: graphFromBack){
            return this.http.post<graphResponse>(environment.url + 'read/AdjacencyList', graph);
        }

		addEdge(metodo: string, graph: graphFromBack){
			return this.http.post<graphResponse>(environment.url + 'exec/edge/add/'+ metodo, graph);
		}

		removeEdge(metodo: string, graph: graphFromBack){
			return this.http.post<graphResponse>(environment.url + 'exec/edge/remove/' + metodo, graph);
		}

		addNode(metodo: string, graph: graphFromBack){
			return this.http.post<graphResponse>(environment.url + 'exec/node/add/' + metodo, graph);
		}

		removeNode(metodo: string, graph: graphFromBack){
			return this.http.post<graphResponse>(environment.url + 'exec/node/remove/'+ metodo , graph);
		}

		getVizinhos(vertice: string, metodo: string, graph: graphFromBack){
			return this.http.post<any>(environment.url + 'exec/node/neighborhood/' + metodo + '/' + vertice, graph);
        }
        
        isConnected(metodo: string, graph: graphFromBack){
			return this.http.post<any>(environment.url + 'exec/is_connected/' + metodo, graph);
        }

        hasCycle(metodo: string, graph: graphFromBack){
			return this.http.post<any>(environment.url + 'exec/has_cycle/' + metodo, graph);
        }

        isForest(metodo: string, graph: graphFromBack){
			return this.http.post<any>(environment.url + 'exec/is_forest/' + metodo, graph);
        }

        isTree(metodo: string, graph: graphFromBack){
			return this.http.post<any>(environment.url + 'exec/is_tree/' + metodo, graph);
        }

        getForestGenerator(metodo: string, graph: graphFromBack){
			return this.http.post<graphResponse>(environment.url + 'exec/get_forest_generator/'+ metodo , graph);
        }
        
        deepfirstSearch(vertice: string, metodo: string, graph: graphFromBack){
			return this.http.post<graphSearch>(environment.url + 'exec/deepfirst_search/'+ metodo  + '/' + vertice , graph);
        }
        
        breadthfirstSearch(vertice: string, metodo: string, graph: graphFromBack){
			return this.http.post<graphSearch>(environment.url + 'exec/breadthfirst_search/'+ metodo  + '/' + vertice , graph);
        }
        
        defineDistances(vertice: string, metodo: string, graph: graphFromBack){
			return this.http.post<any>(environment.url + 'exec/define_distances/'+ metodo  + '/' + vertice , graph);
        }
        
        search(metodo: string, searchType:string, graph: graphFromBack){
			return this.http.post<graphResponse>(environment.url + 'exec/search/' + searchType + '/' + metodo, graph);
        }

        Dijkstra(metodo: string, node:string, graph: graphFromBack){
			return this.http.post<any>(environment.url + 'exec/dijkstra/' + metodo + '/'+ node, graph);
        }
        
}
