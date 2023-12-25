# Imbriqua Structure : Interpreter of BPMN model files (in UML notation) for Imbriqua Engine project

## Context

The Object Management Group® (OMG®) is an international, open membership, not-for-profit technology standards consortium, who create and make evolve specification for modeling language.

Updated in 2010, the BPMN™ (Business Process Model and Notation) specification propose a normative language for business process modeling. This notation is schematic-oriented and highly adaptive. BPMN project can be exchange in XML-structured files. This repository proposes to create a interpreter who generates BPMN class and function code (in RUST) from BPMN metamodel files.

## Goal

 Based on MOF™ (Meta Object Facility) and on UML® (Unified Modeling Language) specification, the BPMN™ 2.0.2 specification publish consumable file of BPMN class model in UML language.

 With the goal to create RUST structure and RUST traits based on BPMN Structute and BPMN Execution semantic, this repository propose to make the following task :

* Generate RUST structures, with attributes and heritage
    * For BPMN definition, using BPMN 2.0.2 CMOF files
    * For BPMN execution, using custom CMOF files 
* Generate SQL up file, including creation of tables, attributes, foreign-key, and contraints
    * For BPMN definition, using BPMN 2.0.2 CMOF files
    * For BPMN execution, using custom CMOF files
* Generate SQL down file, including removal of tables, attributes, foreign-key, and contraints
    * For BPMN definition, using BPMN 2.0.2 CMOF files
    * For BPMN execution, using custom CMOF files
* Generate RUST traits for structures, for BPMN Execution
    * For BPMN execution, using custom CMOF files

The following element aren't in the scope of the project :

* Integrate SQL migration files : All execution don't take in account any old result and old input file. No file provide to migrate database after modification of classes or attributes


## How to use 

## Reference

All resource and files for OMG consortium can be found in the folder "resource" of this repository. It contain actual and previus version of BPMN™, DD™, MOF™, OCL™, UML® and XMI® specification.

## Copyright

Copyright 2023-2024 CHATROUX MARC

This file is part of Imbriqua Structure, a interpreter of BPMN model files (in UML notation) for Imbriqua Engine project

Imbriqua Structure is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Imbriqua Structure is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Imbriqua Structure. If not, see <https://www.gnu.org/licenses/>.

