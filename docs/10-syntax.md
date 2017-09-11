# Syntax & Grammar

Ullage files are plain-old UTF-8. The language itself is built mainly around *words* rather than fancy sigils; more reminiscent of Ruby than C and friends.

[TOC]

## Tokens

Source text is treated as one of four basic token types: *words*, *punctuation*, *literals* and *whitespace*. Words and whitespace are unicode-aware.

### Words

Word tokens start with an alphabetic character or an underscore. They may then contain any number of alphanumeric or underscore characters.

Examples of words are: `foo`, `fn`, `_1` and `∂`. Some words have special meanings in the grammar:

    if unless else while end fn var let print

### Punctuation

Punctuation characters, such as `-` and `!=` are used to represent operators in the language. Currently a handful of punctuation characters are recognised: `=`, `==`, `!`, `!=`, `+`, `-`, `*`, `/`, `(`, `)`, `[`, `]`, `,`, `:`, `<`, and `>`. 

### Literals

Literals are used to represent constant input values to the program. Literals can be used to specify values of any primitive type (`Number`, `Bool` and `String` so far). Numeric literals consist of one or more consecutive digits: `0`, `42`, `1337`. Although `-47` evaluates to a negative number the `-` isn't part of the literal; in fact it is an operator.

### Whitespace

Whitespace tokens are made up of one or more *space* characters. These *space* characters are either Unicode whitespace, such as tabs & spaces, or comments. Comments are introduced with a `#` and continue to the end of the line.

    # This is a comment!

## Types

There are three main base types: `Number`, `String` and `Bool`. These can be extended by creating arrays and tuples.

### `Bool`

A boolean holds a single bit of information. Boolean values can be created wit the literals `true` and `false`; or as the result of a comparison (`foo == 100`).

### `Number`

Number values hold whole numbers, or integers. Currently only decimal numeric literals are supported. All number values are stored in a 64 bit integer value.

### `String`

String literals define a sequence of unicode code points. All strings in the language are UTF-8.

    'I am a string'

### Arrays

An array type is declared by wrapping an existing type in `[]`. For example `[Number]` is the type for an array of `Number` values. To create a new array an array literal can be used:

    [1, 2, 3, 4]

All the values in an array must be of a single type.

### Tuples

Tuples are similar to arrays but can contain values with different types. Tuples are defined by wrapping a comma-separated list of types in `()`. For example the type `(Number, Bool)` defines a two-element tuple with the first element a `Number` and the second a `Bool`. Instances of a tuple can be created by wrapping values in `()`:

    (100, false)
