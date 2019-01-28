# Expression language

The expression language inspiration comes from:

* [Jinja2](http://jinja.pocoo.org/docs/2.10/)
* [Django](https://docs.djangoproject.com/en/2.1/intro/overview/#design-your-templates)
* [Tera](https://github.com/Keats/tera) crate by [Vincent Prouillet](https://github.com/Keats)

## Formula evaluation

Templating engine allows you to specify dynamic value for any object field
with the `$$formula` keyword. Following example shows how to generate UUID v4 value
for the `id` field.

```json
{
    "id": {
        "$$formula": "UUIDV4()"
    }
}
```

The `$$formula` keyword supports:

* literals
* variable access (= JSON fields)
* expressions with arithmetic, relational and logical operators
* filters (variable modification)
* functions (generators)

## Grammar

Full grammar is available [here](https://github.com/balena-io-modules/balena-temen/blob/master/src/parser/grammar.pest).
It's based on the [Pest - The Elegant Parser](https://github.com/pest-parser/pest).

## Literals

Supported literals:

* booleans (`true` or `false`)
* integers
* floats
* strings (text delimited by `""`, `''` or back ticks)

## Variables

All JSON fields values can be accessed as variables. Variable name equals to a field name.
Nested values are accessed by using the dot (`.`) notation or square brackets (`[]`).

Given the following JSON:

```json
{
    "networks": [
        {
            "ssid": "Balena"
        },
        {
            "ssid": "Balena Guest"
        }
    ]
}
```

* `networks.0.ssid` is evaluated as `"Balena"`
* `networks[0]["ssid"]` is evaluated as `"Balena"`

All variables are considered as absolute (evaluation starts from the JSON root) unless they
are prefixed with `this` or `super` keyword. `this` keyword denotes the current object and
`super` keywords denotes the parent object.

Given the following JSON:

```json
{
    "wifi": {
        "ssid": "Balena Guest",
        "id": {
            "$$formula": "super.ssid | SLUGIFY"
        }
    }
}
```

* `wifi.id` contains the `$$formula` keyword with the `super.ssid | SLUGIFY` value
* `super` is evaluated as the `wifi` object
* `super.ssid` is evaluated as the `wifi.ssid` field
* `wifi.ssid` is evaluated as the `"Balena Guest"` string
* `| SLUGIFY` applies `SLUGIFY` filter to the input value
* the whole expression is evaluated as the `"balena-guest"` string
  
The sample JSON evaluates to:

```json
{
    "wifi": {
        "ssid": "Balena Guest",
        "id": "balena-guest"
    }
}
```

Square brackets allows you to use variables as well. Given the following JSON:

```json
{
    "bossId": "123",
    "people": {
        "123": {
            "company": "Balena"
        }
    },
    "bossCompanyName": {
        "$$formula": "people[bossId].company"
    }
}
```

* `bossCompanyName` contains the `$$formula` keyword with the `people[bossId].company` value
* `bossId` is a variable (no `''`, `""` or back ticks) and is evaluated as the `"123"` string
* the intermediate expression is `people["123"].company`
* `people["123"]` is evaluated as the `{ "company": "Balena" }` object
* `people["123"].company` is evaluated as the `"Balena"` string

## Expressions

### Arithmetic operators

Allowed on numbers only.

* `+` - addition
* `-` - subtraction
* `/` - division
* `*` - multiplication
* `%` - modulo

### Relational operators

* `==` - tests values equality
* `!=` - tests values inequality
* `>=` - true if the left value is equal or greater than the right one
* `<=` - true if the right value is equal or greater than the left one
* `>` - true if the left value is greater than the right one
* `<` - true if the right value is greater than the left one

### Logical operators

* `and` - true if the left and right operands are true
* `or` - true if the left or right operands are true
* `not` - negate statement

### Operators precedence

* `()`
* `not`
* `*`, `/`, `%`
* `+`, `-`
* `<`, `<=`, `>`, `>=`
* `==`, `!=`
* `and`
* `or`

## Filters

Variables can be modified by filters separated from the variable by a pipe (`|`).
Multiple filters can be chained.

Example:

```json
{
    "id": {
        "$$formula": "super.ssid | LOWER"
    },
    "ssid": "Balena"
}
```

`id` value will be generated from the `ssid` field value.

### Builtin filters

| Filter | Description |
| --- | --- |
| [`DATE`](#filter-date) | Formats a timestamp as a date (`YYYY-MM-DD`) |
| [`DATETIME`](#filter-datetime) | Formats a timestamp as a date time (`YYYY-MM-DDTHH:MM:SSZ`) |
| [`LOWER`](#filter-lower) | Lower cases a string |
| [`SLUGIFY`](#filter-slugify) |  Transforms a string into a slug |
| [`TIME`](#filter-time) | Formats a timestamp as a time (`HH:MM:SS`) |
| [`TRIM`](#filter-trim) | Removes leading and trailing whitespaces |
| [`UPPER`](#filter-upper) | Upper cases a string |

#### Filter lower

Lower cases a string.

Example:

* `"HaLlO" | LOWER` is resolved as `"hallo"`

#### Filter slugify

String is transformed into ASCII, lower cased, trimmed, spaces are converted to
hyphens and all other (not letters, numbers, hyphens) characters removed.

Example:

* `"  Balena Ltd! " | SLUGIFY` is resolved as `"balena-ltd"`

#### Filter trim

Leading and trailing whitespace characters are removed.

Example:

* `"  aa   " | TRIM` is resolved as `"aa"`

#### Filter date

Formats a timestamp into a date string.

Format defaults to `YYYY-MM-DD`. You can pass your own format as a first filter argument.
Full reference of the format syntax is available in the [chrono documentation].

Example:

* `12345678 | DATE`
* `12345678 | DATE("%Y-%m-%d %H:%M")`

#### Filter time

Formats a timestamp into a time string.

Format defaults to `HH:MM:SS`. You can pass your own format as a first filter argument.
Full reference of the format syntax is available in the [chrono documentation].

Example:

* `12345678 | TIME`
* `12345678 | TIME("%Y-%m-%d %H:%M")`

#### Filter datetime

Formats a timestamp into a date time string.

Format defaults to `YYYY-MM-DDTHH:MM:SSZ`. You can pass your own format as a first filter argument.
Full reference of the format syntax is available in the [chrono documentation].

Example:

* `12345678 | DATETIME`
* `12345678 | DATETIME("%Y-%m-%d %H:%M")`

#### Filter upper

Upper cases a string.

Example:

* `"HaLlO" | UPPER` is resolved as `"HALLO"`

## Functions

Functions can be called without arguments (`UUIDV4()`) or with positional arguments
(`NOW(true)`).

### Builtin functions

| Filter | Description |
| --- | --- |
| [`UUIDV4`](#function-uuidv4) | Generates random UUID v4 |
| [`NOW`](#function-now) | Returns the local date time / timestamp |

#### Function uuidv4

Generates random UUID v4 in a hexadecimal, lower case, notation.

Example:

* `UUIDV4()`

#### Function now

Returns the UTC date time as a string (by default) or as a timestamp (integer).

Example:

* `NOW()`, `NOW(false)` - UTC date time as a RFC 3339 string
* `NOW(true)` - timestamp

[chrono documentation]: https://docs.rs/chrono/*/chrono/format/strftime/index.html
