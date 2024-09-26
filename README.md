# 6Topo

6Topo is an IPv6 topology probing algorithm that operates on a global network scale, utilizing a prefix space tree. Its core observation is the correlation between prefix hierarchical relationships and their distribution in topological space. 6Topo takes IPv6 seed addresses and a prefix list as input, performs variable-dimensional DHC hierarchical clustering guided by prefix information, and generates a prefix space tree.

The algorithm begins with a hierarchical traversal from the root node until it encounters leaf nodes or nodes with prefix lengths greater than or equal to a specified starting prefix length. These nodes' corresponding prefixes are then used as targets for pre-scanning probes. After pre-scanning, 6Topo selectively traverses the prefix space tree based on prefix hierarchical relationships and probing feedback. It generates probing prefixes and conducts iterative topology probing. The probing feedback not only determines which subtrees need to be traversed but also dictates the traversal order.

## Installation

1. [Install Rust - Rust Programming Language (rust-lang.org)](https://www.rust-lang.org/tools/install)

2. Install

   Windows:

   ```shell
   .\install_windows.ps1
   ```

   Linux:

   ```shell
   ./install_linux.sh
   ```

   Mac:

   ```shell
   ./install_macos.sh
   ```

## Usage

Example:

```shell
smap -m ipv6_prefix_tree -b 50m --cool_seconds 1 -a seeds_path=your_seed_path -a prefix_path=your_prefix_path -a extra_node_num=10000  -a budget=1000000000 -a min_prefix_len=48  -a rand_ord=false 
```

## Parameters

- `budget`: Scanning budget, i.e., the number of packets to be sent
- `divide_dim`: Division dimension, generally defaults to 4
- `learning_rate`: Learning rate
- `max_prefix_len`: Maximum prefix length, generally defaults to 64
- `min_prefix_len`: Minimum prefix length, generally 48
- `seeds_path`: Path to seed addresses
- `prefix_path`: Path to the prefix list file (pyasn offline database)
- `min_target_num`: Minimum target number; when the number of <ip,ttl> combinations generated in a topology probing round is less than this value, probing for all current target prefixes will end
- `rand_ord`: Whether to randomly select split nodes
- `allow_supplement_scan`: Whether to allow supplementary scanning (using yarrp6 target prefix generation scheme after probing ends)
- `threshold`: Lower limit of node advantage level; when a node's advantage level is below this value, the node will be immediately discarded
- `extra_node_num`: Number of nodes to extract, defaults to 10,000
- `initial_ttl`: Initial TTL value for each target address, generally defaults to 16
- `gap_limit`: Maximum number of consecutive silences allowed
- `prefix_tree_max_ttl`: Maximum TTL value allowed by the prefix tree algorithm
- `allow_leaf_expand`: Whether to allow leaf node expansion
- `child_max_size`: Maximum number of child nodes that can be split from a node at once
