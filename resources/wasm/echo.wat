(module
  (import "system" "send" (func $send (param $location i32)))

  (memory $0 1)

  (func $echo
    (call $send (i32.const 0))
  )

  (export "receive" (func $echo))
)