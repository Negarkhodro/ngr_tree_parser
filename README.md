# Ngr Tree Parser

This program parses input text files containing data in a tree structure and converts them into more usable data structures.

### Functionality
* Reads `json` files containing tree data
* Parses the tree structure
* Allows access to node values and children
* TODO

### Output 
```bash
 $ cargo run tree-nodes show-tree show
     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/ngr_tree_parser tree-nodes show-tree show`
Diag (ID: 1) (Level: 0)
├───  └───  Car (ID: 2) (Level: 1)
├───  ├───  └───  Iran Khodro (ID: 3) (Level: 2)
├───  ├───  ├───  └───  Peugeot 206 french (ID: 4) (Level: 3)
```