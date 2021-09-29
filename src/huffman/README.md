# Tree Deserialization Format
The tree constructed by huffman coding is serialized to the beginning of the compressed file. After the tree is reconstructed, preorder traversal will be used to find values in the tree. 
Each node starts with a bit to indicate if the node has a value. If the node has a value, the bit will be 1 and the following 8 bits will contain the value. Otherwise, the bit will be 0 and the node's children will follow. An example tree in JSON is shown below. 
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
The tree would serialize to the following string of bits, with text and indentation for clarity.
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
