<style>
table {
   display: inline-block;
}
.sup-add {
    color: #4b4 !important;
    font-weight: bolder !important;
}
.sup-del {
    color: #f66 !important;
    font-weight: bolder !important;
    text-decoration-line: line-through !important;
    text-decoration-thickness: 0.1em !important;
}
</style>

# Supplements for ["The PLY Polygon File Format by Greg Turk"](#the-ply-polygon-file-format-by-greg-turk)

## File Structure

This is the structure of a supplemented PLY file:

##### 1. Header layout:

- Each part of the <ins>"header"</ins> is a <ins class=sup-add>"newline-terminated ASCII string (LF or CRLF)"</ins> that begins with a <ins>"keyword"</ins>. <span class=sup-add>The preferred newline terminator is LF.</span>
- Following the header is one <ins>"list of elements for each element type"</ins>, presented <ins>"in the order </ins><ins class=sup-add>and quantity</ins><ins> described in the header"</ins>.

##### 2. Keyword `ply`:

- The characters <ins class=sup-add>"ply"</ins> must be the first <ins class=sup-add>"three"</ins> characters of the file.

##### 3. Keyword `format`:

- Following <ins>the start of the header</ins> is the keyword <ins>"format"</ins> and a specification of either ASCII or binary format, followed by a <ins class=sup-add>"version identifier"</ins>.
- The header of a binary version of the same object would differ only in substituting the word <ins>"binary_little_endian"</ins> or <ins>"binary_big_endian"</ins> for the word <ins>"ascii"</ins>.
- <span class=sup-add>In <ins>"ascii"</ins> format, each element is a <ins>"newline-terminated ASCII string"</ins> composed of <ins>"space-separated"</ins> data values.</span>
- <span class=sup-add>In <ins>"binary_little_endian"</ins> and <ins>"binary_big_endian"</ins> formats, the data is stored in corresponding byte order, with <ins>"neither whitespace nor newline"</ins> between the data values.</span>

##### 4. Keyword `element` and `property`:

- Next is the description of each of the elements in the polygon file, and within each element description is the specification of the properties. The generic <ins>"element"</ins> <ins class=sup-add>descriptions may be like</ins>:

  ```plaintext
  element <element-name-1> <number-in-file>
  property <data-type> <property-name-1>
  property <data-type> <property-name-2>
  property <data-type> <property-name-3>
  ...
  element <element-name-2> <number-in-file>
  property <data-type> <property-name-1>
  property <data-type> <property-name-2>
  ...
  ```

##### 5. Data types of `property`:

- The <ins>"properties"</ins> listed after an <ins>"element"</ins> line define both the <ins>"data type"</ins> of the property and also the <ins>"order"</ins> in which the property appears for each element. There are two kinds of data types a property may have: scalar and list.

  Here is a list of the <ins class=sup-add>"common scalar data types"</ins> a <ins>"property"</ins> may have:

  | name                                       | description            | number of bytes |
  | ------------------------------------------ | ---------------------- | --------------- |
  | <ins class=sup-add>int8</ins> \| char      | character              | 1               |
  | <ins class=sup-add>uint8</ins> \| uchar    | unsigned character     | 1               |
  | <ins class=sup-add>int16</ins> \| short    | short integer          | 2               |
  | <ins class=sup-add>uint16</ins> \| ushort  | unsigned short integer | 2               |
  | <ins class=sup-add>int32</ins> \| int      | integer                | 4               |
  | <ins class=sup-add>uint32</ins> \| uint    | unsigned integer       | 4               |
  | <ins class=sup-add>float32</ins> \| float  | single-precision float | 4               |
  | <ins class=sup-add>float64</ins> \| double | double-precision float | 8               |

  Here is a list of the <ins class=sup-add>"special scalar data types"</ins> a <ins>"property"</ins> may have:

  | name                             | description           | number of bytes |
  | -------------------------------- | --------------------- | --------------- |
  | <ins class=sup-add>byte</ins>    | byte                  | 1               |
  | <ins class=sup-add>ubyte</ins>   | unsigned byte         | 1               |
  | <ins class=sup-add>half</ins>    | half-precision float  | 2               |
  | <ins class=sup-add>long</ins>    | unsigned long integer | 8               |
  | <ins class=sup-add>ulong</ins>   | long integer          | 8               |
  | <ins class=sup-add>float16</ins> | half-precision float  | 2               |
  | <ins class=sup-add>int64</ins>   | long integer          | 8               |
  | <ins class=sup-add>uint64</ins>  | unsigned long integer | 8               |

- There is a special form of <ins>"property"</ins> definitions that uses the <ins class=sup-add>"list data type"</ins>:

  ```plaintext
  property list <count-data-type> <value-data-type> <property-name>
  ```

  <p class=sup-add>The count and value data types are specified separately and must be scalar, not list types.</p>

  <p class=sup-add>Here is an example:</p>

  ```plaintext
  property list ushort int vertex_index
  ```

  This means that the property "vertex_index" <span class=sup-add>starts with an unsigned short specifying the count of indices</span>, followed by a list containing that many integers. Each integer in this <ins>"variable-length list"</ins> is an index to a vertex.

##### 6. Keyword `comment` and <code class=sup-add>obj_info</code>:

- Comments in files are ordinary keyword-identified lines that begin with the word <ins>"comment"</ins> and <ins class=sup-add>"obj_info"</ins>.

##### 7. Keyword `end_header`:

- <ins class=sup-add>"end_header"</ins> and a line terminator (CRLF or LF) delimits the end of the header.
