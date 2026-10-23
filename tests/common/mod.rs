use phprs::contracts::{SyntaxKind, SyntaxTree};

pub fn verify_tree(tree: &SyntaxTree<'_>) {
    assert!(
        matches!(tree.nodes[tree.root].kind, SyntaxKind::Rule("start")),
        "root: {:?}",
        tree.nodes[tree.root]
    );
    assert_eq!(tree.nodes[tree.root].span, 0..tree.source.len());
    let mut parents = vec![0; tree.nodes.len()];
    for (index, node) in tree.nodes.iter().enumerate() {
        assert!(node.span.start <= node.span.end, "{node:?}");
        assert!(node.span.end <= tree.source.len());
        for &child in &tree.edges[node.children.clone()] {
            assert!(child < index);
            parents[child] += 1;
            let span = &tree.nodes[child].span;
            assert!(
                node.span.start <= span.start && span.end <= node.span.end,
                "parent {node:?}, child {:?}",
                tree.nodes[child]
            );
        }
    }
    parents[tree.root] += 1;
    assert!(
        parents.iter().all(|&count| count == 1),
        "unowned or multiply owned tree nodes: {parents:?}"
    );
    assert_eq!(tree.edges.len(), tree.nodes.len() - 1);
}
