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

* `networks.0.ssid` is resolved as `"Balena"` string
* `networks[0]["ssid"]` is resolved as `"Balena"` string

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

* `wifi.id` contains `$$eval` with the `super.ssid | slugify` value
* `super` is resolved as `wifi`
* `super.ssid` is resolved as `wifi.ssid`
* `wifi.ssid` is resolved as `"Balena Guest"` string
* `| slugify` applies `slugify` filter to the input value
* the whole expression is resolved as `balena-guest`
  
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
    "bossName": {
        "$$eval": "people[bossId].company"
    }
}
``` 

* `bossName` contains `$$eval` with the `people[bossId].company` value
* `bossId` is a variable (no `''`, `""` or back ticks) and is resolved as `"123"` string
* the intermediate expression is `people["123"].company`
* `people["123"]` is resolved as `{ "company": "Balena" }` object
* `people["123"].company` is resolved as `"Balena"` string

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

#### lower

Lowercase a string.

Example:

* `"HaLlO" | lower` is resolved as `"hallo"` string

#### slugify

String is transformed into ASCII, lower cased, trimmed, spaces are converted to
hyphens and all other (not letters, numbers, hyphens) characters removed.

Example:

* `"  Balena Ltd! " | slugify` is resolved as `"balena-ltd"` string

#### trim

Leading and trailing whitespace characters are removed.

#### date

Parse a timestamp into a date string. Format defaults to `YYYY-MM-DD`.

Full reference of the format syntax is available in the [chrono documentation].

Example:

* `ts | date`
* `ts | date(format="%Y-%m-%d %H:%M")`

#### time

Parse a timestamp into a time string. Format defaults to `HH:MM:SS`.

Full reference of the format syntax is available in the [chrono documentation].

Example:

* `ts | time`
* `ts | time(format="%Y-%m-%d %H:%M")`

#### datetime

Parse a timestamp into a date time string. Format defaults to `YYYY-MM-DDTHH:MM:SSZ`.

Full reference of the format syntax is available in the [chrono documentation].

Example:

* `ts | date-time`
* `ts | date-time(format="%Y-%m-%d %H:%M")`

#### upper

Uppercase a string.

Example:

* `"HaLlO" | upper` is resolved as `"HALLO"` string


## Functions

Functions can be called without arguments (`uuidv4()`) or with named arguments
(`now(timestamp=true)`). Positional arguments are not supported.

### Builtin functions

#### uuidv4

Generates random UUID v4 in a hexadecimal notation (string).

#### now

Returns the local datetime as a string or the timestamp as an integer.

Optional arguments:

* `timestamp` - whether to return the timestamp (integer) instead of the date time (string)
  * defaults to `false`
* `utc` - whether to return the UTC date time instead of the local one
  * defaults to `false`
  * argument is ignored if the `timestamp` argument is set to `true`

[chrono documentation]: https://docs.rs/chrono/*/chrono/format/strftime/index.html
