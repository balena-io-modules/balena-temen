# Expression language

The expression language inspiration comes from:
 
* [Jinja2](http://jinja.pocoo.org/docs/2.10/)
* [Django](https://docs.djangoproject.com/en/2.1/intro/overview/#design-your-templates)
* [Tera](https://github.com/Keats/tera) crate by [Vincent Prouillet](https://github.com/Keats)

## Evaluation

Templating engine allows you to specify dynamic value for any object field
with the `$$eval` keyword. Following example shows how to generate UUID v4 value
for the `id` field.

```json
{
    "id": {
        "$$eval": "uuidv4()"
    }
}
```

The `$$eval` keyword supports:
 
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
            "$$eval": "super.ssid | slugify"
        }        
    }
}
```

* `wifi.id` contains the `$$eval` keyword with the `super.ssid | slugify` value
* `super` is evaluated as the `wifi` object
* `super.ssid` is evaluated as the `wifi.ssid` field
* `wifi.ssid` is evaluated as the `"Balena Guest"` string
* `| slugify` applies `slugify` filter to the input value
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
        "$$eval": "people[bossId].company"
    }
}
``` 

* `bossCompanyName` contains the `$$eval` keyword with the `people[bossId].company` value
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
        "$$eval": "super.ssid | lower"
    },
    "ssid": "Balena"
}
```

`id` value will be generated from the `ssid` field value.

### Builtin filters

| Filter | Description |
| --- | --- |
| [`date`](#filter-date) | Formats a timestamp as a date (`YYYY-MM-DD`) |
| [`datetime`](#filter-datetime) | Formats a timestamp as a date time (`YYYY-MM-DDTHH:MM:SSZ`) |
| [`lower`](#filter-lower) | Lower cases a string |
| [`slugify`](#filter-slugify) |  Transforms a string into a slug |
| [`time`](#filter-time) | Formats a timestamp as a time (`HH:MM:SS`) |
| [`trim`](#filter-trim) | Removes leading and trailing whitespaces |
| [`upper`](#filter-upper) | Upper cases a string |

#### Filter lower

Lower cases a string.

Example:

* `"HaLlO" | lower` is resolved as `"hallo"`

#### Filter slugify

String is transformed into ASCII, lower cased, trimmed, spaces are converted to
hyphens and all other (not letters, numbers, hyphens) characters removed.

Example:

* `"  Balena Ltd! " | slugify` is resolved as `"balena-ltd"`

#### Filter trim

Leading and trailing whitespace characters are removed.

Example:

* `"  aa   " | trim` is resolved as `"aa"`

#### Filter date

Formats a timestamp into a date string.

Format defaults to `YYYY-MM-DD` and can be changed via the `format` argument.
Full reference of the format syntax is available in the [chrono documentation].

Example:

* `12345678 | date`
* `12345678 | date(format="%Y-%m-%d %H:%M")`

#### Filter time

Formats a timestamp into a time string.

Format defaults to `HH:MM:SS` and can be changed via the `format` argument.
Full reference of the format syntax is available in the [chrono documentation].

Example:

* `12345678 | time`
* `12345678 | time(format="%Y-%m-%d %H:%M")`

#### Filter datetime

Formats a timestamp into a date time string.

Format defaults to `YYYY-MM-DDTHH:MM:SSZ` and can be changed via the `format` argument.
Full reference of the format syntax is available in the [chrono documentation].

Example:

* `12345678 | datetime`
* `12345678 | datetime(format="%Y-%m-%d %H:%M")`

#### Filter upper

Upper cases a string.

Example:

* `"HaLlO" | upper` is resolved as `"HALLO"`

## Functions

Functions can be called without arguments (`uuidv4()`) or with named arguments
(`now(timestamp=true)`). Positional arguments are not supported.

### Builtin functions

| Filter | Description |
| --- | --- |
| [`uuidv4`](#function-uuidv4) | Generates random UUID v4 |
| [`now`](#function-now) | Returns the local date time / timestamp |

**WARNING**: None of these functions do work if `balena-temen` NPM package is used.
See [#35](https://github.com/balena-io-modules/balena-temen/issues/35) and
[#37](https://github.com/balena-io-modules/balena-temen/issues/37). Waiting
for upstream fixes. There're no issues if you do use `balena-temen` as a Rust crate.

#### Function uuidv4

Generates random UUID v4 in a hexadecimal, lower case, notation.

Example:

* `uuidv4()`

#### Function now

Returns the local date time as a string (by default) or as a timestamp (integer).

Arguments:

* `timestamp` - whether to return the timestamp (integer) instead of the date time (string)
  * defaults to `false`
* `utc` - whether to return the UTC date time instead of the local one
  * defaults to `false`
  * argument is ignored if the `timestamp` argument is set to `true`

[chrono documentation]: https://docs.rs/chrono/*/chrono/format/strftime/index.html
