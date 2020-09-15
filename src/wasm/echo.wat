(module
  (import "system" "send" (func $send (param i32 i32)))

  (memory 1)

  (func (export "allocate") (param $length i32) (result i32)
    (i32.const 0)
  )

  (func (export "receive") (param $address i32) (param $length i32)
    (call $send (local.get $address) (local.get $length))
  )
)