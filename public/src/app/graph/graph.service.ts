import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import {graphFromBack } from './graph-from-back';

@Injectable()
export class GraphService {
	grafosUrl = 'http://localhost:3000/graphs';
	
	constructor(private http: HttpClient) { }

		getGraph() {
			return this.http.get<graphFromBack[]>(this.grafosUrl);
		}
}
