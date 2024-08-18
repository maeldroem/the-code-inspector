# The Code Inspector

A code inspector using given regexes and other info to graph out calls or mentions to other pieces of code,
allowing you to better understand the relation between classes or files in order to identify
which pieces of code the project is most dependent on, or the opposite.

The code inspector can analyze a folder and will build a map that can then be queried with natural language
or can be used to create a graph in SVG format.

To-Do list:

- [ ] Setup CLI
- [ ] Implement logic for storing regex results and additional info in a map that can then be used extensively
- [ ] Implement progressive caching of the analysis process
- [ ] Implement logic for creating SVG graph
- [ ] Implement logic for querying
- [ ] Implement memoization