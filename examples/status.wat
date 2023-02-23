(module
    (import "response" "set_status" (func $response_set_status (param i32)))

    (func $main (param i32) (param i32) (result i32)
        i32.const 3
        call $response_set_status)
    (export "main" (func $main))
)