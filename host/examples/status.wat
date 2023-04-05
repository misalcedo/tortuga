(module
    (import "response" "set_status" (func $response_set_status (param i32)))
    (func (export "main") (param i32 i32) (result i32)
        i32.const 3
        call $response_set_status
        i32.const 0))