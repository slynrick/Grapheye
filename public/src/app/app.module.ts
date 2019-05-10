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
import { HttpClientModule } from '@angular/common/http';



@NgModule({
  imports:      [ BrowserModule, FormsModule, NgxGraphModule, NgxChartsModule,BrowserAnimationsModule, CommonModule, HttpClientModule ],
  declarations: [ AppComponent, GraphComponent ],
  schemas: [CUSTOM_ELEMENTS_SCHEMA],
  bootstrap:    [ AppComponent ],
  providers: [GraphService]
})
export class AppModule { }
