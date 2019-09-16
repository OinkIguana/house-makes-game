/// @func assoc_tree_parent(tree, path)
/// @desc Extracts the parent level of the entry pointed to by the given path in the tree
/// @param {AssocTree} tree The tree
/// @param {int[]} path The path to the entry to find the parent of

var tree = argument[0];
var path = argument[1];
var length = array_length_undef(path);
path[length - 1] = undefined;
return assoc_tree_value_at(tree, path)