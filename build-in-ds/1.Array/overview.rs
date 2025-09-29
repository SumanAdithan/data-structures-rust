// Array -> Vector
// A vector is a data structure that can hold a collection of values
// Vector can contain a element of same data types.
// Vector are resizable. You don't have to declare the size of an array before creating it
// Rust Vectors are zero-indexed and the insertion order is maintained
// Vectors are iterables. They can be used with a for in loop

// Vectors - Big-O time complexity
// Insert/remove from end - O(1) ==> the time complexity is constant

// Insert/remove from beginning - O(n) ==> the time complexity is linear,
// because the index has to be reset for every remaining element in the array

// Access - O(1) ==> the time complexity is constant, because accessing position 1 and 1001 have no difference
// Search - O(n) ==> the time complexity is linear, because searching the element can be the last one in the array

// Push/pop - O(1) ==> the time complexity is constant
// insert/remove/concat/slice/splice - O(n) ==> the time complexity is linear

// forEach/map/filter/reduce - O(n) ==> the time complexity is linear
