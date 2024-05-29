## installation

You can build and install the binary using `cargo`

```cargo install --git https://github.com/kopperud/splitfrequencies```

## usage

Call `splitfrequencies --help` for further info. For example

```splitfrequencies --input primates_cytb*.trees --output splits.csv --burnin 0.1```

If no output file is given, then the results are printed to `stdout`. Example:

```splitfrequencies --input primates_cytb*.trees```

```
split,primates_cytb_JC_run_1.trees,primates_cytb_JC_run_2.trees
[01111110111001110011110],0.001777382803821373,0.001777382803821373
[01100100000000000000000],0.8209286825149966,0.8682514996667408
[01000100000000000000000],0.8062652743834703,0.8589202399466785
[00000100000000010000000],0.17907131748500332,0.13174850033325927
...
```

## algorithm for computing all splits:

Initial step: compute an ordered vector of all tip labels in the second line of the first file. Assume the tip labels are equal across all trees in all files.

### sub-algorithm:
1. Begin at an internal node `N`. 
2. Initialize a bit vector with zero elements, i.e. `[0,0,0,0,...]`, representing presence or absence of the tip in the split.
3. Perform a traversal of the subtree descending from `N`. If a tip is visited, assign a 1 in the bit vector at the index representing the tip.
5. Store the bit vector representing the split

Repeat the above sub-algorithm for all internal nodes `N`, except the root.
The left and right subtrees descending from the root will have complementary splits. 
For example, `[1,1,0,0,0]` for the left subtree, and `[0,0,1,1,1]` for the right subtree.

Trivial tip splits (i.e. `[1,0,0,...]`...) are not included. Symmetric or complementary are not always added, i.e. if the split [1,0,0,1] is recorded, it's not guaranteed that the split [0,1,1,0] is also included.

