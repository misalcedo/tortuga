(module
  (type $t0 (func (param i32 i32)))
  (type $t1 (func (param i32 i32 i32) (result i32)))
  (type $t2 (func (param i32 i32) (result i32)))
  (type $t3 (func (param i32 i32 i32)))
  (type $t4 (func (param i32) (result i32)))
  (type $t5 (func (param i32 i32 i32 i32) (result i32)))
  (type $t6 (func (param i32) (result i64)))
  (type $t7 (func (param i32)))
  (type $t8 (func (param i32 i32 i32 i32)))
  (type $t9 (func))
  (type $t10 (func (param i32 i32 i32 i32 i32)))
  (type $t11 (func (param i64 i32 i32) (result i32)))
  (type $t12 (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (import "system" "send" (func $send (type $t0)))
  (func $_ZN4core3cmp5impls57_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$usize$GT$2lt17h60e6bb0825003046E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
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
  (func $_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_add17h17d4e3fe8fbaf5a4E (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
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
    call $_ZN4core3num23_$LT$impl$u20$usize$GT$15overflowing_add17h1ea0670d57ee4568E
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
  (func $_ZN4core3num23_$LT$impl$u20$usize$GT$15overflowing_add17h1ea0670d57ee4568E (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
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
  (func $_ZN4core3mem4swap17h37ecc353f6f4b476E (type $t0) (param $p0 i32) (param $p1 i32)
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
    call $_ZN4core3ptr23swap_nonoverlapping_one17h53222b37b02ce306E
    i32.const 16
    local.set $l7
    local.get $l4
    local.get $l7
    i32.add
    local.set $l8
    local.get $l8
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
    call $_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h4333194449eef9a7E
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
        call $_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17h802c96e6b0c32abcE
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
            call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h3ebab04155a98e54E
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
          call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h3ebab04155a98e54E
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
  (func $_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h57dd7494a6ffe6c0E (type $t4) (param $p0 i32) (result i32)
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
  (func $_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h15dd7f0b7e2697abE (type $t4) (param $p0 i32) (result i32)
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
    call $_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h57dd7494a6ffe6c0E
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
  (func $_ZN53_$LT$T$u20$as$u20$core..convert..TryFrom$LT$U$GT$$GT$8try_from17h4e6a9792194d931cE (type $t4) (param $p0 i32) (result i32)
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
    call $_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h15dd7f0b7e2697abE
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
  (func $_ZN4core10intrinsics19copy_nonoverlapping17h7e90dcf1c2a4e7eaE (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
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
  (func $_ZN4core10intrinsics19copy_nonoverlapping17h8a91ab5158af1492E (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
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
  (func $_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17h802c96e6b0c32abcE (type $t0) (param $p0 i32) (param $p1 i32)
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
    call $_ZN4core3cmp5impls57_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$usize$GT$2lt17h60e6bb0825003046E
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
      call $_ZN49_$LT$usize$u20$as$u20$core..iter..range..Step$GT$9add_usize17hab1810d75c9076e1E
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
        call $_ZN4core3mem4swap17h37ecc353f6f4b476E
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
  (func $_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h4333194449eef9a7E (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
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
  (func $_ZN49_$LT$usize$u20$as$u20$core..iter..range..Step$GT$9add_usize17hab1810d75c9076e1E (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
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
    call $_ZN53_$LT$T$u20$as$u20$core..convert..TryFrom$LT$U$GT$$GT$8try_from17h4e6a9792194d931cE
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
    call $_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_add17h17d4e3fe8fbaf5a4E
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
  (func $_ZN4core3ptr19swap_nonoverlapping17h9706af556264bcd4E (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
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
    call $_ZN4core3ptr25swap_nonoverlapping_bytes17hb450f6428add72edE
    i32.const 32
    local.set $l15
    local.get $l5
    local.get $l15
    i32.add
    local.set $l16
    local.get $l16
    global.set $g0
    return)
  (func $_ZN4core3ptr23swap_nonoverlapping_one17h53222b37b02ce306E (type $t0) (param $p0 i32) (param $p1 i32)
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
          call $_ZN4core3ptr19swap_nonoverlapping17h9706af556264bcd4E
          br $B1
        end
        local.get $l4
        i32.load
        local.set $l16
        local.get $l16
        call $_ZN4core3ptr4read17h15025caece29c82cE
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
        call $_ZN4core10intrinsics19copy_nonoverlapping17h8a91ab5158af1492E
        local.get $l4
        i32.load offset=4
        local.set $l21
        local.get $l4
        i32.load offset=8
        local.set $l22
        local.get $l21
        local.get $l22
        call $_ZN4core3ptr5write17h80c654da7c1fdcc9E
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
  (func $_ZN4core3ptr4read17h15025caece29c82cE (type $t4) (param $p0 i32) (result i32)
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
    call $_ZN4core10intrinsics19copy_nonoverlapping17h8a91ab5158af1492E
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
  (func $_ZN4core3ptr5write17h80c654da7c1fdcc9E (type $t0) (param $p0 i32) (param $p1 i32)
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
  (func $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h29473964f3461ab7E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
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
    call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h05666c7b7eb6c67fE
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
  (func $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h05666c7b7eb6c67fE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
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
  (func $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h3ebab04155a98e54E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
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
    call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h9934efbdbffc8a1eE
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
  (func $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h9934efbdbffc8a1eE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
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
  (func $_ZN4core3ptr25swap_nonoverlapping_bytes17hb450f6428add72edE (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
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
          call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h29473964f3461ab7E
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
          call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h29473964f3461ab7E
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
          call $_ZN4core10intrinsics19copy_nonoverlapping17h7e90dcf1c2a4e7eaE
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
          call $_ZN4core10intrinsics19copy_nonoverlapping17h7e90dcf1c2a4e7eaE
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
          call $_ZN4core10intrinsics19copy_nonoverlapping17h7e90dcf1c2a4e7eaE
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
      call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h29473964f3461ab7E
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
      call $_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h29473964f3461ab7E
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
      call $_ZN4core10intrinsics19copy_nonoverlapping17h7e90dcf1c2a4e7eaE
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
      call $_ZN4core10intrinsics19copy_nonoverlapping17h7e90dcf1c2a4e7eaE
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
      call $_ZN4core10intrinsics19copy_nonoverlapping17h7e90dcf1c2a4e7eaE
    end
    local.get $l4
    global.set $g0
    return)
  (func $__rust_alloc (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32)
    local.get $p0
    local.get $p1
    call $__rdl_alloc
    local.set $l2
    local.get $l2
    return)
  (func $__rust_dealloc (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    local.get $p0
    local.get $p1
    local.get $p2
    call $__rdl_dealloc
    return)
  (func $__rust_realloc (type $t5) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (result i32)
    (local $l4 i32)
    local.get $p0
    local.get $p1
    local.get $p2
    local.get $p3
    call $__rdl_realloc
    local.set $l4
    local.get $l4
    return)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3eff61b58f08666fE (type $t6) (param $p0 i32) (result i64)
    i64.const -237851497739055091)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hb5328de7baf27548E (type $t6) (param $p0 i32) (result i64)
    i64.const 7878694554369134032)
  (func $_ZN4core3ptr18real_drop_in_place17h06d59ef2124c7ddcE (type $t7) (param $p0 i32))
  (func $_ZN4core3ptr18real_drop_in_place17h23cbb650c6fd0cddE (type $t7) (param $p0 i32)
    (local $l1 i32)
    block $B0
      local.get $p0
      i32.load offset=4
      local.tee $l1
      i32.eqz
      br_if $B0
      local.get $p0
      i32.load
      local.get $l1
      i32.const 1
      call $__rust_dealloc
    end)
  (func $_ZN4core3ptr18real_drop_in_place17hf704f9372cabec9cE (type $t7) (param $p0 i32)
    (local $l1 i32)
    block $B0
      local.get $p0
      i32.load offset=4
      local.tee $l1
      i32.eqz
      br_if $B0
      local.get $p0
      i32.const 8
      i32.add
      i32.load
      local.tee $p0
      i32.eqz
      br_if $B0
      local.get $l1
      local.get $p0
      i32.const 1
      call $__rust_dealloc
    end)
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17h0fa2a855686a822bE (type $t4) (param $p0 i32) (result i32)
    block $B0
      local.get $p0
      br_if $B0
      i32.const 1048768
      i32.const 43
      i32.const 1048736
      call $_ZN4core9panicking5panic17h8634ac164c1f3136E
      unreachable
    end
    local.get $p0)
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17h99a3f707fcc83232E (type $t4) (param $p0 i32) (result i32)
    block $B0
      local.get $p0
      br_if $B0
      i32.const 1048768
      i32.const 43
      i32.const 1048736
      call $_ZN4core9panicking5panic17h8634ac164c1f3136E
      unreachable
    end
    local.get $p0)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h57b0b1f21c89129dE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $p0
    i32.load
    local.set $p0
    block $B0
      block $B1
        block $B2
          block $B3
            local.get $p1
            i32.const 128
            i32.lt_u
            br_if $B3
            local.get $l2
            i32.const 0
            i32.store offset=12
            local.get $p1
            i32.const 2048
            i32.lt_u
            br_if $B2
            block $B4
              local.get $p1
              i32.const 65536
              i32.ge_u
              br_if $B4
              local.get $l2
              local.get $p1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get $l2
              local.get $p1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              local.get $l2
              local.get $p1
              i32.const 12
              i32.shr_u
              i32.const 15
              i32.and
              i32.const 224
              i32.or
              i32.store8 offset=12
              i32.const 3
              local.set $p1
              br $B1
            end
            local.get $l2
            local.get $p1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get $l2
            local.get $p1
            i32.const 18
            i32.shr_u
            i32.const 240
            i32.or
            i32.store8 offset=12
            local.get $l2
            local.get $p1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get $l2
            local.get $p1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 4
            local.set $p1
            br $B1
          end
          block $B5
            local.get $p0
            i32.load offset=8
            local.tee $l3
            local.get $p0
            i32.load offset=4
            i32.ne
            br_if $B5
            local.get $p0
            i32.const 1
            call $_ZN5alloc3vec12Vec$LT$T$GT$7reserve17h670ffb26bd53d582E
            local.get $p0
            i32.load offset=8
            local.set $l3
          end
          local.get $p0
          i32.load
          local.get $l3
          i32.add
          local.get $p1
          i32.store8
          local.get $p0
          local.get $p0
          i32.load offset=8
          i32.const 1
          i32.add
          i32.store offset=8
          br $B0
        end
        local.get $l2
        local.get $p1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get $l2
        local.get $p1
        i32.const 6
        i32.shr_u
        i32.const 31
        i32.and
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set $p1
      end
      local.get $p0
      local.get $p1
      call $_ZN5alloc3vec12Vec$LT$T$GT$7reserve17h670ffb26bd53d582E
      local.get $p0
      local.get $p0
      i32.load offset=8
      local.tee $l3
      local.get $p1
      i32.add
      i32.store offset=8
      local.get $l3
      local.get $p0
      i32.load
      i32.add
      local.get $l2
      i32.const 12
      i32.add
      local.get $p1
      call $memcpy
      drop
    end
    local.get $l2
    i32.const 16
    i32.add
    global.set $g0
    i32.const 0)
  (func $_ZN5alloc3vec12Vec$LT$T$GT$7reserve17h670ffb26bd53d582E (type $t0) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32)
    block $B0
      block $B1
        block $B2
          local.get $p0
          i32.load offset=4
          local.tee $l2
          local.get $p0
          i32.load offset=8
          local.tee $l3
          i32.sub
          local.get $p1
          i32.ge_u
          br_if $B2
          local.get $l3
          local.get $p1
          i32.add
          local.tee $p1
          local.get $l3
          i32.lt_u
          br_if $B0
          local.get $l2
          i32.const 1
          i32.shl
          local.tee $l3
          local.get $p1
          local.get $l3
          local.get $p1
          i32.gt_u
          select
          local.tee $p1
          i32.const 0
          i32.lt_s
          br_if $B0
          block $B3
            block $B4
              local.get $l2
              br_if $B4
              local.get $p1
              i32.const 1
              call $__rust_alloc
              local.set $l2
              br $B3
            end
            local.get $p0
            i32.load
            local.get $l2
            i32.const 1
            local.get $p1
            call $__rust_realloc
            local.set $l2
          end
          local.get $l2
          i32.eqz
          br_if $B1
          local.get $p0
          local.get $p1
          i32.store offset=4
          local.get $p0
          local.get $l2
          i32.store
        end
        return
      end
      local.get $p1
      i32.const 1
      call $_ZN5alloc5alloc18handle_alloc_error17hf1cfd628decbe217E
      unreachable
    end
    call $_ZN5alloc7raw_vec17capacity_overflow17hd5e3f0106831a51cE
    unreachable)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h23ab4e8c6c4a792aE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    local.get $p0
    i32.load
    i32.store offset=4
    local.get $l2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get $p1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get $p1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get $l2
    local.get $p1
    i64.load align=4
    i64.store offset=8
    local.get $l2
    i32.const 4
    i32.add
    i32.const 1048636
    local.get $l2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h7b0c1564f380f2eeE
    local.set $p1
    local.get $l2
    i32.const 32
    i32.add
    global.set $g0
    local.get $p1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17hcdd4c55de6a15b21E (type $t1) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32)
    local.get $p0
    i32.load
    local.tee $p0
    local.get $p2
    call $_ZN5alloc3vec12Vec$LT$T$GT$7reserve17h670ffb26bd53d582E
    local.get $p0
    local.get $p0
    i32.load offset=8
    local.tee $l3
    local.get $p2
    i32.add
    i32.store offset=8
    local.get $l3
    local.get $p0
    i32.load
    i32.add
    local.get $p1
    local.get $p2
    call $memcpy
    drop
    i32.const 0)
  (func $_ZN76_$LT$std..sys_common..thread_local..Key$u20$as$u20$core..ops..drop..Drop$GT$4drop17hd885bcda30b950bdE (type $t7) (param $p0 i32))
  (func $_ZN3std5alloc24default_alloc_error_hook17hc30c66deb02056a9E (type $t0) (param $p0 i32) (param $p1 i32))
  (func $rust_oom (type $t0) (param $p0 i32) (param $p1 i32)
    (local $l2 i32)
    local.get $p0
    local.get $p1
    i32.const 0
    i32.load offset=1049328
    local.tee $l2
    i32.const 1
    local.get $l2
    select
    call_indirect (type $t0) $T0
    unreachable
    unreachable)
  (func $__rdl_alloc (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    block $B0
      i32.const 1049344
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc16malloc_alignment17h533bf653182825f3E
      local.get $p1
      i32.ge_u
      br_if $B0
      i32.const 1049344
      local.get $p1
      local.get $p0
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc8memalign17h5a02630834393434E
      return
    end
    i32.const 1049344
    local.get $p0
    call $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17h40c3085ef28e063eE)
  (func $__rdl_dealloc (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    i32.const 1049344
    local.get $p0
    call $_ZN8dlmalloc8dlmalloc8Dlmalloc4free17hd1431ee938d9d15eE)
  (func $__rdl_realloc (type $t5) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (result i32)
    block $B0
      block $B1
        i32.const 1049344
        call $_ZN8dlmalloc8dlmalloc8Dlmalloc16malloc_alignment17h533bf653182825f3E
        local.get $p2
        i32.ge_u
        br_if $B1
        block $B2
          block $B3
            i32.const 1049344
            call $_ZN8dlmalloc8dlmalloc8Dlmalloc16malloc_alignment17h533bf653182825f3E
            local.get $p2
            i32.ge_u
            br_if $B3
            i32.const 1049344
            local.get $p2
            local.get $p3
            call $_ZN8dlmalloc8dlmalloc8Dlmalloc8memalign17h5a02630834393434E
            local.set $p2
            br $B2
          end
          i32.const 1049344
          local.get $p3
          call $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17h40c3085ef28e063eE
          local.set $p2
        end
        local.get $p2
        br_if $B0
        i32.const 0
        return
      end
      i32.const 1049344
      local.get $p0
      local.get $p3
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc7realloc17h7c736378844fed2aE
      return
    end
    local.get $p2
    local.get $p0
    local.get $p3
    local.get $p1
    local.get $p1
    local.get $p3
    i32.gt_u
    select
    call $memcpy
    local.set $p2
    i32.const 1049344
    local.get $p0
    call $_ZN8dlmalloc8dlmalloc8Dlmalloc4free17hd1431ee938d9d15eE
    local.get $p2)
  (func $rust_begin_unwind (type $t7) (param $p0 i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i64) (local $l5 i32)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l1
    global.set $g0
    local.get $p0
    call $_ZN4core5panic9PanicInfo8location17h3e3164c4042e2a35E
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17h99a3f707fcc83232E
    local.set $l2
    local.get $p0
    call $_ZN4core5panic9PanicInfo7message17h64849a473b74b55fE
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17h0fa2a855686a822bE
    local.set $l3
    local.get $l1
    i32.const 8
    i32.add
    local.get $l2
    call $_ZN4core5panic8Location4file17hf03dc98b95c00bcbE
    local.get $l1
    i64.load offset=8
    local.set $l4
    local.get $l2
    call $_ZN4core5panic8Location4line17h11b7e9ca374cd1e0E
    local.set $l5
    local.get $l1
    local.get $l2
    call $_ZN4core5panic8Location6column17h202347b4fd2792afE
    i32.store offset=28
    local.get $l1
    local.get $l5
    i32.store offset=24
    local.get $l1
    local.get $l4
    i64.store offset=16
    local.get $l1
    i32.const 0
    i32.store offset=36
    local.get $l1
    local.get $l3
    i32.store offset=32
    local.get $l1
    i32.const 32
    i32.add
    i32.const 1048812
    local.get $p0
    call $_ZN4core5panic9PanicInfo7message17h64849a473b74b55fE
    local.get $l1
    i32.const 16
    i32.add
    call $_ZN3std9panicking20rust_panic_with_hook17h4a576d7e01b2f2f2E
    unreachable)
  (func $_ZN3std9panicking20rust_panic_with_hook17h4a576d7e01b2f2f2E (type $t8) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32)
    (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l4
    global.set $g0
    i32.const 1
    local.set $l5
    local.get $p3
    i32.load offset=12
    local.set $l6
    local.get $p3
    i32.load offset=8
    local.set $l7
    local.get $p3
    i32.load offset=4
    local.set $l8
    local.get $p3
    i32.load
    local.set $p3
    block $B0
      block $B1
        block $B2
          block $B3
            i32.const 0
            i32.load offset=1049800
            i32.const 1
            i32.eq
            br_if $B3
            i32.const 0
            i64.const 4294967297
            i64.store offset=1049800
            br $B2
          end
          i32.const 0
          i32.const 0
          i32.load offset=1049804
          i32.const 1
          i32.add
          local.tee $l5
          i32.store offset=1049804
          local.get $l5
          i32.const 2
          i32.gt_u
          br_if $B1
        end
        local.get $l4
        i32.const 16
        i32.add
        local.get $p3
        local.get $l8
        local.get $l7
        local.get $l6
        call $_ZN4core5panic8Location20internal_constructor17h8892f1c8e0dbead5E
        local.get $l4
        local.get $p2
        i32.store offset=40
        local.get $l4
        i32.const 1048752
        i32.store offset=36
        local.get $l4
        i32.const 1
        i32.store offset=32
        i32.const 0
        i32.load offset=1049332
        local.set $p3
        local.get $l4
        local.get $l4
        i32.const 16
        i32.add
        i32.store offset=44
        local.get $p3
        i32.const -1
        i32.le_s
        br_if $B1
        i32.const 0
        local.get $p3
        i32.const 1
        i32.add
        local.tee $p3
        i32.store offset=1049332
        block $B4
          i32.const 0
          i32.load offset=1049340
          local.tee $p2
          i32.eqz
          br_if $B4
          i32.const 0
          i32.load offset=1049336
          local.set $p3
          local.get $l4
          i32.const 8
          i32.add
          local.get $p0
          local.get $p1
          i32.load offset=16
          call_indirect (type $t0) $T0
          local.get $l4
          local.get $l4
          i64.load offset=8
          i64.store offset=32
          local.get $p3
          local.get $l4
          i32.const 32
          i32.add
          local.get $p2
          i32.load offset=12
          call_indirect (type $t0) $T0
          i32.const 0
          i32.load offset=1049332
          local.set $p3
        end
        i32.const 0
        local.get $p3
        i32.const -1
        i32.add
        i32.store offset=1049332
        local.get $l5
        i32.const 1
        i32.le_u
        br_if $B0
      end
      unreachable
      unreachable
    end
    local.get $p0
    local.get $p1
    call $rust_panic
    unreachable)
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h009a14389072927fE (type $t0) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
    global.get $g0
    i32.const 64
    i32.sub
    local.tee $l2
    global.set $g0
    block $B0
      local.get $p1
      i32.load offset=4
      local.tee $l3
      br_if $B0
      local.get $p1
      i32.const 4
      i32.add
      local.set $l3
      local.get $p1
      i32.load
      local.set $l4
      local.get $l2
      i32.const 0
      i32.store offset=32
      local.get $l2
      i64.const 1
      i64.store offset=24
      local.get $l2
      local.get $l2
      i32.const 24
      i32.add
      i32.store offset=36
      local.get $l2
      i32.const 40
      i32.add
      i32.const 16
      i32.add
      local.get $l4
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get $l2
      i32.const 40
      i32.add
      i32.const 8
      i32.add
      local.get $l4
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get $l2
      local.get $l4
      i64.load align=4
      i64.store offset=40
      local.get $l2
      i32.const 36
      i32.add
      i32.const 1048636
      local.get $l2
      i32.const 40
      i32.add
      call $_ZN4core3fmt5write17h7b0c1564f380f2eeE
      drop
      local.get $l2
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      local.tee $l4
      local.get $l2
      i32.load offset=32
      i32.store
      local.get $l2
      local.get $l2
      i64.load offset=24
      i64.store offset=8
      block $B1
        local.get $p1
        i32.load offset=4
        local.tee $l5
        i32.eqz
        br_if $B1
        local.get $p1
        i32.const 8
        i32.add
        i32.load
        local.tee $l6
        i32.eqz
        br_if $B1
        local.get $l5
        local.get $l6
        i32.const 1
        call $__rust_dealloc
      end
      local.get $l3
      local.get $l2
      i64.load offset=8
      i64.store align=4
      local.get $l3
      i32.const 8
      i32.add
      local.get $l4
      i32.load
      i32.store
      local.get $l3
      i32.load
      local.set $l3
    end
    local.get $p1
    i32.const 1
    i32.store offset=4
    local.get $p1
    i32.const 12
    i32.add
    i32.load
    local.set $l4
    local.get $p1
    i32.const 8
    i32.add
    local.tee $p1
    i32.load
    local.set $l5
    local.get $p1
    i64.const 0
    i64.store align=4
    block $B2
      i32.const 12
      i32.const 4
      call $__rust_alloc
      local.tee $p1
      br_if $B2
      i32.const 12
      i32.const 4
      call $_ZN5alloc5alloc18handle_alloc_error17hf1cfd628decbe217E
      unreachable
    end
    local.get $p1
    local.get $l4
    i32.store offset=8
    local.get $p1
    local.get $l5
    i32.store offset=4
    local.get $p1
    local.get $l3
    i32.store
    local.get $p0
    i32.const 1048832
    i32.store offset=4
    local.get $p0
    local.get $p1
    i32.store
    local.get $l2
    i32.const 64
    i32.add
    global.set $g0)
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17hfa4cbf18d8940863E (type $t0) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
    global.get $g0
    i32.const 64
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $p1
    i32.const 4
    i32.add
    local.set $l3
    block $B0
      local.get $p1
      i32.load offset=4
      br_if $B0
      local.get $p1
      i32.load
      local.set $l4
      local.get $l2
      i32.const 0
      i32.store offset=32
      local.get $l2
      i64.const 1
      i64.store offset=24
      local.get $l2
      local.get $l2
      i32.const 24
      i32.add
      i32.store offset=36
      local.get $l2
      i32.const 40
      i32.add
      i32.const 16
      i32.add
      local.get $l4
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get $l2
      i32.const 40
      i32.add
      i32.const 8
      i32.add
      local.get $l4
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get $l2
      local.get $l4
      i64.load align=4
      i64.store offset=40
      local.get $l2
      i32.const 36
      i32.add
      i32.const 1048636
      local.get $l2
      i32.const 40
      i32.add
      call $_ZN4core3fmt5write17h7b0c1564f380f2eeE
      drop
      local.get $l2
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      local.tee $l4
      local.get $l2
      i32.load offset=32
      i32.store
      local.get $l2
      local.get $l2
      i64.load offset=24
      i64.store offset=8
      block $B1
        local.get $p1
        i32.load offset=4
        local.tee $l5
        i32.eqz
        br_if $B1
        local.get $p1
        i32.const 8
        i32.add
        i32.load
        local.tee $p1
        i32.eqz
        br_if $B1
        local.get $l5
        local.get $p1
        i32.const 1
        call $__rust_dealloc
      end
      local.get $l3
      local.get $l2
      i64.load offset=8
      i64.store align=4
      local.get $l3
      i32.const 8
      i32.add
      local.get $l4
      i32.load
      i32.store
    end
    local.get $p0
    i32.const 1048832
    i32.store offset=4
    local.get $p0
    local.get $l3
    i32.store
    local.get $l2
    i32.const 64
    i32.add
    global.set $g0)
  (func $rust_panic (type $t0) (param $p0 i32) (param $p1 i32)
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
    i32.const 8
    i32.add
    call $__rust_start_panic
    drop
    unreachable
    unreachable)
  (func $__rust_start_panic (type $t4) (param $p0 i32) (result i32)
    unreachable
    unreachable)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc16malloc_alignment17h533bf653182825f3E (type $t4) (param $p0 i32) (result i32)
    i32.const 8)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17h40c3085ef28e063eE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i64)
    block $B0
      block $B1
        block $B2
          local.get $p1
          i32.const 245
          i32.lt_u
          br_if $B2
          i32.const 0
          local.set $l2
          local.get $p1
          i32.const -65587
          i32.ge_u
          br_if $B0
          local.get $p1
          i32.const 11
          i32.add
          local.tee $p1
          i32.const -8
          i32.and
          local.set $l3
          local.get $p0
          i32.load offset=4
          local.tee $l4
          i32.eqz
          br_if $B1
          i32.const 0
          local.set $l5
          block $B3
            local.get $p1
            i32.const 8
            i32.shr_u
            local.tee $p1
            i32.eqz
            br_if $B3
            i32.const 31
            local.set $l5
            local.get $l3
            i32.const 16777215
            i32.gt_u
            br_if $B3
            local.get $l3
            i32.const 6
            local.get $p1
            i32.clz
            local.tee $p1
            i32.sub
            i32.const 31
            i32.and
            i32.shr_u
            i32.const 1
            i32.and
            local.get $p1
            i32.const 1
            i32.shl
            i32.sub
            i32.const 62
            i32.add
            local.set $l5
          end
          i32.const 0
          local.get $l3
          i32.sub
          local.set $l2
          block $B4
            block $B5
              block $B6
                local.get $p0
                local.get $l5
                i32.const 2
                i32.shl
                i32.add
                i32.const 272
                i32.add
                i32.load
                local.tee $p1
                i32.eqz
                br_if $B6
                i32.const 0
                local.set $l6
                local.get $l3
                i32.const 0
                i32.const 25
                local.get $l5
                i32.const 1
                i32.shr_u
                i32.sub
                i32.const 31
                i32.and
                local.get $l5
                i32.const 31
                i32.eq
                select
                i32.shl
                local.set $l7
                i32.const 0
                local.set $l8
                loop $L7
                  block $B8
                    local.get $p1
                    i32.load offset=4
                    i32.const -8
                    i32.and
                    local.tee $l9
                    local.get $l3
                    i32.lt_u
                    br_if $B8
                    local.get $l9
                    local.get $l3
                    i32.sub
                    local.tee $l9
                    local.get $l2
                    i32.ge_u
                    br_if $B8
                    local.get $l9
                    local.set $l2
                    local.get $p1
                    local.set $l8
                    local.get $l9
                    br_if $B8
                    i32.const 0
                    local.set $l2
                    local.get $p1
                    local.set $l8
                    br $B5
                  end
                  local.get $p1
                  i32.const 20
                  i32.add
                  i32.load
                  local.tee $l9
                  local.get $l6
                  local.get $l9
                  local.get $p1
                  local.get $l7
                  i32.const 29
                  i32.shr_u
                  i32.const 4
                  i32.and
                  i32.add
                  i32.const 16
                  i32.add
                  i32.load
                  local.tee $p1
                  i32.ne
                  select
                  local.get $l6
                  local.get $l9
                  select
                  local.set $l6
                  local.get $l7
                  i32.const 1
                  i32.shl
                  local.set $l7
                  local.get $p1
                  br_if $L7
                end
                block $B9
                  local.get $l6
                  i32.eqz
                  br_if $B9
                  local.get $l6
                  local.set $p1
                  br $B5
                end
                local.get $l8
                br_if $B4
              end
              i32.const 0
              local.set $l8
              i32.const 2
              local.get $l5
              i32.const 31
              i32.and
              i32.shl
              local.tee $p1
              i32.const 0
              local.get $p1
              i32.sub
              i32.or
              local.get $l4
              i32.and
              local.tee $p1
              i32.eqz
              br_if $B1
              local.get $p0
              local.get $p1
              i32.const 0
              local.get $p1
              i32.sub
              i32.and
              i32.ctz
              i32.const 2
              i32.shl
              i32.add
              i32.const 272
              i32.add
              i32.load
              local.tee $p1
              i32.eqz
              br_if $B1
            end
            loop $L10
              local.get $p1
              i32.load offset=4
              i32.const -8
              i32.and
              local.tee $l6
              local.get $l3
              i32.ge_u
              local.get $l6
              local.get $l3
              i32.sub
              local.tee $l9
              local.get $l2
              i32.lt_u
              i32.and
              local.set $l7
              block $B11
                local.get $p1
                i32.load offset=16
                local.tee $l6
                br_if $B11
                local.get $p1
                i32.const 20
                i32.add
                i32.load
                local.set $l6
              end
              local.get $p1
              local.get $l8
              local.get $l7
              select
              local.set $l8
              local.get $l9
              local.get $l2
              local.get $l7
              select
              local.set $l2
              local.get $l6
              local.set $p1
              local.get $l6
              br_if $L10
            end
            local.get $l8
            i32.eqz
            br_if $B1
          end
          block $B12
            local.get $p0
            i32.load offset=400
            local.tee $p1
            local.get $l3
            i32.lt_u
            br_if $B12
            local.get $l2
            local.get $p1
            local.get $l3
            i32.sub
            i32.ge_u
            br_if $B1
          end
          local.get $p0
          local.get $l8
          call $_ZN8dlmalloc8dlmalloc8Dlmalloc18unlink_large_chunk17h73a5474d5768cd79E
          block $B13
            block $B14
              local.get $l2
              i32.const 16
              i32.lt_u
              br_if $B14
              local.get $l8
              local.get $l3
              i32.const 3
              i32.or
              i32.store offset=4
              local.get $l8
              local.get $l3
              i32.add
              local.tee $p1
              local.get $l2
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $p1
              local.get $l2
              i32.add
              local.get $l2
              i32.store
              block $B15
                local.get $l2
                i32.const 256
                i32.lt_u
                br_if $B15
                local.get $p0
                local.get $p1
                local.get $l2
                call $_ZN8dlmalloc8dlmalloc8Dlmalloc18insert_large_chunk17h897361235d504815E
                br $B13
              end
              local.get $p0
              local.get $l2
              i32.const 3
              i32.shr_u
              local.tee $l2
              i32.const 3
              i32.shl
              i32.add
              i32.const 8
              i32.add
              local.set $l3
              block $B16
                block $B17
                  local.get $p0
                  i32.load
                  local.tee $l6
                  i32.const 1
                  local.get $l2
                  i32.const 31
                  i32.and
                  i32.shl
                  local.tee $l2
                  i32.and
                  i32.eqz
                  br_if $B17
                  local.get $l3
                  i32.load offset=8
                  local.set $l2
                  br $B16
                end
                local.get $p0
                local.get $l6
                local.get $l2
                i32.or
                i32.store
                local.get $l3
                local.set $l2
              end
              local.get $l3
              local.get $p1
              i32.store offset=8
              local.get $l2
              local.get $p1
              i32.store offset=12
              local.get $p1
              local.get $l3
              i32.store offset=12
              local.get $p1
              local.get $l2
              i32.store offset=8
              br $B13
            end
            local.get $l8
            local.get $l2
            local.get $l3
            i32.add
            local.tee $p1
            i32.const 3
            i32.or
            i32.store offset=4
            local.get $l8
            local.get $p1
            i32.add
            local.tee $p1
            local.get $p1
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
          end
          local.get $l8
          i32.const 8
          i32.add
          return
        end
        block $B18
          block $B19
            block $B20
              local.get $p0
              i32.load
              local.tee $l8
              i32.const 16
              local.get $p1
              i32.const 11
              i32.add
              i32.const -8
              i32.and
              local.get $p1
              i32.const 11
              i32.lt_u
              select
              local.tee $l3
              i32.const 3
              i32.shr_u
              local.tee $l2
              i32.const 31
              i32.and
              local.tee $l6
              i32.shr_u
              local.tee $p1
              i32.const 3
              i32.and
              br_if $B20
              local.get $l3
              local.get $p0
              i32.load offset=400
              i32.le_u
              br_if $B1
              local.get $p1
              br_if $B19
              local.get $p0
              i32.load offset=4
              local.tee $p1
              i32.eqz
              br_if $B1
              local.get $p0
              local.get $p1
              i32.const 0
              local.get $p1
              i32.sub
              i32.and
              i32.ctz
              i32.const 2
              i32.shl
              i32.add
              i32.const 272
              i32.add
              i32.load
              local.tee $l6
              i32.load offset=4
              i32.const -8
              i32.and
              local.get $l3
              i32.sub
              local.set $l2
              local.get $l6
              local.set $l7
              loop $L21
                block $B22
                  local.get $l6
                  i32.load offset=16
                  local.tee $p1
                  br_if $B22
                  local.get $l6
                  i32.const 20
                  i32.add
                  i32.load
                  local.tee $p1
                  i32.eqz
                  br_if $B18
                end
                local.get $p1
                i32.load offset=4
                i32.const -8
                i32.and
                local.get $l3
                i32.sub
                local.tee $l6
                local.get $l2
                local.get $l6
                local.get $l2
                i32.lt_u
                local.tee $l6
                select
                local.set $l2
                local.get $p1
                local.get $l7
                local.get $l6
                select
                local.set $l7
                local.get $p1
                local.set $l6
                br $L21
              end
            end
            local.get $p0
            local.get $p1
            i32.const -1
            i32.xor
            i32.const 1
            i32.and
            local.get $l2
            i32.add
            local.tee $l3
            i32.const 3
            i32.shl
            i32.add
            local.tee $l7
            i32.const 16
            i32.add
            i32.load
            local.tee $p1
            i32.const 8
            i32.add
            local.set $l2
            block $B23
              block $B24
                local.get $p1
                i32.load offset=8
                local.tee $l6
                local.get $l7
                i32.const 8
                i32.add
                local.tee $l7
                i32.eq
                br_if $B24
                local.get $l6
                local.get $l7
                i32.store offset=12
                local.get $l7
                local.get $l6
                i32.store offset=8
                br $B23
              end
              local.get $p0
              local.get $l8
              i32.const -2
              local.get $l3
              i32.rotl
              i32.and
              i32.store
            end
            local.get $p1
            local.get $l3
            i32.const 3
            i32.shl
            local.tee $l3
            i32.const 3
            i32.or
            i32.store offset=4
            local.get $p1
            local.get $l3
            i32.add
            local.tee $p1
            local.get $p1
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            br $B0
          end
          block $B25
            block $B26
              local.get $p0
              local.get $p1
              local.get $l6
              i32.shl
              i32.const 2
              local.get $l6
              i32.shl
              local.tee $p1
              i32.const 0
              local.get $p1
              i32.sub
              i32.or
              i32.and
              local.tee $p1
              i32.const 0
              local.get $p1
              i32.sub
              i32.and
              i32.ctz
              local.tee $l2
              i32.const 3
              i32.shl
              i32.add
              local.tee $l7
              i32.const 16
              i32.add
              i32.load
              local.tee $p1
              i32.load offset=8
              local.tee $l6
              local.get $l7
              i32.const 8
              i32.add
              local.tee $l7
              i32.eq
              br_if $B26
              local.get $l6
              local.get $l7
              i32.store offset=12
              local.get $l7
              local.get $l6
              i32.store offset=8
              br $B25
            end
            local.get $p0
            local.get $l8
            i32.const -2
            local.get $l2
            i32.rotl
            i32.and
            i32.store
          end
          local.get $p1
          i32.const 8
          i32.add
          local.set $l6
          local.get $p1
          local.get $l3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get $p1
          local.get $l3
          i32.add
          local.tee $l7
          local.get $l2
          i32.const 3
          i32.shl
          local.tee $l2
          local.get $l3
          i32.sub
          local.tee $l3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $p1
          local.get $l2
          i32.add
          local.get $l3
          i32.store
          block $B27
            local.get $p0
            i32.load offset=400
            local.tee $p1
            i32.eqz
            br_if $B27
            local.get $p0
            local.get $p1
            i32.const 3
            i32.shr_u
            local.tee $l8
            i32.const 3
            i32.shl
            i32.add
            i32.const 8
            i32.add
            local.set $l2
            local.get $p0
            i32.load offset=408
            local.set $p1
            block $B28
              block $B29
                local.get $p0
                i32.load
                local.tee $l9
                i32.const 1
                local.get $l8
                i32.const 31
                i32.and
                i32.shl
                local.tee $l8
                i32.and
                i32.eqz
                br_if $B29
                local.get $l2
                i32.load offset=8
                local.set $l8
                br $B28
              end
              local.get $p0
              local.get $l9
              local.get $l8
              i32.or
              i32.store
              local.get $l2
              local.set $l8
            end
            local.get $l2
            local.get $p1
            i32.store offset=8
            local.get $l8
            local.get $p1
            i32.store offset=12
            local.get $p1
            local.get $l2
            i32.store offset=12
            local.get $p1
            local.get $l8
            i32.store offset=8
          end
          local.get $p0
          local.get $l7
          i32.store offset=408
          local.get $p0
          local.get $l3
          i32.store offset=400
          local.get $l6
          return
        end
        local.get $p0
        local.get $l7
        call $_ZN8dlmalloc8dlmalloc8Dlmalloc18unlink_large_chunk17h73a5474d5768cd79E
        block $B30
          block $B31
            local.get $l2
            i32.const 16
            i32.lt_u
            br_if $B31
            local.get $l7
            local.get $l3
            i32.const 3
            i32.or
            i32.store offset=4
            local.get $l7
            local.get $l3
            i32.add
            local.tee $l3
            local.get $l2
            i32.const 1
            i32.or
            i32.store offset=4
            local.get $l3
            local.get $l2
            i32.add
            local.get $l2
            i32.store
            block $B32
              local.get $p0
              i32.load offset=400
              local.tee $p1
              i32.eqz
              br_if $B32
              local.get $p0
              local.get $p1
              i32.const 3
              i32.shr_u
              local.tee $l8
              i32.const 3
              i32.shl
              i32.add
              i32.const 8
              i32.add
              local.set $l6
              local.get $p0
              i32.load offset=408
              local.set $p1
              block $B33
                block $B34
                  local.get $p0
                  i32.load
                  local.tee $l9
                  i32.const 1
                  local.get $l8
                  i32.const 31
                  i32.and
                  i32.shl
                  local.tee $l8
                  i32.and
                  i32.eqz
                  br_if $B34
                  local.get $l6
                  i32.load offset=8
                  local.set $l8
                  br $B33
                end
                local.get $p0
                local.get $l9
                local.get $l8
                i32.or
                i32.store
                local.get $l6
                local.set $l8
              end
              local.get $l6
              local.get $p1
              i32.store offset=8
              local.get $l8
              local.get $p1
              i32.store offset=12
              local.get $p1
              local.get $l6
              i32.store offset=12
              local.get $p1
              local.get $l8
              i32.store offset=8
            end
            local.get $p0
            local.get $l3
            i32.store offset=408
            local.get $p0
            local.get $l2
            i32.store offset=400
            br $B30
          end
          local.get $l7
          local.get $l2
          local.get $l3
          i32.add
          local.tee $p1
          i32.const 3
          i32.or
          i32.store offset=4
          local.get $l7
          local.get $p1
          i32.add
          local.tee $p1
          local.get $p1
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
        end
        local.get $l7
        i32.const 8
        i32.add
        return
      end
      block $B35
        block $B36
          block $B37
            block $B38
              block $B39
                block $B40
                  local.get $p0
                  i32.load offset=400
                  local.tee $l2
                  local.get $l3
                  i32.ge_u
                  br_if $B40
                  local.get $p0
                  i32.load offset=404
                  local.tee $p1
                  local.get $l3
                  i32.gt_u
                  br_if $B37
                  i32.const 0
                  local.set $l2
                  local.get $l3
                  i32.const 65583
                  i32.add
                  local.tee $l6
                  i32.const 16
                  i32.shr_u
                  memory.grow
                  local.tee $p1
                  i32.const -1
                  i32.eq
                  br_if $B0
                  local.get $p1
                  i32.const 16
                  i32.shl
                  local.tee $l8
                  i32.eqz
                  br_if $B0
                  local.get $p0
                  local.get $p0
                  i32.load offset=416
                  local.get $l6
                  i32.const -65536
                  i32.and
                  local.tee $l5
                  i32.add
                  local.tee $p1
                  i32.store offset=416
                  local.get $p0
                  local.get $p0
                  i32.load offset=420
                  local.tee $l6
                  local.get $p1
                  local.get $l6
                  local.get $p1
                  i32.gt_u
                  select
                  i32.store offset=420
                  local.get $p0
                  i32.load offset=412
                  local.tee $l6
                  i32.eqz
                  br_if $B39
                  local.get $p0
                  i32.const 424
                  i32.add
                  local.tee $l4
                  local.set $p1
                  loop $L41
                    local.get $p1
                    i32.load
                    local.tee $l7
                    local.get $p1
                    i32.load offset=4
                    local.tee $l9
                    i32.add
                    local.get $l8
                    i32.eq
                    br_if $B38
                    local.get $p1
                    i32.load offset=8
                    local.tee $p1
                    br_if $L41
                    br $B36
                  end
                end
                local.get $p0
                i32.load offset=408
                local.set $p1
                block $B42
                  block $B43
                    local.get $l2
                    local.get $l3
                    i32.sub
                    local.tee $l6
                    i32.const 15
                    i32.gt_u
                    br_if $B43
                    local.get $p0
                    i32.const 0
                    i32.store offset=408
                    local.get $p0
                    i32.const 0
                    i32.store offset=400
                    local.get $p1
                    local.get $l2
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    local.get $p1
                    local.get $l2
                    i32.add
                    local.tee $l2
                    i32.const 4
                    i32.add
                    local.set $l3
                    local.get $l2
                    i32.load offset=4
                    i32.const 1
                    i32.or
                    local.set $l2
                    br $B42
                  end
                  local.get $p0
                  local.get $l6
                  i32.store offset=400
                  local.get $p0
                  local.get $p1
                  local.get $l3
                  i32.add
                  local.tee $l7
                  i32.store offset=408
                  local.get $l7
                  local.get $l6
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get $p1
                  local.get $l2
                  i32.add
                  local.get $l6
                  i32.store
                  local.get $l3
                  i32.const 3
                  i32.or
                  local.set $l2
                  local.get $p1
                  i32.const 4
                  i32.add
                  local.set $l3
                end
                local.get $l3
                local.get $l2
                i32.store
                local.get $p1
                i32.const 8
                i32.add
                return
              end
              block $B44
                block $B45
                  local.get $p0
                  i32.load offset=444
                  local.tee $p1
                  i32.eqz
                  br_if $B45
                  local.get $p1
                  local.get $l8
                  i32.le_u
                  br_if $B44
                end
                local.get $p0
                local.get $l8
                i32.store offset=444
              end
              local.get $p0
              i32.const 4095
              i32.store offset=448
              local.get $p0
              local.get $l8
              i32.store offset=424
              i32.const 0
              local.set $p1
              local.get $p0
              i32.const 436
              i32.add
              i32.const 0
              i32.store
              local.get $p0
              i32.const 428
              i32.add
              local.get $l5
              i32.store
              loop $L46
                local.get $p0
                local.get $p1
                i32.add
                local.tee $l6
                i32.const 16
                i32.add
                local.get $l6
                i32.const 8
                i32.add
                local.tee $l7
                i32.store
                local.get $l6
                i32.const 20
                i32.add
                local.get $l7
                i32.store
                local.get $p1
                i32.const 8
                i32.add
                local.tee $p1
                i32.const 256
                i32.ne
                br_if $L46
              end
              local.get $p0
              local.get $l8
              i32.store offset=412
              local.get $p0
              local.get $l5
              i32.const -40
              i32.add
              local.tee $p1
              i32.store offset=404
              local.get $l8
              local.get $p1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $l8
              local.get $p1
              i32.add
              i32.const 40
              i32.store offset=4
              local.get $p0
              i32.const 2097152
              i32.store offset=440
              br $B35
            end
            local.get $p1
            i32.load offset=12
            br_if $B36
            local.get $l8
            local.get $l6
            i32.le_u
            br_if $B36
            local.get $l7
            local.get $l6
            i32.gt_u
            br_if $B36
            local.get $p1
            local.get $l9
            local.get $l5
            i32.add
            i32.store offset=4
            local.get $p0
            local.get $p0
            i32.load offset=412
            local.tee $p1
            i32.const 15
            i32.add
            i32.const -8
            i32.and
            local.tee $l6
            i32.const -8
            i32.add
            i32.store offset=412
            local.get $p0
            local.get $p1
            local.get $l6
            i32.sub
            local.get $p0
            i32.load offset=404
            local.get $l5
            i32.add
            local.tee $l7
            i32.add
            i32.const 8
            i32.add
            local.tee $l8
            i32.store offset=404
            local.get $l6
            i32.const -4
            i32.add
            local.get $l8
            i32.const 1
            i32.or
            i32.store
            local.get $p1
            local.get $l7
            i32.add
            i32.const 40
            i32.store offset=4
            local.get $p0
            i32.const 2097152
            i32.store offset=440
            br $B35
          end
          local.get $p0
          local.get $p1
          local.get $l3
          i32.sub
          local.tee $l2
          i32.store offset=404
          local.get $p0
          local.get $p0
          i32.load offset=412
          local.tee $p1
          local.get $l3
          i32.add
          local.tee $l6
          i32.store offset=412
          local.get $l6
          local.get $l2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $p1
          local.get $l3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get $p1
          i32.const 8
          i32.add
          return
        end
        local.get $p0
        local.get $p0
        i32.load offset=444
        local.tee $p1
        local.get $l8
        local.get $p1
        local.get $l8
        i32.lt_u
        select
        i32.store offset=444
        local.get $l8
        local.get $l5
        i32.add
        local.set $l7
        local.get $l4
        local.set $p1
        block $B47
          block $B48
            loop $L49
              local.get $p1
              i32.load
              local.get $l7
              i32.eq
              br_if $B48
              local.get $p1
              i32.load offset=8
              local.tee $p1
              br_if $L49
              br $B47
            end
          end
          local.get $p1
          i32.load offset=12
          br_if $B47
          local.get $p1
          local.get $l8
          i32.store
          local.get $p1
          local.get $p1
          i32.load offset=4
          local.get $l5
          i32.add
          i32.store offset=4
          local.get $l8
          local.get $l3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get $l8
          local.get $l3
          i32.add
          local.set $p1
          local.get $l7
          local.get $l8
          i32.sub
          local.get $l3
          i32.sub
          local.set $l3
          block $B50
            block $B51
              block $B52
                local.get $p0
                i32.load offset=412
                local.get $l7
                i32.eq
                br_if $B52
                local.get $p0
                i32.load offset=408
                local.get $l7
                i32.eq
                br_if $B51
                block $B53
                  local.get $l7
                  i32.load offset=4
                  local.tee $l2
                  i32.const 3
                  i32.and
                  i32.const 1
                  i32.ne
                  br_if $B53
                  block $B54
                    block $B55
                      local.get $l2
                      i32.const -8
                      i32.and
                      local.tee $l6
                      i32.const 256
                      i32.lt_u
                      br_if $B55
                      local.get $p0
                      local.get $l7
                      call $_ZN8dlmalloc8dlmalloc8Dlmalloc18unlink_large_chunk17h73a5474d5768cd79E
                      br $B54
                    end
                    block $B56
                      local.get $l7
                      i32.load offset=12
                      local.tee $l9
                      local.get $l7
                      i32.load offset=8
                      local.tee $l5
                      i32.eq
                      br_if $B56
                      local.get $l5
                      local.get $l9
                      i32.store offset=12
                      local.get $l9
                      local.get $l5
                      i32.store offset=8
                      br $B54
                    end
                    local.get $p0
                    local.get $p0
                    i32.load
                    i32.const -2
                    local.get $l2
                    i32.const 3
                    i32.shr_u
                    i32.rotl
                    i32.and
                    i32.store
                  end
                  local.get $l6
                  local.get $l3
                  i32.add
                  local.set $l3
                  local.get $l7
                  local.get $l6
                  i32.add
                  local.set $l7
                end
                local.get $l7
                local.get $l7
                i32.load offset=4
                i32.const -2
                i32.and
                i32.store offset=4
                local.get $p1
                local.get $l3
                i32.const 1
                i32.or
                i32.store offset=4
                local.get $p1
                local.get $l3
                i32.add
                local.get $l3
                i32.store
                block $B57
                  local.get $l3
                  i32.const 256
                  i32.lt_u
                  br_if $B57
                  local.get $p0
                  local.get $p1
                  local.get $l3
                  call $_ZN8dlmalloc8dlmalloc8Dlmalloc18insert_large_chunk17h897361235d504815E
                  br $B50
                end
                local.get $p0
                local.get $l3
                i32.const 3
                i32.shr_u
                local.tee $l2
                i32.const 3
                i32.shl
                i32.add
                i32.const 8
                i32.add
                local.set $l3
                block $B58
                  block $B59
                    local.get $p0
                    i32.load
                    local.tee $l6
                    i32.const 1
                    local.get $l2
                    i32.const 31
                    i32.and
                    i32.shl
                    local.tee $l2
                    i32.and
                    i32.eqz
                    br_if $B59
                    local.get $l3
                    i32.load offset=8
                    local.set $l2
                    br $B58
                  end
                  local.get $p0
                  local.get $l6
                  local.get $l2
                  i32.or
                  i32.store
                  local.get $l3
                  local.set $l2
                end
                local.get $l3
                local.get $p1
                i32.store offset=8
                local.get $l2
                local.get $p1
                i32.store offset=12
                local.get $p1
                local.get $l3
                i32.store offset=12
                local.get $p1
                local.get $l2
                i32.store offset=8
                br $B50
              end
              local.get $p0
              local.get $p1
              i32.store offset=412
              local.get $p0
              local.get $p0
              i32.load offset=404
              local.get $l3
              i32.add
              local.tee $l3
              i32.store offset=404
              local.get $p1
              local.get $l3
              i32.const 1
              i32.or
              i32.store offset=4
              br $B50
            end
            local.get $p0
            local.get $p1
            i32.store offset=408
            local.get $p0
            local.get $p0
            i32.load offset=400
            local.get $l3
            i32.add
            local.tee $l3
            i32.store offset=400
            local.get $p1
            local.get $l3
            i32.const 1
            i32.or
            i32.store offset=4
            local.get $p1
            local.get $l3
            i32.add
            local.get $l3
            i32.store
          end
          local.get $l8
          i32.const 8
          i32.add
          return
        end
        local.get $l4
        local.set $p1
        block $B60
          loop $L61
            block $B62
              local.get $p1
              i32.load
              local.tee $l7
              local.get $l6
              i32.gt_u
              br_if $B62
              local.get $l7
              local.get $p1
              i32.load offset=4
              i32.add
              local.tee $l7
              local.get $l6
              i32.gt_u
              br_if $B60
            end
            local.get $p1
            i32.load offset=8
            local.set $p1
            br $L61
          end
        end
        local.get $p0
        local.get $l8
        i32.store offset=412
        local.get $p0
        local.get $l5
        i32.const -40
        i32.add
        local.tee $p1
        i32.store offset=404
        local.get $l8
        local.get $p1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get $l8
        local.get $p1
        i32.add
        i32.const 40
        i32.store offset=4
        local.get $p0
        i32.const 2097152
        i32.store offset=440
        local.get $l6
        local.get $l7
        i32.const -32
        i32.add
        i32.const -8
        i32.and
        i32.const -8
        i32.add
        local.tee $p1
        local.get $p1
        local.get $l6
        i32.const 16
        i32.add
        i32.lt_u
        select
        local.tee $l9
        i32.const 27
        i32.store offset=4
        local.get $l4
        i64.load align=4
        local.set $l10
        local.get $l9
        i32.const 16
        i32.add
        local.get $l4
        i32.const 8
        i32.add
        i64.load align=4
        i64.store align=4
        local.get $l9
        local.get $l10
        i64.store offset=8 align=4
        local.get $p0
        i32.const 436
        i32.add
        i32.const 0
        i32.store
        local.get $p0
        i32.const 428
        i32.add
        local.get $l5
        i32.store
        local.get $p0
        local.get $l8
        i32.store offset=424
        local.get $p0
        i32.const 432
        i32.add
        local.get $l9
        i32.const 8
        i32.add
        i32.store
        local.get $l9
        i32.const 28
        i32.add
        local.set $p1
        loop $L63
          local.get $p1
          i32.const 7
          i32.store
          local.get $l7
          local.get $p1
          i32.const 4
          i32.add
          local.tee $p1
          i32.gt_u
          br_if $L63
        end
        local.get $l9
        local.get $l6
        i32.eq
        br_if $B35
        local.get $l9
        local.get $l9
        i32.load offset=4
        i32.const -2
        i32.and
        i32.store offset=4
        local.get $l6
        local.get $l9
        local.get $l6
        i32.sub
        local.tee $p1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get $l9
        local.get $p1
        i32.store
        block $B64
          local.get $p1
          i32.const 256
          i32.lt_u
          br_if $B64
          local.get $p0
          local.get $l6
          local.get $p1
          call $_ZN8dlmalloc8dlmalloc8Dlmalloc18insert_large_chunk17h897361235d504815E
          br $B35
        end
        local.get $p0
        local.get $p1
        i32.const 3
        i32.shr_u
        local.tee $l7
        i32.const 3
        i32.shl
        i32.add
        i32.const 8
        i32.add
        local.set $p1
        block $B65
          block $B66
            local.get $p0
            i32.load
            local.tee $l8
            i32.const 1
            local.get $l7
            i32.const 31
            i32.and
            i32.shl
            local.tee $l7
            i32.and
            i32.eqz
            br_if $B66
            local.get $p1
            i32.load offset=8
            local.set $l7
            br $B65
          end
          local.get $p0
          local.get $l8
          local.get $l7
          i32.or
          i32.store
          local.get $p1
          local.set $l7
        end
        local.get $p1
        local.get $l6
        i32.store offset=8
        local.get $l7
        local.get $l6
        i32.store offset=12
        local.get $l6
        local.get $p1
        i32.store offset=12
        local.get $l6
        local.get $l7
        i32.store offset=8
      end
      local.get $p0
      i32.load offset=404
      local.tee $p1
      local.get $l3
      i32.le_u
      br_if $B0
      local.get $p0
      local.get $p1
      local.get $l3
      i32.sub
      local.tee $l2
      i32.store offset=404
      local.get $p0
      local.get $p0
      i32.load offset=412
      local.tee $p1
      local.get $l3
      i32.add
      local.tee $l6
      i32.store offset=412
      local.get $l6
      local.get $l2
      i32.const 1
      i32.or
      i32.store offset=4
      local.get $p1
      local.get $l3
      i32.const 3
      i32.or
      i32.store offset=4
      local.get $p1
      i32.const 8
      i32.add
      return
    end
    local.get $l2)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc18unlink_large_chunk17h73a5474d5768cd79E (type $t0) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
    local.get $p1
    i32.load offset=24
    local.set $l2
    block $B0
      block $B1
        block $B2
          local.get $p1
          i32.load offset=12
          local.tee $l3
          local.get $p1
          i32.ne
          br_if $B2
          local.get $p1
          i32.const 20
          i32.const 16
          local.get $p1
          i32.const 20
          i32.add
          local.tee $l3
          i32.load
          local.tee $l4
          select
          i32.add
          i32.load
          local.tee $l5
          br_if $B1
          i32.const 0
          local.set $l3
          br $B0
        end
        local.get $p1
        i32.load offset=8
        local.tee $l5
        local.get $l3
        i32.store offset=12
        local.get $l3
        local.get $l5
        i32.store offset=8
        br $B0
      end
      local.get $l3
      local.get $p1
      i32.const 16
      i32.add
      local.get $l4
      select
      local.set $l4
      loop $L3
        local.get $l4
        local.set $l6
        block $B4
          local.get $l5
          local.tee $l3
          i32.const 20
          i32.add
          local.tee $l4
          i32.load
          local.tee $l5
          br_if $B4
          local.get $l3
          i32.const 16
          i32.add
          local.set $l4
          local.get $l3
          i32.load offset=16
          local.set $l5
        end
        local.get $l5
        br_if $L3
      end
      local.get $l6
      i32.const 0
      i32.store
    end
    block $B5
      local.get $l2
      i32.eqz
      br_if $B5
      block $B6
        block $B7
          local.get $p0
          local.get $p1
          i32.load offset=28
          i32.const 2
          i32.shl
          i32.add
          i32.const 272
          i32.add
          local.tee $l5
          i32.load
          local.get $p1
          i32.eq
          br_if $B7
          local.get $l2
          i32.const 16
          i32.const 20
          local.get $l2
          i32.load offset=16
          local.get $p1
          i32.eq
          select
          i32.add
          local.get $l3
          i32.store
          local.get $l3
          br_if $B6
          br $B5
        end
        local.get $l5
        local.get $l3
        i32.store
        local.get $l3
        br_if $B6
        local.get $p0
        local.get $p0
        i32.load offset=4
        i32.const -2
        local.get $p1
        i32.load offset=28
        i32.rotl
        i32.and
        i32.store offset=4
        return
      end
      local.get $l3
      local.get $l2
      i32.store offset=24
      block $B8
        local.get $p1
        i32.load offset=16
        local.tee $l5
        i32.eqz
        br_if $B8
        local.get $l3
        local.get $l5
        i32.store offset=16
        local.get $l5
        local.get $l3
        i32.store offset=24
      end
      local.get $p1
      i32.const 20
      i32.add
      i32.load
      local.tee $l5
      i32.eqz
      br_if $B5
      local.get $l3
      i32.const 20
      i32.add
      local.get $l5
      i32.store
      local.get $l5
      local.get $l3
      i32.store offset=24
      return
    end)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc18insert_large_chunk17h897361235d504815E (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
    block $B0
      block $B1
        local.get $p2
        i32.const 8
        i32.shr_u
        local.tee $l3
        br_if $B1
        i32.const 0
        local.set $l4
        br $B0
      end
      i32.const 31
      local.set $l4
      local.get $p2
      i32.const 16777215
      i32.gt_u
      br_if $B0
      local.get $p2
      i32.const 6
      local.get $l3
      i32.clz
      local.tee $l4
      i32.sub
      i32.const 31
      i32.and
      i32.shr_u
      i32.const 1
      i32.and
      local.get $l4
      i32.const 1
      i32.shl
      i32.sub
      i32.const 62
      i32.add
      local.set $l4
    end
    local.get $p1
    i64.const 0
    i64.store offset=16 align=4
    local.get $p1
    local.get $l4
    i32.store offset=28
    local.get $p0
    local.get $l4
    i32.const 2
    i32.shl
    i32.add
    i32.const 272
    i32.add
    local.set $l3
    block $B2
      block $B3
        block $B4
          block $B5
            block $B6
              local.get $p0
              i32.load offset=4
              local.tee $l5
              i32.const 1
              local.get $l4
              i32.const 31
              i32.and
              i32.shl
              local.tee $l6
              i32.and
              i32.eqz
              br_if $B6
              local.get $l3
              i32.load
              local.tee $l3
              i32.load offset=4
              i32.const -8
              i32.and
              local.get $p2
              i32.ne
              br_if $B5
              local.get $l3
              local.set $l4
              br $B4
            end
            local.get $p0
            local.get $l5
            local.get $l6
            i32.or
            i32.store offset=4
            local.get $l3
            local.get $p1
            i32.store
            local.get $p1
            local.get $l3
            i32.store offset=24
            br $B2
          end
          local.get $p2
          i32.const 0
          i32.const 25
          local.get $l4
          i32.const 1
          i32.shr_u
          i32.sub
          i32.const 31
          i32.and
          local.get $l4
          i32.const 31
          i32.eq
          select
          i32.shl
          local.set $p0
          loop $L7
            local.get $l3
            local.get $p0
            i32.const 29
            i32.shr_u
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee $l5
            i32.load
            local.tee $l4
            i32.eqz
            br_if $B3
            local.get $p0
            i32.const 1
            i32.shl
            local.set $p0
            local.get $l4
            local.set $l3
            local.get $l4
            i32.load offset=4
            i32.const -8
            i32.and
            local.get $p2
            i32.ne
            br_if $L7
          end
        end
        local.get $l4
        i32.load offset=8
        local.tee $p0
        local.get $p1
        i32.store offset=12
        local.get $l4
        local.get $p1
        i32.store offset=8
        local.get $p1
        i32.const 0
        i32.store offset=24
        local.get $p1
        local.get $l4
        i32.store offset=12
        local.get $p1
        local.get $p0
        i32.store offset=8
        return
      end
      local.get $l5
      local.get $p1
      i32.store
      local.get $p1
      local.get $l3
      i32.store offset=24
    end
    local.get $p1
    local.get $p1
    i32.store offset=12
    local.get $p1
    local.get $p1
    i32.store offset=8)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc7realloc17h7c736378844fed2aE (type $t1) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32)
    i32.const 0
    local.set $l3
    block $B0
      local.get $p2
      i32.const -65588
      i32.gt_u
      br_if $B0
      i32.const 16
      local.get $p2
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get $p2
      i32.const 11
      i32.lt_u
      select
      local.set $l4
      local.get $p1
      i32.const -4
      i32.add
      local.tee $l5
      i32.load
      local.tee $l6
      i32.const -8
      i32.and
      local.set $l7
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  block $B7
                    local.get $l6
                    i32.const 3
                    i32.and
                    i32.eqz
                    br_if $B7
                    local.get $p1
                    i32.const -8
                    i32.add
                    local.tee $l8
                    local.get $l7
                    i32.add
                    local.set $l9
                    local.get $l7
                    local.get $l4
                    i32.ge_u
                    br_if $B6
                    local.get $p0
                    i32.load offset=412
                    local.get $l9
                    i32.eq
                    br_if $B5
                    local.get $p0
                    i32.load offset=408
                    local.get $l9
                    i32.eq
                    br_if $B4
                    local.get $l9
                    i32.load offset=4
                    local.tee $l6
                    i32.const 2
                    i32.and
                    br_if $B1
                    local.get $l6
                    i32.const -8
                    i32.and
                    local.tee $l10
                    local.get $l7
                    i32.add
                    local.tee $l7
                    local.get $l4
                    i32.ge_u
                    br_if $B3
                    br $B1
                  end
                  local.get $l4
                  i32.const 256
                  i32.lt_u
                  br_if $B1
                  local.get $l7
                  local.get $l4
                  i32.const 4
                  i32.or
                  i32.lt_u
                  br_if $B1
                  local.get $l7
                  local.get $l4
                  i32.sub
                  i32.const 131073
                  i32.ge_u
                  br_if $B1
                  br $B2
                end
                local.get $l7
                local.get $l4
                i32.sub
                local.tee $p2
                i32.const 16
                i32.lt_u
                br_if $B2
                local.get $l5
                local.get $l4
                local.get $l6
                i32.const 1
                i32.and
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get $l8
                local.get $l4
                i32.add
                local.tee $l3
                local.get $p2
                i32.const 3
                i32.or
                i32.store offset=4
                local.get $l9
                local.get $l9
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                local.get $p0
                local.get $l3
                local.get $p2
                call $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17hebbed6771d16a3fcE
                br $B2
              end
              local.get $p0
              i32.load offset=404
              local.get $l7
              i32.add
              local.tee $l7
              local.get $l4
              i32.le_u
              br_if $B1
              local.get $l5
              local.get $l4
              local.get $l6
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get $l8
              local.get $l4
              i32.add
              local.tee $p2
              local.get $l7
              local.get $l4
              i32.sub
              local.tee $l3
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $p0
              local.get $l3
              i32.store offset=404
              local.get $p0
              local.get $p2
              i32.store offset=412
              br $B2
            end
            local.get $p0
            i32.load offset=400
            local.get $l7
            i32.add
            local.tee $l7
            local.get $l4
            i32.lt_u
            br_if $B1
            block $B8
              block $B9
                local.get $l7
                local.get $l4
                i32.sub
                local.tee $p2
                i32.const 15
                i32.gt_u
                br_if $B9
                local.get $l5
                local.get $l6
                i32.const 1
                i32.and
                local.get $l7
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get $l8
                local.get $l7
                i32.add
                local.tee $p2
                local.get $p2
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                i32.const 0
                local.set $p2
                i32.const 0
                local.set $l3
                br $B8
              end
              local.get $l5
              local.get $l4
              local.get $l6
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get $l8
              local.get $l4
              i32.add
              local.tee $l3
              local.get $p2
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $l8
              local.get $l7
              i32.add
              local.tee $l4
              local.get $p2
              i32.store
              local.get $l4
              local.get $l4
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
            end
            local.get $p0
            local.get $l3
            i32.store offset=408
            local.get $p0
            local.get $p2
            i32.store offset=400
            br $B2
          end
          local.get $l7
          local.get $l4
          i32.sub
          local.set $p2
          block $B10
            block $B11
              local.get $l10
              i32.const 256
              i32.lt_u
              br_if $B11
              local.get $p0
              local.get $l9
              call $_ZN8dlmalloc8dlmalloc8Dlmalloc18unlink_large_chunk17h73a5474d5768cd79E
              br $B10
            end
            block $B12
              local.get $l9
              i32.load offset=12
              local.tee $l3
              local.get $l9
              i32.load offset=8
              local.tee $l9
              i32.eq
              br_if $B12
              local.get $l9
              local.get $l3
              i32.store offset=12
              local.get $l3
              local.get $l9
              i32.store offset=8
              br $B10
            end
            local.get $p0
            local.get $p0
            i32.load
            i32.const -2
            local.get $l6
            i32.const 3
            i32.shr_u
            i32.rotl
            i32.and
            i32.store
          end
          block $B13
            local.get $p2
            i32.const 16
            i32.lt_u
            br_if $B13
            local.get $l5
            local.get $l4
            local.get $l5
            i32.load
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get $l8
            local.get $l4
            i32.add
            local.tee $l3
            local.get $p2
            i32.const 3
            i32.or
            i32.store offset=4
            local.get $l8
            local.get $l7
            i32.add
            local.tee $l4
            local.get $l4
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get $p0
            local.get $l3
            local.get $p2
            call $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17hebbed6771d16a3fcE
            br $B2
          end
          local.get $l5
          local.get $l7
          local.get $l5
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get $l8
          local.get $l7
          i32.add
          local.tee $p2
          local.get $p2
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
        end
        local.get $p1
        local.set $l3
        br $B0
      end
      local.get $p0
      local.get $p2
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17h40c3085ef28e063eE
      local.tee $l4
      i32.eqz
      br_if $B0
      local.get $l4
      local.get $p1
      local.get $p2
      local.get $l5
      i32.load
      local.tee $l3
      i32.const -8
      i32.and
      i32.const 4
      i32.const 8
      local.get $l3
      i32.const 3
      i32.and
      select
      i32.sub
      local.tee $l3
      local.get $l3
      local.get $p2
      i32.gt_u
      select
      call $memcpy
      local.set $p2
      local.get $p0
      local.get $p1
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc4free17hd1431ee938d9d15eE
      local.get $p2
      return
    end
    local.get $l3)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17hebbed6771d16a3fcE (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
    local.get $p1
    local.get $p2
    i32.add
    local.set $l3
    block $B0
      block $B1
        block $B2
          local.get $p1
          i32.load offset=4
          local.tee $l4
          i32.const 1
          i32.and
          br_if $B2
          local.get $l4
          i32.const 3
          i32.and
          i32.eqz
          br_if $B1
          local.get $p1
          i32.load
          local.tee $l4
          local.get $p2
          i32.add
          local.set $p2
          block $B3
            local.get $p0
            i32.load offset=408
            local.get $p1
            local.get $l4
            i32.sub
            local.tee $p1
            i32.ne
            br_if $B3
            local.get $l3
            i32.load offset=4
            i32.const 3
            i32.and
            i32.const 3
            i32.ne
            br_if $B2
            local.get $p0
            local.get $p2
            i32.store offset=400
            local.get $l3
            local.get $l3
            i32.load offset=4
            i32.const -2
            i32.and
            i32.store offset=4
            local.get $p1
            local.get $p2
            i32.const 1
            i32.or
            i32.store offset=4
            local.get $l3
            local.get $p2
            i32.store
            return
          end
          block $B4
            local.get $l4
            i32.const 256
            i32.lt_u
            br_if $B4
            local.get $p0
            local.get $p1
            call $_ZN8dlmalloc8dlmalloc8Dlmalloc18unlink_large_chunk17h73a5474d5768cd79E
            br $B2
          end
          block $B5
            local.get $p1
            i32.load offset=12
            local.tee $l5
            local.get $p1
            i32.load offset=8
            local.tee $l6
            i32.eq
            br_if $B5
            local.get $l6
            local.get $l5
            i32.store offset=12
            local.get $l5
            local.get $l6
            i32.store offset=8
            br $B2
          end
          local.get $p0
          local.get $p0
          i32.load
          i32.const -2
          local.get $l4
          i32.const 3
          i32.shr_u
          i32.rotl
          i32.and
          i32.store
        end
        block $B6
          local.get $l3
          i32.load offset=4
          local.tee $l4
          i32.const 2
          i32.and
          i32.eqz
          br_if $B6
          local.get $l3
          local.get $l4
          i32.const -2
          i32.and
          i32.store offset=4
          local.get $p1
          local.get $p2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $p1
          local.get $p2
          i32.add
          local.get $p2
          i32.store
          br $B0
        end
        block $B7
          block $B8
            local.get $p0
            i32.load offset=412
            local.get $l3
            i32.eq
            br_if $B8
            local.get $p0
            i32.load offset=408
            local.get $l3
            i32.ne
            br_if $B7
            local.get $p0
            local.get $p1
            i32.store offset=408
            local.get $p0
            local.get $p0
            i32.load offset=400
            local.get $p2
            i32.add
            local.tee $p2
            i32.store offset=400
            local.get $p1
            local.get $p2
            i32.const 1
            i32.or
            i32.store offset=4
            local.get $p1
            local.get $p2
            i32.add
            local.get $p2
            i32.store
            return
          end
          local.get $p0
          local.get $p1
          i32.store offset=412
          local.get $p0
          local.get $p0
          i32.load offset=404
          local.get $p2
          i32.add
          local.tee $p2
          i32.store offset=404
          local.get $p1
          local.get $p2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $p1
          local.get $p0
          i32.load offset=408
          i32.ne
          br_if $B1
          local.get $p0
          i32.const 0
          i32.store offset=400
          local.get $p0
          i32.const 0
          i32.store offset=408
          return
        end
        local.get $l4
        i32.const -8
        i32.and
        local.tee $l5
        local.get $p2
        i32.add
        local.set $p2
        block $B9
          block $B10
            local.get $l5
            i32.const 256
            i32.lt_u
            br_if $B10
            local.get $p0
            local.get $l3
            call $_ZN8dlmalloc8dlmalloc8Dlmalloc18unlink_large_chunk17h73a5474d5768cd79E
            br $B9
          end
          block $B11
            local.get $l3
            i32.load offset=12
            local.tee $l5
            local.get $l3
            i32.load offset=8
            local.tee $l3
            i32.eq
            br_if $B11
            local.get $l3
            local.get $l5
            i32.store offset=12
            local.get $l5
            local.get $l3
            i32.store offset=8
            br $B9
          end
          local.get $p0
          local.get $p0
          i32.load
          i32.const -2
          local.get $l4
          i32.const 3
          i32.shr_u
          i32.rotl
          i32.and
          i32.store
        end
        local.get $p1
        local.get $p2
        i32.const 1
        i32.or
        i32.store offset=4
        local.get $p1
        local.get $p2
        i32.add
        local.get $p2
        i32.store
        local.get $p1
        local.get $p0
        i32.load offset=408
        i32.ne
        br_if $B0
        local.get $p0
        local.get $p2
        i32.store offset=400
      end
      return
    end
    block $B12
      local.get $p2
      i32.const 256
      i32.lt_u
      br_if $B12
      local.get $p0
      local.get $p1
      local.get $p2
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc18insert_large_chunk17h897361235d504815E
      return
    end
    local.get $p0
    local.get $p2
    i32.const 3
    i32.shr_u
    local.tee $l3
    i32.const 3
    i32.shl
    i32.add
    i32.const 8
    i32.add
    local.set $p2
    block $B13
      block $B14
        local.get $p0
        i32.load
        local.tee $l4
        i32.const 1
        local.get $l3
        i32.const 31
        i32.and
        i32.shl
        local.tee $l3
        i32.and
        i32.eqz
        br_if $B14
        local.get $p2
        i32.load offset=8
        local.set $p0
        br $B13
      end
      local.get $p0
      local.get $l4
      local.get $l3
      i32.or
      i32.store
      local.get $p2
      local.set $p0
    end
    local.get $p2
    local.get $p1
    i32.store offset=8
    local.get $p0
    local.get $p1
    i32.store offset=12
    local.get $p1
    local.get $p2
    i32.store offset=12
    local.get $p1
    local.get $p0
    i32.store offset=8)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc4free17hd1431ee938d9d15eE (type $t0) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
    local.get $p1
    i32.const -8
    i32.add
    local.tee $l2
    local.get $p1
    i32.const -4
    i32.add
    i32.load
    local.tee $l3
    i32.const -8
    i32.and
    local.tee $p1
    i32.add
    local.set $l4
    block $B0
      block $B1
        block $B2
          block $B3
            local.get $l3
            i32.const 1
            i32.and
            br_if $B3
            local.get $l3
            i32.const 3
            i32.and
            i32.eqz
            br_if $B2
            local.get $l2
            i32.load
            local.tee $l3
            local.get $p1
            i32.add
            local.set $p1
            block $B4
              local.get $p0
              i32.load offset=408
              local.get $l2
              local.get $l3
              i32.sub
              local.tee $l2
              i32.ne
              br_if $B4
              local.get $l4
              i32.load offset=4
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if $B3
              local.get $p0
              local.get $p1
              i32.store offset=400
              local.get $l4
              local.get $l4
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
              local.get $l2
              local.get $p1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $l2
              local.get $p1
              i32.add
              local.get $p1
              i32.store
              return
            end
            block $B5
              local.get $l3
              i32.const 256
              i32.lt_u
              br_if $B5
              local.get $p0
              local.get $l2
              call $_ZN8dlmalloc8dlmalloc8Dlmalloc18unlink_large_chunk17h73a5474d5768cd79E
              br $B3
            end
            block $B6
              local.get $l2
              i32.load offset=12
              local.tee $l5
              local.get $l2
              i32.load offset=8
              local.tee $l6
              i32.eq
              br_if $B6
              local.get $l6
              local.get $l5
              i32.store offset=12
              local.get $l5
              local.get $l6
              i32.store offset=8
              br $B3
            end
            local.get $p0
            local.get $p0
            i32.load
            i32.const -2
            local.get $l3
            i32.const 3
            i32.shr_u
            i32.rotl
            i32.and
            i32.store
          end
          block $B7
            block $B8
              local.get $l4
              i32.load offset=4
              local.tee $l3
              i32.const 2
              i32.and
              i32.eqz
              br_if $B8
              local.get $l4
              local.get $l3
              i32.const -2
              i32.and
              i32.store offset=4
              local.get $l2
              local.get $p1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get $l2
              local.get $p1
              i32.add
              local.get $p1
              i32.store
              br $B7
            end
            block $B9
              block $B10
                local.get $p0
                i32.load offset=412
                local.get $l4
                i32.eq
                br_if $B10
                local.get $p0
                i32.load offset=408
                local.get $l4
                i32.ne
                br_if $B9
                local.get $p0
                local.get $l2
                i32.store offset=408
                local.get $p0
                local.get $p0
                i32.load offset=400
                local.get $p1
                i32.add
                local.tee $p1
                i32.store offset=400
                local.get $l2
                local.get $p1
                i32.const 1
                i32.or
                i32.store offset=4
                local.get $l2
                local.get $p1
                i32.add
                local.get $p1
                i32.store
                return
              end
              local.get $p0
              local.get $l2
              i32.store offset=412
              local.get $p0
              local.get $p0
              i32.load offset=404
              local.get $p1
              i32.add
              local.tee $p1
              i32.store offset=404
              local.get $l2
              local.get $p1
              i32.const 1
              i32.or
              i32.store offset=4
              block $B11
                local.get $l2
                local.get $p0
                i32.load offset=408
                i32.ne
                br_if $B11
                local.get $p0
                i32.const 0
                i32.store offset=400
                local.get $p0
                i32.const 0
                i32.store offset=408
              end
              local.get $p0
              i32.load offset=440
              local.tee $l3
              local.get $p1
              i32.ge_u
              br_if $B2
              local.get $p0
              i32.load offset=412
              local.tee $p1
              i32.eqz
              br_if $B2
              block $B12
                local.get $p0
                i32.load offset=404
                local.tee $l5
                i32.const 41
                i32.lt_u
                br_if $B12
                local.get $p0
                i32.const 424
                i32.add
                local.set $l2
                loop $L13
                  block $B14
                    local.get $l2
                    i32.load
                    local.tee $l4
                    local.get $p1
                    i32.gt_u
                    br_if $B14
                    local.get $l4
                    local.get $l2
                    i32.load offset=4
                    i32.add
                    local.get $p1
                    i32.gt_u
                    br_if $B12
                  end
                  local.get $l2
                  i32.load offset=8
                  local.tee $l2
                  br_if $L13
                end
              end
              block $B15
                block $B16
                  local.get $p0
                  i32.const 432
                  i32.add
                  i32.load
                  local.tee $p1
                  br_if $B16
                  i32.const 4095
                  local.set $l2
                  br $B15
                end
                i32.const 0
                local.set $l2
                loop $L17
                  local.get $l2
                  i32.const 1
                  i32.add
                  local.set $l2
                  local.get $p1
                  i32.load offset=8
                  local.tee $p1
                  br_if $L17
                end
                local.get $l2
                i32.const 4095
                local.get $l2
                i32.const 4095
                i32.gt_u
                select
                local.set $l2
              end
              local.get $p0
              local.get $l2
              i32.store offset=448
              local.get $l5
              local.get $l3
              i32.le_u
              br_if $B2
              local.get $p0
              i32.const -1
              i32.store offset=440
              return
            end
            local.get $l3
            i32.const -8
            i32.and
            local.tee $l5
            local.get $p1
            i32.add
            local.set $p1
            block $B18
              block $B19
                local.get $l5
                i32.const 256
                i32.lt_u
                br_if $B19
                local.get $p0
                local.get $l4
                call $_ZN8dlmalloc8dlmalloc8Dlmalloc18unlink_large_chunk17h73a5474d5768cd79E
                br $B18
              end
              block $B20
                local.get $l4
                i32.load offset=12
                local.tee $l5
                local.get $l4
                i32.load offset=8
                local.tee $l4
                i32.eq
                br_if $B20
                local.get $l4
                local.get $l5
                i32.store offset=12
                local.get $l5
                local.get $l4
                i32.store offset=8
                br $B18
              end
              local.get $p0
              local.get $p0
              i32.load
              i32.const -2
              local.get $l3
              i32.const 3
              i32.shr_u
              i32.rotl
              i32.and
              i32.store
            end
            local.get $l2
            local.get $p1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get $l2
            local.get $p1
            i32.add
            local.get $p1
            i32.store
            local.get $l2
            local.get $p0
            i32.load offset=408
            i32.ne
            br_if $B7
            local.get $p0
            local.get $p1
            i32.store offset=400
            br $B2
          end
          local.get $p1
          i32.const 256
          i32.lt_u
          br_if $B1
          local.get $p0
          local.get $l2
          local.get $p1
          call $_ZN8dlmalloc8dlmalloc8Dlmalloc18insert_large_chunk17h897361235d504815E
          local.get $p0
          local.get $p0
          i32.load offset=448
          i32.const -1
          i32.add
          local.tee $l2
          i32.store offset=448
          local.get $l2
          br_if $B2
          local.get $p0
          i32.const 432
          i32.add
          i32.load
          local.tee $p1
          br_if $B0
          local.get $p0
          i32.const 4095
          i32.store offset=448
          return
        end
        return
      end
      local.get $p0
      local.get $p1
      i32.const 3
      i32.shr_u
      local.tee $l4
      i32.const 3
      i32.shl
      i32.add
      i32.const 8
      i32.add
      local.set $p1
      block $B21
        block $B22
          local.get $p0
          i32.load
          local.tee $l3
          i32.const 1
          local.get $l4
          i32.const 31
          i32.and
          i32.shl
          local.tee $l4
          i32.and
          i32.eqz
          br_if $B22
          local.get $p1
          i32.load offset=8
          local.set $p0
          br $B21
        end
        local.get $p0
        local.get $l3
        local.get $l4
        i32.or
        i32.store
        local.get $p1
        local.set $p0
      end
      local.get $p1
      local.get $l2
      i32.store offset=8
      local.get $p0
      local.get $l2
      i32.store offset=12
      local.get $l2
      local.get $p1
      i32.store offset=12
      local.get $l2
      local.get $p0
      i32.store offset=8
      return
    end
    i32.const 0
    local.set $l2
    loop $L23
      local.get $l2
      i32.const 1
      i32.add
      local.set $l2
      local.get $p1
      i32.load offset=8
      local.tee $p1
      br_if $L23
    end
    local.get $p0
    local.get $l2
    i32.const 4095
    local.get $l2
    i32.const 4095
    i32.gt_u
    select
    i32.store offset=448)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc8memalign17h5a02630834393434E (type $t1) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32)
    i32.const 0
    local.set $l3
    block $B0
      i32.const -65587
      local.get $p1
      i32.const 16
      local.get $p1
      i32.const 16
      i32.gt_u
      select
      local.tee $p1
      i32.sub
      local.get $p2
      i32.le_u
      br_if $B0
      local.get $p0
      local.get $p1
      i32.const 16
      local.get $p2
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get $p2
      i32.const 11
      i32.lt_u
      select
      local.tee $l4
      i32.add
      i32.const 12
      i32.add
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17h40c3085ef28e063eE
      local.tee $p2
      i32.eqz
      br_if $B0
      local.get $p2
      i32.const -8
      i32.add
      local.set $l3
      block $B1
        block $B2
          local.get $p1
          i32.const -1
          i32.add
          local.tee $l5
          local.get $p2
          i32.and
          br_if $B2
          local.get $l3
          local.set $p1
          br $B1
        end
        local.get $p2
        i32.const -4
        i32.add
        local.tee $l6
        i32.load
        local.tee $l7
        i32.const -8
        i32.and
        local.get $l5
        local.get $p2
        i32.add
        i32.const 0
        local.get $p1
        i32.sub
        i32.and
        i32.const -8
        i32.add
        local.tee $p2
        local.get $p2
        local.get $p1
        i32.add
        local.get $p2
        local.get $l3
        i32.sub
        i32.const 16
        i32.gt_u
        select
        local.tee $p1
        local.get $l3
        i32.sub
        local.tee $p2
        i32.sub
        local.set $l5
        block $B3
          local.get $l7
          i32.const 3
          i32.and
          i32.eqz
          br_if $B3
          local.get $p1
          local.get $l5
          local.get $p1
          i32.load offset=4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store offset=4
          local.get $p1
          local.get $l5
          i32.add
          local.tee $l5
          local.get $l5
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $l6
          local.get $p2
          local.get $l6
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get $p1
          local.get $p1
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $p0
          local.get $l3
          local.get $p2
          call $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17hebbed6771d16a3fcE
          br $B1
        end
        local.get $l3
        i32.load
        local.set $l3
        local.get $p1
        local.get $l5
        i32.store offset=4
        local.get $p1
        local.get $l3
        local.get $p2
        i32.add
        i32.store
      end
      block $B4
        local.get $p1
        i32.load offset=4
        local.tee $p2
        i32.const 3
        i32.and
        i32.eqz
        br_if $B4
        local.get $p2
        i32.const -8
        i32.and
        local.tee $l3
        local.get $l4
        i32.const 16
        i32.add
        i32.le_u
        br_if $B4
        local.get $p1
        local.get $l4
        local.get $p2
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get $p1
        local.get $l4
        i32.add
        local.tee $p2
        local.get $l3
        local.get $l4
        i32.sub
        local.tee $l4
        i32.const 3
        i32.or
        i32.store offset=4
        local.get $p1
        local.get $l3
        i32.add
        local.tee $l3
        local.get $l3
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get $p0
        local.get $p2
        local.get $l4
        call $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17hebbed6771d16a3fcE
      end
      local.get $p1
      i32.const 8
      i32.add
      local.set $l3
    end
    local.get $l3)
  (func $_ZN5alloc5alloc18handle_alloc_error17hf1cfd628decbe217E (type $t0) (param $p0 i32) (param $p1 i32)
    local.get $p0
    local.get $p1
    call $rust_oom
    unreachable)
  (func $_ZN5alloc7raw_vec17capacity_overflow17hd5e3f0106831a51cE (type $t9)
    i32.const 1048871
    i32.const 17
    i32.const 1048888
    call $_ZN4core9panicking5panic17h8634ac164c1f3136E
    unreachable)
  (func $_ZN4core3ptr18real_drop_in_place17h92d8df8ff4cc66e7E (type $t7) (param $p0 i32))
  (func $_ZN4core9panicking18panic_bounds_check17h837d2d4a7b6446ecE (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l3 i32)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    local.get $p2
    i32.store offset=4
    local.get $l3
    local.get $p1
    i32.store
    local.get $l3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get $l3
    i32.const 44
    i32.add
    i32.const 13
    i32.store
    local.get $l3
    i64.const 2
    i64.store offset=12 align=4
    local.get $l3
    i32.const 1048972
    i32.store offset=8
    local.get $l3
    i32.const 13
    i32.store offset=36
    local.get $l3
    local.get $l3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get $l3
    local.get $l3
    i32.store offset=40
    local.get $l3
    local.get $l3
    i32.const 4
    i32.add
    i32.store offset=32
    local.get $l3
    i32.const 8
    i32.add
    local.get $p0
    call $_ZN4core9panicking9panic_fmt17hf798d32b3aba5420E
    unreachable)
  (func $_ZN4core9panicking5panic17h8634ac164c1f3136E (type $t3) (param $p0 i32) (param $p1 i32) (param $p2 i32)
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
    i32.const 1048904
    i32.store offset=4
    local.get $l2
    i32.const 1
    i32.store
    local.get $l2
    call $rust_begin_unwind
    unreachable)
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hfe0dd1fa9e963fe4E (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i64.load32_u
    i32.const 1
    local.get $p1
    call $_ZN4core3fmt3num3imp7fmt_u6417h17a4175f5d6d8495E)
  (func $_ZN4core3fmt5write17h7b0c1564f380f2eeE (type $t1) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32)
    global.get $g0
    i32.const 64
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    i32.const 36
    i32.add
    local.get $p1
    i32.store
    local.get $l3
    i32.const 52
    i32.add
    local.get $p2
    i32.const 20
    i32.add
    i32.load
    local.tee $l4
    i32.store
    local.get $l3
    i32.const 3
    i32.store8 offset=56
    local.get $l3
    i32.const 44
    i32.add
    local.get $p2
    i32.load offset=16
    local.tee $l5
    local.get $l4
    i32.const 3
    i32.shl
    i32.add
    i32.store
    local.get $l3
    i64.const 137438953472
    i64.store offset=8
    local.get $l3
    local.get $p0
    i32.store offset=32
    i32.const 0
    local.set $l6
    local.get $l3
    i32.const 0
    i32.store offset=24
    local.get $l3
    i32.const 0
    i32.store offset=16
    local.get $l3
    local.get $l5
    i32.store offset=48
    local.get $l3
    local.get $l5
    i32.store offset=40
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              local.get $p2
              i32.load offset=8
              local.tee $l7
              br_if $B4
              local.get $p2
              i32.load
              local.set $l8
              local.get $p2
              i32.load offset=4
              local.tee $l9
              local.get $l4
              local.get $l4
              local.get $l9
              i32.gt_u
              select
              local.tee $l10
              i32.eqz
              br_if $B3
              i32.const 1
              local.set $l4
              local.get $p0
              local.get $l8
              i32.load
              local.get $l8
              i32.load offset=4
              local.get $p1
              i32.load offset=12
              call_indirect (type $t1) $T0
              br_if $B0
              local.get $l8
              i32.const 12
              i32.add
              local.set $p2
              i32.const 1
              local.set $l6
              loop $L5
                block $B6
                  local.get $l5
                  i32.load
                  local.get $l3
                  i32.const 8
                  i32.add
                  local.get $l5
                  i32.const 4
                  i32.add
                  i32.load
                  call_indirect (type $t2) $T0
                  i32.eqz
                  br_if $B6
                  i32.const 1
                  local.set $l4
                  br $B0
                end
                local.get $l6
                local.get $l10
                i32.ge_u
                br_if $B3
                local.get $p2
                i32.const -4
                i32.add
                local.set $p0
                local.get $p2
                i32.load
                local.set $p1
                local.get $p2
                i32.const 8
                i32.add
                local.set $p2
                local.get $l5
                i32.const 8
                i32.add
                local.set $l5
                i32.const 1
                local.set $l4
                local.get $l6
                i32.const 1
                i32.add
                local.set $l6
                local.get $l3
                i32.load offset=32
                local.get $p0
                i32.load
                local.get $p1
                local.get $l3
                i32.load offset=36
                i32.load offset=12
                call_indirect (type $t1) $T0
                i32.eqz
                br_if $L5
                br $B0
              end
            end
            local.get $p2
            i32.load
            local.set $l8
            local.get $p2
            i32.load offset=4
            local.tee $l9
            local.get $p2
            i32.const 12
            i32.add
            i32.load
            local.tee $l5
            local.get $l5
            local.get $l9
            i32.gt_u
            select
            local.tee $l10
            i32.eqz
            br_if $B3
            i32.const 1
            local.set $l4
            local.get $p0
            local.get $l8
            i32.load
            local.get $l8
            i32.load offset=4
            local.get $p1
            i32.load offset=12
            call_indirect (type $t1) $T0
            br_if $B0
            local.get $l8
            i32.const 12
            i32.add
            local.set $p2
            local.get $l7
            i32.const 16
            i32.add
            local.set $l5
            i32.const 1
            local.set $l6
            loop $L7
              local.get $l3
              local.get $l5
              i32.const -8
              i32.add
              i32.load
              i32.store offset=12
              local.get $l3
              local.get $l5
              i32.const 16
              i32.add
              i32.load8_u
              i32.store8 offset=56
              local.get $l3
              local.get $l5
              i32.const -4
              i32.add
              i32.load
              i32.store offset=8
              i32.const 0
              local.set $p1
              i32.const 0
              local.set $p0
              block $B8
                block $B9
                  block $B10
                    block $B11
                      local.get $l5
                      i32.const 8
                      i32.add
                      i32.load
                      br_table $B11 $B10 $B9 $B8 $B11
                    end
                    local.get $l5
                    i32.const 12
                    i32.add
                    i32.load
                    local.set $l4
                    i32.const 1
                    local.set $p0
                    br $B8
                  end
                  block $B12
                    local.get $l5
                    i32.const 12
                    i32.add
                    i32.load
                    local.tee $l7
                    local.get $l3
                    i32.load offset=52
                    local.tee $l4
                    i32.ge_u
                    br_if $B12
                    i32.const 0
                    local.set $p0
                    local.get $l3
                    i32.load offset=48
                    local.get $l7
                    i32.const 3
                    i32.shl
                    i32.add
                    local.tee $l7
                    i32.load offset=4
                    i32.const 14
                    i32.ne
                    br_if $B8
                    local.get $l7
                    i32.load
                    i32.load
                    local.set $l4
                    i32.const 1
                    local.set $p0
                    br $B8
                  end
                  i32.const 1049308
                  local.get $l7
                  local.get $l4
                  call $_ZN4core9panicking18panic_bounds_check17h837d2d4a7b6446ecE
                  unreachable
                end
                i32.const 0
                local.set $p0
                local.get $l3
                i32.load offset=40
                local.tee $l7
                local.get $l3
                i32.load offset=44
                i32.eq
                br_if $B8
                local.get $l3
                local.get $l7
                i32.const 8
                i32.add
                i32.store offset=40
                i32.const 0
                local.set $p0
                local.get $l7
                i32.load offset=4
                i32.const 14
                i32.ne
                br_if $B8
                local.get $l7
                i32.load
                i32.load
                local.set $l4
                i32.const 1
                local.set $p0
              end
              local.get $l3
              local.get $l4
              i32.store offset=20
              local.get $l3
              local.get $p0
              i32.store offset=16
              block $B13
                block $B14
                  block $B15
                    block $B16
                      block $B17
                        block $B18
                          block $B19
                            local.get $l5
                            i32.load
                            br_table $B15 $B18 $B19 $B13 $B15
                          end
                          local.get $l3
                          i32.load offset=40
                          local.tee $p0
                          local.get $l3
                          i32.load offset=44
                          i32.ne
                          br_if $B17
                          br $B13
                        end
                        local.get $l5
                        i32.const 4
                        i32.add
                        i32.load
                        local.tee $p0
                        local.get $l3
                        i32.load offset=52
                        local.tee $l4
                        i32.ge_u
                        br_if $B16
                        local.get $l3
                        i32.load offset=48
                        local.get $p0
                        i32.const 3
                        i32.shl
                        i32.add
                        local.tee $p0
                        i32.load offset=4
                        i32.const 14
                        i32.ne
                        br_if $B13
                        local.get $p0
                        i32.load
                        i32.load
                        local.set $l4
                        br $B14
                      end
                      local.get $l3
                      local.get $p0
                      i32.const 8
                      i32.add
                      i32.store offset=40
                      local.get $p0
                      i32.load offset=4
                      i32.const 14
                      i32.ne
                      br_if $B13
                      local.get $p0
                      i32.load
                      i32.load
                      local.set $l4
                      br $B14
                    end
                    i32.const 1049308
                    local.get $p0
                    local.get $l4
                    call $_ZN4core9panicking18panic_bounds_check17h837d2d4a7b6446ecE
                    unreachable
                  end
                  local.get $l5
                  i32.const 4
                  i32.add
                  i32.load
                  local.set $l4
                end
                i32.const 1
                local.set $p1
              end
              local.get $l3
              local.get $l4
              i32.store offset=28
              local.get $l3
              local.get $p1
              i32.store offset=24
              block $B20
                block $B21
                  local.get $l5
                  i32.const -16
                  i32.add
                  i32.load
                  i32.const 1
                  i32.eq
                  br_if $B21
                  local.get $l3
                  i32.load offset=40
                  local.tee $l4
                  local.get $l3
                  i32.load offset=44
                  i32.eq
                  br_if $B2
                  local.get $l3
                  local.get $l4
                  i32.const 8
                  i32.add
                  i32.store offset=40
                  br $B20
                end
                local.get $l5
                i32.const -12
                i32.add
                i32.load
                local.tee $l4
                local.get $l3
                i32.load offset=52
                local.tee $p0
                i32.ge_u
                br_if $B1
                local.get $l3
                i32.load offset=48
                local.get $l4
                i32.const 3
                i32.shl
                i32.add
                local.set $l4
              end
              block $B22
                local.get $l4
                i32.load
                local.get $l3
                i32.const 8
                i32.add
                local.get $l4
                i32.const 4
                i32.add
                i32.load
                call_indirect (type $t2) $T0
                i32.eqz
                br_if $B22
                i32.const 1
                local.set $l4
                br $B0
              end
              local.get $l6
              local.get $l10
              i32.ge_u
              br_if $B3
              local.get $p2
              i32.const -4
              i32.add
              local.set $p0
              local.get $p2
              i32.load
              local.set $p1
              local.get $p2
              i32.const 8
              i32.add
              local.set $p2
              local.get $l5
              i32.const 36
              i32.add
              local.set $l5
              i32.const 1
              local.set $l4
              local.get $l6
              i32.const 1
              i32.add
              local.set $l6
              local.get $l3
              i32.load offset=32
              local.get $p0
              i32.load
              local.get $p1
              local.get $l3
              i32.load offset=36
              i32.load offset=12
              call_indirect (type $t1) $T0
              i32.eqz
              br_if $L7
              br $B0
            end
          end
          block $B23
            local.get $l9
            local.get $l6
            i32.le_u
            br_if $B23
            i32.const 1
            local.set $l4
            local.get $l3
            i32.load offset=32
            local.get $l8
            local.get $l6
            i32.const 3
            i32.shl
            i32.add
            local.tee $l5
            i32.load
            local.get $l5
            i32.load offset=4
            local.get $l3
            i32.load offset=36
            i32.load offset=12
            call_indirect (type $t1) $T0
            br_if $B0
          end
          i32.const 0
          local.set $l4
          br $B0
        end
        i32.const 1048988
        i32.const 43
        i32.const 1049052
        call $_ZN4core9panicking5panic17h8634ac164c1f3136E
        unreachable
      end
      i32.const 1049292
      local.get $l4
      local.get $p0
      call $_ZN4core9panicking18panic_bounds_check17h837d2d4a7b6446ecE
      unreachable
    end
    local.get $l3
    i32.const 64
    i32.add
    global.set $g0
    local.get $l4)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h4b559cca434c02ebE (type $t6) (param $p0 i32) (result i64)
    i64.const -237851497739055091)
  (func $_ZN4core5panic9PanicInfo7message17h64849a473b74b55fE (type $t4) (param $p0 i32) (result i32)
    local.get $p0
    i32.load offset=8)
  (func $_ZN4core5panic9PanicInfo8location17h3e3164c4042e2a35E (type $t4) (param $p0 i32) (result i32)
    local.get $p0
    i32.load offset=12)
  (func $_ZN4core5panic8Location20internal_constructor17h8892f1c8e0dbead5E (type $t10) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32)
    local.get $p0
    local.get $p4
    i32.store offset=12
    local.get $p0
    local.get $p3
    i32.store offset=8
    local.get $p0
    local.get $p2
    i32.store offset=4
    local.get $p0
    local.get $p1
    i32.store)
  (func $_ZN4core5panic8Location4file17hf03dc98b95c00bcbE (type $t0) (param $p0 i32) (param $p1 i32)
    local.get $p0
    local.get $p1
    i64.load align=4
    i64.store align=4)
  (func $_ZN4core5panic8Location4line17h11b7e9ca374cd1e0E (type $t4) (param $p0 i32) (result i32)
    local.get $p0
    i32.load offset=8)
  (func $_ZN4core5panic8Location6column17h202347b4fd2792afE (type $t4) (param $p0 i32) (result i32)
    local.get $p0
    i32.load offset=12)
  (func $_ZN4core3fmt10ArgumentV110show_usize17hd9dfa4b645fc087dE (type $t2) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i64.load32_u
    i32.const 1
    local.get $p1
    call $_ZN4core3fmt3num3imp7fmt_u6417h17a4175f5d6d8495E)
  (func $_ZN4core3fmt3num3imp7fmt_u6417h17a4175f5d6d8495E (type $t11) (param $p0 i64) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i64) (local $l6 i32) (local $l7 i32) (local $l8 i32)
    global.get $g0
    i32.const 48
    i32.sub
    local.tee $l3
    global.set $g0
    i32.const 39
    local.set $l4
    block $B0
      block $B1
        local.get $p0
        i64.const 10000
        i64.ge_u
        br_if $B1
        local.get $p0
        local.set $l5
        br $B0
      end
      i32.const 39
      local.set $l4
      loop $L2
        local.get $l3
        i32.const 9
        i32.add
        local.get $l4
        i32.add
        local.tee $l6
        i32.const -4
        i32.add
        local.get $p0
        local.get $p0
        i64.const 10000
        i64.div_u
        local.tee $l5
        i64.const 10000
        i64.mul
        i64.sub
        i32.wrap_i64
        local.tee $l7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee $l8
        i32.const 1
        i32.shl
        i32.const 1049068
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get $l6
        i32.const -2
        i32.add
        local.get $l7
        local.get $l8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.const 1049068
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get $l4
        i32.const -4
        i32.add
        local.set $l4
        local.get $p0
        i64.const 99999999
        i64.gt_u
        local.set $l6
        local.get $l5
        local.set $p0
        local.get $l6
        br_if $L2
      end
    end
    block $B3
      local.get $l5
      i32.wrap_i64
      local.tee $l6
      i32.const 99
      i32.le_s
      br_if $B3
      local.get $l3
      i32.const 9
      i32.add
      local.get $l4
      i32.const -2
      i32.add
      local.tee $l4
      i32.add
      local.get $l5
      i32.wrap_i64
      local.tee $l6
      local.get $l6
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee $l6
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.const 1049068
      i32.add
      i32.load16_u align=1
      i32.store16 align=1
    end
    block $B4
      block $B5
        local.get $l6
        i32.const 10
        i32.lt_s
        br_if $B5
        local.get $l3
        i32.const 9
        i32.add
        local.get $l4
        i32.const -2
        i32.add
        local.tee $l4
        i32.add
        local.get $l6
        i32.const 1
        i32.shl
        i32.const 1049068
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        br $B4
      end
      local.get $l3
      i32.const 9
      i32.add
      local.get $l4
      i32.const -1
      i32.add
      local.tee $l4
      i32.add
      local.get $l6
      i32.const 48
      i32.add
      i32.store8
    end
    local.get $p2
    local.get $p1
    i32.const 1048904
    i32.const 0
    local.get $l3
    i32.const 9
    i32.add
    local.get $l4
    i32.add
    i32.const 39
    local.get $l4
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17hbda17a40a9acc297E
    local.set $l4
    local.get $l3
    i32.const 48
    i32.add
    global.set $g0
    local.get $l4)
  (func $_ZN4core3fmt9Formatter12pad_integral17hbda17a40a9acc297E (type $t12) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32) (param $p5 i32) (result i32)
    (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32)
    block $B0
      block $B1
        local.get $p1
        i32.eqz
        br_if $B1
        i32.const 43
        i32.const 1114112
        local.get $p0
        i32.load
        local.tee $l6
        i32.const 1
        i32.and
        local.tee $p1
        select
        local.set $l7
        local.get $p1
        local.get $p5
        i32.add
        local.set $l8
        br $B0
      end
      local.get $p5
      i32.const 1
      i32.add
      local.set $l8
      local.get $p0
      i32.load
      local.set $l6
      i32.const 45
      local.set $l7
    end
    block $B2
      block $B3
        local.get $l6
        i32.const 4
        i32.and
        br_if $B3
        i32.const 0
        local.set $p2
        br $B2
      end
      i32.const 0
      local.set $l9
      block $B4
        local.get $p3
        i32.eqz
        br_if $B4
        local.get $p3
        local.set $l10
        local.get $p2
        local.set $p1
        loop $L5
          local.get $l9
          local.get $p1
          i32.load8_u
          i32.const 192
          i32.and
          i32.const 128
          i32.eq
          i32.add
          local.set $l9
          local.get $p1
          i32.const 1
          i32.add
          local.set $p1
          local.get $l10
          i32.const -1
          i32.add
          local.tee $l10
          br_if $L5
        end
      end
      local.get $l8
      local.get $p3
      i32.add
      local.get $l9
      i32.sub
      local.set $l8
    end
    i32.const 1
    local.set $p1
    block $B6
      block $B7
        local.get $p0
        i32.load offset=8
        i32.const 1
        i32.eq
        br_if $B7
        local.get $p0
        local.get $l7
        local.get $p2
        local.get $p3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha6f9580d9685fbbbE
        br_if $B6
        local.get $p0
        i32.load offset=24
        local.get $p4
        local.get $p5
        local.get $p0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type $t1) $T0
        return
      end
      block $B8
        local.get $p0
        i32.const 12
        i32.add
        i32.load
        local.tee $l9
        local.get $l8
        i32.gt_u
        br_if $B8
        local.get $p0
        local.get $l7
        local.get $p2
        local.get $p3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha6f9580d9685fbbbE
        br_if $B6
        local.get $p0
        i32.load offset=24
        local.get $p4
        local.get $p5
        local.get $p0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type $t1) $T0
        return
      end
      block $B9
        block $B10
          local.get $l6
          i32.const 8
          i32.and
          br_if $B10
          i32.const 0
          local.set $p1
          local.get $l9
          local.get $l8
          i32.sub
          local.tee $l9
          local.set $l8
          block $B11
            block $B12
              block $B13
                i32.const 1
                local.get $p0
                i32.load8_u offset=48
                local.tee $l10
                local.get $l10
                i32.const 3
                i32.eq
                select
                br_table $B11 $B12 $B13 $B12 $B11
              end
              local.get $l9
              i32.const 1
              i32.shr_u
              local.set $p1
              local.get $l9
              i32.const 1
              i32.add
              i32.const 1
              i32.shr_u
              local.set $l8
              br $B11
            end
            i32.const 0
            local.set $l8
            local.get $l9
            local.set $p1
          end
          local.get $p1
          i32.const 1
          i32.add
          local.set $p1
          loop $L14
            local.get $p1
            i32.const -1
            i32.add
            local.tee $p1
            i32.eqz
            br_if $B9
            local.get $p0
            i32.load offset=24
            local.get $p0
            i32.load offset=4
            local.get $p0
            i32.load offset=28
            i32.load offset=16
            call_indirect (type $t2) $T0
            i32.eqz
            br_if $L14
          end
          i32.const 1
          return
        end
        i32.const 1
        local.set $p1
        local.get $p0
        i32.const 1
        i32.store8 offset=48
        local.get $p0
        i32.const 48
        i32.store offset=4
        local.get $p0
        local.get $l7
        local.get $p2
        local.get $p3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha6f9580d9685fbbbE
        br_if $B6
        i32.const 0
        local.set $p1
        local.get $l9
        local.get $l8
        i32.sub
        local.tee $l10
        local.set $p3
        block $B15
          block $B16
            block $B17
              i32.const 1
              local.get $p0
              i32.load8_u offset=48
              local.tee $l9
              local.get $l9
              i32.const 3
              i32.eq
              select
              br_table $B15 $B16 $B17 $B16 $B15
            end
            local.get $l10
            i32.const 1
            i32.shr_u
            local.set $p1
            local.get $l10
            i32.const 1
            i32.add
            i32.const 1
            i32.shr_u
            local.set $p3
            br $B15
          end
          i32.const 0
          local.set $p3
          local.get $l10
          local.set $p1
        end
        local.get $p1
        i32.const 1
        i32.add
        local.set $p1
        block $B18
          loop $L19
            local.get $p1
            i32.const -1
            i32.add
            local.tee $p1
            i32.eqz
            br_if $B18
            local.get $p0
            i32.load offset=24
            local.get $p0
            i32.load offset=4
            local.get $p0
            i32.load offset=28
            i32.load offset=16
            call_indirect (type $t2) $T0
            i32.eqz
            br_if $L19
          end
          i32.const 1
          return
        end
        local.get $p0
        i32.load offset=4
        local.set $l10
        i32.const 1
        local.set $p1
        local.get $p0
        i32.load offset=24
        local.get $p4
        local.get $p5
        local.get $p0
        i32.load offset=28
        i32.load offset=12
        call_indirect (type $t1) $T0
        br_if $B6
        local.get $p3
        i32.const 1
        i32.add
        local.set $l9
        local.get $p0
        i32.load offset=28
        local.set $p3
        local.get $p0
        i32.load offset=24
        local.set $p0
        loop $L20
          block $B21
            local.get $l9
            i32.const -1
            i32.add
            local.tee $l9
            br_if $B21
            i32.const 0
            return
          end
          i32.const 1
          local.set $p1
          local.get $p0
          local.get $l10
          local.get $p3
          i32.load offset=16
          call_indirect (type $t2) $T0
          i32.eqz
          br_if $L20
          br $B6
        end
      end
      local.get $p0
      i32.load offset=4
      local.set $l10
      i32.const 1
      local.set $p1
      local.get $p0
      local.get $l7
      local.get $p2
      local.get $p3
      call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha6f9580d9685fbbbE
      br_if $B6
      local.get $p0
      i32.load offset=24
      local.get $p4
      local.get $p5
      local.get $p0
      i32.load offset=28
      i32.load offset=12
      call_indirect (type $t1) $T0
      br_if $B6
      local.get $l8
      i32.const 1
      i32.add
      local.set $l9
      local.get $p0
      i32.load offset=28
      local.set $p3
      local.get $p0
      i32.load offset=24
      local.set $p0
      loop $L22
        block $B23
          local.get $l9
          i32.const -1
          i32.add
          local.tee $l9
          br_if $B23
          i32.const 0
          return
        end
        i32.const 1
        local.set $p1
        local.get $p0
        local.get $l10
        local.get $p3
        i32.load offset=16
        call_indirect (type $t2) $T0
        i32.eqz
        br_if $L22
      end
    end
    local.get $p1)
  (func $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha6f9580d9685fbbbE (type $t5) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (result i32)
    (local $l4 i32)
    block $B0
      block $B1
        local.get $p1
        i32.const 1114112
        i32.eq
        br_if $B1
        i32.const 1
        local.set $l4
        local.get $p0
        i32.load offset=24
        local.get $p1
        local.get $p0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=16
        call_indirect (type $t2) $T0
        br_if $B0
      end
      block $B2
        local.get $p2
        br_if $B2
        i32.const 0
        return
      end
      local.get $p0
      i32.load offset=24
      local.get $p2
      local.get $p3
      local.get $p0
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type $t1) $T0
      local.set $l4
    end
    local.get $l4)
  (func $memcpy (type $t1) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
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
  (table $T0 17 17 funcref)
  (memory $memory 17)
  (global $g0 (mut i32) (i32.const 1048576))
  (global $__data_end i32 (i32.const 1049808))
  (global $__heap_base i32 (i32.const 1049808))
  (export "memory" (memory 0))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (export "receive" (func $receive))
  (elem $e0 (i32.const 1) $_ZN3std5alloc24default_alloc_error_hook17hc30c66deb02056a9E $_ZN76_$LT$std..sys_common..thread_local..Key$u20$as$u20$core..ops..drop..Drop$GT$4drop17hd885bcda30b950bdE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17hcdd4c55de6a15b21E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h57b0b1f21c89129dE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h23ab4e8c6c4a792aE $_ZN4core3ptr18real_drop_in_place17h06d59ef2124c7ddcE $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3eff61b58f08666fE $_ZN4core3ptr18real_drop_in_place17hf704f9372cabec9cE $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h009a14389072927fE $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17hfa4cbf18d8940863E $_ZN4core3ptr18real_drop_in_place17h23cbb650c6fd0cddE $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hb5328de7baf27548E $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hfe0dd1fa9e963fe4E $_ZN4core3fmt10ArgumentV110show_usize17hd9dfa4b645fc087dE $_ZN4core3ptr18real_drop_in_place17h92d8df8ff4cc66e7E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h4b559cca434c02ebE)
  (data $d0 (i32.const 1048576) "add\5csrc\5clib.rs\00\00\00\00\10\00\0e\00\00\00\0c\00\00\00\09\00\00\00attempt to add with overflow\02\00\00\00\04\00\00\00\04\00\00\00\03\00\00\00\04\00\00\00\05\00\00\00/rustc/f3e1a954d2ead4e2fc197c7da7d71e6c61bad196/src/libcore/macros/mod.rs\00\00\00T\00\10\00I\00\00\00\0f\00\00\00(\00\00\00\06\00\00\00\00\00\00\00\01\00\00\00\07\00\00\00called `Option::unwrap()` on a `None` value\00\08\00\00\00\10\00\00\00\04\00\00\00\09\00\00\00\0a\00\00\00\0b\00\00\00\0c\00\00\00\04\00\00\00\0c\00\00\00src/liballoc/raw_vec.rscapacity overflow\10\01\10\00\17\00\00\00\09\03\00\00\05\00\00\00\0f\00\00\00\00\00\00\00\01\00\00\00\10\00\00\00index out of bounds: the len is  but the index is \00\00X\01\10\00 \00\00\00x\01\10\00\12\00\00\00called `Option::unwrap()` on a `None` valuesrc/libcore/option.rs\c7\01\10\00\15\00\00\00}\01\00\00\15\00\00\0000010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899src/libcore/fmt/mod.rs\00\00\b4\02\10\00\16\00\00\00S\04\00\00(\00\00\00\b4\02\10\00\16\00\00\00^\04\00\00(\00\00\00")
  (data $d1 (i32.const 1049328) "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"))
