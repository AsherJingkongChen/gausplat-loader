# The Syntax for Polygon File Format

> Authored by Asher Chen, Nov 2024.

## Backus-Naur Form

```bnf
<blank> ::=
    " " | "\t"
<newline> ::=
    "\r"? "\n"
<number> ::=
    [0-9]
<word> ::=
    [A-Z] | [a-z] | <number> | "_" |
    "." | "," | ":" | ";" | "'" | "\"" |
    "!" | "?" | "-" | "+" | "*" | "/" |
    "@" | "#" | "$" | "%" | "^" | "(" |
    ")" | "[" | "]" | "{" | "}" | "|" |
    "\\" | "~" | "`" | "<" | ">" | "=" |
    "&"

<polygon_header> ::=
    <header_start> <newline>
    (<header_block> <newline>)*
    <header_end> <newline>

    <header_start> ::=
        "ply" <newline>
        "format" <blank>+ <format> <blank>+ <version>

        <format> ::=
            "ascii" |
            "binary_big_endian" |
            "binary_little_endian"
        <version> ::=
            <word>+

    <header_block> ::=
        "comment" <blank>+ <comment> |
        "element" <blank>+ <element_name> <blank>+ <element_size> |
        "obj_info" <blank>+ <comment> |
        "property" <blank>+ <property_kind> <blank>+ <property_name>

        <comment> ::=
            (<word> | <blank>)*
        <element_name> ::=
            <word>+
        <element_size> ::=
            <number>+
        <property_kind> ::=
            "list" <blank>+ <scalar_property_kind> <blank>+ <scalar_property_kind> |
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
        "end_header"
```
