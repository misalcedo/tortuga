(module
  (type $t0 (func (param i32 i32)))
  (type $t1 (func (param i32) (result i32)))
  (type $t2 (func (param i32 i32 i32)))
  (type $t3 (func (param i32 i32) (result i32)))
  (type $t4 (func (param i32)))
  (type $t5 (func (param i32) (result i64)))
  (type $t6 (func (param i32 i32 i32) (result i32)))
  (import "system" "send" (func $send (type $t0)))
  (func $_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h60101fdf81435f3eE (type $t1) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32)
    global.get $g0
    local.set $l1
    i32.const 16
    local.set $l2
    local.get $l1
    local.get $l2
    i32.sub
    local.set $l3
    local.get $l3
    local.get $p0
    i32.store offset=12
    local.get $l3
    i32.load offset=12
    local.set $l4
    local.get $l4
    return)
  (func $_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h096bafc0f8cdfffdE (type $t1) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32)
    global.get $g0
    local.set $l1
    i32.const 16
    local.set $l2
    local.get $l1
    local.get $l2
    i32.sub
    local.set $l3
    local.get $l3
    global.set $g0
    local.get $l3
    local.get $p0
    i32.store offset=12
    local.get $l3
    i32.load offset=12
    local.set $l4
    local.get $l4
    call $_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h60101fdf81435f3eE
    local.set $l5
    i32.const 16
    local.set $l6
    local.get $l3
    local.get $l6
    i32.add
    local.set $l7
    local.get $l7
    global.set $g0
    local.get $l5
    return)
  (func $_ZN53_$LT$T$u20$as$u20$core..convert..TryFrom$LT$U$GT$$GT$8try_from17h1cbe9638398e8a8bE (type $t1) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32)
    global.get $g0
    local.set $l1
    i32.const 16
    local.set $l2
    local.get $l1
    local.get $l2
    i32.sub
    local.set $l3
    local.get $l3
    global.set $g0
    local.get $l3
    local.get $p0
    i32.store offset=8
    local.get $l3
    i32.load offset=8
    local.set $l4
    local.get $l4
    call $_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h096bafc0f8cdfffdE
    local.set $l5
    local.get $l3
    local.get $l5
    i32.store offset=12
    local.get $l3
    i32.load offset=12
    local.set $l6
    i32.const 16
    local.set $l7
    local.get $l3
    local.get $l7
    i32.add
    local.set $l8
    local.get $l8
    global.set $g0
    local.get $l6
    return)
  (func $_ZN49_$LT$usize$u20$as$u20$core..iter..range..Step$GT$9add_usize17h900b1ef28267dcf9E (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i32) (local $l15 i32) (local $l16 i32) (local $l17 i32)
    global.get $g0
    local.set $l3
    i32.const 32
    local.set $l4
    local.get $l3
    local.get $l4
    i32.sub
    local.set $l5
    local.get $l5
    global.set $g0
    local.get $l5
    local.get $p1
    i32.store offset=16
    local.get $l5
    local.get $p2
    i32.store offset=20
    local.get $l5
    i32.load offset=20
    local.set $l6
    local.get $l6
    call $_ZN53_$LT$T$u20$as$u20$core..convert..TryFrom$LT$U$GT$$GT$8try_from17h1cbe9638398e8a8bE
    local.set $l7
    local.get $l5
    local.get $l7
    i32.store offset=24
    local.get $l5
    i32.load offset=24
    local.set $l8
    local.get $l5
    local.get $l8
    i32.store offset=28
    local.get $l5
    i32.load offset=16
    local.set $l9
    local.get $l9
    i32.load
    local.set $l10
    local.get $l5
    i32.load offset=28
    local.set $l11
    i32.const 8
    local.set $l12
    local.get $l5
    local.get $l12
    i32.add
    local.set $l13
    local.get $l13
    local.get $l10
    local.get $l11
    call $_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_add17h2609249236c21fd8E
    local.get $l5
    i32.load offset=12 align=1
    local.set $l14
    local.get $l5
    i32.load offset=8 align=1
    local.set $l15
    local.get $p0
    local.get $l14
    i32.store offset=4
    local.get $p0
    local.get $l15
    i32.store
    i32.const 32
    local.set $l16
    local.get $l5
    local.get $l16
    i32.add
    local.set $l17
    local.get $l17
    global.set $g0
    return)
  (func $_ZN4core3ptr19swap_nonoverlapping17h12f98ca59d04729cE (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i32) (local $l15 i32) (local $l16 i32)
    global.get $g0
    local.set $l3
    i32.const 32
    local.set $l4
    local.get $l3
    local.get $l4
    i32.sub
    local.set $l5
    local.get $l5
    global.set $g0
    i32.const 4
    local.set $l6
    local.get $l5
    local.get $p0
    i32.store offset=4
    local.get $l5
    local.get $p1
    i32.store offset=8
    local.get $l5
    local.get $p2
    i32.store offset=12
    local.get $l5
    i32.load offset=4
    local.set $l7
    local.get $l5
    local.get $l7
    i32.store offset=16
    local.get $l5
    i32.load offset=8
    local.set $l8
    local.get $l5
    local.get $l8
    i32.store offset=20
    local.get $l5
    local.get $l6
    i32.store offset=28
    local.get $l5
    i32.load offset=28
    local.set $l9
    local.get $l5
    i32.load offset=12
    local.set $l10
    local.get $l9
    local.get $l10
    i32.mul
    local.set $l11
    local.get $l5
    local.get $l11
    i32.store offset=24
    local.get $l5
    i32.load offset=16
    local.set $l12
    local.get $l5
    i32.load offset=20
    local.set $l13
    local.get $l5
    i32.load offset=24
    local.set $l14
    local.get $l12
    local.get $l13
    local.get $l14
    call $_ZN4core3ptr25swap_nonoverlapping_bytes17h06c22e7c65e0a6e2E
    i32.const 32
    local.set $l15
    local.get $l5
    local.get $l15
    i32.add
    local.set $l16
    local.get $l16
    global.set $g0
    return)
  (func $_ZN4core3ptr23swap_nonoverlapping_one17h239a439e42f2e0bcE (type $t0) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i32) (local $l15 i32) (local $l16 i32) (local $l17 i32) (local $l18 i32) (local $l19 i32) (local $l20 i32) (local $l21 i32) (local $l22 i32) (local $l23 i32) (local $l24 i32)
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
    i32.const 4
    local.set $l5
    local.get $l4
    local.get $p0
    i32.store
    local.get $l4
    local.get $p1
    i32.store offset=4
    local.get $l4
    local.get $l5
    i32.store offset=12
    local.get $l4
    i32.load offset=12
    local.set $l6
    i32.const 32
    local.set $l7
    local.get $l6
    local.set $l8
    local.get $l7
    local.set $l9
    local.get $l8
    local.get $l9
    i32.lt_u
    local.set $l10
    i32.const 1
    local.set $l11
    local.get $l10
    local.get $l11
    i32.and
    local.set $l12
    block $B0
      block $B1
        block $B2
          local.get $l12
          br_if $B2
          i32.const 1
          local.set $l13
          local.get $l4
          i32.load
          local.set $l14
          local.get $l4
          i32.load offset=4
          local.set $l15
          local.get $l14
          local.get $l15
          local.get $l13
          call $_ZN4core3ptr19swap_nonoverlapping17h12f98ca59d04729cE
          br $B1
        end
        local.get $l4
        i32.load
        local.set $l16
        local.get $l16
        call $_ZN4core3ptr4read17h9f31a11f86ed52c9E
        local.set $l17
        local.get $l4
        local.get $l17
        i32.store offset=8
        i32.const 1
        local.set $l18
        local.get $l4
        i32.load offset=4
        local.set $l19
        local.get $l4
        i32.load
        local.set $l20
        local.get $l19
        local.get $l20
        local.get $l18
        call $_ZN4core10intrinsics19copy_nonoverlapping17hb302ec6b1402a42eE
        local.get $l4
        i32.load offset=4
        local.set $l21
        local.get $l4
        i32.load offset=8
        local.set $l22
        local.get $l21
        local.get $l22
        call $_ZN4core3ptr5write17h28f0c4446ae52111E
        br $B0
      end
    end
    i32.const 16
    local.set $l23
    local.get $l4
    local.get $l23
    i32.add
    local.set $l24
    local.get $l24
    global.set $g0
    return)
  (func $_ZN4core3ptr4read17h9f31a11f86ed52c9E (type $t1) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i32) (local $l15 i32) (local $l16 i32)
    global.get $g0
    local.set $l1
    i32.const 32
    local.set $l2
    local.get $l1
    local.get $l2
    i32.sub
    local.set $l3
    local.get $l3
    global.set $g0
    local.get $l3
    local.get $p0
    i32.store offset=4
    local.get $l3
    i32.load offset=12
    local.set $l4
    local.get $l3
    local.get $l4
    i32.store offset=8
    i32.const 8
    local.set $l5
    local.get $l3
    local.get $l5
    i32.add
    local.set $l6
    local.get $l6
    local.set $l7
    local.get $l3
    i32.load offset=4
    local.set $l8
    local.get $l3
    local.get $l7
    i32.store offset=24
    local.get $l3
    i32.load offset=24
    local.set $l9
    local.get $l3
    local.get $l9
    i32.store offset=28
    local.get $l3
    i32.load offset=28
    local.set $l10
    i32.const 1
    local.set $l11
    local.get $l8
    local.get $l10
    local.get $l11
    call $_ZN4core10intrinsics19copy_nonoverlapping17hb302ec6b1402a42eE
    local.get $l3
    i32.load offset=8
    local.set $l12
    local.get $l3
    local.get $l12
    i32.store offset=16
    local.get $l3
    i32.load offset=16
    local.set $l13
    local.get $l3
    local.get $l13
    i32.store offset=20
    local.get $l3
    i32.load offset=20
    local.set $l14
    i32.const 32
    local.set $l15
    local.get $l3
    local.get $l15
    i32.add
    local.set $l16
    local.get $l16
    global.set $g0
    local.get $l14
    return)
  (func $_ZN4core3ptr5write17h28f0c4446ae52111E (type $t0) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
    global.get $g0
    local.set $l2
    i32.const 16
    local.set $l3
    local.get $l2
    local.get $l3
    i32.sub
    local.set $l4
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
    i32.store
    return)
  (func $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h396a8cf6f8a9d6beE (type $t3) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32)
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
    call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17hfbdbb12a7193b4dcE
    local.set $l7
    i32.const 16
    local.set $l8
    local.get $l4
    local.get $l8
    i32.add
    local.set $l9
    local.get $l9
    global.set $g0
    local.get $l7
    return)
  (func $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17hfbdbb12a7193b4dcE (type $t3) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32)
    global.get $g0
    local.set $l2
    i32.const 16
    local.set $l3
    local.get $l2
    local.get $l3
    i32.sub
    local.set $l4
    local.get $l4
    local.get $p0
    i32.store offset=4
    local.get $l4
    local.get $p1
    i32.store offset=8
    local.get $l4
    i32.load offset=4
    local.set $l5
    local.get $l4
    i32.load offset=8
    local.set $l6
    i32.const 2
    local.set $l7
    local.get $l6
    local.get $l7
    i32.shl
    local.set $l8
    local.get $l5
    local.get $l8
    i32.add
    local.set $l9
    local.get $l4
    local.get $l9
    i32.store offset=12
    local.get $l4
    i32.load offset=12
    local.set $l10
    local.get $l10
    return)
  (func $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h765a127b5dbb9fa2E (type $t3) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32)
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
    call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17hea136c2e98869e93E
    local.set $l7
    i32.const 16
    local.set $l8
    local.get $l4
    local.get $l8
    i32.add
    local.set $l9
    local.get $l9
    global.set $g0
    local.get $l7
    return)
  (func $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17hea136c2e98869e93E (type $t3) (param $p0 i32) (param $p1 i32) (result i32)
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
    local.get $p0
    i32.store offset=4
    local.get $l4
    local.get $p1
    i32.store offset=8
    local.get $l4
    i32.load offset=4
    local.set $l5
    local.get $l4
    i32.load offset=8
    local.set $l6
    local.get $l5
    local.get $l6
    i32.add
    local.set $l7
    local.get $l4
    local.get $l7
    i32.store offset=12
    local.get $l4
    i32.load offset=12
    local.set $l8
    local.get $l8
    return)
  (func $_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_add17h2609249236c21fd8E (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i32) (local $l15 i32) (local $l16 i32) (local $l17 i32) (local $l18 i32) (local $l19 i32) (local $l20 i32) (local $l21 i32)
    global.get $g0
    local.set $l3
    i32.const 32
    local.set $l4
    local.get $l3
    local.get $l4
    i32.sub
    local.set $l5
    local.get $l5
    global.set $g0
    local.get $l5
    local.get $p1
    i32.store offset=8
    local.get $l5
    local.get $p2
    i32.store offset=12
    local.get $l5
    i32.load offset=8
    local.set $l6
    local.get $l5
    i32.load offset=12
    local.set $l7
    local.get $l5
    local.get $l6
    local.get $l7
    call $_ZN4core3num23_$LT$impl$u20$usize$GT$15overflowing_add17he76603bde0877d9eE
    local.get $l5
    i32.load align=1
    local.set $l8
    local.get $l5
    i32.load8_u offset=4
    local.set $l9
    local.get $l5
    local.get $l8
    i32.store offset=24
    i32.const 1
    local.set $l10
    local.get $l9
    local.get $l10
    i32.and
    local.set $l11
    local.get $l5
    local.get $l11
    i32.store8 offset=31
    local.get $l5
    i32.load8_u offset=31
    local.set $l12
    i32.const 1
    local.set $l13
    local.get $l12
    local.get $l13
    i32.and
    local.set $l14
    block $B0
      block $B1
        local.get $l14
        br_if $B1
        i32.const 1
        local.set $l15
        local.get $l5
        i32.load offset=24
        local.set $l16
        local.get $l5
        local.get $l16
        i32.store offset=20
        local.get $l5
        local.get $l15
        i32.store offset=16
        br $B0
      end
      i32.const 0
      local.set $l17
      local.get $l5
      local.get $l17
      i32.store offset=16
    end
    local.get $l5
    i32.load offset=16
    local.set $l18
    local.get $l5
    i32.load offset=20
    local.set $l19
    local.get $p0
    local.get $l19
    i32.store offset=4
    local.get $p0
    local.get $l18
    i32.store
    i32.const 32
    local.set $l20
    local.get $l5
    local.get $l20
    i32.add
    local.set $l21
    local.get $l21
    global.set $g0
    return)
  (func $_ZN4core3num23_$LT$impl$u20$usize$GT$15overflowing_add17he76603bde0877d9eE (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i32) (local $l15 i32) (local $l16 i32) (local $l17 i32) (local $l18 i32) (local $l19 i32)
    global.get $g0
    local.set $l3
    i32.const 32
    local.set $l4
    local.get $l3
    local.get $l4
    i32.sub
    local.set $l5
    local.get $l5
    local.get $p1
    i32.store
    local.get $l5
    local.get $p2
    i32.store offset=4
    local.get $l5
    i32.load
    local.set $l6
    local.get $l5
    i32.load offset=4
    local.set $l7
    local.get $l6
    local.get $l7
    i32.add
    local.set $l8
    local.get $l8
    local.get $l6
    i32.lt_u
    local.set $l9
    i32.const 1
    local.set $l10
    local.get $l9
    local.get $l10
    i32.and
    local.set $l11
    local.get $l5
    local.get $l8
    i32.store offset=24
    local.get $l5
    local.get $l11
    i32.store8 offset=28
    local.get $l5
    i32.load offset=24
    local.set $l12
    local.get $l5
    i32.load8_u offset=28
    local.set $l13
    local.get $l5
    local.get $l12
    i32.store offset=16
    i32.const 1
    local.set $l14
    local.get $l13
    local.get $l14
    i32.and
    local.set $l15
    local.get $l5
    local.get $l15
    i32.store8 offset=23
    local.get $l5
    i32.load offset=16
    local.set $l16
    local.get $l5
    i32.load8_u offset=23
    local.set $l17
    local.get $l5
    local.get $l16
    i32.store offset=8
    local.get $l5
    local.get $l17
    i32.store8 offset=12
    local.get $l5
    i32.load offset=8
    local.set $l18
    local.get $l5
    i32.load8_u offset=12
    local.set $l19
    local.get $p0
    local.get $l19
    i32.store8 offset=4
    local.get $p0
    local.get $l18
    i32.store
    return)
  (func $_ZN4core3ptr25swap_nonoverlapping_bytes17h06c22e7c65e0a6e2E (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i32) (local $l15 i32) (local $l16 i32) (local $l17 i32) (local $l18 i32) (local $l19 i32) (local $l20 i32) (local $l21 i32) (local $l22 i32) (local $l23 i32) (local $l24 i32) (local $l25 i32) (local $l26 i32) (local $l27 i64) (local $l28 i64) (local $l29 i64) (local $l30 i64) (local $l31 i32) (local $l32 i32) (local $l33 i32) (local $l34 i32) (local $l35 i32) (local $l36 i32) (local $l37 i32) (local $l38 i32) (local $l39 i32) (local $l40 i32) (local $l41 i32) (local $l42 i32) (local $l43 i32) (local $l44 i32) (local $l45 i32) (local $l46 i32) (local $l47 i32) (local $l48 i32) (local $l49 i32) (local $l50 i32) (local $l51 i32) (local $l52 i32) (local $l53 i32) (local $l54 i32) (local $l55 i32) (local $l56 i32) (local $l57 i32) (local $l58 i32) (local $l59 i32) (local $l60 i32) (local $l61 i32) (local $l62 i32) (local $l63 i32) (local $l64 i32) (local $l65 i32) (local $l66 i32) (local $l67 i32) (local $l68 i32) (local $l69 i32) (local $l70 i32) (local $l71 i32) (local $l72 i32) (local $l73 i32) (local $l74 i32) (local $l75 i32) (local $l76 i32)
    global.get $g0
    local.set $l3
    local.get $l3
    local.set $l4
    i32.const 192
    local.set $l5
    local.get $l3
    local.get $l5
    i32.sub
    local.set $l6
    i32.const -32
    local.set $l7
    local.get $l6
    local.get $l7
    i32.and
    local.set $l6
    local.get $l6
    global.set $g0
    i32.const 32
    local.set $l8
    local.get $l6
    local.get $p0
    i32.store offset=12
    local.get $l6
    local.get $p1
    i32.store offset=16
    local.get $l6
    local.get $p2
    i32.store offset=20
    local.get $l6
    local.get $l8
    i32.store offset=140
    local.get $l6
    i32.load offset=140
    local.set $l9
    local.get $l6
    local.get $l9
    i32.store offset=24
    i32.const 0
    local.set $l10
    local.get $l6
    local.get $l10
    i32.store offset=28
    block $B0
      block $B1
        loop $L2
          local.get $l6
          i32.load offset=28
          local.set $l11
          local.get $l6
          i32.load offset=24
          local.set $l12
          local.get $l11
          local.get $l12
          i32.add
          local.set $l13
          local.get $l6
          i32.load offset=20
          local.set $l14
          local.get $l13
          local.set $l15
          local.get $l14
          local.set $l16
          local.get $l15
          local.get $l16
          i32.le_u
          local.set $l17
          i32.const 1
          local.set $l18
          local.get $l17
          local.get $l18
          i32.and
          local.set $l19
          block $B3
            local.get $l19
            br_if $B3
            local.get $l6
            i32.load offset=28
            local.set $l20
            local.get $l6
            i32.load offset=20
            local.set $l21
            local.get $l20
            local.set $l22
            local.get $l21
            local.set $l23
            local.get $l22
            local.get $l23
            i32.lt_u
            local.set $l24
            i32.const 1
            local.set $l25
            local.get $l24
            local.get $l25
            i32.and
            local.set $l26
            local.get $l26
            br_if $B1
            br $B0
          end
          local.get $l6
          i64.load offset=160
          local.set $l27
          local.get $l6
          i64.load offset=168
          local.set $l28
          local.get $l6
          i64.load offset=176
          local.set $l29
          local.get $l6
          i64.load offset=184
          local.set $l30
          local.get $l6
          local.get $l30
          i64.store offset=56
          local.get $l6
          local.get $l29
          i64.store offset=48
          local.get $l6
          local.get $l28
          i64.store offset=40
          local.get $l6
          local.get $l27
          i64.store offset=32
          i32.const 32
          local.set $l31
          local.get $l6
          local.get $l31
          i32.add
          local.set $l32
          local.get $l32
          local.set $l33
          local.get $l6
          local.get $l33
          i32.store offset=152
          local.get $l6
          i32.load offset=152
          local.set $l34
          local.get $l6
          local.get $l34
          i32.store offset=156
          local.get $l6
          i32.load offset=156
          local.set $l35
          local.get $l6
          local.get $l35
          i32.store offset=76
          local.get $l6
          i32.load offset=12
          local.set $l36
          local.get $l6
          i32.load offset=28
          local.set $l37
          local.get $l36
          local.get $l37
          call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h765a127b5dbb9fa2E
          local.set $l38
          local.get $l6
          local.get $l38
          i32.store offset=80
          local.get $l6
          i32.load offset=16
          local.set $l39
          local.get $l6
          i32.load offset=28
          local.set $l40
          local.get $l39
          local.get $l40
          call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h765a127b5dbb9fa2E
          local.set $l41
          local.get $l6
          local.get $l41
          i32.store offset=84
          local.get $l6
          i32.load offset=80
          local.set $l42
          local.get $l6
          i32.load offset=76
          local.set $l43
          local.get $l6
          i32.load offset=24
          local.set $l44
          local.get $l42
          local.get $l43
          local.get $l44
          call $_ZN4core10intrinsics19copy_nonoverlapping17h949ee98b7b8a689eE
          local.get $l6
          i32.load offset=84
          local.set $l45
          local.get $l6
          i32.load offset=80
          local.set $l46
          local.get $l6
          i32.load offset=24
          local.set $l47
          local.get $l45
          local.get $l46
          local.get $l47
          call $_ZN4core10intrinsics19copy_nonoverlapping17h949ee98b7b8a689eE
          local.get $l6
          i32.load offset=76
          local.set $l48
          local.get $l6
          i32.load offset=84
          local.set $l49
          local.get $l6
          i32.load offset=24
          local.set $l50
          local.get $l48
          local.get $l49
          local.get $l50
          call $_ZN4core10intrinsics19copy_nonoverlapping17h949ee98b7b8a689eE
          local.get $l6
          i32.load offset=24
          local.set $l51
          local.get $l6
          i32.load offset=28
          local.set $l52
          local.get $l52
          local.get $l51
          i32.add
          local.set $l53
          local.get $l6
          local.get $l53
          i32.store offset=28
          br $L2
        end
      end
      i32.const 88
      local.set $l54
      local.get $l6
      local.get $l54
      i32.add
      local.set $l55
      local.get $l55
      local.set $l56
      local.get $l6
      i32.load offset=20
      local.set $l57
      local.get $l6
      i32.load offset=28
      local.set $l58
      local.get $l57
      local.get $l58
      i32.sub
      local.set $l59
      local.get $l6
      local.get $l59
      i32.store offset=124
      local.get $l6
      local.get $l56
      i32.store offset=144
      local.get $l6
      i32.load offset=144
      local.set $l60
      local.get $l6
      local.get $l60
      i32.store offset=148
      local.get $l6
      i32.load offset=148
      local.set $l61
      local.get $l6
      local.get $l61
      i32.store offset=128
      local.get $l6
      i32.load offset=12
      local.set $l62
      local.get $l6
      i32.load offset=28
      local.set $l63
      local.get $l62
      local.get $l63
      call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h765a127b5dbb9fa2E
      local.set $l64
      local.get $l6
      local.get $l64
      i32.store offset=132
      local.get $l6
      i32.load offset=16
      local.set $l65
      local.get $l6
      i32.load offset=28
      local.set $l66
      local.get $l65
      local.get $l66
      call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h765a127b5dbb9fa2E
      local.set $l67
      local.get $l6
      local.get $l67
      i32.store offset=136
      local.get $l6
      i32.load offset=132
      local.set $l68
      local.get $l6
      i32.load offset=128
      local.set $l69
      local.get $l6
      i32.load offset=124
      local.set $l70
      local.get $l68
      local.get $l69
      local.get $l70
      call $_ZN4core10intrinsics19copy_nonoverlapping17h949ee98b7b8a689eE
      local.get $l6
      i32.load offset=136
      local.set $l71
      local.get $l6
      i32.load offset=132
      local.set $l72
      local.get $l6
      i32.load offset=124
      local.set $l73
      local.get $l71
      local.get $l72
      local.get $l73
      call $_ZN4core10intrinsics19copy_nonoverlapping17h949ee98b7b8a689eE
      local.get $l6
      i32.load offset=128
      local.set $l74
      local.get $l6
      i32.load offset=136
      local.set $l75
      local.get $l6
      i32.load offset=124
      local.set $l76
      local.get $l74
      local.get $l75
      local.get $l76
      call $_ZN4core10intrinsics19copy_nonoverlapping17h949ee98b7b8a689eE
    end
    local.get $l4
    global.set $g0
    return)
  (func $receive (type $t0) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i32) (local $l15 i32) (local $l16 i32) (local $l17 i32) (local $l18 i32) (local $l19 i32) (local $l20 i32) (local $l21 i32) (local $l22 i32) (local $l23 i32) (local $l24 i32) (local $l25 i32) (local $l26 i32) (local $l27 i32) (local $l28 i32) (local $l29 i32) (local $l30 i32) (local $l31 i32) (local $l32 i32) (local $l33 i32) (local $l34 i32) (local $l35 i32) (local $l36 i32) (local $l37 i32) (local $l38 i32) (local $l39 i32) (local $l40 i32) (local $l41 i32) (local $l42 i32) (local $l43 i32) (local $l44 i32)
    global.get $g0
    local.set $l2
    i32.const 80
    local.set $l3
    local.get $l2
    local.get $l3
    i32.sub
    local.set $l4
    local.get $l4
    global.set $g0
    i32.const 0
    local.set $l5
    local.get $l4
    local.get $p0
    i32.store offset=16
    local.get $l4
    local.get $p1
    i32.store offset=20
    local.get $l4
    i32.load offset=16
    local.set $l6
    local.get $l4
    local.get $l6
    i32.store offset=24
    local.get $l4
    local.get $l5
    i32.store offset=28
    local.get $l4
    i32.load offset=20
    local.set $l7
    local.get $l4
    local.get $l5
    i32.store offset=32
    local.get $l4
    local.get $l7
    i32.store offset=36
    local.get $l4
    i32.load offset=32
    local.set $l8
    local.get $l4
    i32.load offset=36
    local.set $l9
    i32.const 8
    local.set $l10
    local.get $l4
    local.get $l10
    i32.add
    local.set $l11
    local.get $l11
    local.get $l8
    local.get $l9
    call $_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h5ad4046268c475aaE
    local.get $l4
    i32.load offset=12 align=1
    local.set $l12
    local.get $l4
    i32.load offset=8 align=1
    local.set $l13
    local.get $l4
    local.get $l13
    i32.store offset=40
    local.get $l4
    local.get $l12
    i32.store offset=44
    block $B0
      loop $L1
        i32.const 40
        local.set $l14
        local.get $l4
        local.get $l14
        i32.add
        local.set $l15
        local.get $l4
        local.get $l15
        call $_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17h97b876963eccc90fE
        local.get $l4
        i32.load align=1
        local.set $l16
        local.get $l4
        i32.load offset=4 align=1
        local.set $l17
        local.get $l4
        local.get $l17
        i32.store offset=60
        local.get $l4
        local.get $l16
        i32.store offset=56
        local.get $l4
        i32.load offset=56
        local.set $l18
        block $B2
          block $B3
            block $B4
              local.get $l18
              br_table $B4 $B3 $B4
            end
            local.get $l4
            i32.load offset=24
            local.set $l19
            local.get $l4
            i32.load offset=20
            local.set $l20
            local.get $l19
            local.get $l20
            call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h396a8cf6f8a9d6beE
            local.set $l21
            local.get $l4
            local.get $l21
            i32.store offset=76
            br $B2
          end
          local.get $l4
          i32.load offset=60
          local.set $l22
          local.get $l4
          local.get $l22
          i32.store offset=68
          local.get $l4
          i32.load offset=68
          local.set $l23
          local.get $l4
          local.get $l23
          i32.store offset=52
          local.get $l4
          i32.load offset=52
          local.set $l24
          local.get $l4
          local.get $l24
          i32.store offset=72
          local.get $l4
          i32.load offset=24
          local.set $l25
          local.get $l4
          i32.load offset=72
          local.set $l26
          local.get $l25
          local.get $l26
          call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h396a8cf6f8a9d6beE
          local.set $l27
          local.get $l27
          i32.load
          local.set $l28
          local.get $l4
          i32.load offset=28
          local.set $l29
          local.get $l29
          local.get $l28
          i32.add
          local.set $l30
          local.get $l30
          local.get $l29
          i32.lt_u
          local.set $l31
          i32.const 1
          local.set $l32
          local.get $l31
          local.get $l32
          i32.and
          local.set $l33
          local.get $l33
          br_if $B0
          local.get $l4
          local.get $l30
          i32.store offset=28
          br $L1
        end
      end
      i32.const 1
      local.set $l34
      local.get $l4
      i32.load offset=28
      local.set $l35
      local.get $l4
      i32.load offset=76
      local.set $l36
      local.get $l36
      local.get $l35
      i32.store
      local.get $l4
      i32.load offset=76
      local.set $l37
      local.get $l37
      local.get $l34
      call $send
      i32.const 80
      local.set $l38
      local.get $l4
      local.get $l38
      i32.add
      local.set $l39
      local.get $l39
      global.set $g0
      return
    end
    i32.const 1048608
    local.set $l40
    local.get $l40
    local.set $l41
    i32.const 28
    local.set $l42
    i32.const 1048592
    local.set $l43
    local.get $l43
    local.set $l44
    local.get $l41
    local.get $l42
    local.get $l44
    call $_ZN4core9panicking5panic17h8634ac164c1f3136E
    unreachable
    unreachable)
  (func $rust_begin_unwind (type $t4) (param $p0 i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32)
    global.get $g0
    local.set $l1
    i32.const 16
    local.set $l2
    local.get $l1
    local.get $l2
    i32.sub
    local.set $l3
    local.get $l3
    local.get $p0
    i32.store offset=12
    loop $L0
      br $L0
    end)
  (func $_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17h97b876963eccc90fE (type $t0) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i32) (local $l15 i32) (local $l16 i32) (local $l17 i32) (local $l18 i32) (local $l19 i32) (local $l20 i32) (local $l21 i32) (local $l22 i32) (local $l23 i32) (local $l24 i32) (local $l25 i32) (local $l26 i32) (local $l27 i32) (local $l28 i32) (local $l29 i32) (local $l30 i32) (local $l31 i32) (local $l32 i32) (local $l33 i32) (local $l34 i32) (local $l35 i32) (local $l36 i32) (local $l37 i32) (local $l38 i32) (local $l39 i32) (local $l40 i32) (local $l41 i32) (local $l42 i32) (local $l43 i32) (local $l44 i32) (local $l45 i32) (local $l46 i32) (local $l47 i32) (local $l48 i32) (local $l49 i32) (local $l50 i32) (local $l51 i32)
    global.get $g0
    local.set $l2
    i32.const 48
    local.set $l3
    local.get $l2
    local.get $l3
    i32.sub
    local.set $l4
    local.get $l4
    global.set $g0
    i32.const 0
    local.set $l5
    local.get $l4
    local.get $p1
    i32.store offset=20
    local.get $l4
    local.get $l5
    i32.store8 offset=47
    local.get $l4
    i32.load offset=20
    local.set $l6
    local.get $l4
    i32.load offset=20
    local.set $l7
    i32.const 4
    local.set $l8
    local.get $l7
    local.get $l8
    i32.add
    local.set $l9
    local.get $l6
    local.get $l9
    call $_ZN4core3cmp5impls57_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$usize$GT$2lt17h14373f30e52f06d9E
    local.set $l10
    i32.const 1
    local.set $l11
    local.get $l10
    local.get $l11
    i32.and
    local.set $l12
    block $B0
      block $B1
        local.get $l12
        br_if $B1
        i32.const 0
        local.set $l13
        local.get $l4
        local.get $l13
        i32.store offset=24
        br $B0
      end
      local.get $l4
      i32.load offset=20
      local.set $l14
      i32.const 1
      local.set $l15
      local.get $l4
      local.get $l15
      i32.store8 offset=47
      i32.const 8
      local.set $l16
      local.get $l4
      local.get $l16
      i32.add
      local.set $l17
      local.get $l17
      local.get $l14
      local.get $l15
      call $_ZN49_$LT$usize$u20$as$u20$core..iter..range..Step$GT$9add_usize17h900b1ef28267dcf9E
      local.get $l4
      i32.load offset=8 align=1
      local.set $l18
      local.get $l4
      i32.load offset=12 align=1
      local.set $l19
      local.get $l4
      local.get $l19
      i32.store offset=36
      local.get $l4
      local.get $l18
      i32.store offset=32
      i32.const 1
      local.set $l20
      local.get $l4
      i32.load offset=32
      local.set $l21
      local.get $l21
      local.set $l22
      local.get $l20
      local.set $l23
      local.get $l22
      local.get $l23
      i32.eq
      local.set $l24
      i32.const 1
      local.set $l25
      local.get $l24
      local.get $l25
      i32.and
      local.set $l26
      block $B2
        block $B3
          local.get $l26
          br_if $B3
          i32.const 0
          local.set $l27
          local.get $l4
          local.get $l27
          i32.store offset=24
          br $B2
        end
        i32.const 40
        local.set $l28
        local.get $l4
        local.get $l28
        i32.add
        local.set $l29
        local.get $l29
        local.set $l30
        i32.const 0
        local.set $l31
        local.get $l4
        local.get $l31
        i32.store8 offset=47
        local.get $l4
        i32.load offset=36
        local.set $l32
        local.get $l4
        local.get $l32
        i32.store offset=40
        local.get $l4
        i32.load offset=20
        local.set $l33
        local.get $l30
        local.get $l33
        call $_ZN4core3mem4swap17ha54ca3edc38de5b6E
        i32.const 1
        local.set $l34
        local.get $l4
        i32.load offset=40
        local.set $l35
        local.get $l4
        local.get $l35
        i32.store offset=28
        local.get $l4
        local.get $l34
        i32.store offset=24
      end
      i32.const 1
      local.set $l36
      local.get $l4
      i32.load offset=32
      local.set $l37
      local.get $l37
      local.set $l38
      local.get $l36
      local.set $l39
      local.get $l38
      local.get $l39
      i32.eq
      local.set $l40
      i32.const 1
      local.set $l41
      local.get $l40
      local.get $l41
      i32.and
      local.set $l42
      block $B4
        block $B5
          local.get $l42
          br_if $B5
          br $B4
        end
        local.get $l4
        i32.load8_u offset=47
        local.set $l43
        i32.const 1
        local.set $l44
        local.get $l43
        local.get $l44
        i32.and
        local.set $l45
        local.get $l45
        i32.eqz
        br_if $B4
        i32.const 0
        local.set $l46
        local.get $l4
        local.get $l46
        i32.store8 offset=47
      end
      i32.const 0
      local.set $l47
      local.get $l4
      local.get $l47
      i32.store8 offset=47
    end
    local.get $l4
    i32.load offset=24
    local.set $l48
    local.get $l4
    i32.load offset=28
    local.set $l49
    local.get $p0
    local.get $l49
    i32.store offset=4
    local.get $p0
    local.get $l48
    i32.store
    i32.const 48
    local.set $l50
    local.get $l4
    local.get $l50
    i32.add
    local.set $l51
    local.get $l51
    global.set $g0
    return)
  (func $_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h5ad4046268c475aaE (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32)
    global.get $g0
    local.set $l3
    i32.const 16
    local.set $l4
    local.get $l3
    local.get $l4
    i32.sub
    local.set $l5
    local.get $l5
    local.get $p1
    i32.store offset=8
    local.get $l5
    local.get $p2
    i32.store offset=12
    local.get $l5
    i32.load offset=8
    local.set $l6
    local.get $l5
    i32.load offset=12
    local.set $l7
    local.get $p0
    local.get $l7
    i32.store offset=4
    local.get $p0
    local.get $l6
    i32.store
    return)
  (func $_ZN4core10intrinsics19copy_nonoverlapping17h949ee98b7b8a689eE (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32)
    global.get $g0
    local.set $l3
    i32.const 16
    local.set $l4
    local.get $l3
    local.get $l4
    i32.sub
    local.set $l5
    local.get $l5
    global.set $g0
    local.get $l5
    local.get $p0
    i32.store offset=4
    local.get $l5
    local.get $p1
    i32.store offset=8
    local.get $l5
    local.get $p2
    i32.store offset=12
    local.get $l5
    i32.load offset=4
    local.set $l6
    local.get $l5
    i32.load offset=8
    local.set $l7
    local.get $l5
    i32.load offset=12
    local.set $l8
    i32.const 0
    local.set $l9
    local.get $l8
    local.get $l9
    i32.shl
    local.set $l10
    local.get $l7
    local.get $l6
    local.get $l10
    call $memcpy
    drop
    i32.const 16
    local.set $l11
    local.get $l5
    local.get $l11
    i32.add
    local.set $l12
    local.get $l12
    global.set $g0
    return)
  (func $_ZN4core10intrinsics19copy_nonoverlapping17hb302ec6b1402a42eE (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32)
    global.get $g0
    local.set $l3
    i32.const 16
    local.set $l4
    local.get $l3
    local.get $l4
    i32.sub
    local.set $l5
    local.get $l5
    global.set $g0
    local.get $l5
    local.get $p0
    i32.store offset=4
    local.get $l5
    local.get $p1
    i32.store offset=8
    local.get $l5
    local.get $p2
    i32.store offset=12
    local.get $l5
    i32.load offset=4
    local.set $l6
    local.get $l5
    i32.load offset=8
    local.set $l7
    local.get $l5
    i32.load offset=12
    local.set $l8
    i32.const 2
    local.set $l9
    local.get $l8
    local.get $l9
    i32.shl
    local.set $l10
    local.get $l7
    local.get $l6
    local.get $l10
    call $memcpy
    drop
    i32.const 16
    local.set $l11
    local.get $l5
    local.get $l11
    i32.add
    local.set $l12
    local.get $l12
    global.set $g0
    return)
  (func $_ZN4core3cmp5impls57_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$usize$GT$2lt17h14373f30e52f06d9E (type $t3) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32)
    global.get $g0
    local.set $l2
    i32.const 16
    local.set $l3
    local.get $l2
    local.get $l3
    i32.sub
    local.set $l4
    local.get $l4
    local.get $p0
    i32.store offset=8
    local.get $l4
    local.get $p1
    i32.store offset=12
    local.get $l4
    i32.load offset=8
    local.set $l5
    local.get $l5
    i32.load
    local.set $l6
    local.get $l4
    i32.load offset=12
    local.set $l7
    local.get $l7
    i32.load
    local.set $l8
    local.get $l6
    local.set $l9
    local.get $l8
    local.set $l10
    local.get $l9
    local.get $l10
    i32.lt_u
    local.set $l11
    i32.const 1
    local.set $l12
    local.get $l11
    local.get $l12
    i32.and
    local.set $l13
    local.get $l13
    return)
  (func $_ZN4core3mem4swap17ha54ca3edc38de5b6E (type $t0) (param $p0 i32) (param $p1 i32)
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
    call $_ZN4core3ptr23swap_nonoverlapping_one17h239a439e42f2e0bcE
    i32.const 16
    local.set $l7
    local.get $l4
    local.get $l7
    i32.add
    local.set $l8
    local.get $l8
    global.set $g0
    return)
  (func $_ZN4core3ptr18real_drop_in_place17h92d8df8ff4cc66e7E (type $t4) (param $p0 i32))
  (func $_ZN4core9panicking5panic17h8634ac164c1f3136E (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    i64.const 4
    i64.store offset=16
    local.get $l3
    i64.const 1
    i64.store offset=4 align=4
    local.get $l3
    local.get $p1
    i32.store offset=28
    local.get $l3
    local.get $p0
    i32.store offset=24
    local.get $l3
    local.get $l3
    i32.const 24
    i32.add
    i32.store
    local.get $l3
    local.get $p2
    call $_ZN4core9panicking9panic_fmt17hf798d32b3aba5420E
    unreachable)
  (func $_ZN4core9panicking9panic_fmt17hf798d32b3aba5420E (type $t0) (param $p0 i32) (param $p1 i32)
    (local $l2 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    local.get $p1
    i32.store offset=12
    local.get $l2
    local.get $p0
    i32.store offset=8
    local.get $l2
    i32.const 1048636
    i32.store offset=4
    local.get $l2
    i32.const 1
    i32.store
    local.get $l2
    call $rust_begin_unwind
    unreachable)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h4b559cca434c02ebE (type $t5) (param $p0 i32) (result i64)
    i64.const -237851497739055091)
  (func $memcpy (type $t6) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32)
    block $B0
      local.get $p2
      i32.eqz
      br_if $B0
      local.get $p0
      local.set $l3
      loop $L1
        local.get $l3
        local.get $p1
        i32.load8_u
        i32.store8
        local.get $l3
        i32.const 1
        i32.add
        local.set $l3
        local.get $p1
        i32.const 1
        i32.add
        local.set $p1
        local.get $p2
        i32.const -1
        i32.add
        local.tee $p2
        br_if $L1
      end
    end
    local.get $p0)
  (table $T0 3 3 funcref)
  (memory $memory 17)
  (global $g0 (mut i32) (i32.const 1048576))
  (global $__data_end i32 (i32.const 1048652))
  (global $__heap_base i32 (i32.const 1048652))
  (export "memory" (memory 0))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (export "receive" (func $receive))
  (elem $e0 (i32.const 1) $_ZN4core3ptr18real_drop_in_place17h92d8df8ff4cc66e7E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h4b559cca434c02ebE)
  (data $d0 (i32.const 1048576) "add\5csrc\5cmain.rs\00\00\00\10\00\0f\00\00\00\11\00\00\00\09\00\00\00attempt to add with overflow\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00"))
