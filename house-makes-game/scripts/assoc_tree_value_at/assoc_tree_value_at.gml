/// @func assoc_tree_value_at(tree, path)
/// @desc Extracts the value from the assoc tree at the given path.
/// @param {AssocTree} tree The tree to extract a value from
/// @param {int[]} path The path to find the value at

var tree = argument[0];
var path = argument[1];

// if an entry in the path is undefined, it signals the end of the path
var length = array_length_undef(path);

for (var i = 0; i < length; ++i) {
	var entry = tree[path[i]];
	tree = entry[1];
}

return tree;