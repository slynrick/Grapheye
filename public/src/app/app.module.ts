import { NgModule, CUSTOM_ELEMENTS_SCHEMA } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { FormsModule } from '@angular/forms';

import { AppComponent } from './app.component';
import { NgxGraphModule } from '@swimlane/ngx-graph';
import { NgxChartsModule } from '@swimlane/ngx-charts';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { GraphComponent } from './graph/graph.component';
import { GraphService } from './graph/graph.service';
import { CommonModule } from '@angular/common';
import { HttpClientModule, HTTP_INTERCEPTORS } from '@angular/common/http';
import { TopoComponent } from './core/topo/topo.component';
import { AppRoutingModule } from './app-routing.module';
import {NgbModule} from '@ng-bootstrap/ng-bootstrap';
import { GraphOptionsComponent } from './graph-options/graph-options.component';
import { GraphViewerComponent } from './graph-viewer/graph-viewer.component';


@NgModule({
  imports:      
    [ 
        BrowserModule, 
        FormsModule, 
        NgxGraphModule,
        NgxChartsModule,
        BrowserAnimationsModule, 
        CommonModule, 
        HttpClientModule, 
        AppRoutingModule,
        NgbModule
    ],
  declarations: 
    [ 
        AppComponent, 
        GraphComponent, 
        TopoComponent, GraphOptionsComponent, GraphViewerComponent 
    ],
  schemas: 
    [
        CUSTOM_ELEMENTS_SCHEMA
    ],
  bootstrap: [ AppComponent ],
  providers: [ GraphService ]
})
export class AppModule { }
