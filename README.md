# approximate-length-search

Find a path with a (bad) approximated length as fast as a BFS

## How to run?

```sh
$ cargo run --release
```

## How fast is it?

Here is the output of the benchmark of the algorithm for a graph with 20 thousand vertices and density of 0.01:
```
Fill the graph - 250.07ms
Approximate length search - 17.81ms
The path is valid
```

Yep, that is milliseconds, not seconds.

## Should I use it?

No, you shouldn't use this for any real world case, this is only a proof of concept that I created because I have seen so many perfect algorithms for unweighted graphs, but only a few approximations, and then I had this idea (that have gone worse than I expected). This is almost instantaneous but almost never gives a good approximation.

If you want an algorithm for finding paths with a specific length you should see [fixed-length-search](https://github.com/TiagoCavalcante/fixed-length-search) and [fixed-cost-search](https://github.com/TiagoCavalcante/fixed-cost-search), both are also blazingly fast.
