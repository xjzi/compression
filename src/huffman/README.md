# Tree Deserialization Format
Each node starts with a bit to flag if the node has a value. If the node has a value, then the bit will be set to true and the following 8 bits will contain the value. Otherwise, the bit will be set to false and the node's children will follow. For example, consider the following tree. 
```
{
	Value: null,
	Left: {
		Value: 00000000,
		Left: null,
		Right: null
	},
	Right: {
		Value: null,
		Left: {
			Value: 10000000,
			Left: null,
			Right: null
		},
		Right: {
			Value: 01000000,
			Left: null,
			Right: null
		}
	}
}
```
There is redundant information in this format because if there's a non null value, there can't be any children. In the binary format, a single bit marks whether there will be children or a value. The tree would serialize to the following string of bits, with annotations for clarity.
```
has value: 0
	left
		has value: 1
		value: 00000000
	right
		has value : 0
			left
				has value: 1
				value: 10000000
			right 
				has value: 1
				value: 01000000
```
