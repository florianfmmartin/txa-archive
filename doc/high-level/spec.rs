/*  Types
 *
 *  Basic : void, int, float, str, bool
 *
 *  Complex :
 *      Arrays : [$type]
 *      Tuples : ($type $type ...)
 *      Maps   : {$type $type}
 *      Enums  : | $tag $tag ... `...`$enum |
 *
 *  Type alisases : type $var = $type
 */

#null   // null
12_000  // int
3.1416  // float
"tree"  // str
#true   // bool

[3 4 5 6 7 8 9 10] // [int]
[[3 4] [45] [6 9]] // [[int]]
(3 "hello!" 1.644) // (int str float)
{"yo" 2 "hello" 5} // {str int}

type color = | #red #green #blue |
type bool  = | #true #false |
type null  = | #null |

{"car" #red "grass" #green}    // {str color}
{"car" #purple "grass" #green} // Error: no enum found where #purple and #green coexists

type all-color? = | ...color ...null #purple |

{
  "car"   #purple
  "grass" #green
  "house" #null
}
// {str all-color?}

/*
 *  Let bindings
 *
 *  Scoped to the module or the current do ... end block
 *
 *  Acceptable variable names are formed from lowercase letters, digits, `-`, `?`, `!`
 *  They must start by a letter
 *
 *  Let bindings prefixed with `~` are considered private
 */

let variable-name = 3
~let variable = 4

/*
 *  Modules
 *
 *  Files are automatically a module
 *
 *  Modules have let bindings or types
 *  You may import the module with aliasing
 *
 *  Modules are defined with module ... end
 */

let std-lib = require "std"   // imports the whole `std` module as `std-lib`
let std-array = std-lib.array // rebind sub-module

let std-array-2 = (require "std").array // same but one-line

let my-module = module
  let my-value = 3

  ~let other-value = 4

  let cool-value = + my-value other-value
end

print my-module.cool-value  // prints `7`
print my-module.other-value // can't access so doesn't compile

/*
 *  Blocks
 *
 *  Arbitray blocks to isolate computation
 */

let fourteen = block
  let six = 6
  let seven = + six 1
  let fourteen = * seven 2
  fourteen
end

/*
 *  Functions
 *
 *  Functions are always typed
 *
 *  fn $type_1 $var_1 ... $type_n $var_n to $type_return do $expr end
 */

let my-adder = fn int x int y to int do + x y end

/*
 *  Piping
 *
 *  The `in` keyword pipes the result of the expression on it's left to the `_` symbol in the
 *  expression on the right
 */

let fourteen = 6 in + _ 1 in * _ 2

/*
 *  Conditionnals with `match`
 *
 *  `match` verifies the cond is equal to #true and if so does the computation
 *  ending the `match` block
 */

let is-bigger-than-3 = fn int n to int do
                         match n with
                           > n 3 do #true  end
                           #true do #false end
                         end
                       end

/*
 *  Doing anything
 *
 *  To run a file it must have a function `main` of type
 *  [str] $var to std.result where
 *
 *  type result = | #ok #error |
 */

let std = require "std"

let main = fn [str] args to std.result do
  let my-number = fourteen
  let bigger-than-3? = is-bigger-than-3 my-number
  match bigger-than-3? with
    #false do #error end
    #true  do #ok end
  end
end

