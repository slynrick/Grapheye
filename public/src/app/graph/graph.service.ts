import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import {graphFromBack } from './graph-from-back';
import { environment } from 'src/environments/environment';

@Injectable()
export class GraphService {
	
	constructor(private http: HttpClient) { }

		getGraph() {
			return this.http.get<graphFromBack>(environment.url+'exec/generate/5');
		}
}
