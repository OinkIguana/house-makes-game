/// @description Setup main menu

menu_tree = [
	[t("New Game"), main_menu_new_game],
	[t("Load Game"), [
		[t("Back"), -1],
		// TODO: check for save files and insert them here instead of just having empty cells
		[t("Empty"), undefined],
		[t("Empty"), undefined],
		[t("Empty"), undefined],
	]],
	[t("Settings"), [
		[t("Back"), -1],
	]],
	[t("Quit"), main_menu_quit],
];

menu_position = [0];