# Graph APP Console Ver
This application is a console version from [Graph APP](https://github.com/ThyagoFRTS/graph-application). The original app needs local installation of GTK4, that was not generated on application build. 

## Features
- Tree detection
- Display adjacency between two nodes
- Get all neighbors from selected nodes
- Visit all edges
- Display selected node degree
- Export .dot file to feed graphviz framework
- Export svg graph from .dot file

Obs: Graph input not persists in memory, every click interations load txt file and free itself memory after close function call (as rust feature), except by load button that save nodes info.

## Setup Dependencies
Make sure to download graphviz binaries and put on root directory, this is the more important dependency to plot graph.
The especific version used was [8.0.3](https://gitlab.com/api/v4/projects/4207231/packages/generic/graphviz-releases/8.0.3/windows_10_msbuild_Release_graphviz-8.0.3-win32.zip)

- Download Graphviz binaries from [official page](https://graphviz.org/download/)
- Paste zip file content in program root directory

The three folder will be like this:
<pre>
├── /.fingerprint
├── /build
├── /deps
├── /examples
└── /Graphviz
    └── /bin
        └── some_files.some_extension
├── /incremental
├── graph-app-console.exe
├── input.txt
└── some_files.some_extension
</pre>

# How to load your graph file
Fist, your graph need to stay in txt file in respective format:
- First line have "D" to digraph and "ND" to bidirectional graph
- Other line must be tuple of nodes. Brackets, braces, parentheses and letters are optional. Numbers and comma are required.

After txt file in required format, you need past your file on root app directory with name input.txt

Important, if your nodes sequence starts with 1 or contains gaps in the count sequence, like [(A1,A4),(A1,A2),(A2,A4)], this nodes will be replaced to [(v0,v2),(v0,v1),(v1,v2)]. Input file examples is above:
```
D
v0,v3
v1,v4
v3,v1
```
```
ND
node0,node3
node1,node4
node3,node1
```
```
ND
A0,A3
A1,A4
A3,A1
```

## Tech
- [Graphviz](https://graphviz.org/)
