(module
  (import "system" "send" (func $send (param externref i32 i32)))

  (memory (export "io") 1)

  (func (export "allocate") (param $length i32) (result i32)
    (i32.const 0)
  )

  (func (export "receive") (param $source externref) (param $address i32) (param $length i32)
    (call $send (local.get $source) (local.get $address) (local.get $length))
  )
)