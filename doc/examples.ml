import Array from "std"

export sum

let square = fn int n to int do
               ** n 2
             end

let square-alt = fn int n to int do n in * _ _ end

let fourteen = block do
  let six = 6
  let seven = six in + _ 1
  * seven 2
end

let add-then-square = fn int x int y to int do
                        + x y in
                        square _
                      end

let sum = fn [int] ns to int do
            Array.reduce ns 0
            fn int acc int cur to int do
              + acc cur
            end
          end

let square-all-elements = fn [int] ns to [int] do
                            Array.map ns square
                          end

type color = | #red #green #blue |

let decide-color = fn int n to color do
                     match n with
                       < n 100 do #red   end
                       < n 200 do #green end
                       #t      do #blue  end
                   end

