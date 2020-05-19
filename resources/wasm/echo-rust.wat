(module
  (type $t0 (func (param i32 i32)))
  (import "system" "send" (func $send (type $t0)))
  (func $receive (type $t0) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32)
    global.get $g0
    local.set $l2
    i32.const 16
    local.set $l3
    local.get $l2
    local.get $l3
    i32.sub
    local.set $l4
    local.get $l4
    global.set $g0
    local.get $l4
    local.get $p0
    i32.store offset=8
    local.get $l4
    local.get $p1
    i32.store offset=12
    local.get $l4
    i32.load offset=8
    local.set $l5
    local.get $l4
    i32.load offset=12
    local.set $l6
    local.get $l5
    local.get $l6
    call $send
    i32.const 16
    local.set $l7
    local.get $l4
    local.get $l7
    i32.add
    local.set $l8
    local.get $l8
    global.set $g0
    return)
  (table $T0 1 1 funcref)
  (memory $memory 16)
  (global $g0 (mut i32) (i32.const 1048576))
  (global $__data_end i32 (i32.const 1048576))
  (global $__heap_base i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (export "receive" (func $receive)))
