/// @description Move the menu selection and activate options

var menu_depth = array_length_undef(menu_position);
var current_menu = assoc_tree_parent(menu_tree, menu_position);
var current_option = menu_position[menu_depth - 1];

if (keyboard_check_pressed(vk_down)) {
	menu_position[menu_depth - 1] = (current_option + 1) % array_length_1d(current_menu);
} else if (keyboard_check_pressed(vk_up)) {
	var length = array_length_1d(current_menu);
	menu_position[menu_depth - 1] = (current_option - 1 + length) % length;
} else if (keyboard_check_pressed(vk_enter)) {
	var entry = assoc_tree_value_at(menu_tree, menu_position);
	if (entry == -1) {
		menu_position[menu_depth - 1] = undefined;
	} else if (is_array(entry)) {
		menu_position[menu_depth] = 0;	
	} else if (!is_undefined(entry) && script_exists(entry)) {
		script_execute(entry);	
	}
}