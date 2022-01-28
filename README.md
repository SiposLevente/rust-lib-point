# Rust point library
This library implements the basic functions of a generic 2D point. This library was made mainly for learning and practice purpose

# Functions

## new
Returns a new `Point<T>`.

### Arguments
#### `x: T`
Initial "x" coordinate.

#### `y: T`
Initial "y" coordinate.

## Getters and setters
### Getters:
<ul>
  <li>get_x: T</li>
  <li>get_y: T</li>
</ul>

### Setters:
<ul>
  <li>set_x: T</li>
  <li>set_y: T</li>
</ul>

## offset
Offsets the point by the given ammount.
### Arguments
#### `x: T`
Offset ammount for "x" coordinate.

#### `y: T`
Offset ammount for "y" coordinate.

## distance
Returns the original points distance from a given point.

### Argument
#### `other_point: Point<T>`
The other point which the distance will be calculated from.

## `to_string`
Returns the point as a string

# Operators

## Algebraic operators
<ul>
  <li>Addition</li>
  <li>Substitution</li>
</ul>

## Logical operators
<ul>
  <li>Equity</li>
</ul>
