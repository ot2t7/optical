# The Data Format
- any: **errors, format is not self describing**
- bool: a byte which is either 0x01==true or 0x00==false
- signed/unsigned integers: the normal big endian byte representation
- floats: also normal big endian byte representation
- string: var int as the string length in bytes, and then the bytes of the string. UTF-8
- char: **errors, this doesnt exist in the format**
- byte array: the rest of the packet as bytes
- option: boolean where true==Some, false==None
- unit: nothing
- unit struct: nothing
- sequence: var int representing length, then each element
- tuple: each element of the tuple
- map: **errors, this doesnt exist in the format**
- struct: tuple of each value
enum stuff:
- unit variant: var int representing variant index, then nothing
- newtype variant: var int representing variant index, then inner type
- tuple variant: var int representing variant index, then tuple
- struct variant: var int representing variant index, then tuple

# TODO
- Organize everything, comment everything, general cleanup.