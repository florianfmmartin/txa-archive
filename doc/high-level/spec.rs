/*  Types
 *
 *  Basic : void, int, float, str, bool
 *
 *  Complex :
 *      Arrays  : [$type]
 *      Tuples  : #[$type $type ...]
 *      Maps    : {$type $type}
 *      Records : #{$key $type $key $type ...}
 *      Enums   : | $tag $tag<$type> ... `...`$enum |
 *      Fns     : \(int int int io)
 *
 *  Type alisases : type $var = $type
 */

()      // void
#null   // null
12_000  // int
3.1416  // float
"tree"  // str
#true   // bool

[3 4 5 6 7 8 9 10] // [int]
[[3 4] [45] [6 9]] // [[int]]
#[3 "hello!" 1.64] // #(int str float)
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

// Generics and voidness
type option<T> = | #none #some<T> |
type equivalentOption<T> = | #none<void> #some<T> |
type either<A, B> = | #left<A> #right<B> |

let string-of-maybe-int = match maybe-int with
                            #none do == "()" (unwrap maybe-int in void-to-string _) end
                            #some do unwrap maybe-int in int-to-string _ end
                          end

type error<T> = | #ok<T> #error<str> |
let maybe-an-int-from-error = match error-or-value with
  #ok do #some (compute-with-value-returns-an-int (unwrap error-or-value)) end
  #error do #none end
end

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
 *  Side-effects
 *
 *  Side-effects are managed through contexts
 *
 *  The possible contexts are:
 *      - state
 *      - io
 *      - file-io
 *      - async
 *      - network
 *
 *  Context are provided to event handlers
 *
 *  Functions needing a context must explicitly declare that need
 *
 *  Some std functions like `print` and `state-get` (functions doing side-effects) require that the
 *  function calling them as the necessary context for them to run
 */

// Example
// Let's say we have a function that adds two numbers and prints the result

// An incorrect signature would be
let adder = fn int a int b to int do
              let n = + a b
              print n
              n
            end

// Is is incorrect because `print` needs the `io` context but it is not available here

// A correct signature would be
let adder = fn int a int b to int with io do
              let n = + a b
              print n
              n
            end

// Here the `io` context is explicitly declared and so we can `print` the result

/*
 *  Why manage side-effects in such a way
 *
 *  The first concern that arises when dealing with side-effects is that it is very verbose
 *
 *  The idea behind the verbosity is to decourage users to have side-effects deep down the
 *  function calls tree
 *
 *  It should encourage users to have side-effects as close to the event handler as possible so that
 *  must of the code written is pure, and so testable, code
 */

/* 
 *  Event handlers
 *
 *  The VM as an event queue that is populated through networks call via web-sockets
 *  At initialization a `main` event is pushed onto the queue to start the proccess
 *
 *  (WIP) to register events you must declare a event handlers map of type {str \(str str)}
 *  Contexts are ignored for the `event` map
 */

events {
  "main" main
  "get-html" app-html
}

/*
 * State management
 *
 * The state type must be set in a state type
 * 
 * It can then be accessed with `state-get` of type `\(void $state-type)`
 * Set with `state-set` of type `\($state-type void)`
 * Updated with `state-update` of type `\(\($state-type $state-type) void)`
 *
 * All these functions require the `state` context
 */

state #{
  #hello "bonjour"
  #counter 0
}

/*
 * Async
 *
 * Functions can `wait` for event to have started at one point with the `wait` keyword
 *
 * You may `wait #an-event`, `wait [#an-event-1 #an-event-2]` or `wait ()` any event
 * 
 * Once an event matching what you asked as started you unlock and can be run next time it is your
 * turn in queue
 */

