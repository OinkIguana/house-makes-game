/// @func array_length_undef(array)
/// @desc Returns the length of the array, assuming that an `undefined` entry signals the end of the array
/// @param {Array} array The array to check the length of

var array = argument[0];
var length = array_length_1d(array);
for (var i = 0; i < length; ++i) {
	if (is_undefined(array[i])) {
		return i;
	}
}
return length;