/*
 *  Stack based assembly language
 * 
 *  Syntax is word separated by space
 *
 *  Case is ignored
 *
 *  Memory can be written with define $var do ... endef where $var is a variable name containing
 *  letters and `-`, must start with a letter
 *
 *  Memory can be accessed with `$`$var, that's a litteral `$` followd by a variblae name
 *
 *  Strings are transformed to numbers automatically
 *
 *  Comments are /*...*/ and //
 */

// Words
pop // a b -- a
dup // a -- a a
ovr // a b -- a b a
swp // a b -- b a
mch // 0 a -- 0
    // 1 b -- b -- branches to next `end`

whl // a 0 -- a -- branches to next `end`
    // a 1 -- a -- continues until `end` then restarts

end // a -- a -- helps with branching

add // a b -- (a + b)
sub // a b -- (a - b)
mul // a b -- (a * b)
div // a b -- (a / b)
    // a 0 -- 0
    // 0 a -- 0
mod // a b -- (a % b)
equ // a b -- (a == b)
neq // a b -- (a != b)
lst // a b -- (a < b)
grt // a b -- (a > b)

// any none 0 value `t` is true
and // a b -- (a /\ b)
ior // a b -- (a \/ b)
xor // 0 t -- 1
    // t 0 -- 1
    // 0 0 -- 0
    // t t -- 0
not // 0 -- 1
    // t -- 0

mrd // addr -- ...content length
mwr // ...content length addr -- () -- puts the `...content length` in memory at `addr` will not fail but will overwrite

sop // ...content length utf? -- () -- puts the `...content` on stdout in utf-8 if `utf?` is truthy else in ints
sip // int? -- (..content length | int) -- takes a string puts it in an `int` if `int?` is truthy else puts a str as `...content length` MIGHT FAIL

frd // ...content_fn length_fn -- ...content_rd length_rd -- takes a filename str reads this file and outputs it as a read str
fwr // ...content_w length_w ...content_fn length_fn -- () -- takes a str to write and a filename str and writes it MIGHT FAIL
fmp // ...content_dn length_dr option -- ...content_rd length_rd -- takes a filename and an option int and gives a str to read based on the manipulation to do

// WebSocket
// xyz => x is w for WebSocket
//     => y is c for client
//             s for server
//     => z is w for creating web socket
//             o for open event
//             m for message event
//             c for close event
//             t for termination
//             s for sending data

w_w // ip1 ip2 ip3 ip4 port -- () -- creates a websocket instance at ip1.ip2.ip3.ip4:port
w_o // addr -- () -- takes code to execute on open event
w_m // addr -- () -- takes code to execute on message event
w_c // addr -- () -- takes code to execute on close event
w_t // () -- () -- closes the instance
w_s // ...content length -- () -- sends data

// Examples
define square do // n -- (n * n)
  dup // n -- n n
  mul // a b -- (a * b)
endef

define add-then-square do // a b -- ((a + b) ** 2)
  add
  $square
endef

define fizzbuzz-once do // a -- | "fizzbuzz" "fizz" "buzz" a |
  dup   // a a
  3     // a a 3
  mod   // a | 0..2 |
  0     // a | 0..2 | 0
  equ   // a | 0 1 |
  ovr   // a | 0 1 | a
  5     // a | 0 1 | a 5
  mod   // a | 0 1 | | 0..4 |
  0     // a | 0 1 | | 0..4 | 0
  equ   // a | 0 1 | | 0 1 |
  and   // a | 0 1 |
  "fizzbuzz"    // a | 0 1 | "fizzbuzz"
  mch           // a 0 "fizzbuzz" -- a 0
                // a 1 "fizzbuzz" -- a "fizzbuzz" -- branch to next `end`
  pop   // a 0 -- a

  dup   // a a
  3     // a a 3
  mod   // a | 0..2 |
  0     // a | 0..2 | 0
  equ   // a | 0 1 |
  "fizz"    // a | 0 1 | "fizz"
  mch       // a 0 "fizz" -- a 0
            // a 1 "fizz" -- a "fizz" -- branch to next `end`
  pop   // a 0 -- a

  dup   // a a
  5     // a a 5
  mod   // a | 0..4 |
  0     // a | 0..4 | 0
  equ   // a | 0 1 |
  "buzz"    // a | 0 1 | "buzz"
  mch       // a 0 "buzz" -- a 0
            // a 1 "buzz" -- a "buzz" -- branch to next `end`
  pop // a 0 -- a
  dup // a a

  end // a "fizzbuzz"
      // a "fizz"
      // a "buzz"
      // a a

  swp // a ffba -- ffba a
  pop // ffba
endef

// without comments
define fizzbuzz-once do
  dup

  3 mod
  0 equ

  ovr

  5 mod
  0 equ

  and

  "fizzbuzz" mch

  pop dup

  3 mod
  0 equ

  "fizz" mch

  pop dup

  5 mod
  0 equ

  "buzz" mch

  pop dup

  end swp pop
endef

