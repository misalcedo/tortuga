(module
  ;; Import our print function 
  (import "system" "print" (func $print (param i32 i32)))

  ;; Define a single page memory of 64KB.
  (memory $0 1)

  ;; Store the Hello World string at byte offset 0
  (data (i32.const 0) "Hello, World!")

  ;; Define a function to be called from our host
  (func $helloworld
    (call $print (i32.const 0) (i32.const 13))
  )

  ;; Export the wasmprint function for the host to call.
  (export "helloworld" (func $helloworld))
)