(module
  (import "pong" "send" (func $send (param i32 i32)))

  (memory (export "io") 1)
  (data (i32.const 0) "Ping!\n")

  (func (export "allocate") (param $length i32) (result i32)
    (i32.const 7)
  )

  (func (export "receive") (param $source externref) (param $address i32) (param $length i32)
    (call $send (i32.const 0) (i32.const 6))
  )
)