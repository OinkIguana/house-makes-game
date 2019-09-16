/// @description Draw the menu

var menu_depth = array_length_undef(menu_position);
var current_menu = assoc_tree_parent(menu_tree, menu_position);
var current_option = menu_position[menu_depth - 1];

var length = array_length_1d(current_menu);

for (var i = 0; i < length; ++i) {
	var entry = current_menu[i];
	if (i == current_option) {
		draw_set_color(c_red);	
	} else if (is_undefined(entry[1])) {
		draw_set_color(c_gray);
	} else {
		draw_set_color(c_white);	
	}
	draw_text(32, 32 + 32 * i, entry[0]);
}