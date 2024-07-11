# Master Thesis

This thesis presents the implementation of algorithms for identifying maximum planar subgraphs, developed using the Rust programming language. It includes a structure for representing undirected graphs, two exact algorithms based on Integer Linear Programming, and approximation algorithms based on the match-and-merge framework.

## Installment

To get started, ensure you have the Rust environment and Cargo tool installed. Follow the instructions on the [Rust official page](https://www.rust-lang.org/tools/install).

The implementation is written in Rust 1.75.0 and is compatible with this version or newer.

After installation, execute the following command to build the library:
``` cargo build --lib ```. This will generate a file named ```libmps_analyser.rlib```, which can be included in other Rust projects by adding a dependency to this file in the  ``` Cargo.toml ``` of the new project.

## Documentation

Documentation is not provided as a separate file but can be generated using Cargo. Run the following command to build the documentation in HTML format: ```cargo doc```.

The generated documentation can be opened in any web browser. Additionally, each function or method usable by other projects is thoroughly documented within the source code.

## Experiments

The research experiments conducted as part of this thesis can be replicated using the   ```src/tests/input_generation.rs``` file. To run these experiments, uncomment the   ```#[test]``` lines in the file and execute the tests.
