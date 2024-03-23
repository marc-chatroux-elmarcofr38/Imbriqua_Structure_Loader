# Imbriqua level

1. Create __Imbriqua Structure__ Rust project
    * How to use : periodic execution
    * Task
        * 
    * Input
        * BPMN Diagrams and Modeling metamodel (XML type file)
        * BPMN Execution Semantics metamodel (XML type file)
    * Output
        * BPMN classes in Rust (diagrams and modeling) (same Rust module file, API-friendly structure)
        * BPMN Execution function (diagrams and modeling) (same Rust module file, API-friendly structure)

2.  Create __Imbriqua Core__ Rust project
    * How to use : continues execution
    * Task
        * Read, update and create diagram instance (stored on permanent database)
        * Executing diagram instance
    * Input
        * Metamodel classes and functions
        * PostgreSQL database content
        * BPMN model of 
        * "BPMN to Rust" extension
    * Output
        * PostgreSQL database request
        * API responses
        * HTML responses

3. Create __Imbriqua Server Process__ BPMN project
    * How to use : continues improvement
    * Task
        * Represent function of Imbriqua server
    * Input 
        * none
    * Output
        * BPMN file of Imbriqua server process

# Imbrique Structure level

1. Create Logs manager
    * With file configuration
    * With backup configuration
    * Writing on a logs file
2. Generate classes from BPMN file without dependencies (minimal BPMN20.cmof)
    * Can 
3. Implement
4. 
5. 


# Imbrique Core level

1. Create Logs manager, with file configuration
    * Use __Imbriqua Structure level__ logs manager
2. Create integration
3. 
4. 
5. 


# Imbrique Server Process level

1. none
2. Create simple server process
    * Start with server
    * Logs static text
    * Stop
3. 
4. 
5.  