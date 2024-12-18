# The Syntax for Polygon File Format

> Authored by Asher Chen, Nov 2024.

## Backus-Naur Form

<details>
<summary>
    <strong>Click to expand</strong>
</summary>

```abnf
<newline> ::= "\r"? "\n"
<number>  ::= [0-9]
<space>   ::= " "
<word>    ::=
    [A-Z] | [a-z] | <number> | "_" |
    "." | "," | ":" | ";" | "'" | "\"" |
    "!" | "?" | "-" | "+" | "*" | "/" |
    "@" | "#" | "$" | "%" | "^" | "(" |
    ")" | "[" | "]" | "{" | "}" | "|" |
    "\\" | "~" | "`" | "<" | ">" | "=" |
    "&"

<polygon_header> ::=
    <header_start>
    (<header_block> <newline>)*
    <header_end>

    <header_start> ::=
        "ply" <newline>
        "format" <space>+ <format> <space>+ <version> <newline>

        <format> ::=
            "ascii" | "binary_big_endian" | "binary_little_endian"
        <version> ::= <word>+

    <header_block> ::=
        "comment" <space>+ <comment> |
        "element" <space>+ <element_name> <space>+ <element_size> |
        "property" <space>+ <property_kind> <space>+ <property_name> |
        "obj_info" <space>+ <comment>

        <comment> ::=
            (<word> | <space>)*
        <element_name> ::=
            <word>+
        <element_size> ::=
            <number>+
        <property_kind> ::=
            "list" <space>+ <scalar_property_kind> <space>+ <scalar_property_kind> |
            <scalar_property_kind>
        <property_name> ::=
            <word>+
        <property_size> ::=
            <number>+
        <scalar_property_kind> ::=
            "u"? "char" |
            "u"? "short" |
            "u"? "int" ("8" | "16" | "32")? |
            "float" ("32" | "64")? |
            "double" |
            <special_scalar_property_kind>
        <special_scalar_property_kind> ::=
            "half" | "float16" |
            "u"? ("long" | "int64")

    <header_end> ::=
        "end_header" <newline>
```

</details>

<!---->
