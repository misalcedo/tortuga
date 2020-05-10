(module
  ;; Import our print function 
  (import "system" "print" (func $print (param $location i32) (param $length i32)))

  ;; Define a single page memory of 64KB.
  (memory $0 1)

  ;; Store the Hello World string at byte offset 0
  (data (i32.const 0) "Hello, World from Add!")
  (data (i32.const 22) "Hello, World from Subtract!")
  (data (i32.const 49) "Hello, World from Multiply!")
  (data (i32.const 76) "Hello, World from Divide!")

  (func $add
    (call $print (i32.const 0) (i32.const 22))
  )

  (func $subtract
    (call $print (i32.const 22) (i32.const 27))
  )

  (func $multiply
    (call $print (i32.const 49) (i32.const 27))
  )

  (func $divide
    (call $print (i32.const 76) (i32.const 26))
  )

  (export "add" (func $add))
  (export "subtract" (func $subtract))
  (export "multiply" (func $multiply))
  (export "divide" (func $divide))
)