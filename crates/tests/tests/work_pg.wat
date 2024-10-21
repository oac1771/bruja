(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func (param i32 i32 i32) (result i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (param i32 i32 i32 i32)))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func (param i32) (result i32)))
  (type (;7;) (func (param i64 i32 i32) (result i32)))
  (type (;8;) (func (param i32 i32 i32 i32 i32 i32)))
  (type (;9;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;10;) (func))
  (func (;0;) (type 6) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 8
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.const 245
                  i32.ge_u
                  if  ;; label = @8
                    local.get 0
                    i32.const -65587
                    i32.ge_u
                    br_if 7 (;@1;)
                    local.get 0
                    i32.const 11
                    i32.add
                    local.tee 0
                    i32.const -8
                    i32.and
                    local.set 5
                    i32.const 1051976
                    i32.load
                    local.tee 9
                    i32.eqz
                    br_if 4 (;@4;)
                    i32.const 0
                    local.get 5
                    i32.sub
                    local.set 3
                    block (result i32)  ;; label = @9
                      i32.const 0
                      local.get 5
                      i32.const 256
                      i32.lt_u
                      br_if 0 (;@9;)
                      drop
                      i32.const 31
                      local.get 5
                      i32.const 16777215
                      i32.gt_u
                      br_if 0 (;@9;)
                      drop
                      local.get 5
                      i32.const 6
                      local.get 0
                      i32.const 8
                      i32.shr_u
                      i32.clz
                      local.tee 0
                      i32.sub
                      i32.shr_u
                      i32.const 1
                      i32.and
                      local.get 0
                      i32.const 1
                      i32.shl
                      i32.sub
                      i32.const 62
                      i32.add
                    end
                    local.tee 7
                    i32.const 2
                    i32.shl
                    i32.const 1051564
                    i32.add
                    i32.load
                    local.tee 2
                    i32.eqz
                    if  ;; label = @9
                      i32.const 0
                      local.set 0
                      br 2 (;@7;)
                    end
                    i32.const 0
                    local.set 0
                    local.get 5
                    i32.const 25
                    local.get 7
                    i32.const 1
                    i32.shr_u
                    i32.sub
                    i32.const 0
                    local.get 7
                    i32.const 31
                    i32.ne
                    select
                    i32.shl
                    local.set 4
                    loop  ;; label = @9
                      block  ;; label = @10
                        local.get 2
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.tee 6
                        local.get 5
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 6
                        local.get 5
                        i32.sub
                        local.tee 6
                        local.get 3
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 2
                        local.set 1
                        local.get 6
                        local.tee 3
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 3
                        local.get 1
                        local.set 0
                        br 4 (;@6;)
                      end
                      local.get 2
                      i32.load offset=20
                      local.tee 6
                      local.get 0
                      local.get 6
                      local.get 2
                      local.get 4
                      i32.const 29
                      i32.shr_u
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      i32.load
                      local.tee 2
                      i32.ne
                      select
                      local.get 0
                      local.get 6
                      select
                      local.set 0
                      local.get 4
                      i32.const 1
                      i32.shl
                      local.set 4
                      local.get 2
                      br_if 0 (;@9;)
                    end
                    br 1 (;@7;)
                  end
                  i32.const 1051972
                  i32.load
                  local.tee 2
                  i32.const 16
                  local.get 0
                  i32.const 11
                  i32.add
                  i32.const 504
                  i32.and
                  local.get 0
                  i32.const 11
                  i32.lt_u
                  select
                  local.tee 5
                  i32.const 3
                  i32.shr_u
                  local.tee 0
                  i32.shr_u
                  local.tee 1
                  i32.const 3
                  i32.and
                  if  ;; label = @8
                    block  ;; label = @9
                      local.get 1
                      i32.const -1
                      i32.xor
                      i32.const 1
                      i32.and
                      local.get 0
                      i32.add
                      local.tee 1
                      i32.const 3
                      i32.shl
                      local.tee 0
                      i32.const 1051708
                      i32.add
                      local.tee 4
                      local.get 0
                      i32.const 1051716
                      i32.add
                      i32.load
                      local.tee 0
                      i32.load offset=8
                      local.tee 3
                      i32.ne
                      if  ;; label = @10
                        local.get 3
                        local.get 4
                        i32.store offset=12
                        local.get 4
                        local.get 3
                        i32.store offset=8
                        br 1 (;@9;)
                      end
                      i32.const 1051972
                      local.get 2
                      i32.const -2
                      local.get 1
                      i32.rotl
                      i32.and
                      i32.store
                    end
                    local.get 0
                    i32.const 8
                    i32.add
                    local.set 3
                    local.get 0
                    local.get 1
                    i32.const 3
                    i32.shl
                    local.tee 1
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    local.get 0
                    local.get 1
                    i32.add
                    local.tee 0
                    local.get 0
                    i32.load offset=4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    br 7 (;@1;)
                  end
                  local.get 5
                  i32.const 1051980
                  i32.load
                  i32.le_u
                  br_if 3 (;@4;)
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 1
                      i32.eqz
                      if  ;; label = @10
                        i32.const 1051976
                        i32.load
                        local.tee 0
                        i32.eqz
                        br_if 6 (;@4;)
                        local.get 0
                        i32.ctz
                        i32.const 2
                        i32.shl
                        i32.const 1051564
                        i32.add
                        i32.load
                        local.tee 1
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 5
                        i32.sub
                        local.set 3
                        local.get 1
                        local.set 2
                        loop  ;; label = @11
                          block  ;; label = @12
                            local.get 1
                            i32.load offset=16
                            local.tee 0
                            br_if 0 (;@12;)
                            local.get 1
                            i32.load offset=20
                            local.tee 0
                            br_if 0 (;@12;)
                            local.get 2
                            i32.load offset=24
                            local.set 7
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 2
                                local.get 2
                                i32.load offset=12
                                local.tee 0
                                i32.eq
                                if  ;; label = @15
                                  local.get 2
                                  i32.const 20
                                  i32.const 16
                                  local.get 2
                                  i32.load offset=20
                                  local.tee 0
                                  select
                                  i32.add
                                  i32.load
                                  local.tee 1
                                  br_if 1 (;@14;)
                                  i32.const 0
                                  local.set 0
                                  br 2 (;@13;)
                                end
                                local.get 2
                                i32.load offset=8
                                local.tee 1
                                local.get 0
                                i32.store offset=12
                                local.get 0
                                local.get 1
                                i32.store offset=8
                                br 1 (;@13;)
                              end
                              local.get 2
                              i32.const 20
                              i32.add
                              local.get 2
                              i32.const 16
                              i32.add
                              local.get 0
                              select
                              local.set 4
                              loop  ;; label = @14
                                local.get 4
                                local.set 6
                                local.get 1
                                local.tee 0
                                i32.const 20
                                i32.add
                                local.get 0
                                i32.const 16
                                i32.add
                                local.get 0
                                i32.load offset=20
                                local.tee 1
                                select
                                local.set 4
                                local.get 0
                                i32.const 20
                                i32.const 16
                                local.get 1
                                select
                                i32.add
                                i32.load
                                local.tee 1
                                br_if 0 (;@14;)
                              end
                              local.get 6
                              i32.const 0
                              i32.store
                            end
                            local.get 7
                            i32.eqz
                            br_if 4 (;@8;)
                            local.get 2
                            local.get 2
                            i32.load offset=28
                            i32.const 2
                            i32.shl
                            i32.const 1051564
                            i32.add
                            local.tee 1
                            i32.load
                            i32.ne
                            if  ;; label = @13
                              local.get 7
                              i32.const 16
                              i32.const 20
                              local.get 7
                              i32.load offset=16
                              local.get 2
                              i32.eq
                              select
                              i32.add
                              local.get 0
                              i32.store
                              local.get 0
                              i32.eqz
                              br_if 5 (;@8;)
                              br 4 (;@9;)
                            end
                            local.get 1
                            local.get 0
                            i32.store
                            local.get 0
                            br_if 3 (;@9;)
                            i32.const 1051976
                            i32.const 1051976
                            i32.load
                            i32.const -2
                            local.get 2
                            i32.load offset=28
                            i32.rotl
                            i32.and
                            i32.store
                            br 4 (;@8;)
                          end
                          local.get 0
                          i32.load offset=4
                          i32.const -8
                          i32.and
                          local.get 5
                          i32.sub
                          local.tee 1
                          local.get 3
                          local.get 1
                          local.get 3
                          i32.lt_u
                          local.tee 1
                          select
                          local.set 3
                          local.get 0
                          local.get 2
                          local.get 1
                          select
                          local.set 2
                          local.get 0
                          local.set 1
                          br 0 (;@11;)
                        end
                        unreachable
                      end
                      block  ;; label = @10
                        i32.const 2
                        local.get 0
                        i32.shl
                        local.tee 4
                        i32.const 0
                        local.get 4
                        i32.sub
                        i32.or
                        local.get 1
                        local.get 0
                        i32.shl
                        i32.and
                        i32.ctz
                        local.tee 1
                        i32.const 3
                        i32.shl
                        local.tee 0
                        i32.const 1051708
                        i32.add
                        local.tee 4
                        local.get 0
                        i32.const 1051716
                        i32.add
                        i32.load
                        local.tee 0
                        i32.load offset=8
                        local.tee 3
                        i32.ne
                        if  ;; label = @11
                          local.get 3
                          local.get 4
                          i32.store offset=12
                          local.get 4
                          local.get 3
                          i32.store offset=8
                          br 1 (;@10;)
                        end
                        i32.const 1051972
                        local.get 2
                        i32.const -2
                        local.get 1
                        i32.rotl
                        i32.and
                        i32.store
                      end
                      local.get 0
                      local.get 5
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 5
                      i32.add
                      local.tee 6
                      local.get 1
                      i32.const 3
                      i32.shl
                      local.tee 1
                      local.get 5
                      i32.sub
                      local.tee 4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 1
                      i32.add
                      local.get 4
                      i32.store
                      i32.const 1051980
                      i32.load
                      local.tee 3
                      if  ;; label = @10
                        local.get 3
                        i32.const -8
                        i32.and
                        i32.const 1051708
                        i32.add
                        local.set 1
                        i32.const 1051988
                        i32.load
                        local.set 2
                        block (result i32)  ;; label = @11
                          i32.const 1051972
                          i32.load
                          local.tee 5
                          i32.const 1
                          local.get 3
                          i32.const 3
                          i32.shr_u
                          i32.shl
                          local.tee 3
                          i32.and
                          i32.eqz
                          if  ;; label = @12
                            i32.const 1051972
                            local.get 3
                            local.get 5
                            i32.or
                            i32.store
                            local.get 1
                            br 1 (;@11;)
                          end
                          local.get 1
                          i32.load offset=8
                        end
                        local.set 3
                        local.get 1
                        local.get 2
                        i32.store offset=8
                        local.get 3
                        local.get 2
                        i32.store offset=12
                        local.get 2
                        local.get 1
                        i32.store offset=12
                        local.get 2
                        local.get 3
                        i32.store offset=8
                      end
                      local.get 0
                      i32.const 8
                      i32.add
                      local.set 3
                      i32.const 1051988
                      local.get 6
                      i32.store
                      i32.const 1051980
                      local.get 4
                      i32.store
                      br 8 (;@1;)
                    end
                    local.get 0
                    local.get 7
                    i32.store offset=24
                    local.get 2
                    i32.load offset=16
                    local.tee 1
                    if  ;; label = @9
                      local.get 0
                      local.get 1
                      i32.store offset=16
                      local.get 1
                      local.get 0
                      i32.store offset=24
                    end
                    local.get 2
                    i32.load offset=20
                    local.tee 1
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 1
                    i32.store offset=20
                    local.get 1
                    local.get 0
                    i32.store offset=24
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 3
                      i32.const 16
                      i32.ge_u
                      if  ;; label = @10
                        local.get 2
                        local.get 5
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 2
                        local.get 5
                        i32.add
                        local.tee 4
                        local.get 3
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 3
                        local.get 4
                        i32.add
                        local.get 3
                        i32.store
                        i32.const 1051980
                        i32.load
                        local.tee 6
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 6
                        i32.const -8
                        i32.and
                        i32.const 1051708
                        i32.add
                        local.set 0
                        i32.const 1051988
                        i32.load
                        local.set 1
                        block (result i32)  ;; label = @11
                          i32.const 1051972
                          i32.load
                          local.tee 5
                          i32.const 1
                          local.get 6
                          i32.const 3
                          i32.shr_u
                          i32.shl
                          local.tee 6
                          i32.and
                          i32.eqz
                          if  ;; label = @12
                            i32.const 1051972
                            local.get 5
                            local.get 6
                            i32.or
                            i32.store
                            local.get 0
                            br 1 (;@11;)
                          end
                          local.get 0
                          i32.load offset=8
                        end
                        local.set 6
                        local.get 0
                        local.get 1
                        i32.store offset=8
                        local.get 6
                        local.get 1
                        i32.store offset=12
                        local.get 1
                        local.get 0
                        i32.store offset=12
                        local.get 1
                        local.get 6
                        i32.store offset=8
                        br 1 (;@9;)
                      end
                      local.get 2
                      local.get 3
                      local.get 5
                      i32.add
                      local.tee 0
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 2
                      i32.add
                      local.tee 0
                      local.get 0
                      i32.load offset=4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      br 1 (;@8;)
                    end
                    i32.const 1051988
                    local.get 4
                    i32.store
                    i32.const 1051980
                    local.get 3
                    i32.store
                  end
                  local.get 2
                  i32.const 8
                  i32.add
                  local.set 3
                  br 6 (;@1;)
                end
                local.get 0
                local.get 1
                i32.or
                i32.eqz
                if  ;; label = @7
                  i32.const 0
                  local.set 1
                  i32.const 2
                  local.get 7
                  i32.shl
                  local.tee 0
                  i32.const 0
                  local.get 0
                  i32.sub
                  i32.or
                  local.get 9
                  i32.and
                  local.tee 0
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 0
                  i32.ctz
                  i32.const 2
                  i32.shl
                  i32.const 1051564
                  i32.add
                  i32.load
                  local.set 0
                end
                local.get 0
                i32.eqz
                br_if 1 (;@5;)
              end
              loop  ;; label = @6
                local.get 0
                local.get 1
                local.get 0
                i32.load offset=4
                i32.const -8
                i32.and
                local.tee 4
                local.get 5
                i32.sub
                local.tee 6
                local.get 3
                i32.lt_u
                local.tee 7
                select
                local.set 9
                local.get 0
                i32.load offset=16
                local.tee 2
                i32.eqz
                if  ;; label = @7
                  local.get 0
                  i32.load offset=20
                  local.set 2
                end
                local.get 1
                local.get 9
                local.get 4
                local.get 5
                i32.lt_u
                local.tee 0
                select
                local.set 1
                local.get 3
                local.get 6
                local.get 3
                local.get 7
                select
                local.get 0
                select
                local.set 3
                local.get 2
                local.tee 0
                br_if 0 (;@6;)
              end
            end
            local.get 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            i32.const 1051980
            i32.load
            local.tee 0
            i32.le_u
            local.get 3
            local.get 0
            local.get 5
            i32.sub
            i32.ge_u
            i32.and
            br_if 0 (;@4;)
            local.get 1
            i32.load offset=24
            local.set 7
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                local.get 1
                i32.load offset=12
                local.tee 0
                i32.eq
                if  ;; label = @7
                  local.get 1
                  i32.const 20
                  i32.const 16
                  local.get 1
                  i32.load offset=20
                  local.tee 0
                  select
                  i32.add
                  i32.load
                  local.tee 2
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 0
                  br 2 (;@5;)
                end
                local.get 1
                i32.load offset=8
                local.tee 2
                local.get 0
                i32.store offset=12
                local.get 0
                local.get 2
                i32.store offset=8
                br 1 (;@5;)
              end
              local.get 1
              i32.const 20
              i32.add
              local.get 1
              i32.const 16
              i32.add
              local.get 0
              select
              local.set 4
              loop  ;; label = @6
                local.get 4
                local.set 6
                local.get 2
                local.tee 0
                i32.const 20
                i32.add
                local.get 0
                i32.const 16
                i32.add
                local.get 0
                i32.load offset=20
                local.tee 2
                select
                local.set 4
                local.get 0
                i32.const 20
                i32.const 16
                local.get 2
                select
                i32.add
                i32.load
                local.tee 2
                br_if 0 (;@6;)
              end
              local.get 6
              i32.const 0
              i32.store
            end
            local.get 7
            i32.eqz
            br_if 2 (;@2;)
            local.get 1
            local.get 1
            i32.load offset=28
            i32.const 2
            i32.shl
            i32.const 1051564
            i32.add
            local.tee 2
            i32.load
            i32.ne
            if  ;; label = @5
              local.get 7
              i32.const 16
              i32.const 20
              local.get 7
              i32.load offset=16
              local.get 1
              i32.eq
              select
              i32.add
              local.get 0
              i32.store
              local.get 0
              i32.eqz
              br_if 3 (;@2;)
              br 2 (;@3;)
            end
            local.get 2
            local.get 0
            i32.store
            local.get 0
            br_if 1 (;@3;)
            i32.const 1051976
            i32.const 1051976
            i32.load
            i32.const -2
            local.get 1
            i32.load offset=28
            i32.rotl
            i32.and
            i32.store
            br 2 (;@2;)
          end
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 5
                    i32.const 1051980
                    i32.load
                    local.tee 1
                    i32.gt_u
                    if  ;; label = @9
                      local.get 5
                      i32.const 1051984
                      i32.load
                      local.tee 0
                      i32.ge_u
                      if  ;; label = @10
                        local.get 5
                        i32.const 65583
                        i32.add
                        i32.const -65536
                        i32.and
                        local.tee 2
                        i32.const 16
                        i32.shr_u
                        memory.grow
                        local.set 0
                        local.get 8
                        i32.const 4
                        i32.add
                        local.tee 1
                        i32.const 0
                        i32.store offset=8
                        local.get 1
                        i32.const 0
                        local.get 2
                        i32.const -65536
                        i32.and
                        local.get 0
                        i32.const -1
                        i32.eq
                        local.tee 2
                        select
                        i32.store offset=4
                        local.get 1
                        i32.const 0
                        local.get 0
                        i32.const 16
                        i32.shl
                        local.get 2
                        select
                        i32.store
                        local.get 8
                        i32.load offset=4
                        local.tee 1
                        i32.eqz
                        if  ;; label = @11
                          i32.const 0
                          local.set 3
                          br 10 (;@1;)
                        end
                        local.get 8
                        i32.load offset=12
                        local.set 6
                        i32.const 1051996
                        local.get 8
                        i32.load offset=8
                        local.tee 3
                        i32.const 1051996
                        i32.load
                        i32.add
                        local.tee 0
                        i32.store
                        i32.const 1052000
                        i32.const 1052000
                        i32.load
                        local.tee 2
                        local.get 0
                        local.get 0
                        local.get 2
                        i32.lt_u
                        select
                        i32.store
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 1051992
                            i32.load
                            local.tee 2
                            if  ;; label = @13
                              i32.const 1051692
                              local.set 0
                              loop  ;; label = @14
                                local.get 1
                                local.get 0
                                i32.load
                                local.tee 4
                                local.get 0
                                i32.load offset=4
                                local.tee 7
                                i32.add
                                i32.eq
                                br_if 2 (;@12;)
                                local.get 0
                                i32.load offset=8
                                local.tee 0
                                br_if 0 (;@14;)
                              end
                              br 2 (;@11;)
                            end
                            i32.const 1052008
                            i32.load
                            local.tee 0
                            i32.const 0
                            local.get 0
                            local.get 1
                            i32.le_u
                            select
                            i32.eqz
                            if  ;; label = @13
                              i32.const 1052008
                              local.get 1
                              i32.store
                            end
                            i32.const 1052012
                            i32.const 4095
                            i32.store
                            i32.const 1051704
                            local.get 6
                            i32.store
                            i32.const 1051696
                            local.get 3
                            i32.store
                            i32.const 1051692
                            local.get 1
                            i32.store
                            i32.const 1051720
                            i32.const 1051708
                            i32.store
                            i32.const 1051728
                            i32.const 1051716
                            i32.store
                            i32.const 1051716
                            i32.const 1051708
                            i32.store
                            i32.const 1051736
                            i32.const 1051724
                            i32.store
                            i32.const 1051724
                            i32.const 1051716
                            i32.store
                            i32.const 1051744
                            i32.const 1051732
                            i32.store
                            i32.const 1051732
                            i32.const 1051724
                            i32.store
                            i32.const 1051752
                            i32.const 1051740
                            i32.store
                            i32.const 1051740
                            i32.const 1051732
                            i32.store
                            i32.const 1051760
                            i32.const 1051748
                            i32.store
                            i32.const 1051748
                            i32.const 1051740
                            i32.store
                            i32.const 1051768
                            i32.const 1051756
                            i32.store
                            i32.const 1051756
                            i32.const 1051748
                            i32.store
                            i32.const 1051776
                            i32.const 1051764
                            i32.store
                            i32.const 1051764
                            i32.const 1051756
                            i32.store
                            i32.const 1051784
                            i32.const 1051772
                            i32.store
                            i32.const 1051772
                            i32.const 1051764
                            i32.store
                            i32.const 1051780
                            i32.const 1051772
                            i32.store
                            i32.const 1051792
                            i32.const 1051780
                            i32.store
                            i32.const 1051788
                            i32.const 1051780
                            i32.store
                            i32.const 1051800
                            i32.const 1051788
                            i32.store
                            i32.const 1051796
                            i32.const 1051788
                            i32.store
                            i32.const 1051808
                            i32.const 1051796
                            i32.store
                            i32.const 1051804
                            i32.const 1051796
                            i32.store
                            i32.const 1051816
                            i32.const 1051804
                            i32.store
                            i32.const 1051812
                            i32.const 1051804
                            i32.store
                            i32.const 1051824
                            i32.const 1051812
                            i32.store
                            i32.const 1051820
                            i32.const 1051812
                            i32.store
                            i32.const 1051832
                            i32.const 1051820
                            i32.store
                            i32.const 1051828
                            i32.const 1051820
                            i32.store
                            i32.const 1051840
                            i32.const 1051828
                            i32.store
                            i32.const 1051836
                            i32.const 1051828
                            i32.store
                            i32.const 1051848
                            i32.const 1051836
                            i32.store
                            i32.const 1051856
                            i32.const 1051844
                            i32.store
                            i32.const 1051844
                            i32.const 1051836
                            i32.store
                            i32.const 1051864
                            i32.const 1051852
                            i32.store
                            i32.const 1051852
                            i32.const 1051844
                            i32.store
                            i32.const 1051872
                            i32.const 1051860
                            i32.store
                            i32.const 1051860
                            i32.const 1051852
                            i32.store
                            i32.const 1051880
                            i32.const 1051868
                            i32.store
                            i32.const 1051868
                            i32.const 1051860
                            i32.store
                            i32.const 1051888
                            i32.const 1051876
                            i32.store
                            i32.const 1051876
                            i32.const 1051868
                            i32.store
                            i32.const 1051896
                            i32.const 1051884
                            i32.store
                            i32.const 1051884
                            i32.const 1051876
                            i32.store
                            i32.const 1051904
                            i32.const 1051892
                            i32.store
                            i32.const 1051892
                            i32.const 1051884
                            i32.store
                            i32.const 1051912
                            i32.const 1051900
                            i32.store
                            i32.const 1051900
                            i32.const 1051892
                            i32.store
                            i32.const 1051920
                            i32.const 1051908
                            i32.store
                            i32.const 1051908
                            i32.const 1051900
                            i32.store
                            i32.const 1051928
                            i32.const 1051916
                            i32.store
                            i32.const 1051916
                            i32.const 1051908
                            i32.store
                            i32.const 1051936
                            i32.const 1051924
                            i32.store
                            i32.const 1051924
                            i32.const 1051916
                            i32.store
                            i32.const 1051944
                            i32.const 1051932
                            i32.store
                            i32.const 1051932
                            i32.const 1051924
                            i32.store
                            i32.const 1051952
                            i32.const 1051940
                            i32.store
                            i32.const 1051940
                            i32.const 1051932
                            i32.store
                            i32.const 1051960
                            i32.const 1051948
                            i32.store
                            i32.const 1051948
                            i32.const 1051940
                            i32.store
                            i32.const 1051968
                            i32.const 1051956
                            i32.store
                            i32.const 1051956
                            i32.const 1051948
                            i32.store
                            i32.const 1051992
                            local.get 1
                            i32.const 15
                            i32.add
                            i32.const -8
                            i32.and
                            local.tee 0
                            i32.const 8
                            i32.sub
                            local.tee 2
                            i32.store
                            i32.const 1051964
                            i32.const 1051956
                            i32.store
                            i32.const 1051984
                            local.get 3
                            i32.const 40
                            i32.sub
                            local.tee 4
                            local.get 1
                            local.get 0
                            i32.sub
                            i32.add
                            i32.const 8
                            i32.add
                            local.tee 0
                            i32.store
                            local.get 2
                            local.get 0
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            local.get 1
                            local.get 4
                            i32.add
                            i32.const 40
                            i32.store offset=4
                            i32.const 1052004
                            i32.const 2097152
                            i32.store
                            br 8 (;@4;)
                          end
                          local.get 2
                          local.get 4
                          i32.lt_u
                          local.get 1
                          local.get 2
                          i32.le_u
                          i32.or
                          br_if 0 (;@11;)
                          local.get 0
                          i32.load offset=12
                          local.tee 4
                          i32.const 1
                          i32.and
                          br_if 0 (;@11;)
                          local.get 4
                          i32.const 1
                          i32.shr_u
                          local.get 6
                          i32.eq
                          br_if 3 (;@8;)
                        end
                        i32.const 1052008
                        i32.const 1052008
                        i32.load
                        local.tee 0
                        local.get 1
                        local.get 0
                        local.get 1
                        i32.lt_u
                        select
                        i32.store
                        local.get 1
                        local.get 3
                        i32.add
                        local.set 4
                        i32.const 1051692
                        local.set 0
                        block  ;; label = @11
                          block  ;; label = @12
                            loop  ;; label = @13
                              local.get 4
                              local.get 0
                              i32.load
                              i32.ne
                              if  ;; label = @14
                                local.get 0
                                i32.load offset=8
                                local.tee 0
                                br_if 1 (;@13;)
                                br 2 (;@12;)
                              end
                            end
                            local.get 0
                            i32.load offset=12
                            local.tee 7
                            i32.const 1
                            i32.and
                            br_if 0 (;@12;)
                            local.get 7
                            i32.const 1
                            i32.shr_u
                            local.get 6
                            i32.eq
                            br_if 1 (;@11;)
                          end
                          i32.const 1051692
                          local.set 0
                          loop  ;; label = @12
                            block  ;; label = @13
                              local.get 2
                              local.get 0
                              i32.load
                              local.tee 4
                              i32.ge_u
                              if  ;; label = @14
                                local.get 4
                                local.get 0
                                i32.load offset=4
                                i32.add
                                local.tee 7
                                local.get 2
                                i32.gt_u
                                br_if 1 (;@13;)
                              end
                              local.get 0
                              i32.load offset=8
                              local.set 0
                              br 1 (;@12;)
                            end
                          end
                          i32.const 1051992
                          local.get 1
                          i32.const 15
                          i32.add
                          i32.const -8
                          i32.and
                          local.tee 0
                          i32.const 8
                          i32.sub
                          local.tee 4
                          i32.store
                          i32.const 1051984
                          local.get 3
                          i32.const 40
                          i32.sub
                          local.tee 9
                          local.get 1
                          local.get 0
                          i32.sub
                          i32.add
                          i32.const 8
                          i32.add
                          local.tee 0
                          i32.store
                          local.get 4
                          local.get 0
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 1
                          local.get 9
                          i32.add
                          i32.const 40
                          i32.store offset=4
                          i32.const 1052004
                          i32.const 2097152
                          i32.store
                          local.get 2
                          local.get 7
                          i32.const 32
                          i32.sub
                          i32.const -8
                          i32.and
                          i32.const 8
                          i32.sub
                          local.tee 0
                          local.get 0
                          local.get 2
                          i32.const 16
                          i32.add
                          i32.lt_u
                          select
                          local.tee 4
                          i32.const 27
                          i32.store offset=4
                          i32.const 1051692
                          i64.load align=4
                          local.set 10
                          local.get 4
                          i32.const 16
                          i32.add
                          i32.const 1051700
                          i64.load align=4
                          i64.store align=4
                          local.get 4
                          local.get 10
                          i64.store offset=8 align=4
                          i32.const 1051704
                          local.get 6
                          i32.store
                          i32.const 1051696
                          local.get 3
                          i32.store
                          i32.const 1051692
                          local.get 1
                          i32.store
                          i32.const 1051700
                          local.get 4
                          i32.const 8
                          i32.add
                          i32.store
                          local.get 4
                          i32.const 28
                          i32.add
                          local.set 0
                          loop  ;; label = @12
                            local.get 0
                            i32.const 7
                            i32.store
                            local.get 0
                            i32.const 4
                            i32.add
                            local.tee 0
                            local.get 7
                            i32.lt_u
                            br_if 0 (;@12;)
                          end
                          local.get 2
                          local.get 4
                          i32.eq
                          br_if 7 (;@4;)
                          local.get 4
                          local.get 4
                          i32.load offset=4
                          i32.const -2
                          i32.and
                          i32.store offset=4
                          local.get 2
                          local.get 4
                          local.get 2
                          i32.sub
                          local.tee 0
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 4
                          local.get 0
                          i32.store
                          local.get 0
                          i32.const 256
                          i32.ge_u
                          if  ;; label = @12
                            local.get 2
                            local.get 0
                            call 12
                            br 8 (;@4;)
                          end
                          local.get 0
                          i32.const -8
                          i32.and
                          i32.const 1051708
                          i32.add
                          local.set 1
                          block (result i32)  ;; label = @12
                            i32.const 1051972
                            i32.load
                            local.tee 4
                            i32.const 1
                            local.get 0
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 0
                            i32.and
                            i32.eqz
                            if  ;; label = @13
                              i32.const 1051972
                              local.get 0
                              local.get 4
                              i32.or
                              i32.store
                              local.get 1
                              br 1 (;@12;)
                            end
                            local.get 1
                            i32.load offset=8
                          end
                          local.set 0
                          local.get 1
                          local.get 2
                          i32.store offset=8
                          local.get 0
                          local.get 2
                          i32.store offset=12
                          local.get 2
                          local.get 1
                          i32.store offset=12
                          local.get 2
                          local.get 0
                          i32.store offset=8
                          br 7 (;@4;)
                        end
                        local.get 0
                        local.get 1
                        i32.store
                        local.get 0
                        local.get 0
                        i32.load offset=4
                        local.get 3
                        i32.add
                        i32.store offset=4
                        local.get 1
                        i32.const 15
                        i32.add
                        i32.const -8
                        i32.and
                        i32.const 8
                        i32.sub
                        local.tee 2
                        local.get 5
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 4
                        i32.const 15
                        i32.add
                        i32.const -8
                        i32.and
                        i32.const 8
                        i32.sub
                        local.tee 3
                        local.get 2
                        local.get 5
                        i32.add
                        local.tee 0
                        i32.sub
                        local.set 5
                        local.get 3
                        i32.const 1051992
                        i32.load
                        i32.eq
                        br_if 3 (;@7;)
                        local.get 3
                        i32.const 1051988
                        i32.load
                        i32.eq
                        br_if 4 (;@6;)
                        local.get 3
                        i32.load offset=4
                        local.tee 1
                        i32.const 3
                        i32.and
                        i32.const 1
                        i32.eq
                        if  ;; label = @11
                          local.get 3
                          local.get 1
                          i32.const -8
                          i32.and
                          local.tee 1
                          call 9
                          local.get 1
                          local.get 5
                          i32.add
                          local.set 5
                          local.get 1
                          local.get 3
                          i32.add
                          local.tee 3
                          i32.load offset=4
                          local.set 1
                        end
                        local.get 3
                        local.get 1
                        i32.const -2
                        i32.and
                        i32.store offset=4
                        local.get 0
                        local.get 5
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 0
                        local.get 5
                        i32.add
                        local.get 5
                        i32.store
                        local.get 5
                        i32.const 256
                        i32.ge_u
                        if  ;; label = @11
                          local.get 0
                          local.get 5
                          call 12
                          br 6 (;@5;)
                        end
                        local.get 5
                        i32.const -8
                        i32.and
                        i32.const 1051708
                        i32.add
                        local.set 1
                        block (result i32)  ;; label = @11
                          i32.const 1051972
                          i32.load
                          local.tee 4
                          i32.const 1
                          local.get 5
                          i32.const 3
                          i32.shr_u
                          i32.shl
                          local.tee 3
                          i32.and
                          i32.eqz
                          if  ;; label = @12
                            i32.const 1051972
                            local.get 3
                            local.get 4
                            i32.or
                            i32.store
                            local.get 1
                            br 1 (;@11;)
                          end
                          local.get 1
                          i32.load offset=8
                        end
                        local.set 4
                        local.get 1
                        local.get 0
                        i32.store offset=8
                        local.get 4
                        local.get 0
                        i32.store offset=12
                        local.get 0
                        local.get 1
                        i32.store offset=12
                        local.get 0
                        local.get 4
                        i32.store offset=8
                        br 5 (;@5;)
                      end
                      i32.const 1051984
                      local.get 0
                      local.get 5
                      i32.sub
                      local.tee 1
                      i32.store
                      i32.const 1051992
                      i32.const 1051992
                      i32.load
                      local.tee 0
                      local.get 5
                      i32.add
                      local.tee 2
                      i32.store
                      local.get 2
                      local.get 1
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 5
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      i32.const 8
                      i32.add
                      local.set 3
                      br 8 (;@1;)
                    end
                    i32.const 1051988
                    i32.load
                    local.set 0
                    block  ;; label = @9
                      local.get 1
                      local.get 5
                      i32.sub
                      local.tee 2
                      i32.const 15
                      i32.le_u
                      if  ;; label = @10
                        i32.const 1051988
                        i32.const 0
                        i32.store
                        i32.const 1051980
                        i32.const 0
                        i32.store
                        local.get 0
                        local.get 1
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 0
                        local.get 1
                        i32.add
                        local.tee 1
                        local.get 1
                        i32.load offset=4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        br 1 (;@9;)
                      end
                      i32.const 1051980
                      local.get 2
                      i32.store
                      i32.const 1051988
                      local.get 0
                      local.get 5
                      i32.add
                      local.tee 4
                      i32.store
                      local.get 4
                      local.get 2
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 1
                      i32.add
                      local.get 2
                      i32.store
                      local.get 0
                      local.get 5
                      i32.const 3
                      i32.or
                      i32.store offset=4
                    end
                    local.get 0
                    i32.const 8
                    i32.add
                    local.set 3
                    br 7 (;@1;)
                  end
                  local.get 0
                  local.get 3
                  local.get 7
                  i32.add
                  i32.store offset=4
                  i32.const 1051992
                  i32.const 1051992
                  i32.load
                  local.tee 0
                  i32.const 15
                  i32.add
                  i32.const -8
                  i32.and
                  local.tee 1
                  i32.const 8
                  i32.sub
                  local.tee 2
                  i32.store
                  i32.const 1051984
                  i32.const 1051984
                  i32.load
                  local.get 3
                  i32.add
                  local.tee 4
                  local.get 0
                  local.get 1
                  i32.sub
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 2
                  local.get 1
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  local.get 4
                  i32.add
                  i32.const 40
                  i32.store offset=4
                  i32.const 1052004
                  i32.const 2097152
                  i32.store
                  br 3 (;@4;)
                end
                i32.const 1051992
                local.get 0
                i32.store
                i32.const 1051984
                i32.const 1051984
                i32.load
                local.get 5
                i32.add
                local.tee 1
                i32.store
                local.get 0
                local.get 1
                i32.const 1
                i32.or
                i32.store offset=4
                br 1 (;@5;)
              end
              i32.const 1051988
              local.get 0
              i32.store
              i32.const 1051980
              i32.const 1051980
              i32.load
              local.get 5
              i32.add
              local.tee 1
              i32.store
              local.get 0
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 1
              i32.store
            end
            local.get 2
            i32.const 8
            i32.add
            local.set 3
            br 3 (;@1;)
          end
          i32.const 0
          local.set 3
          i32.const 1051984
          i32.load
          local.tee 0
          local.get 5
          i32.le_u
          br_if 2 (;@1;)
          i32.const 1051984
          local.get 0
          local.get 5
          i32.sub
          local.tee 1
          i32.store
          i32.const 1051992
          i32.const 1051992
          i32.load
          local.tee 0
          local.get 5
          i32.add
          local.tee 2
          i32.store
          local.get 2
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 5
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          i32.const 8
          i32.add
          local.set 3
          br 2 (;@1;)
        end
        local.get 0
        local.get 7
        i32.store offset=24
        local.get 1
        i32.load offset=16
        local.tee 2
        if  ;; label = @3
          local.get 0
          local.get 2
          i32.store offset=16
          local.get 2
          local.get 0
          i32.store offset=24
        end
        local.get 1
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 2
        i32.store offset=20
        local.get 2
        local.get 0
        i32.store offset=24
      end
      block  ;; label = @2
        local.get 3
        i32.const 16
        i32.ge_u
        if  ;; label = @3
          local.get 1
          local.get 5
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 1
          local.get 5
          i32.add
          local.tee 0
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          i32.add
          local.get 3
          i32.store
          local.get 3
          i32.const 256
          i32.ge_u
          if  ;; label = @4
            local.get 0
            local.get 3
            call 12
            br 2 (;@2;)
          end
          local.get 3
          i32.const -8
          i32.and
          i32.const 1051708
          i32.add
          local.set 2
          block (result i32)  ;; label = @4
            i32.const 1051972
            i32.load
            local.tee 4
            i32.const 1
            local.get 3
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 3
            i32.and
            i32.eqz
            if  ;; label = @5
              i32.const 1051972
              local.get 3
              local.get 4
              i32.or
              i32.store
              local.get 2
              br 1 (;@4;)
            end
            local.get 2
            i32.load offset=8
          end
          local.set 4
          local.get 2
          local.get 0
          i32.store offset=8
          local.get 4
          local.get 0
          i32.store offset=12
          local.get 0
          local.get 2
          i32.store offset=12
          local.get 0
          local.get 4
          i32.store offset=8
          br 1 (;@2;)
        end
        local.get 1
        local.get 3
        local.get 5
        i32.add
        local.tee 0
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 1
        i32.add
        local.tee 0
        local.get 0
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
      end
      local.get 1
      i32.const 8
      i32.add
      local.set 3
    end
    local.get 8
    i32.const 16
    i32.add
    global.set 0
    local.get 3)
  (func (;1;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 9
      local.get 0
      i32.load offset=8
      local.tee 3
      i32.or
      if  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          i32.add
          local.set 8
          block  ;; label = @4
            local.get 0
            i32.load offset=12
            local.tee 5
            i32.eqz
            if  ;; label = @5
              local.get 1
              local.set 3
              br 1 (;@4;)
            end
            local.get 1
            local.set 3
            loop  ;; label = @5
              local.get 3
              local.tee 4
              local.get 8
              i32.eq
              br_if 2 (;@3;)
              block (result i32)  ;; label = @6
                local.get 3
                i32.const 1
                i32.add
                local.get 3
                i32.load8_s
                local.tee 7
                i32.const 0
                i32.ge_s
                br_if 0 (;@6;)
                drop
                local.get 3
                i32.const 2
                i32.add
                local.get 7
                i32.const -32
                i32.lt_u
                br_if 0 (;@6;)
                drop
                local.get 3
                i32.const 3
                i32.add
                local.get 7
                i32.const -16
                i32.lt_u
                br_if 0 (;@6;)
                drop
                local.get 7
                i32.const 255
                i32.and
                i32.const 18
                i32.shl
                i32.const 1835008
                i32.and
                local.get 3
                i32.load8_u offset=3
                i32.const 63
                i32.and
                local.get 3
                i32.load8_u offset=2
                i32.const 63
                i32.and
                i32.const 6
                i32.shl
                local.get 3
                i32.load8_u offset=1
                i32.const 63
                i32.and
                i32.const 12
                i32.shl
                i32.or
                i32.or
                i32.or
                i32.const 1114112
                i32.eq
                br_if 3 (;@3;)
                local.get 3
                i32.const 4
                i32.add
              end
              local.tee 3
              local.get 6
              local.get 4
              i32.sub
              i32.add
              local.set 6
              local.get 5
              i32.const 1
              i32.sub
              local.tee 5
              br_if 0 (;@5;)
            end
          end
          local.get 3
          local.get 8
          i32.eq
          br_if 0 (;@3;)
          local.get 3
          i32.load8_s
          local.tee 4
          i32.const 0
          i32.ge_s
          local.get 4
          i32.const -32
          i32.lt_u
          i32.or
          local.get 4
          i32.const -16
          i32.lt_u
          i32.or
          i32.eqz
          if  ;; label = @4
            local.get 4
            i32.const 255
            i32.and
            i32.const 18
            i32.shl
            i32.const 1835008
            i32.and
            local.get 3
            i32.load8_u offset=3
            i32.const 63
            i32.and
            local.get 3
            i32.load8_u offset=2
            i32.const 63
            i32.and
            i32.const 6
            i32.shl
            local.get 3
            i32.load8_u offset=1
            i32.const 63
            i32.and
            i32.const 12
            i32.shl
            i32.or
            i32.or
            i32.or
            i32.const 1114112
            i32.eq
            br_if 1 (;@3;)
          end
          block  ;; label = @4
            local.get 6
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            local.get 6
            i32.le_u
            if  ;; label = @5
              local.get 2
              local.get 6
              i32.eq
              br_if 1 (;@4;)
              br 2 (;@3;)
            end
            local.get 1
            local.get 6
            i32.add
            i32.load8_s
            i32.const -64
            i32.lt_s
            br_if 1 (;@3;)
          end
          local.get 6
          local.set 2
        end
        local.get 9
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=4
        local.set 12
        block  ;; label = @3
          local.get 2
          i32.const 16
          i32.ge_u
          if  ;; label = @4
            block (result i32)  ;; label = @5
              i32.const 0
              local.set 5
              i32.const 0
              local.set 7
              i32.const 0
              local.set 6
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  local.get 1
                  i32.const 3
                  i32.add
                  i32.const -4
                  i32.and
                  local.tee 3
                  local.get 1
                  i32.sub
                  local.tee 10
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 2
                  local.get 10
                  i32.sub
                  local.tee 8
                  i32.const 4
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 8
                  i32.const 3
                  i32.and
                  local.set 9
                  i32.const 0
                  local.set 4
                  block  ;; label = @8
                    local.get 1
                    local.get 3
                    i32.eq
                    local.tee 11
                    br_if 0 (;@8;)
                    block  ;; label = @9
                      local.get 1
                      local.get 3
                      i32.sub
                      local.tee 7
                      i32.const -4
                      i32.gt_u
                      if  ;; label = @10
                        i32.const 0
                        local.set 3
                        br 1 (;@9;)
                      end
                      i32.const 0
                      local.set 3
                      loop  ;; label = @10
                        local.get 4
                        local.get 1
                        local.get 3
                        i32.add
                        local.tee 5
                        i32.load8_s
                        i32.const -65
                        i32.gt_s
                        i32.add
                        local.get 5
                        i32.const 1
                        i32.add
                        i32.load8_s
                        i32.const -65
                        i32.gt_s
                        i32.add
                        local.get 5
                        i32.const 2
                        i32.add
                        i32.load8_s
                        i32.const -65
                        i32.gt_s
                        i32.add
                        local.get 5
                        i32.const 3
                        i32.add
                        i32.load8_s
                        i32.const -65
                        i32.gt_s
                        i32.add
                        local.set 4
                        local.get 3
                        i32.const 4
                        i32.add
                        local.tee 3
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 11
                    br_if 0 (;@8;)
                    local.get 1
                    local.get 3
                    i32.add
                    local.set 5
                    loop  ;; label = @9
                      local.get 4
                      local.get 5
                      i32.load8_s
                      i32.const -65
                      i32.gt_s
                      i32.add
                      local.set 4
                      local.get 5
                      i32.const 1
                      i32.add
                      local.set 5
                      local.get 7
                      i32.const 1
                      i32.add
                      local.tee 7
                      br_if 0 (;@9;)
                    end
                  end
                  local.get 1
                  local.get 10
                  i32.add
                  local.set 3
                  block  ;; label = @8
                    local.get 9
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 3
                    local.get 8
                    i32.const -4
                    i32.and
                    i32.add
                    local.tee 5
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    local.set 6
                    local.get 9
                    i32.const 1
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 6
                    local.get 5
                    i32.load8_s offset=1
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.set 6
                    local.get 9
                    i32.const 2
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 6
                    local.get 5
                    i32.load8_s offset=2
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.set 6
                  end
                  local.get 8
                  i32.const 2
                  i32.shr_u
                  local.set 8
                  local.get 4
                  local.get 6
                  i32.add
                  local.set 7
                  loop  ;; label = @8
                    local.get 3
                    local.set 6
                    local.get 8
                    i32.eqz
                    br_if 2 (;@6;)
                    i32.const 192
                    local.get 8
                    local.get 8
                    i32.const 192
                    i32.ge_u
                    select
                    local.tee 9
                    i32.const 3
                    i32.and
                    local.set 10
                    local.get 9
                    i32.const 2
                    i32.shl
                    local.set 11
                    i32.const 0
                    local.set 5
                    local.get 8
                    i32.const 4
                    i32.ge_u
                    if  ;; label = @9
                      local.get 3
                      local.get 11
                      i32.const 1008
                      i32.and
                      i32.add
                      local.set 13
                      local.get 3
                      local.set 4
                      loop  ;; label = @10
                        local.get 4
                        i32.load
                        local.tee 3
                        i32.const -1
                        i32.xor
                        i32.const 7
                        i32.shr_u
                        local.get 3
                        i32.const 6
                        i32.shr_u
                        i32.or
                        i32.const 16843009
                        i32.and
                        local.get 5
                        i32.add
                        local.get 4
                        i32.load offset=4
                        local.tee 3
                        i32.const -1
                        i32.xor
                        i32.const 7
                        i32.shr_u
                        local.get 3
                        i32.const 6
                        i32.shr_u
                        i32.or
                        i32.const 16843009
                        i32.and
                        i32.add
                        local.get 4
                        i32.load offset=8
                        local.tee 3
                        i32.const -1
                        i32.xor
                        i32.const 7
                        i32.shr_u
                        local.get 3
                        i32.const 6
                        i32.shr_u
                        i32.or
                        i32.const 16843009
                        i32.and
                        i32.add
                        local.get 4
                        i32.load offset=12
                        local.tee 3
                        i32.const -1
                        i32.xor
                        i32.const 7
                        i32.shr_u
                        local.get 3
                        i32.const 6
                        i32.shr_u
                        i32.or
                        i32.const 16843009
                        i32.and
                        i32.add
                        local.set 5
                        local.get 4
                        i32.const 16
                        i32.add
                        local.tee 4
                        local.get 13
                        i32.ne
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 8
                    local.get 9
                    i32.sub
                    local.set 8
                    local.get 6
                    local.get 11
                    i32.add
                    local.set 3
                    local.get 5
                    i32.const 8
                    i32.shr_u
                    i32.const 16711935
                    i32.and
                    local.get 5
                    i32.const 16711935
                    i32.and
                    i32.add
                    i32.const 65537
                    i32.mul
                    i32.const 16
                    i32.shr_u
                    local.get 7
                    i32.add
                    local.set 7
                    local.get 10
                    i32.eqz
                    br_if 0 (;@8;)
                  end
                  block (result i32)  ;; label = @8
                    local.get 6
                    local.get 9
                    i32.const 252
                    i32.and
                    i32.const 2
                    i32.shl
                    i32.add
                    local.tee 3
                    i32.load
                    local.tee 4
                    i32.const -1
                    i32.xor
                    i32.const 7
                    i32.shr_u
                    local.get 4
                    i32.const 6
                    i32.shr_u
                    i32.or
                    i32.const 16843009
                    i32.and
                    local.tee 4
                    local.get 10
                    i32.const 1
                    i32.eq
                    br_if 0 (;@8;)
                    drop
                    local.get 4
                    local.get 3
                    i32.load offset=4
                    local.tee 6
                    i32.const -1
                    i32.xor
                    i32.const 7
                    i32.shr_u
                    local.get 6
                    i32.const 6
                    i32.shr_u
                    i32.or
                    i32.const 16843009
                    i32.and
                    i32.add
                    local.tee 4
                    local.get 10
                    i32.const 2
                    i32.eq
                    br_if 0 (;@8;)
                    drop
                    local.get 3
                    i32.load offset=8
                    local.tee 3
                    i32.const -1
                    i32.xor
                    i32.const 7
                    i32.shr_u
                    local.get 3
                    i32.const 6
                    i32.shr_u
                    i32.or
                    i32.const 16843009
                    i32.and
                    local.get 4
                    i32.add
                  end
                  local.tee 3
                  i32.const 8
                  i32.shr_u
                  i32.const 459007
                  i32.and
                  local.get 3
                  i32.const 16711935
                  i32.and
                  i32.add
                  i32.const 65537
                  i32.mul
                  i32.const 16
                  i32.shr_u
                  local.get 7
                  i32.add
                  br 2 (;@5;)
                end
                i32.const 0
                local.get 2
                i32.eqz
                br_if 1 (;@5;)
                drop
                local.get 2
                i32.const 3
                i32.and
                local.set 3
                local.get 2
                i32.const 4
                i32.ge_u
                if  ;; label = @7
                  local.get 2
                  i32.const -4
                  i32.and
                  local.set 6
                  loop  ;; label = @8
                    local.get 7
                    local.get 1
                    local.get 5
                    i32.add
                    local.tee 4
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 4
                    i32.const 1
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 4
                    i32.const 2
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 4
                    i32.const 3
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.set 7
                    local.get 6
                    local.get 5
                    i32.const 4
                    i32.add
                    local.tee 5
                    i32.ne
                    br_if 0 (;@8;)
                  end
                end
                local.get 3
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                local.get 5
                i32.add
                local.set 4
                loop  ;; label = @7
                  local.get 7
                  local.get 4
                  i32.load8_s
                  i32.const -65
                  i32.gt_s
                  i32.add
                  local.set 7
                  local.get 4
                  i32.const 1
                  i32.add
                  local.set 4
                  local.get 3
                  i32.const 1
                  i32.sub
                  local.tee 3
                  br_if 0 (;@7;)
                end
              end
              local.get 7
            end
            local.set 4
            br 1 (;@3;)
          end
          local.get 2
          i32.eqz
          if  ;; label = @4
            i32.const 0
            local.set 4
            br 1 (;@3;)
          end
          local.get 2
          i32.const 3
          i32.and
          local.set 5
          block  ;; label = @4
            local.get 2
            i32.const 4
            i32.lt_u
            if  ;; label = @5
              i32.const 0
              local.set 4
              i32.const 0
              local.set 6
              br 1 (;@4;)
            end
            local.get 2
            i32.const 12
            i32.and
            local.set 7
            i32.const 0
            local.set 4
            i32.const 0
            local.set 6
            loop  ;; label = @5
              local.get 4
              local.get 1
              local.get 6
              i32.add
              local.tee 3
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 3
              i32.const 1
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 3
              i32.const 2
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 3
              i32.const 3
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set 4
              local.get 7
              local.get 6
              i32.const 4
              i32.add
              local.tee 6
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 6
          i32.add
          local.set 3
          loop  ;; label = @4
            local.get 4
            local.get 3
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 4
            local.get 3
            i32.const 1
            i32.add
            local.set 3
            local.get 5
            i32.const 1
            i32.sub
            local.tee 5
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 4
          local.get 12
          i32.lt_u
          if  ;; label = @4
            local.get 12
            local.get 4
            i32.sub
            local.set 3
            i32.const 0
            local.set 4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.load8_u offset=32
                  i32.const 1
                  i32.sub
                  br_table 0 (;@7;) 1 (;@6;) 2 (;@5;)
                end
                local.get 3
                local.set 4
                i32.const 0
                local.set 3
                br 1 (;@5;)
              end
              local.get 3
              i32.const 1
              i32.shr_u
              local.set 4
              local.get 3
              i32.const 1
              i32.add
              i32.const 1
              i32.shr_u
              local.set 3
            end
            local.get 4
            i32.const 1
            i32.add
            local.set 4
            local.get 0
            i32.load offset=16
            local.set 5
            local.get 0
            i32.load offset=24
            local.set 6
            local.get 0
            i32.load offset=20
            local.set 0
            loop  ;; label = @5
              local.get 4
              i32.const 1
              i32.sub
              local.tee 4
              i32.eqz
              br_if 2 (;@3;)
              local.get 0
              local.get 5
              local.get 6
              i32.load offset=16
              call_indirect (type 0)
              i32.eqz
              br_if 0 (;@5;)
            end
            i32.const 1
            return
          end
          br 2 (;@1;)
        end
        local.get 0
        local.get 1
        local.get 2
        local.get 6
        i32.load offset=12
        call_indirect (type 1)
        if (result i32)  ;; label = @3
          i32.const 1
        else
          i32.const 0
          local.set 4
          block (result i32)  ;; label = @4
            loop  ;; label = @5
              local.get 3
              local.get 3
              local.get 4
              i32.eq
              br_if 1 (;@4;)
              drop
              local.get 4
              i32.const 1
              i32.add
              local.set 4
              local.get 0
              local.get 5
              local.get 6
              i32.load offset=16
              call_indirect (type 0)
              i32.eqz
              br_if 0 (;@5;)
            end
            local.get 4
            i32.const 1
            i32.sub
          end
          local.get 3
          i32.lt_u
        end
        return
      end
      local.get 0
      i32.load offset=20
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=24
      i32.load offset=12
      call_indirect (type 1)
      return
    end
    local.get 0
    i32.load offset=20
    local.get 1
    local.get 2
    local.get 0
    i32.load offset=24
    i32.load offset=12
    call_indirect (type 1))
  (func (;2;) (type 3) (param i32)
    (local i32 i32 i32 i32 i32)
    local.get 0
    i32.const 8
    i32.sub
    local.tee 1
    local.get 0
    i32.const 4
    i32.sub
    i32.load
    local.tee 3
    i32.const -8
    i32.and
    local.tee 0
    i32.add
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 1
            i32.and
            br_if 0 (;@4;)
            local.get 3
            i32.const 2
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 1
            i32.load
            local.tee 3
            local.get 0
            i32.add
            local.set 0
            local.get 1
            local.get 3
            i32.sub
            local.tee 1
            i32.const 1051988
            i32.load
            i32.eq
            if  ;; label = @5
              local.get 2
              i32.load offset=4
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if 1 (;@4;)
              i32.const 1051980
              local.get 0
              i32.store
              local.get 2
              local.get 2
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 2
              local.get 0
              i32.store
              return
            end
            local.get 1
            local.get 3
            call 9
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.load offset=4
              local.tee 3
              i32.const 2
              i32.and
              i32.eqz
              if  ;; label = @6
                local.get 2
                i32.const 1051992
                i32.load
                i32.eq
                br_if 2 (;@4;)
                local.get 2
                i32.const 1051988
                i32.load
                i32.eq
                br_if 5 (;@1;)
                local.get 2
                local.get 3
                i32.const -8
                i32.and
                local.tee 2
                call 9
                local.get 1
                local.get 0
                local.get 2
                i32.add
                local.tee 0
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 0
                local.get 1
                i32.add
                local.get 0
                i32.store
                local.get 1
                i32.const 1051988
                i32.load
                i32.ne
                br_if 1 (;@5;)
                i32.const 1051980
                local.get 0
                i32.store
                return
              end
              local.get 2
              local.get 3
              i32.const -2
              i32.and
              i32.store offset=4
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 0
              i32.store
            end
            local.get 0
            i32.const 256
            i32.lt_u
            br_if 2 (;@2;)
            local.get 1
            local.get 0
            call 12
            i32.const 0
            local.set 1
            i32.const 1052012
            i32.const 1052012
            i32.load
            i32.const 1
            i32.sub
            local.tee 0
            i32.store
            local.get 0
            br_if 1 (;@3;)
            i32.const 1051700
            i32.load
            local.tee 0
            if  ;; label = @5
              loop  ;; label = @6
                local.get 1
                i32.const 1
                i32.add
                local.set 1
                local.get 0
                i32.load offset=8
                local.tee 0
                br_if 0 (;@6;)
              end
            end
            i32.const 1052012
            i32.const 4095
            local.get 1
            local.get 1
            i32.const 4095
            i32.le_u
            select
            i32.store
            return
          end
          i32.const 1051992
          local.get 1
          i32.store
          i32.const 1051984
          i32.const 1051984
          i32.load
          local.get 0
          i32.add
          local.tee 0
          i32.store
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          i32.const 1051988
          i32.load
          local.get 1
          i32.eq
          if  ;; label = @4
            i32.const 1051980
            i32.const 0
            i32.store
            i32.const 1051988
            i32.const 0
            i32.store
          end
          local.get 0
          i32.const 1052004
          i32.load
          local.tee 3
          i32.le_u
          br_if 0 (;@3;)
          i32.const 1051992
          i32.load
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          block  ;; label = @4
            i32.const 1051984
            i32.load
            local.tee 4
            i32.const 41
            i32.lt_u
            br_if 0 (;@4;)
            i32.const 1051692
            local.set 0
            loop  ;; label = @5
              local.get 2
              local.get 0
              i32.load
              local.tee 5
              i32.ge_u
              if  ;; label = @6
                local.get 5
                local.get 0
                i32.load offset=4
                i32.add
                local.get 2
                i32.gt_u
                br_if 2 (;@4;)
              end
              local.get 0
              i32.load offset=8
              local.tee 0
              br_if 0 (;@5;)
            end
          end
          i32.const 1051700
          i32.load
          local.tee 0
          if  ;; label = @4
            loop  ;; label = @5
              local.get 1
              i32.const 1
              i32.add
              local.set 1
              local.get 0
              i32.load offset=8
              local.tee 0
              br_if 0 (;@5;)
            end
          end
          i32.const 1052012
          i32.const 4095
          local.get 1
          local.get 1
          i32.const 4095
          i32.le_u
          select
          i32.store
          local.get 3
          local.get 4
          i32.ge_u
          br_if 0 (;@3;)
          i32.const 1052004
          i32.const -1
          i32.store
        end
        return
      end
      local.get 0
      i32.const -8
      i32.and
      i32.const 1051708
      i32.add
      local.set 2
      block (result i32)  ;; label = @2
        i32.const 1051972
        i32.load
        local.tee 3
        i32.const 1
        local.get 0
        i32.const 3
        i32.shr_u
        i32.shl
        local.tee 0
        i32.and
        i32.eqz
        if  ;; label = @3
          i32.const 1051972
          local.get 0
          local.get 3
          i32.or
          i32.store
          local.get 2
          br 1 (;@2;)
        end
        local.get 2
        i32.load offset=8
      end
      local.set 0
      local.get 2
      local.get 1
      i32.store offset=8
      local.get 0
      local.get 1
      i32.store offset=12
      local.get 1
      local.get 2
      i32.store offset=12
      local.get 1
      local.get 0
      i32.store offset=8
      return
    end
    i32.const 1051988
    local.get 1
    i32.store
    i32.const 1051980
    i32.const 1051980
    i32.load
    local.get 0
    i32.add
    local.tee 0
    i32.store
    local.get 1
    local.get 0
    i32.const 1
    i32.or
    i32.store offset=4
    local.get 0
    local.get 1
    i32.add
    local.get 0
    i32.store)
  (func (;3;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 3
    i32.store8 offset=44
    local.get 3
    i32.const 32
    i32.store offset=28
    local.get 3
    i32.const 0
    i32.store offset=40
    local.get 3
    local.get 1
    i32.store offset=36
    local.get 3
    local.get 0
    i32.store offset=32
    local.get 3
    i32.const 0
    i32.store offset=20
    local.get 3
    i32.const 0
    i32.store offset=12
    block (result i32)  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load offset=16
            local.tee 10
            i32.eqz
            if  ;; label = @5
              local.get 2
              i32.load offset=12
              local.tee 0
              i32.eqz
              br_if 1 (;@4;)
              local.get 2
              i32.load offset=8
              local.set 1
              local.get 0
              i32.const 3
              i32.shl
              local.set 5
              local.get 0
              i32.const 1
              i32.sub
              i32.const 536870911
              i32.and
              i32.const 1
              i32.add
              local.set 7
              local.get 2
              i32.load
              local.set 0
              loop  ;; label = @6
                local.get 0
                i32.const 4
                i32.add
                i32.load
                local.tee 4
                if  ;; label = @7
                  local.get 3
                  i32.load offset=32
                  local.get 0
                  i32.load
                  local.get 4
                  local.get 3
                  i32.load offset=36
                  i32.load offset=12
                  call_indirect (type 1)
                  br_if 4 (;@3;)
                end
                local.get 1
                i32.load
                local.get 3
                i32.const 12
                i32.add
                local.get 1
                i32.load offset=4
                call_indirect (type 0)
                br_if 3 (;@3;)
                local.get 1
                i32.const 8
                i32.add
                local.set 1
                local.get 0
                i32.const 8
                i32.add
                local.set 0
                local.get 5
                i32.const 8
                i32.sub
                local.tee 5
                br_if 0 (;@6;)
              end
              br 1 (;@4;)
            end
            local.get 2
            i32.load offset=20
            local.tee 0
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.const 5
            i32.shl
            local.set 11
            local.get 0
            i32.const 1
            i32.sub
            i32.const 134217727
            i32.and
            i32.const 1
            i32.add
            local.set 7
            local.get 2
            i32.load offset=8
            local.set 8
            local.get 2
            i32.load
            local.set 0
            loop  ;; label = @5
              local.get 0
              i32.const 4
              i32.add
              i32.load
              local.tee 1
              if  ;; label = @6
                local.get 3
                i32.load offset=32
                local.get 0
                i32.load
                local.get 1
                local.get 3
                i32.load offset=36
                i32.load offset=12
                call_indirect (type 1)
                br_if 3 (;@3;)
              end
              local.get 3
              local.get 5
              local.get 10
              i32.add
              local.tee 1
              i32.const 16
              i32.add
              i32.load
              i32.store offset=28
              local.get 3
              local.get 1
              i32.const 28
              i32.add
              i32.load8_u
              i32.store8 offset=44
              local.get 3
              local.get 1
              i32.const 24
              i32.add
              i32.load
              i32.store offset=40
              local.get 1
              i32.const 12
              i32.add
              i32.load
              local.set 4
              i32.const 0
              local.set 9
              i32.const 0
              local.set 6
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 1
                    i32.const 8
                    i32.add
                    i32.load
                    i32.const 1
                    i32.sub
                    br_table 0 (;@8;) 2 (;@6;) 1 (;@7;)
                  end
                  local.get 4
                  i32.const 3
                  i32.shl
                  local.get 8
                  i32.add
                  local.tee 12
                  i32.load offset=4
                  br_if 1 (;@6;)
                  local.get 12
                  i32.load
                  local.set 4
                end
                i32.const 1
                local.set 6
              end
              local.get 3
              local.get 4
              i32.store offset=16
              local.get 3
              local.get 6
              i32.store offset=12
              local.get 1
              i32.const 4
              i32.add
              i32.load
              local.set 4
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 1
                    i32.load
                    i32.const 1
                    i32.sub
                    br_table 0 (;@8;) 2 (;@6;) 1 (;@7;)
                  end
                  local.get 4
                  i32.const 3
                  i32.shl
                  local.get 8
                  i32.add
                  local.tee 6
                  i32.load offset=4
                  br_if 1 (;@6;)
                  local.get 6
                  i32.load
                  local.set 4
                end
                i32.const 1
                local.set 9
              end
              local.get 3
              local.get 4
              i32.store offset=24
              local.get 3
              local.get 9
              i32.store offset=20
              local.get 8
              local.get 1
              i32.const 20
              i32.add
              i32.load
              i32.const 3
              i32.shl
              i32.add
              local.tee 1
              i32.load
              local.get 3
              i32.const 12
              i32.add
              local.get 1
              i32.load offset=4
              call_indirect (type 0)
              br_if 2 (;@3;)
              local.get 0
              i32.const 8
              i32.add
              local.set 0
              local.get 11
              local.get 5
              i32.const 32
              i32.add
              local.tee 5
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 7
          local.get 2
          i32.load offset=4
          i32.ge_u
          br_if 1 (;@2;)
          local.get 3
          i32.load offset=32
          local.get 2
          i32.load
          local.get 7
          i32.const 3
          i32.shl
          i32.add
          local.tee 0
          i32.load
          local.get 0
          i32.load offset=4
          local.get 3
          i32.load offset=36
          i32.load offset=12
          call_indirect (type 1)
          i32.eqz
          br_if 1 (;@2;)
        end
        i32.const 1
        br 1 (;@1;)
      end
      i32.const 0
    end
    local.get 3
    i32.const 48
    i32.add
    global.set 0)
  (func (;4;) (type 4) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 6
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load
              local.tee 4
              i32.load offset=12
              i32.eqz
              if  ;; label = @6
                local.get 4
                i32.const -1
                i32.store offset=12
                local.get 3
                local.get 3
                local.get 2
                i32.const 3
                i32.add
                i32.const -4
                i32.and
                local.get 2
                i32.sub
                local.tee 10
                i32.sub
                i32.const 7
                i32.and
                i32.const 0
                local.get 3
                local.get 10
                i32.ge_u
                select
                local.tee 1
                i32.sub
                local.set 5
                block (result i32)  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 1
                      local.get 3
                      i32.le_u
                      if  ;; label = @10
                        block  ;; label = @11
                          local.get 1
                          i32.eqz
                          br_if 0 (;@11;)
                          block (result i32)  ;; label = @12
                            local.get 2
                            local.get 3
                            i32.add
                            local.tee 1
                            local.get 2
                            local.get 5
                            i32.add
                            local.tee 7
                            i32.sub
                            local.tee 8
                            i32.const 1
                            i32.sub
                            local.get 1
                            i32.const 1
                            i32.sub
                            local.tee 9
                            i32.load8_u
                            i32.const 10
                            i32.eq
                            br_if 0 (;@12;)
                            drop
                            local.get 7
                            local.get 9
                            i32.eq
                            br_if 1 (;@11;)
                            local.get 8
                            i32.const 2
                            i32.sub
                            local.get 1
                            i32.const 2
                            i32.sub
                            local.tee 9
                            i32.load8_u
                            i32.const 10
                            i32.eq
                            br_if 0 (;@12;)
                            drop
                            local.get 7
                            local.get 9
                            i32.eq
                            br_if 1 (;@11;)
                            local.get 8
                            i32.const 3
                            i32.sub
                            local.get 1
                            i32.const 3
                            i32.sub
                            local.tee 9
                            i32.load8_u
                            i32.const 10
                            i32.eq
                            br_if 0 (;@12;)
                            drop
                            local.get 7
                            local.get 9
                            i32.eq
                            br_if 1 (;@11;)
                            local.get 8
                            i32.const 4
                            i32.sub
                            local.get 1
                            i32.const 4
                            i32.sub
                            local.tee 9
                            i32.load8_u
                            i32.const 10
                            i32.eq
                            br_if 0 (;@12;)
                            drop
                            local.get 7
                            local.get 9
                            i32.eq
                            br_if 1 (;@11;)
                            local.get 8
                            i32.const 5
                            i32.sub
                            local.get 1
                            i32.const 5
                            i32.sub
                            local.tee 9
                            i32.load8_u
                            i32.const 10
                            i32.eq
                            br_if 0 (;@12;)
                            drop
                            local.get 7
                            local.get 9
                            i32.eq
                            br_if 1 (;@11;)
                            local.get 8
                            i32.const 6
                            i32.sub
                            local.get 1
                            i32.const 6
                            i32.sub
                            local.tee 9
                            i32.load8_u
                            i32.const 10
                            i32.eq
                            br_if 0 (;@12;)
                            drop
                            local.get 7
                            local.get 9
                            i32.eq
                            br_if 1 (;@11;)
                            local.get 8
                            i32.const 7
                            i32.sub
                            local.get 1
                            i32.const 7
                            i32.sub
                            local.tee 1
                            i32.load8_u
                            i32.const 10
                            i32.eq
                            br_if 0 (;@12;)
                            drop
                            local.get 1
                            local.get 7
                            i32.eq
                            br_if 1 (;@11;)
                            local.get 8
                            i32.const 8
                            i32.sub
                          end
                          local.get 5
                          i32.add
                          local.set 1
                          br 3 (;@8;)
                        end
                        local.get 10
                        local.get 3
                        local.get 3
                        local.get 10
                        i32.gt_u
                        select
                        local.set 7
                        loop  ;; label = @11
                          local.get 7
                          local.get 5
                          local.tee 1
                          i32.lt_u
                          if  ;; label = @12
                            local.get 1
                            i32.const 8
                            i32.sub
                            local.set 5
                            local.get 1
                            local.get 2
                            i32.add
                            local.tee 8
                            i32.const 4
                            i32.sub
                            i32.load
                            i32.const 168430090
                            i32.xor
                            local.tee 10
                            i32.const 16843009
                            i32.sub
                            local.get 10
                            i32.const -1
                            i32.xor
                            i32.and
                            local.get 8
                            i32.const 8
                            i32.sub
                            i32.load
                            i32.const 168430090
                            i32.xor
                            local.tee 8
                            i32.const 16843009
                            i32.sub
                            local.get 8
                            i32.const -1
                            i32.xor
                            i32.and
                            i32.or
                            i32.const -2139062144
                            i32.and
                            i32.eqz
                            br_if 1 (;@11;)
                          end
                        end
                        local.get 1
                        local.get 3
                        i32.gt_u
                        br_if 1 (;@9;)
                        local.get 2
                        i32.const 1
                        i32.sub
                        local.set 5
                        loop  ;; label = @11
                          i32.const 0
                          local.get 1
                          i32.eqz
                          br_if 4 (;@7;)
                          drop
                          local.get 1
                          local.get 5
                          i32.add
                          local.get 1
                          i32.const 1
                          i32.sub
                          local.set 1
                          i32.load8_u
                          i32.const 10
                          i32.ne
                          br_if 0 (;@11;)
                        end
                        br 2 (;@8;)
                      end
                      global.get 0
                      i32.const 48
                      i32.sub
                      local.tee 0
                      global.set 0
                      local.get 0
                      local.get 3
                      i32.store offset=4
                      local.get 0
                      local.get 5
                      i32.store
                      local.get 0
                      i32.const 2
                      i32.store offset=12
                      local.get 0
                      i32.const 1051416
                      i32.store offset=8
                      local.get 0
                      i64.const 2
                      i64.store offset=20 align=4
                      local.get 0
                      local.get 0
                      i32.const 4
                      i32.add
                      i64.extend_i32_u
                      i64.const 21474836480
                      i64.or
                      i64.store offset=40
                      local.get 0
                      local.get 0
                      i64.extend_i32_u
                      i64.const 21474836480
                      i64.or
                      i64.store offset=32
                      local.get 0
                      local.get 0
                      i32.const 32
                      i32.add
                      i32.store offset=16
                      local.get 0
                      i32.const 8
                      i32.add
                      i32.const 1051332
                      call 29
                      unreachable
                    end
                    global.get 0
                    i32.const 48
                    i32.sub
                    local.tee 0
                    global.set 0
                    local.get 0
                    local.get 3
                    i32.store offset=4
                    local.get 0
                    local.get 1
                    i32.store
                    local.get 0
                    i32.const 2
                    i32.store offset=12
                    local.get 0
                    i32.const 1051448
                    i32.store offset=8
                    local.get 0
                    i64.const 2
                    i64.store offset=20 align=4
                    local.get 0
                    local.get 0
                    i32.const 4
                    i32.add
                    i64.extend_i32_u
                    i64.const 21474836480
                    i64.or
                    i64.store offset=40
                    local.get 0
                    local.get 0
                    i64.extend_i32_u
                    i64.const 21474836480
                    i64.or
                    i64.store offset=32
                    local.get 0
                    local.get 0
                    i32.const 32
                    i32.add
                    i32.store offset=16
                    local.get 0
                    i32.const 8
                    i32.add
                    i32.const 1051348
                    call 29
                    unreachable
                  end
                  i32.const 1
                end
                local.set 5
                local.get 6
                local.get 1
                i32.store offset=4
                local.get 6
                local.get 5
                i32.store
                local.get 6
                i32.load
                i32.eqz
                if  ;; label = @7
                  block  ;; label = @8
                    local.get 4
                    i32.const 24
                    i32.add
                    i32.load
                    local.tee 1
                    i32.eqz
                    if  ;; label = @9
                      i32.const 0
                      local.set 1
                      br 1 (;@8;)
                    end
                    local.get 1
                    local.get 4
                    i32.const 20
                    i32.add
                    i32.load
                    i32.add
                    i32.const 1
                    i32.sub
                    i32.load8_u
                    i32.const 10
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 1
                    local.get 4
                    i32.const 0
                    i32.store8 offset=28
                    local.get 4
                    i32.const 24
                    i32.add
                    i32.const 0
                    i32.store
                  end
                  local.get 3
                  local.get 4
                  i32.load offset=16
                  local.get 1
                  i32.sub
                  i32.ge_u
                  if  ;; label = @8
                    local.get 0
                    local.get 4
                    i32.const 16
                    i32.add
                    local.get 2
                    local.get 3
                    call 20
                    br 6 (;@2;)
                  end
                  local.get 4
                  i32.load offset=20
                  local.get 1
                  i32.add
                  local.get 2
                  local.get 3
                  call 55
                  drop
                  local.get 0
                  i32.const 4
                  i32.store8
                  local.get 4
                  i32.const 24
                  i32.add
                  local.get 1
                  local.get 3
                  i32.add
                  i32.store
                  br 5 (;@2;)
                end
                local.get 3
                local.get 6
                i32.load offset=4
                i32.const 1
                i32.add
                local.tee 1
                i32.lt_u
                br_if 5 (;@1;)
                local.get 4
                i32.const 24
                i32.add
                local.tee 7
                i32.load
                local.tee 5
                i32.eqz
                br_if 3 (;@3;)
                local.get 1
                local.get 4
                i32.load offset=16
                local.get 5
                i32.sub
                i32.lt_u
                if  ;; label = @7
                  local.get 4
                  i32.const 20
                  i32.add
                  i32.load
                  local.get 5
                  i32.add
                  local.get 2
                  local.get 1
                  call 55
                  drop
                  local.get 7
                  local.get 1
                  local.get 5
                  i32.add
                  local.tee 5
                  i32.store
                  br 3 (;@4;)
                end
                local.get 6
                i32.const 8
                i32.add
                local.get 4
                i32.const 16
                i32.add
                local.get 2
                local.get 1
                call 20
                local.get 6
                i32.load8_u offset=8
                i32.const 4
                i32.eq
                br_if 1 (;@5;)
                local.get 0
                local.get 6
                i64.load offset=8
                i64.store align=4
                br 4 (;@2;)
              end
              global.get 0
              i32.const 48
              i32.sub
              local.tee 0
              global.set 0
              local.get 0
              i32.const 1
              i32.store offset=12
              local.get 0
              i32.const 1050868
              i32.store offset=8
              local.get 0
              i64.const 1
              i64.store offset=20 align=4
              local.get 0
              local.get 0
              i32.const 47
              i32.add
              i64.extend_i32_u
              i64.const 124554051584
              i64.or
              i64.store offset=32
              local.get 0
              local.get 0
              i32.const 32
              i32.add
              i32.store offset=16
              local.get 0
              i32.const 8
              i32.add
              i32.const 1049768
              call 29
              unreachable
            end
            local.get 4
            i32.const 24
            i32.add
            i32.load
            local.set 5
          end
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.const 0
          i32.store8 offset=28
          local.get 4
          i32.const 24
          i32.add
          i32.const 0
          i32.store
        end
        local.get 1
        local.get 2
        i32.add
        local.set 2
        local.get 3
        local.get 1
        i32.sub
        local.tee 1
        local.get 4
        i32.load offset=16
        i32.ge_u
        if  ;; label = @3
          local.get 0
          local.get 4
          i32.const 16
          i32.add
          local.get 2
          local.get 1
          call 20
          br 1 (;@2;)
        end
        local.get 4
        i32.const 20
        i32.add
        i32.load
        local.get 2
        local.get 1
        call 55
        drop
        local.get 0
        i32.const 4
        i32.store8
        local.get 4
        i32.const 24
        i32.add
        local.get 1
        i32.store
      end
      local.get 4
      local.get 4
      i32.load offset=12
      i32.const 1
      i32.add
      i32.store offset=12
      local.get 6
      i32.const 32
      i32.add
      global.set 0
      return
    end
    local.get 6
    i32.const 0
    i32.store offset=24
    local.get 6
    i32.const 1
    i32.store offset=12
    local.get 6
    i32.const 1048928
    i32.store offset=8
    local.get 6
    i64.const 4
    i64.store offset=16 align=4
    local.get 6
    i32.const 8
    i32.add
    i32.const 1048936
    call 29
    unreachable)
  (func (;5;) (type 2) (param i32 i32)
    (local i32 i32)
    local.get 0
    local.get 1
    i32.add
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 3
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.tee 3
        local.get 1
        i32.add
        local.set 1
        local.get 0
        local.get 3
        i32.sub
        local.tee 0
        i32.const 1051988
        i32.load
        i32.eq
        if  ;; label = @3
          local.get 2
          i32.load offset=4
          i32.const 3
          i32.and
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          i32.const 1051980
          local.get 1
          i32.store
          local.get 2
          local.get 2
          i32.load offset=4
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 0
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 1
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        local.get 3
        call 9
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load offset=4
            local.tee 3
            i32.const 2
            i32.and
            i32.eqz
            if  ;; label = @5
              local.get 2
              i32.const 1051992
              i32.load
              i32.eq
              br_if 2 (;@3;)
              local.get 2
              i32.const 1051988
              i32.load
              i32.eq
              br_if 3 (;@2;)
              local.get 2
              local.get 3
              i32.const -8
              i32.and
              local.tee 2
              call 9
              local.get 0
              local.get 1
              local.get 2
              i32.add
              local.tee 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 1
              i32.store
              local.get 0
              i32.const 1051988
              i32.load
              i32.ne
              br_if 1 (;@4;)
              i32.const 1051980
              local.get 1
              i32.store
              return
            end
            local.get 2
            local.get 3
            i32.const -2
            i32.and
            i32.store offset=4
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            local.get 1
            i32.add
            local.get 1
            i32.store
          end
          local.get 1
          i32.const 256
          i32.ge_u
          if  ;; label = @4
            local.get 0
            local.get 1
            call 12
            return
          end
          local.get 1
          i32.const -8
          i32.and
          i32.const 1051708
          i32.add
          local.set 2
          block (result i32)  ;; label = @4
            i32.const 1051972
            i32.load
            local.tee 3
            i32.const 1
            local.get 1
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 1
            i32.and
            i32.eqz
            if  ;; label = @5
              i32.const 1051972
              local.get 1
              local.get 3
              i32.or
              i32.store
              local.get 2
              br 1 (;@4;)
            end
            local.get 2
            i32.load offset=8
          end
          local.set 1
          local.get 2
          local.get 0
          i32.store offset=8
          local.get 1
          local.get 0
          i32.store offset=12
          local.get 0
          local.get 2
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=8
          return
        end
        i32.const 1051992
        local.get 0
        i32.store
        i32.const 1051984
        i32.const 1051984
        i32.load
        local.get 1
        i32.add
        local.tee 1
        i32.store
        local.get 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        i32.const 1051988
        i32.load
        i32.ne
        br_if 1 (;@1;)
        i32.const 1051980
        i32.const 0
        i32.store
        i32.const 1051988
        i32.const 0
        i32.store
        return
      end
      i32.const 1051988
      local.get 0
      i32.store
      i32.const 1051980
      i32.const 1051980
      i32.load
      local.get 1
      i32.add
      local.tee 1
      i32.store
      local.get 0
      local.get 1
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 0
      local.get 1
      i32.add
      local.get 1
      i32.store
    end)
  (func (;6;) (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block  ;; label = @1
      i32.const -65587
      i32.const 16
      local.get 0
      local.get 0
      i32.const 16
      i32.le_u
      select
      local.tee 0
      i32.sub
      local.get 1
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 16
      local.get 1
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get 1
      i32.const 11
      i32.lt_u
      select
      local.tee 4
      i32.add
      i32.const 12
      i32.add
      call 0
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 8
      i32.sub
      local.set 1
      block  ;; label = @2
        local.get 0
        i32.const 1
        i32.sub
        local.tee 3
        local.get 2
        i32.and
        i32.eqz
        if  ;; label = @3
          local.get 1
          local.set 0
          br 1 (;@2;)
        end
        local.get 2
        i32.const 4
        i32.sub
        local.tee 5
        i32.load
        local.tee 6
        i32.const -8
        i32.and
        local.get 2
        local.get 3
        i32.add
        i32.const 0
        local.get 0
        i32.sub
        i32.and
        i32.const 8
        i32.sub
        local.tee 2
        local.get 0
        i32.const 0
        local.get 2
        local.get 1
        i32.sub
        i32.const 16
        i32.le_u
        select
        i32.add
        local.tee 0
        local.get 1
        i32.sub
        local.tee 2
        i32.sub
        local.set 3
        local.get 6
        i32.const 3
        i32.and
        if  ;; label = @3
          local.get 0
          local.get 3
          local.get 0
          i32.load offset=4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          i32.add
          local.tee 3
          local.get 3
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 5
          local.get 2
          local.get 5
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 1
          local.get 2
          i32.add
          local.tee 3
          local.get 3
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 2
          call 5
          br 1 (;@2;)
        end
        local.get 1
        i32.load
        local.set 1
        local.get 0
        local.get 3
        i32.store offset=4
        local.get 0
        local.get 1
        local.get 2
        i32.add
        i32.store
      end
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.const -8
        i32.and
        local.tee 2
        local.get 4
        i32.const 16
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 0
        local.get 4
        local.get 1
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 0
        local.get 4
        i32.add
        local.tee 1
        local.get 2
        local.get 4
        i32.sub
        local.tee 4
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 2
        i32.add
        local.tee 2
        local.get 2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        local.get 4
        call 5
      end
      local.get 0
      i32.const 8
      i32.add
      local.set 3
    end
    local.get 3)
  (func (;7;) (type 0) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const -64
    i32.add
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load8_u
                i32.const 1
                i32.sub
                br_table 1 (;@5;) 2 (;@4;) 3 (;@3;) 0 (;@6;)
              end
              local.get 2
              local.get 0
              i32.load offset=4
              i32.store offset=4
              i32.const 1051465
              i32.load8_u
              drop
              i32.const 20
              i32.const 1
              call 37
              local.tee 0
              i32.eqz
              br_if 4 (;@1;)
              local.get 0
              i32.const 16
              i32.add
              i32.const 1050368
              i32.load align=1
              i32.store align=1
              local.get 0
              i32.const 8
              i32.add
              i32.const 1050360
              i64.load align=1
              i64.store align=1
              local.get 0
              i32.const 1050352
              i64.load align=1
              i64.store align=1
              local.get 2
              i32.const 20
              i32.store offset=16
              local.get 2
              local.get 0
              i32.store offset=12
              local.get 2
              i32.const 20
              i32.store offset=8
              local.get 2
              i32.const 3
              i32.store offset=44
              local.get 2
              i32.const 1049716
              i32.store offset=40
              local.get 2
              i64.const 2
              i64.store offset=52 align=4
              local.get 2
              local.get 2
              i32.const 4
              i32.add
              i64.extend_i32_u
              i64.const 4294967296
              i64.or
              i64.store offset=32
              local.get 2
              local.get 2
              i32.const 8
              i32.add
              i64.extend_i32_u
              i64.const 8589934592
              i64.or
              i64.store offset=24
              local.get 2
              local.get 2
              i32.const 24
              i32.add
              i32.store offset=48
              local.get 1
              i32.load offset=20
              local.get 1
              i32.load offset=24
              local.get 2
              i32.const 40
              i32.add
              call 3
              local.set 0
              local.get 2
              i32.load offset=8
              local.tee 1
              i32.eqz
              br_if 3 (;@2;)
              local.get 2
              i32.load offset=12
              local.get 1
              i32.const 1
              call 46
              br 3 (;@2;)
            end
            local.get 0
            i32.load8_u offset=1
            local.set 0
            local.get 2
            i32.const 1
            i32.store offset=44
            local.get 2
            i32.const 1048864
            i32.store offset=40
            local.get 2
            i64.const 1
            i64.store offset=52 align=4
            local.get 2
            local.get 2
            i32.const 24
            i32.add
            i64.extend_i32_u
            i64.const 12884901888
            i64.or
            i64.store offset=8
            local.get 2
            local.get 0
            i32.const 2
            i32.shl
            local.tee 0
            i32.const 1050436
            i32.add
            i32.load
            i32.store offset=28
            local.get 2
            local.get 0
            i32.const 1050600
            i32.add
            i32.load
            i32.store offset=24
            local.get 2
            local.get 2
            i32.const 8
            i32.add
            i32.store offset=48
            local.get 1
            i32.load offset=20
            local.get 1
            i32.load offset=24
            local.get 2
            i32.const 40
            i32.add
            call 3
            local.set 0
            br 2 (;@2;)
          end
          local.get 0
          i32.load offset=4
          local.tee 0
          i32.load
          local.get 0
          i32.load offset=4
          local.get 1
          call 54
          local.set 0
          br 1 (;@2;)
        end
        local.get 0
        i32.load offset=4
        local.tee 0
        i32.load
        local.get 1
        local.get 0
        i32.load offset=4
        i32.load offset=16
        call_indirect (type 0)
        local.set 0
      end
      local.get 2
      i32.const -64
      i32.sub
      global.set 0
      local.get 0
      return
    end
    i32.const 1
    i32.const 20
    call 34
    unreachable)
  (func (;8;) (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store offset=4
    block (result i32)  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 128
          i32.ge_u
          if  ;; label = @4
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=6
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=4
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=5
            i32.const 3
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=4
          i32.const 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=5
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=4
        i32.const 2
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=7
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=6
      local.get 2
      local.get 1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=5
      local.get 2
      local.get 1
      i32.const 18
      i32.shr_u
      i32.const 7
      i32.and
      i32.const 240
      i32.or
      i32.store8 offset=4
      i32.const 4
    end
    local.set 1
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    i32.load offset=8
    local.get 2
    i32.const 4
    i32.add
    local.get 1
    call 4
    local.get 2
    i32.load8_u offset=8
    local.tee 4
    i32.const 4
    i32.ne
    if  ;; label = @1
      local.get 0
      i32.load offset=4
      local.set 1
      local.get 2
      i64.load offset=8
      local.set 7
      local.get 0
      i32.load8_u
      local.tee 3
      i32.const 4
      i32.le_u
      local.get 3
      i32.const 3
      i32.ne
      i32.and
      i32.eqz
      if  ;; label = @2
        local.get 1
        i32.load
        local.tee 5
        local.get 1
        i32.const 4
        i32.add
        i32.load
        local.tee 3
        i32.load
        call_indirect (type 3)
        local.get 3
        i32.load offset=4
        local.tee 6
        if  ;; label = @3
          local.get 5
          local.get 6
          local.get 3
          i32.load offset=8
          call 46
        end
        local.get 1
        i32.const 12
        i32.const 4
        call 46
      end
      local.get 0
      local.get 7
      i64.store align=4
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 4
    i32.const 4
    i32.ne)
  (func (;9;) (type 2) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 0
    i32.load offset=12
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 256
        i32.ge_u
        if  ;; label = @3
          local.get 0
          i32.load offset=24
          local.set 3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              local.get 2
              i32.eq
              if  ;; label = @6
                local.get 0
                i32.const 20
                i32.const 16
                local.get 0
                i32.load offset=20
                local.tee 2
                select
                i32.add
                i32.load
                local.tee 1
                br_if 1 (;@5;)
                i32.const 0
                local.set 2
                br 2 (;@4;)
              end
              local.get 0
              i32.load offset=8
              local.tee 1
              local.get 2
              i32.store offset=12
              local.get 2
              local.get 1
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 0
            i32.const 20
            i32.add
            local.get 0
            i32.const 16
            i32.add
            local.get 2
            select
            local.set 4
            loop  ;; label = @5
              local.get 4
              local.set 5
              local.get 1
              local.tee 2
              i32.const 20
              i32.add
              local.get 2
              i32.const 16
              i32.add
              local.get 2
              i32.load offset=20
              local.tee 1
              select
              local.set 4
              local.get 2
              i32.const 20
              i32.const 16
              local.get 1
              select
              i32.add
              i32.load
              local.tee 1
              br_if 0 (;@5;)
            end
            local.get 5
            i32.const 0
            i32.store
          end
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 0
          i32.load offset=28
          i32.const 2
          i32.shl
          i32.const 1051564
          i32.add
          local.tee 1
          i32.load
          i32.ne
          if  ;; label = @4
            local.get 3
            i32.const 16
            i32.const 20
            local.get 3
            i32.load offset=16
            local.get 0
            i32.eq
            select
            i32.add
            local.get 2
            i32.store
            local.get 2
            i32.eqz
            br_if 3 (;@1;)
            br 2 (;@2;)
          end
          local.get 1
          local.get 2
          i32.store
          local.get 2
          br_if 1 (;@2;)
          i32.const 1051976
          i32.const 1051976
          i32.load
          i32.const -2
          local.get 0
          i32.load offset=28
          i32.rotl
          i32.and
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        i32.load offset=8
        local.tee 0
        local.get 2
        i32.ne
        if  ;; label = @3
          local.get 0
          local.get 2
          i32.store offset=12
          local.get 2
          local.get 0
          i32.store offset=8
          return
        end
        i32.const 1051972
        i32.const 1051972
        i32.load
        i32.const -2
        local.get 1
        i32.const 3
        i32.shr_u
        i32.rotl
        i32.and
        i32.store
        return
      end
      local.get 2
      local.get 3
      i32.store offset=24
      local.get 0
      i32.load offset=16
      local.tee 1
      if  ;; label = @2
        local.get 2
        local.get 1
        i32.store offset=16
        local.get 1
        local.get 2
        i32.store offset=24
      end
      local.get 0
      i32.load offset=20
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      i32.store offset=20
      local.get 0
      local.get 2
      i32.store offset=24
    end)
  (func (;10;) (type 7) (param i64 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 8
    global.set 0
    i32.const 39
    local.set 3
    block  ;; label = @1
      local.get 0
      i64.const 10000
      i64.lt_u
      if  ;; label = @2
        local.get 0
        local.set 14
        br 1 (;@1;)
      end
      loop  ;; label = @2
        local.get 8
        i32.const 9
        i32.add
        local.get 3
        i32.add
        local.tee 4
        i32.const 4
        i32.sub
        local.get 0
        local.get 0
        i64.const 10000
        i64.div_u
        local.tee 14
        i64.const 10000
        i64.mul
        i64.sub
        i32.wrap_i64
        local.tee 5
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 6
        i32.const 1
        i32.shl
        i32.const 1051088
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 4
        i32.const 2
        i32.sub
        local.get 5
        local.get 6
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.const 1051088
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 3
        i32.const 4
        i32.sub
        local.set 3
        local.get 0
        i64.const 99999999
        i64.gt_u
        local.get 14
        local.set 0
        br_if 0 (;@2;)
      end
    end
    local.get 14
    i32.wrap_i64
    local.tee 4
    i32.const 99
    i32.gt_u
    if  ;; label = @1
      local.get 3
      i32.const 2
      i32.sub
      local.tee 3
      local.get 8
      i32.const 9
      i32.add
      i32.add
      local.get 14
      i32.wrap_i64
      local.tee 4
      local.get 4
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 4
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.const 1051088
      i32.add
      i32.load16_u align=1
      i32.store16 align=1
    end
    block  ;; label = @1
      local.get 4
      i32.const 10
      i32.ge_u
      if  ;; label = @2
        local.get 3
        i32.const 2
        i32.sub
        local.tee 3
        local.get 8
        i32.const 9
        i32.add
        i32.add
        local.get 4
        i32.const 1
        i32.shl
        i32.const 1051088
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        br 1 (;@1;)
      end
      local.get 3
      i32.const 1
      i32.sub
      local.tee 3
      local.get 8
      i32.const 9
      i32.add
      i32.add
      local.get 4
      i32.const 48
      i32.or
      i32.store8
    end
    block (result i32)  ;; label = @1
      local.get 8
      i32.const 9
      i32.add
      local.get 3
      i32.add
      local.set 10
      i32.const 39
      local.get 3
      i32.sub
      local.set 4
      i32.const 1
      block (result i32)  ;; label = @2
        local.get 1
        i32.eqz
        if  ;; label = @3
          local.get 2
          i32.load offset=28
          local.set 1
          i32.const 45
          local.set 6
          local.get 4
          i32.const 1
          i32.add
          br 1 (;@2;)
        end
        i32.const 43
        i32.const 1114112
        local.get 2
        i32.load offset=28
        local.tee 1
        i32.const 1
        i32.and
        local.tee 3
        select
        local.set 6
        local.get 3
        local.get 4
        i32.add
      end
      local.set 3
      i32.const 0
      local.get 1
      i32.const 4
      i32.and
      select
      local.set 5
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load
          i32.eqz
          if  ;; label = @4
            i32.const 1
            local.set 1
            local.get 2
            i32.load offset=20
            local.tee 3
            local.get 2
            i32.load offset=24
            local.tee 2
            local.get 6
            local.get 5
            call 27
            br_if 1 (;@3;)
            br 2 (;@2;)
          end
          local.get 3
          local.get 2
          i32.load offset=4
          local.tee 7
          i32.ge_u
          if  ;; label = @4
            i32.const 1
            local.set 1
            local.get 2
            i32.load offset=20
            local.tee 3
            local.get 2
            i32.load offset=24
            local.tee 2
            local.get 6
            local.get 5
            call 27
            br_if 1 (;@3;)
            br 2 (;@2;)
          end
          local.get 1
          i32.const 8
          i32.and
          if  ;; label = @4
            local.get 2
            i32.load offset=16
            local.set 12
            local.get 2
            i32.const 48
            i32.store offset=16
            local.get 2
            i32.load8_u offset=32
            local.set 13
            i32.const 1
            local.set 1
            local.get 2
            i32.const 1
            i32.store8 offset=32
            local.get 2
            i32.load offset=20
            local.tee 9
            local.get 2
            i32.load offset=24
            local.tee 11
            local.get 6
            local.get 5
            call 27
            br_if 1 (;@3;)
            local.get 7
            local.get 3
            i32.sub
            i32.const 1
            i32.add
            local.set 1
            block  ;; label = @5
              loop  ;; label = @6
                local.get 1
                i32.const 1
                i32.sub
                local.tee 1
                i32.eqz
                br_if 1 (;@5;)
                local.get 9
                i32.const 48
                local.get 11
                i32.load offset=16
                call_indirect (type 0)
                i32.eqz
                br_if 0 (;@6;)
              end
              i32.const 1
              br 4 (;@1;)
            end
            i32.const 1
            local.set 1
            local.get 9
            local.get 10
            local.get 4
            local.get 11
            i32.load offset=12
            call_indirect (type 1)
            br_if 1 (;@3;)
            local.get 2
            local.get 13
            i32.store8 offset=32
            local.get 2
            local.get 12
            i32.store offset=16
            i32.const 0
            local.set 1
            br 1 (;@3;)
          end
          local.get 7
          local.get 3
          i32.sub
          local.set 3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.load8_u offset=32
                local.tee 1
                i32.const 1
                i32.sub
                br_table 0 (;@6;) 1 (;@5;) 0 (;@6;) 2 (;@4;)
              end
              local.get 3
              local.set 1
              i32.const 0
              local.set 3
              br 1 (;@4;)
            end
            local.get 3
            i32.const 1
            i32.shr_u
            local.set 1
            local.get 3
            i32.const 1
            i32.add
            i32.const 1
            i32.shr_u
            local.set 3
          end
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 2
          i32.load offset=16
          local.set 9
          local.get 2
          i32.load offset=24
          local.set 7
          local.get 2
          i32.load offset=20
          local.set 2
          block  ;; label = @4
            loop  ;; label = @5
              local.get 1
              i32.const 1
              i32.sub
              local.tee 1
              i32.eqz
              br_if 1 (;@4;)
              local.get 2
              local.get 9
              local.get 7
              i32.load offset=16
              call_indirect (type 0)
              i32.eqz
              br_if 0 (;@5;)
            end
            i32.const 1
            br 3 (;@1;)
          end
          i32.const 1
          local.set 1
          local.get 2
          local.get 7
          local.get 6
          local.get 5
          call 27
          br_if 0 (;@3;)
          local.get 2
          local.get 10
          local.get 4
          local.get 7
          i32.load offset=12
          call_indirect (type 1)
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          loop  ;; label = @4
            i32.const 0
            local.get 1
            local.get 3
            i32.eq
            br_if 3 (;@1;)
            drop
            local.get 1
            i32.const 1
            i32.add
            local.set 1
            local.get 2
            local.get 9
            local.get 7
            i32.load offset=16
            call_indirect (type 0)
            i32.eqz
            br_if 0 (;@4;)
          end
          local.get 1
          i32.const 1
          i32.sub
          local.get 3
          i32.lt_u
          br 2 (;@1;)
        end
        local.get 1
        br 1 (;@1;)
      end
      local.get 3
      local.get 10
      local.get 4
      local.get 2
      i32.load offset=12
      call_indirect (type 1)
    end
    local.get 8
    i32.const 48
    i32.add
    global.set 0)
  (func (;11;) (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block (result i32)  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 128
          i32.ge_u
          if  ;; label = @4
            local.get 3
            i32.const 0
            i32.store offset=12
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.lt_u
            if  ;; label = @5
              local.get 3
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 3
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 3
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              br 3 (;@2;)
            end
            local.get 3
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 3
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 3
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 3
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 7
            i32.and
            i32.const 240
            i32.or
            i32.store8 offset=12
            i32.const 4
            br 2 (;@2;)
          end
          local.get 0
          i32.load offset=8
          local.tee 7
          local.get 0
          i32.load
          i32.eq
          if  ;; label = @4
            global.get 0
            i32.const 32
            i32.sub
            local.tee 2
            global.set 0
            local.get 0
            i32.load
            local.tee 5
            i32.const 1
            i32.add
            local.tee 4
            i32.eqz
            if  ;; label = @5
              i32.const 0
              i32.const 0
              call 34
              unreachable
            end
            i32.const 8
            local.get 5
            i32.const 1
            i32.shl
            local.tee 6
            local.get 4
            local.get 4
            local.get 6
            i32.lt_u
            select
            local.tee 4
            local.get 4
            i32.const 8
            i32.le_u
            select
            local.tee 4
            i32.const -1
            i32.xor
            i32.const 31
            i32.shr_u
            local.set 6
            local.get 2
            local.get 5
            if (result i32)  ;; label = @5
              local.get 2
              local.get 5
              i32.store offset=28
              local.get 2
              local.get 0
              i32.load offset=4
              i32.store offset=20
              i32.const 1
            else
              i32.const 0
            end
            i32.store offset=24
            local.get 2
            i32.const 8
            i32.add
            local.get 6
            local.get 4
            local.get 2
            i32.const 20
            i32.add
            call 17
            local.get 2
            i32.load offset=8
            if  ;; label = @5
              local.get 2
              i32.load offset=12
              local.get 2
              i32.load offset=16
              call 34
              unreachable
            end
            local.get 2
            i32.load offset=12
            local.set 5
            local.get 0
            local.get 4
            i32.store
            local.get 0
            local.get 5
            i32.store offset=4
            local.get 2
            i32.const 32
            i32.add
            global.set 0
          end
          local.get 0
          local.get 7
          i32.const 1
          i32.add
          i32.store offset=8
          local.get 0
          i32.load offset=4
          local.get 7
          i32.add
          local.get 1
          i32.store8
          br 2 (;@1;)
        end
        local.get 3
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 3
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
      end
      local.set 1
      local.get 1
      local.get 0
      i32.load
      local.get 0
      i32.load offset=8
      local.tee 2
      i32.sub
      i32.gt_u
      if  ;; label = @2
        local.get 0
        local.get 2
        local.get 1
        call 16
        local.get 0
        i32.load offset=8
        local.set 2
      end
      local.get 0
      i32.load offset=4
      local.get 2
      i32.add
      local.get 3
      i32.const 12
      i32.add
      local.get 1
      call 55
      drop
      local.get 0
      local.get 1
      local.get 2
      i32.add
      i32.store offset=8
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;12;) (type 2) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 0
    i64.const 0
    i64.store offset=16 align=4
    local.get 0
    block (result i32)  ;; label = @1
      i32.const 0
      local.get 1
      i32.const 256
      i32.lt_u
      br_if 0 (;@1;)
      drop
      i32.const 31
      local.get 1
      i32.const 16777215
      i32.gt_u
      br_if 0 (;@1;)
      drop
      local.get 1
      i32.const 6
      local.get 1
      i32.const 8
      i32.shr_u
      i32.clz
      local.tee 3
      i32.sub
      i32.shr_u
      i32.const 1
      i32.and
      local.get 3
      i32.const 1
      i32.shl
      i32.sub
      i32.const 62
      i32.add
    end
    local.tee 2
    i32.store offset=28
    local.get 2
    i32.const 2
    i32.shl
    i32.const 1051564
    i32.add
    local.set 4
    i32.const 1
    local.get 2
    i32.shl
    local.tee 3
    i32.const 1051976
    i32.load
    i32.and
    i32.eqz
    if  ;; label = @1
      local.get 4
      local.get 0
      i32.store
      local.get 0
      local.get 4
      i32.store offset=24
      local.get 0
      local.get 0
      i32.store offset=12
      local.get 0
      local.get 0
      i32.store offset=8
      i32.const 1051976
      i32.const 1051976
      i32.load
      local.get 3
      i32.or
      i32.store
      return
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 4
        i32.load
        local.tee 3
        i32.load offset=4
        i32.const -8
        i32.and
        i32.eq
        if  ;; label = @3
          local.get 3
          local.set 2
          br 1 (;@2;)
        end
        local.get 1
        i32.const 25
        local.get 2
        i32.const 1
        i32.shr_u
        i32.sub
        i32.const 0
        local.get 2
        i32.const 31
        i32.ne
        select
        i32.shl
        local.set 5
        loop  ;; label = @3
          local.get 3
          local.get 5
          i32.const 29
          i32.shr_u
          i32.const 4
          i32.and
          i32.add
          i32.const 16
          i32.add
          local.tee 4
          i32.load
          local.tee 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 5
          i32.const 1
          i32.shl
          local.set 5
          local.get 2
          local.set 3
          local.get 2
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 1
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.load offset=8
      local.tee 1
      local.get 0
      i32.store offset=12
      local.get 2
      local.get 0
      i32.store offset=8
      local.get 0
      i32.const 0
      i32.store offset=24
      local.get 0
      local.get 2
      i32.store offset=12
      local.get 0
      local.get 1
      i32.store offset=8
      return
    end
    local.get 4
    local.get 0
    i32.store
    local.get 0
    local.get 3
    i32.store offset=24
    local.get 0
    local.get 0
    i32.store offset=12
    local.get 0
    local.get 0
    i32.store offset=8)
  (func (;13;) (type 0) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    block (result i32)  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 128
          i32.ge_u
          if  ;; label = @4
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=12
          i32.const 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=15
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=14
      local.get 2
      local.get 1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=13
      local.get 2
      local.get 1
      i32.const 18
      i32.shr_u
      i32.const 7
      i32.and
      i32.const 240
      i32.or
      i32.store8 offset=12
      i32.const 4
    end
    local.set 1
    local.get 1
    local.get 0
    i32.load offset=8
    local.tee 3
    i32.load
    local.get 3
    i32.load offset=8
    local.tee 0
    i32.sub
    i32.gt_u
    if  ;; label = @1
      local.get 3
      local.get 0
      local.get 1
      call 16
      local.get 3
      i32.load offset=8
      local.set 0
    end
    local.get 3
    i32.load offset=4
    local.get 0
    i32.add
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call 55
    drop
    local.get 3
    local.get 0
    local.get 1
    i32.add
    i32.store offset=8
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;14;) (type 2) (param i32 i32)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.load
    i32.const -2147483648
    i32.eq
    if  ;; label = @1
      local.get 1
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 44
      i32.add
      local.tee 4
      i32.const 0
      i32.store
      local.get 2
      i64.const 4294967296
      i64.store offset=36 align=4
      local.get 2
      i32.const 36
      i32.add
      i32.const 1048648
      local.get 3
      call 3
      drop
      local.get 2
      i32.const 32
      i32.add
      local.get 4
      i32.load
      local.tee 3
      i32.store
      local.get 2
      local.get 2
      i64.load offset=36 align=4
      local.tee 5
      i64.store offset=24
      local.get 1
      i32.const 8
      i32.add
      local.get 3
      i32.store
      local.get 1
      local.get 5
      i64.store align=4
    end
    local.get 1
    i64.load align=4
    local.set 5
    local.get 1
    i64.const 4294967296
    i64.store align=4
    local.get 2
    i32.const 16
    i32.add
    local.tee 3
    local.get 1
    i32.const 8
    i32.add
    local.tee 1
    i32.load
    i32.store
    local.get 1
    i32.const 0
    i32.store
    i32.const 1051465
    i32.load8_u
    drop
    local.get 2
    local.get 5
    i64.store offset=8
    i32.const 12
    i32.const 4
    call 37
    local.tee 1
    i32.eqz
    if  ;; label = @1
      i32.const 4
      i32.const 12
      call 53
      unreachable
    end
    local.get 1
    local.get 2
    i64.load offset=8
    i64.store align=4
    local.get 1
    i32.const 8
    i32.add
    local.get 3
    i32.load
    i32.store
    local.get 0
    i32.const 1050264
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 48
    i32.add
    global.set 0)
  (func (;15;) (type 8) (param i32 i32 i32 i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 6
    global.set 0
    i32.const 1051560
    i32.const 1051560
    i32.load
    local.tee 7
    i32.const 1
    i32.add
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        local.get 7
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        i32.const 1052020
        i32.load8_u
        br_if 0 (;@2;)
        i32.const 1052020
        i32.const 1
        i32.store8
        i32.const 1052016
        i32.const 1052016
        i32.load
        i32.const 1
        i32.add
        i32.store
        local.get 6
        local.get 5
        i32.store8 offset=29
        local.get 6
        local.get 4
        i32.store8 offset=28
        local.get 6
        local.get 3
        i32.store offset=24
        local.get 6
        local.get 2
        i32.store offset=20
        local.get 6
        i32.const 1050336
        i32.store offset=16
        local.get 6
        i32.const 1
        i32.store offset=12
        i32.const 1051548
        i32.load
        local.tee 2
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        i32.const 1051548
        local.get 2
        i32.const 1
        i32.add
        i32.store
        i32.const 1051548
        i32.const 1051552
        i32.load
        if (result i32)  ;; label = @3
          local.get 6
          local.get 0
          local.get 1
          i32.load offset=16
          call_indirect (type 2)
          local.get 6
          local.get 6
          i64.load
          i64.store offset=12 align=4
          i32.const 1051552
          i32.load
          local.get 6
          i32.const 12
          i32.add
          i32.const 1051556
          i32.load
          i32.load offset=20
          call_indirect (type 2)
          i32.const 1051548
          i32.load
          i32.const 1
          i32.sub
        else
          local.get 2
        end
        i32.store
        i32.const 1052020
        i32.const 0
        i32.store8
        local.get 4
        br_if 1 (;@1;)
      end
      unreachable
    end
    unreachable)
  (func (;16;) (type 5) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 1
    local.get 1
    local.get 2
    i32.add
    local.tee 2
    i32.gt_u
    if  ;; label = @1
      i32.const 0
      i32.const 0
      call 34
      unreachable
    end
    i32.const 1
    local.set 1
    i32.const 8
    local.get 0
    i32.load
    local.tee 5
    i32.const 1
    i32.shl
    local.tee 4
    local.get 2
    local.get 2
    local.get 4
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 8
    i32.le_u
    select
    local.tee 2
    i32.const -1
    i32.xor
    i32.const 31
    i32.shr_u
    local.set 4
    block  ;; label = @1
      local.get 5
      i32.eqz
      if  ;; label = @2
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      local.get 3
      local.get 5
      i32.store offset=28
      local.get 3
      local.get 0
      i32.load offset=4
      i32.store offset=20
    end
    local.get 3
    local.get 1
    i32.store offset=24
    local.get 3
    i32.const 8
    i32.add
    local.get 4
    local.get 2
    local.get 3
    i32.const 20
    i32.add
    call 17
    local.get 3
    i32.load offset=8
    if  ;; label = @1
      local.get 3
      i32.load offset=12
      local.get 3
      i32.load offset=16
      call 34
      unreachable
    end
    local.get 3
    i32.load offset=12
    local.set 1
    local.get 0
    local.get 2
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 3
    i32.const 32
    i32.add
    global.set 0)
  (func (;17;) (type 4) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        if  ;; label = @3
          local.get 2
          i32.const 0
          i32.lt_s
          br_if 1 (;@2;)
          block (result i32)  ;; label = @4
            local.get 3
            i32.load offset=4
            if  ;; label = @5
              local.get 3
              i32.load offset=8
              local.tee 6
              if  ;; label = @6
                block (result i32)  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 3
                            i32.load
                            local.tee 10
                            i32.const 4
                            i32.sub
                            local.tee 7
                            i32.load
                            local.tee 12
                            i32.const -8
                            i32.and
                            local.tee 8
                            i32.const 4
                            i32.const 8
                            local.get 12
                            i32.const 3
                            i32.and
                            local.tee 3
                            select
                            local.get 6
                            i32.add
                            i32.ge_u
                            if  ;; label = @13
                              local.get 3
                              i32.const 0
                              local.get 6
                              i32.const 39
                              i32.add
                              local.tee 11
                              local.get 8
                              i32.lt_u
                              select
                              br_if 1 (;@12;)
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 1
                                  i32.const 9
                                  i32.ge_u
                                  if  ;; label = @16
                                    local.get 1
                                    local.get 2
                                    call 6
                                    local.tee 13
                                    br_if 1 (;@15;)
                                    i32.const 0
                                    br 9 (;@7;)
                                  end
                                  local.get 2
                                  i32.const -65588
                                  i32.gt_u
                                  br_if 1 (;@14;)
                                  i32.const 16
                                  local.get 2
                                  i32.const 11
                                  i32.add
                                  i32.const -8
                                  i32.and
                                  local.get 2
                                  i32.const 11
                                  i32.lt_u
                                  select
                                  local.set 4
                                  block  ;; label = @16
                                    local.get 3
                                    i32.eqz
                                    if  ;; label = @17
                                      local.get 4
                                      i32.const 256
                                      i32.lt_u
                                      local.get 8
                                      local.get 4
                                      i32.const 4
                                      i32.or
                                      i32.lt_u
                                      i32.or
                                      local.get 8
                                      local.get 4
                                      i32.sub
                                      i32.const 131073
                                      i32.ge_u
                                      i32.or
                                      br_if 1 (;@16;)
                                      br 9 (;@8;)
                                    end
                                    local.get 10
                                    i32.const 8
                                    i32.sub
                                    local.tee 9
                                    local.get 8
                                    i32.add
                                    local.set 5
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            local.get 4
                                            local.get 8
                                            i32.gt_u
                                            if  ;; label = @21
                                              local.get 5
                                              i32.const 1051992
                                              i32.load
                                              i32.eq
                                              br_if 4 (;@17;)
                                              local.get 5
                                              i32.const 1051988
                                              i32.load
                                              i32.eq
                                              br_if 2 (;@19;)
                                              local.get 5
                                              i32.load offset=4
                                              local.tee 3
                                              i32.const 2
                                              i32.and
                                              br_if 5 (;@16;)
                                              local.get 3
                                              i32.const -8
                                              i32.and
                                              local.tee 3
                                              local.get 8
                                              i32.add
                                              local.tee 11
                                              local.get 4
                                              i32.lt_u
                                              br_if 5 (;@16;)
                                              local.get 5
                                              local.get 3
                                              call 9
                                              local.get 11
                                              local.get 4
                                              i32.sub
                                              local.tee 5
                                              i32.const 16
                                              i32.lt_u
                                              br_if 1 (;@20;)
                                              local.get 7
                                              local.get 4
                                              local.get 7
                                              i32.load
                                              i32.const 1
                                              i32.and
                                              i32.or
                                              i32.const 2
                                              i32.or
                                              i32.store
                                              local.get 4
                                              local.get 9
                                              i32.add
                                              local.tee 6
                                              local.get 5
                                              i32.const 3
                                              i32.or
                                              i32.store offset=4
                                              local.get 9
                                              local.get 11
                                              i32.add
                                              local.tee 3
                                              local.get 3
                                              i32.load offset=4
                                              i32.const 1
                                              i32.or
                                              i32.store offset=4
                                              local.get 6
                                              local.get 5
                                              call 5
                                              br 13 (;@8;)
                                            end
                                            local.get 8
                                            local.get 4
                                            i32.sub
                                            local.tee 6
                                            i32.const 15
                                            i32.gt_u
                                            br_if 2 (;@18;)
                                            br 12 (;@8;)
                                          end
                                          local.get 7
                                          local.get 11
                                          local.get 7
                                          i32.load
                                          i32.const 1
                                          i32.and
                                          i32.or
                                          i32.const 2
                                          i32.or
                                          i32.store
                                          local.get 9
                                          local.get 11
                                          i32.add
                                          local.tee 3
                                          local.get 3
                                          i32.load offset=4
                                          i32.const 1
                                          i32.or
                                          i32.store offset=4
                                          br 11 (;@8;)
                                        end
                                        i32.const 1051980
                                        i32.load
                                        local.get 8
                                        i32.add
                                        local.tee 3
                                        local.get 4
                                        i32.lt_u
                                        br_if 2 (;@16;)
                                        block  ;; label = @19
                                          local.get 3
                                          local.get 4
                                          i32.sub
                                          local.tee 5
                                          i32.const 15
                                          i32.le_u
                                          if  ;; label = @20
                                            local.get 7
                                            local.get 12
                                            i32.const 1
                                            i32.and
                                            local.get 3
                                            i32.or
                                            i32.const 2
                                            i32.or
                                            i32.store
                                            local.get 3
                                            local.get 9
                                            i32.add
                                            local.tee 3
                                            local.get 3
                                            i32.load offset=4
                                            i32.const 1
                                            i32.or
                                            i32.store offset=4
                                            i32.const 0
                                            local.set 5
                                            i32.const 0
                                            local.set 6
                                            br 1 (;@19;)
                                          end
                                          local.get 7
                                          local.get 4
                                          local.get 12
                                          i32.const 1
                                          i32.and
                                          i32.or
                                          i32.const 2
                                          i32.or
                                          i32.store
                                          local.get 4
                                          local.get 9
                                          i32.add
                                          local.tee 6
                                          local.get 5
                                          i32.const 1
                                          i32.or
                                          i32.store offset=4
                                          local.get 3
                                          local.get 9
                                          i32.add
                                          local.tee 3
                                          local.get 5
                                          i32.store
                                          local.get 3
                                          local.get 3
                                          i32.load offset=4
                                          i32.const -2
                                          i32.and
                                          i32.store offset=4
                                        end
                                        i32.const 1051988
                                        local.get 6
                                        i32.store
                                        i32.const 1051980
                                        local.get 5
                                        i32.store
                                        br 10 (;@8;)
                                      end
                                      local.get 7
                                      local.get 4
                                      local.get 12
                                      i32.const 1
                                      i32.and
                                      i32.or
                                      i32.const 2
                                      i32.or
                                      i32.store
                                      local.get 4
                                      local.get 9
                                      i32.add
                                      local.tee 3
                                      local.get 6
                                      i32.const 3
                                      i32.or
                                      i32.store offset=4
                                      local.get 5
                                      local.get 5
                                      i32.load offset=4
                                      i32.const 1
                                      i32.or
                                      i32.store offset=4
                                      local.get 3
                                      local.get 6
                                      call 5
                                      br 9 (;@8;)
                                    end
                                    i32.const 1051984
                                    i32.load
                                    local.get 8
                                    i32.add
                                    local.tee 3
                                    local.get 4
                                    i32.gt_u
                                    br_if 7 (;@9;)
                                  end
                                  local.get 2
                                  call 0
                                  local.tee 3
                                  i32.eqz
                                  br_if 1 (;@14;)
                                  local.get 3
                                  local.get 10
                                  i32.const -4
                                  i32.const -8
                                  local.get 7
                                  i32.load
                                  local.tee 3
                                  i32.const 3
                                  i32.and
                                  select
                                  local.get 3
                                  i32.const -8
                                  i32.and
                                  i32.add
                                  local.tee 3
                                  local.get 2
                                  local.get 2
                                  local.get 3
                                  i32.gt_u
                                  select
                                  call 55
                                  local.get 10
                                  call 2
                                  br 8 (;@7;)
                                end
                                local.get 13
                                local.get 10
                                local.get 6
                                local.get 2
                                local.get 2
                                local.get 6
                                i32.gt_u
                                select
                                call 55
                                drop
                                local.get 7
                                i32.load
                                local.tee 3
                                i32.const -8
                                i32.and
                                local.tee 5
                                local.get 6
                                i32.const 4
                                i32.const 8
                                local.get 3
                                i32.const 3
                                i32.and
                                local.tee 3
                                select
                                i32.add
                                i32.lt_u
                                br_if 3 (;@11;)
                                local.get 3
                                i32.const 0
                                local.get 5
                                local.get 11
                                i32.gt_u
                                select
                                br_if 4 (;@10;)
                                local.get 10
                                call 2
                              end
                              local.get 13
                              br 6 (;@7;)
                            end
                            i32.const 1048737
                            i32.const 46
                            i32.const 1048784
                            call 26
                            unreachable
                          end
                          i32.const 1048800
                          i32.const 46
                          i32.const 1048848
                          call 26
                          unreachable
                        end
                        i32.const 1048737
                        i32.const 46
                        i32.const 1048784
                        call 26
                        unreachable
                      end
                      i32.const 1048800
                      i32.const 46
                      i32.const 1048848
                      call 26
                      unreachable
                    end
                    local.get 7
                    local.get 4
                    local.get 12
                    i32.const 1
                    i32.and
                    i32.or
                    i32.const 2
                    i32.or
                    i32.store
                    local.get 4
                    local.get 9
                    i32.add
                    local.tee 6
                    local.get 3
                    local.get 4
                    i32.sub
                    local.tee 3
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    i32.const 1051984
                    local.get 3
                    i32.store
                    i32.const 1051992
                    local.get 6
                    i32.store
                    local.get 10
                    br 1 (;@7;)
                  end
                  local.get 10
                end
                br 2 (;@4;)
              end
            end
            local.get 1
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            drop
            i32.const 1051465
            i32.load8_u
            drop
            local.get 2
            local.get 1
            call 37
          end
          local.tee 3
          if  ;; label = @4
            local.get 0
            local.get 2
            i32.store offset=8
            local.get 0
            local.get 3
            i32.store offset=4
            local.get 0
            i32.const 0
            i32.store
            return
          end
          local.get 0
          local.get 2
          i32.store offset=8
          local.get 0
          local.get 1
          i32.store offset=4
          br 2 (;@1;)
        end
        local.get 0
        i32.const 0
        i32.store offset=4
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.store offset=4
    end
    local.get 0
    i32.const 1
    i32.store)
  (func (;18;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 8
    i32.add
    local.get 0
    i32.load offset=8
    local.get 1
    local.get 2
    call 4
    local.get 3
    i32.load8_u offset=8
    local.tee 4
    i32.const 4
    i32.ne
    if  ;; label = @1
      local.get 0
      i32.load offset=4
      local.set 1
      local.get 3
      i64.load offset=8
      local.set 7
      local.get 0
      i32.load8_u
      local.tee 2
      i32.const 4
      i32.le_u
      local.get 2
      i32.const 3
      i32.ne
      i32.and
      i32.eqz
      if  ;; label = @2
        local.get 1
        i32.load
        local.tee 5
        local.get 1
        i32.const 4
        i32.add
        i32.load
        local.tee 2
        i32.load
        call_indirect (type 3)
        local.get 2
        i32.load offset=4
        local.tee 6
        if  ;; label = @3
          local.get 5
          local.get 6
          local.get 2
          i32.load offset=8
          call 46
        end
        local.get 1
        i32.const 12
        i32.const 4
        call 46
      end
      local.get 0
      local.get 7
      i64.store align=4
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0
    local.get 4
    i32.const 4
    i32.ne)
  (func (;19;) (type 2) (param i32 i32)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.load
    i32.const -2147483648
    i32.eq
    if  ;; label = @1
      local.get 1
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 28
      i32.add
      local.tee 4
      i32.const 0
      i32.store
      local.get 2
      i64.const 4294967296
      i64.store offset=20 align=4
      local.get 2
      i32.const 20
      i32.add
      i32.const 1048648
      local.get 3
      call 3
      drop
      local.get 2
      i32.const 16
      i32.add
      local.get 4
      i32.load
      local.tee 3
      i32.store
      local.get 2
      local.get 2
      i64.load offset=20 align=4
      local.tee 5
      i64.store offset=8
      local.get 1
      i32.const 8
      i32.add
      local.get 3
      i32.store
      local.get 1
      local.get 5
      i64.store align=4
    end
    local.get 0
    i32.const 1050264
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set 0)
  (func (;20;) (type 4) (param i32 i32 i32 i32)
    (local i32 i32)
    local.get 1
    i32.load offset=8
    local.tee 4
    i32.eqz
    local.get 1
    i32.load
    local.tee 5
    local.get 4
    i32.sub
    local.get 3
    i32.ge_u
    i32.or
    i32.eqz
    if  ;; label = @1
      local.get 1
      i32.const 0
      i32.store offset=8
      local.get 1
      i32.const 0
      i32.store8 offset=12
      i32.const 0
      local.set 4
    end
    local.get 3
    local.get 5
    i32.lt_u
    if  ;; label = @1
      local.get 1
      i32.load offset=4
      local.get 4
      i32.add
      local.get 2
      local.get 3
      call 55
      drop
      local.get 0
      i32.const 4
      i32.store8
      local.get 1
      local.get 3
      local.get 4
      i32.add
      i32.store offset=8
      return
    end
    local.get 0
    i64.const 4
    i64.store align=4
    local.get 1
    i32.const 0
    i32.store8 offset=12)
  (func (;21;) (type 2) (param i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 0
    global.set 0
    i32.const 1051464
    i32.load8_u
    if  ;; label = @1
      local.get 0
      i32.const 2
      i32.store offset=12
      local.get 0
      i32.const 1050164
      i32.store offset=8
      local.get 0
      i64.const 1
      i64.store offset=20 align=4
      local.get 0
      local.get 1
      i32.store offset=44
      local.get 0
      local.get 0
      i32.const 44
      i32.add
      i64.extend_i32_u
      i64.const 21474836480
      i64.or
      i64.store offset=32
      local.get 0
      local.get 0
      i32.const 32
      i32.add
      i32.store offset=16
      local.get 0
      i32.const 8
      i32.add
      i32.const 1050204
      call 29
      unreachable
    end
    local.get 0
    i32.const 48
    i32.add
    global.set 0)
  (func (;22;) (type 3) (param i32)
    (local i32 i32 i32)
    local.get 0
    i32.load offset=4
    local.set 1
    local.get 0
    i32.load8_u
    local.tee 0
    i32.const 4
    i32.le_u
    local.get 0
    i32.const 3
    i32.ne
    i32.and
    i32.eqz
    if  ;; label = @1
      local.get 1
      i32.load
      local.tee 2
      local.get 1
      i32.const 4
      i32.add
      i32.load
      local.tee 0
      i32.load
      call_indirect (type 3)
      local.get 0
      i32.load offset=4
      local.tee 3
      if  ;; label = @2
        local.get 2
        local.get 3
        local.get 0
        i32.load offset=8
        call 46
      end
      local.get 1
      i32.const 12
      i32.const 4
      call 46
    end)
  (func (;23;) (type 1) (param i32 i32 i32) (result i32)
    (local i32)
    local.get 2
    local.get 0
    i32.load offset=8
    local.tee 0
    i32.load
    local.get 0
    i32.load offset=8
    local.tee 3
    i32.sub
    i32.gt_u
    if  ;; label = @1
      local.get 0
      local.get 3
      local.get 2
      call 16
      local.get 0
      i32.load offset=8
      local.set 3
    end
    local.get 0
    i32.load offset=4
    local.get 3
    i32.add
    local.get 1
    local.get 2
    call 55
    drop
    local.get 0
    local.get 2
    local.get 3
    i32.add
    i32.store offset=8
    i32.const 0)
  (func (;24;) (type 1) (param i32 i32 i32) (result i32)
    (local i32)
    local.get 2
    local.get 0
    i32.load
    local.get 0
    i32.load offset=8
    local.tee 3
    i32.sub
    i32.gt_u
    if  ;; label = @1
      local.get 0
      local.get 3
      local.get 2
      call 16
      local.get 0
      i32.load offset=8
      local.set 3
    end
    local.get 0
    i32.load offset=4
    local.get 3
    i32.add
    local.get 1
    local.get 2
    call 55
    drop
    local.get 0
    local.get 2
    local.get 3
    i32.add
    i32.store offset=8
    i32.const 0)
  (func (;25;) (type 2) (param i32 i32)
    (local i32 i32)
    i32.const 1051465
    i32.load8_u
    drop
    local.get 1
    i32.load offset=4
    local.set 2
    local.get 1
    i32.load
    local.set 3
    i32.const 8
    i32.const 4
    call 37
    local.tee 1
    i32.eqz
    if  ;; label = @1
      i32.const 4
      i32.const 8
      call 53
      unreachable
    end
    local.get 1
    local.get 2
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 0
    i32.const 1050280
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func (;26;) (type 5) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 0
    i32.store offset=16
    local.get 3
    i32.const 1
    i32.store offset=4
    local.get 3
    i64.const 4
    i64.store offset=8 align=4
    local.get 3
    local.get 1
    i32.store offset=28
    local.get 3
    local.get 0
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 24
    i32.add
    i32.store
    local.get 3
    local.get 2
    call 29
    unreachable)
  (func (;27;) (type 9) (param i32 i32 i32 i32) (result i32)
    block  ;; label = @1
      block (result i32)  ;; label = @2
        local.get 2
        i32.const 1114112
        i32.ne
        if  ;; label = @3
          i32.const 1
          local.get 0
          local.get 2
          local.get 1
          i32.load offset=16
          call_indirect (type 0)
          br_if 1 (;@2;)
          drop
        end
        local.get 3
        br_if 1 (;@1;)
        i32.const 0
      end
      return
    end
    local.get 0
    local.get 3
    i32.const 0
    local.get 1
    i32.load offset=12
    call_indirect (type 1))
  (func (;28;) (type 10)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    i32.const 0
    i32.store offset=24
    local.get 5
    i32.const 1
    i32.store offset=12
    local.get 5
    i32.const 1048596
    i32.store offset=8
    local.get 5
    i64.const 4
    i64.store offset=16 align=4
    local.get 5
    i32.const 8
    i32.add
    local.set 7
    global.get 0
    i32.const 80
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 6
    i32.store offset=12
    local.get 3
    i32.const 1049840
    i32.store offset=8
    block  ;; label = @1
      block  ;; label = @2
        block (result i32)  ;; label = @3
          global.get 0
          i32.const 32
          i32.sub
          local.tee 0
          global.set 0
          block  ;; label = @4
            block  ;; label = @5
              i32.const 1051504
              i32.load8_u
              i32.eqz
              br_if 0 (;@5;)
              i32.const 1052024
              i32.load8_u
              i32.eqz
              if  ;; label = @6
                i32.const 1052024
                i32.const 1
                i32.store8
                i32.const 1052028
                i32.const 0
                i32.store
                br 1 (;@5;)
              end
              i32.const 1052028
              i32.load
              local.set 2
              i32.const 1052028
              i32.const 0
              i32.store
              local.get 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              i32.load8_u offset=8
              local.set 1
              local.get 2
              i32.const 1
              i32.store8 offset=8
              local.get 0
              local.get 1
              i32.store8 offset=7
              local.get 1
              br_if 1 (;@4;)
              local.get 0
              i32.const 4
              i32.store8 offset=8
              local.get 0
              local.get 2
              i32.const 12
              i32.add
              i32.store offset=16
              local.get 0
              i32.const 8
              i32.add
              i32.const 1048624
              local.get 7
              call 3
              local.set 4
              local.get 0
              i32.load8_u offset=8
              local.set 1
              block  ;; label = @6
                local.get 4
                if  ;; label = @7
                  local.get 0
                  i32.load offset=12
                  local.set 4
                  i32.const 2
                  local.get 0
                  i32.load8_u offset=8
                  local.get 1
                  i32.const 4
                  i32.eq
                  local.tee 1
                  select
                  i32.const 255
                  i32.and
                  local.tee 6
                  i32.const 4
                  i32.le_u
                  local.get 6
                  i32.const 3
                  i32.ne
                  i32.and
                  br_if 1 (;@6;)
                  i32.const 1049864
                  local.get 4
                  local.get 1
                  select
                  local.tee 1
                  i32.load
                  local.tee 6
                  local.get 1
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 4
                  i32.load
                  call_indirect (type 3)
                  local.get 4
                  i32.load offset=4
                  local.tee 8
                  if  ;; label = @8
                    local.get 6
                    local.get 8
                    local.get 4
                    i32.load offset=8
                    call 46
                  end
                  local.get 1
                  i32.const 12
                  i32.const 4
                  call 46
                  br 1 (;@6;)
                end
                local.get 0
                i32.load offset=12
                local.set 4
                local.get 1
                i32.const 3
                i32.ne
                local.get 1
                i32.const 4
                i32.le_u
                i32.and
                br_if 0 (;@6;)
                local.get 4
                i32.load
                local.tee 6
                local.get 4
                i32.const 4
                i32.add
                i32.load
                local.tee 1
                i32.load
                call_indirect (type 3)
                local.get 1
                i32.load offset=4
                local.tee 8
                if  ;; label = @7
                  local.get 6
                  local.get 8
                  local.get 1
                  i32.load offset=8
                  call 46
                end
                local.get 4
                i32.const 12
                i32.const 4
                call 46
              end
              local.get 2
              i32.const 0
              i32.store8 offset=8
              i32.const 1052028
              i32.load
              local.set 1
              i32.const 1052028
              local.get 2
              i32.store
              local.get 0
              local.get 1
              i32.store offset=8
              block  ;; label = @6
                local.get 1
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                local.get 1
                i32.load
                local.tee 2
                i32.const 1
                i32.sub
                i32.store
                local.get 2
                i32.const 1
                i32.ne
                br_if 0 (;@6;)
                local.get 0
                i32.const 8
                i32.add
                i32.load
                local.tee 2
                i32.const 12
                i32.add
                i32.load
                local.tee 1
                if  ;; label = @7
                  local.get 2
                  i32.const 16
                  i32.add
                  i32.load
                  local.get 1
                  i32.const 1
                  call 46
                end
                block  ;; label = @7
                  local.get 2
                  i32.const -1
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 2
                  local.get 2
                  i32.load offset=4
                  local.tee 1
                  i32.const 1
                  i32.sub
                  i32.store offset=4
                  local.get 1
                  i32.const 1
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 24
                  i32.const 4
                  call 46
                end
              end
              i32.const 1
              local.set 1
            end
            local.get 0
            i32.const 32
            i32.add
            global.set 0
            local.get 1
            br 1 (;@3;)
          end
          local.get 0
          i64.const 0
          i64.store offset=20 align=4
          local.get 0
          i64.const 17179869185
          i64.store offset=12 align=4
          local.get 0
          i32.const 1049908
          i32.store offset=8
          local.get 0
          i32.const 7
          i32.add
          local.get 0
          i32.const 8
          i32.add
          call 30
          unreachable
        end
        i32.eqz
        if  ;; label = @3
          i32.const 1051540
          i32.load8_u
          i32.const 3
          i32.ne
          local.tee 0
          if  ;; label = @4
            global.get 0
            i32.const 16
            i32.sub
            local.tee 1
            global.set 0
            local.get 0
            if  ;; label = @5
              local.get 1
              i32.const 1051508
              i32.store offset=4
              local.get 1
              local.get 1
              i32.const 15
              i32.add
              i32.store offset=8
              local.get 1
              i32.const 4
              i32.add
              local.set 4
              global.get 0
              i32.const 32
              i32.sub
              local.tee 2
              global.set 0
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        i32.const 1051540
                        i32.load8_u
                        local.tee 0
                        i32.const 2
                        i32.ge_u
                        if  ;; label = @11
                          local.get 0
                          i32.const 3
                          i32.sub
                          br_if 4 (;@7;)
                          br 1 (;@10;)
                        end
                        i32.const 1051540
                        i32.const 2
                        i32.store8
                        local.get 4
                        i32.load
                        local.set 0
                        local.get 4
                        i32.const 0
                        i32.store
                        local.get 0
                        i32.eqz
                        br_if 1 (;@9;)
                        i32.const 1051465
                        i32.load8_u
                        drop
                        i32.const 1024
                        i32.const 1
                        call 37
                        local.tee 4
                        i32.eqz
                        br_if 2 (;@8;)
                        local.get 0
                        i64.const 0
                        i64.store align=4
                        local.get 0
                        i32.const 0
                        i32.store8 offset=28
                        local.get 0
                        i32.const 0
                        i32.store offset=24
                        local.get 0
                        local.get 4
                        i32.store offset=20
                        local.get 0
                        i64.const 4398046511104
                        i64.store offset=12 align=4
                        local.get 0
                        i32.const 8
                        i32.add
                        i32.const 0
                        i32.store8
                        i32.const 1051540
                        i32.const 3
                        i32.store8
                      end
                      local.get 2
                      i32.const 32
                      i32.add
                      global.set 0
                      br 3 (;@6;)
                    end
                    i32.const 1050020
                    call 48
                    unreachable
                  end
                  i32.const 1
                  i32.const 1024
                  call 34
                  unreachable
                end
                local.get 2
                i32.const 0
                i32.store offset=24
                local.get 2
                i32.const 1
                i32.store offset=12
                local.get 2
                i32.const 1050428
                i32.store offset=8
                local.get 2
                i64.const 4
                i64.store offset=16 align=4
                local.get 2
                i32.const 8
                i32.add
                i32.const 1050004
                call 29
                unreachable
              end
            end
            local.get 1
            i32.const 16
            i32.add
            global.set 0
          end
          local.get 3
          i32.const 1051508
          i32.store offset=28
          local.get 3
          local.get 3
          i32.const 28
          i32.add
          i32.store offset=40
          local.get 3
          i32.const 16
          i32.add
          local.set 1
          global.get 0
          i32.const 32
          i32.sub
          local.tee 0
          global.set 0
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              i32.const 40
              i32.add
              i32.load
              i32.load
              local.tee 2
              i32.load
              i32.const 1052021
              i32.ne
              if  ;; label = @6
                local.get 2
                i32.load8_u offset=8
                local.set 4
                local.get 2
                i32.const 1
                i32.store8 offset=8
                local.get 0
                local.get 4
                i32.store8 offset=4
                local.get 4
                i32.eqz
                br_if 1 (;@5;)
                local.get 0
                i64.const 0
                i64.store offset=20 align=4
                local.get 0
                i64.const 17179869185
                i64.store offset=12 align=4
                local.get 0
                i32.const 1049908
                i32.store offset=8
                local.get 0
                i32.const 4
                i32.add
                local.get 0
                i32.const 8
                i32.add
                call 30
                unreachable
              end
              local.get 2
              i32.load offset=4
              i32.const 1
              i32.add
              local.tee 4
              if  ;; label = @6
                local.get 2
                local.get 4
                i32.store offset=4
                br 2 (;@4;)
              end
              global.get 0
              i32.const 16
              i32.sub
              local.tee 0
              global.set 0
              local.get 0
              i32.const 38
              i32.store offset=12
              local.get 0
              i32.const 1050036
              i32.store offset=8
              global.get 0
              i32.const 32
              i32.sub
              local.tee 3
              global.set 0
              local.get 3
              i32.const 1
              i32.store offset=4
              local.get 3
              i32.const 1050876
              i32.store
              local.get 3
              i64.const 1
              i64.store offset=12 align=4
              local.get 3
              local.get 0
              i32.const 8
              i32.add
              i64.extend_i32_u
              i64.const 128849018880
              i64.or
              i64.store offset=24
              local.get 3
              local.get 3
              i32.const 24
              i32.add
              i32.store offset=8
              local.get 3
              i32.const 1050112
              call 29
              unreachable
            end
            local.get 2
            i32.const 1
            i32.store offset=4
            local.get 2
            i32.const 1052021
            i32.store
          end
          local.get 0
          local.get 2
          i32.store offset=4
          local.get 0
          i32.const 4
          i32.store8 offset=8
          local.get 0
          local.get 0
          i32.const 4
          i32.add
          i32.store offset=16
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.const 8
              i32.add
              i32.const 1048672
              local.get 7
              call 3
              if  ;; label = @6
                local.get 0
                i32.load8_u offset=8
                i32.const 4
                i32.ne
                br_if 1 (;@5;)
                local.get 1
                i64.const 4509131545247746
                i64.store align=4
                br 2 (;@4;)
              end
              local.get 1
              i32.const 4
              i32.store8
              local.get 0
              i32.load offset=12
              local.set 2
              local.get 0
              i32.load8_u offset=8
              local.tee 1
              i32.const 4
              i32.le_u
              local.get 1
              i32.const 3
              i32.ne
              i32.and
              br_if 1 (;@4;)
              local.get 2
              i32.load
              local.tee 4
              local.get 2
              i32.const 4
              i32.add
              i32.load
              local.tee 1
              i32.load
              call_indirect (type 3)
              local.get 1
              i32.load offset=4
              local.tee 7
              if  ;; label = @6
                local.get 4
                local.get 7
                local.get 1
                i32.load offset=8
                call 46
              end
              local.get 2
              i32.const 12
              i32.const 4
              call 46
              br 1 (;@4;)
            end
            local.get 1
            local.get 0
            i64.load offset=8
            i64.store align=4
          end
          local.get 0
          i32.load offset=4
          local.tee 2
          local.get 2
          i32.load offset=4
          i32.const 1
          i32.sub
          local.tee 1
          i32.store offset=4
          local.get 1
          i32.eqz
          if  ;; label = @4
            local.get 2
            i32.const 0
            i32.store8 offset=8
            local.get 2
            i32.const 0
            i32.store
          end
          local.get 0
          i32.const 32
          i32.add
          global.set 0
          local.get 3
          i32.load8_u offset=16
          i32.const 4
          i32.ne
          br_if 1 (;@2;)
        end
        local.get 3
        i32.const 80
        i32.add
        global.set 0
        br 1 (;@1;)
      end
      local.get 3
      local.get 3
      i64.load offset=16
      i64.store offset=32
      local.get 3
      i32.const 2
      i32.store offset=44
      local.get 3
      i32.const 1049808
      i32.store offset=40
      local.get 3
      i64.const 2
      i64.store offset=52 align=4
      local.get 3
      local.get 3
      i32.const 32
      i32.add
      i64.extend_i32_u
      i64.const 17179869184
      i64.or
      i64.store offset=72
      local.get 3
      local.get 3
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.const 12884901888
      i64.or
      i64.store offset=64
      local.get 3
      local.get 3
      i32.const -64
      i32.sub
      i32.store offset=48
      local.get 3
      i32.const 40
      i32.add
      i32.const 1049824
      call 29
      unreachable
    end
    local.get 5
    i32.const 32
    i32.add
    global.set 0)
  (func (;29;) (type 2) (param i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 1
    i32.store16 offset=28
    local.get 2
    local.get 1
    i32.store offset=24
    local.get 2
    local.get 0
    i32.store offset=20
    local.get 2
    i32.const 1050928
    i32.store offset=16
    local.get 2
    i32.const 1
    i32.store offset=12
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 2
    i32.const 12
    i32.add
    local.tee 0
    i32.load offset=8
    local.tee 2
    i32.eqz
    if  ;; label = @1
      i32.const 1050248
      call 48
      unreachable
    end
    local.get 1
    local.get 0
    i32.load offset=12
    i32.store offset=12
    local.get 1
    local.get 0
    i32.store offset=8
    local.get 1
    local.get 2
    i32.store offset=4
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    local.get 1
    i32.const 4
    i32.add
    local.tee 1
    i32.load
    local.tee 2
    i32.load offset=12
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load offset=4
            br_table 0 (;@4;) 1 (;@3;) 2 (;@2;)
          end
          local.get 3
          br_if 1 (;@2;)
          i32.const 1
          local.set 2
          i32.const 0
          local.set 3
          br 2 (;@1;)
        end
        local.get 3
        br_if 0 (;@2;)
        local.get 2
        i32.load
        local.tee 2
        i32.load offset=4
        local.set 3
        local.get 2
        i32.load
        local.set 2
        br 1 (;@1;)
      end
      local.get 0
      local.get 2
      i32.store offset=12
      local.get 0
      i32.const -2147483648
      i32.store
      local.get 0
      i32.const 1050316
      local.get 1
      i32.load offset=4
      local.tee 0
      i32.load offset=8
      local.get 1
      i32.load offset=8
      local.get 0
      i32.load8_u offset=16
      local.get 0
      i32.load8_u offset=17
      call 15
      unreachable
    end
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store
    local.get 0
    i32.const 1050296
    local.get 1
    i32.load offset=4
    local.tee 0
    i32.load offset=8
    local.get 1
    i32.load offset=8
    local.get 0
    i32.load8_u offset=16
    local.get 0
    i32.load8_u offset=17
    call 15
    unreachable)
  (func (;30;) (type 2) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 1048604
    i32.store offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    global.get 0
    i32.const 112
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 1048608
    i32.store offset=12
    local.get 0
    local.get 2
    i32.const 8
    i32.add
    i32.store offset=8
    local.get 0
    i32.const 1048608
    i32.store offset=20
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    i32.store offset=16
    local.get 0
    i32.const 1050944
    i32.store offset=24
    local.get 0
    i32.const 2
    i32.store offset=28
    block  ;; label = @1
      local.get 1
      i32.load
      i32.eqz
      if  ;; label = @2
        local.get 0
        i32.const 3
        i32.store offset=92
        local.get 0
        i32.const 1051004
        i32.store offset=88
        local.get 0
        i64.const 3
        i64.store offset=100 align=4
        local.get 0
        local.get 0
        i32.const 16
        i32.add
        i64.extend_i32_u
        i64.const 133143986176
        i64.or
        i64.store offset=72
        local.get 0
        local.get 0
        i32.const 8
        i32.add
        i64.extend_i32_u
        i64.const 133143986176
        i64.or
        i64.store offset=64
        br 1 (;@1;)
      end
      local.get 0
      i32.const 48
      i32.add
      local.get 1
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 0
      i32.const 40
      i32.add
      local.get 1
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 0
      local.get 1
      i64.load align=4
      i64.store offset=32
      local.get 0
      i32.const 4
      i32.store offset=92
      local.get 0
      i32.const 1051056
      i32.store offset=88
      local.get 0
      i64.const 4
      i64.store offset=100 align=4
      local.get 0
      local.get 0
      i32.const 16
      i32.add
      i64.extend_i32_u
      i64.const 133143986176
      i64.or
      i64.store offset=80
      local.get 0
      local.get 0
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.const 133143986176
      i64.or
      i64.store offset=72
      local.get 0
      local.get 0
      i32.const 32
      i32.add
      i64.extend_i32_u
      i64.const 137438953472
      i64.or
      i64.store offset=64
    end
    local.get 0
    local.get 0
    i32.const 24
    i32.add
    i64.extend_i32_u
    i64.const 128849018880
    i64.or
    i64.store offset=56
    local.get 0
    local.get 0
    i32.const 56
    i32.add
    i32.store offset=96
    local.get 0
    i32.const 88
    i32.add
    i32.const 1049960
    call 29
    unreachable)
  (func (;31;) (type 0) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.load
    local.tee 0
    local.get 0
    i32.const 31
    i32.shr_s
    local.tee 2
    i32.xor
    local.get 2
    i32.sub
    i64.extend_i32_u
    local.get 0
    i32.const -1
    i32.xor
    i32.const 31
    i32.shr_u
    local.get 1
    call 10)
  (func (;32;) (type 3) (param i32)
    (local i32)
    local.get 0
    i32.load
    local.tee 1
    i32.const -2147483648
    i32.or
    i32.const -2147483648
    i32.ne
    if  ;; label = @1
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call 46
    end)
  (func (;33;) (type 3) (param i32)
    (local i32)
    local.get 0
    i32.load
    local.tee 1
    if  ;; label = @1
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call 46
    end)
  (func (;34;) (type 2) (param i32 i32)
    local.get 0
    i32.eqz
    if  ;; label = @1
      global.get 0
      i32.const 32
      i32.sub
      local.tee 0
      global.set 0
      local.get 0
      i32.const 0
      i32.store offset=24
      local.get 0
      i32.const 1
      i32.store offset=12
      local.get 0
      i32.const 1050784
      i32.store offset=8
      local.get 0
      i64.const 4
      i64.store offset=16 align=4
      local.get 0
      i32.const 8
      i32.add
      i32.const 1050820
      call 29
      unreachable
    end
    local.get 0
    local.get 1
    call 53
    unreachable)
  (func (;35;) (type 0) (param i32 i32) (result i32)
    local.get 1
    i32.load offset=20
    i32.const 1050836
    i32.const 14
    local.get 1
    i32.load offset=24
    i32.load offset=12
    call_indirect (type 1))
  (func (;36;) (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 0))
  (func (;37;) (type 0) (param i32 i32) (result i32)
    block (result i32)  ;; label = @1
      local.get 1
      i32.const 9
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 0
        call 6
        br 1 (;@1;)
      end
      local.get 0
      call 0
    end)
  (func (;38;) (type 2) (param i32 i32)
    local.get 0
    i64.const -2989668174502565848
    i64.store offset=8
    local.get 0
    i64.const -8255713724082750831
    i64.store)
  (func (;39;) (type 2) (param i32 i32)
    local.get 0
    i64.const 9172487606043731407
    i64.store offset=8
    local.get 0
    i64.const -8877450274954529964
    i64.store)
  (func (;40;) (type 2) (param i32 i32)
    local.get 0
    i64.const 7199936582794304877
    i64.store offset=8
    local.get 0
    i64.const -5076933981314334344
    i64.store)
  (func (;41;) (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call 54)
  (func (;42;) (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.get 1
    call 54)
  (func (;43;) (type 2) (param i32 i32)
    local.get 0
    i32.const 1050280
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func (;44;) (type 0) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call 1)
  (func (;45;) (type 0) (param i32 i32) (result i32)
    local.get 1
    i32.load offset=20
    local.get 1
    i32.load offset=24
    local.get 0
    call 3)
  (func (;46;) (type 5) (param i32 i32 i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 4
        i32.sub
        i32.load
        local.tee 2
        i32.const -8
        i32.and
        local.tee 3
        i32.const 4
        i32.const 8
        local.get 2
        i32.const 3
        i32.and
        local.tee 2
        select
        local.get 1
        i32.add
        i32.ge_u
        if  ;; label = @3
          local.get 2
          i32.const 0
          local.get 3
          local.get 1
          i32.const 39
          i32.add
          i32.gt_u
          select
          br_if 1 (;@2;)
          local.get 0
          call 2
          br 2 (;@1;)
        end
        i32.const 1048737
        i32.const 46
        i32.const 1048784
        call 26
        unreachable
      end
      i32.const 1048800
      i32.const 46
      i32.const 1048848
      call 26
      unreachable
    end)
  (func (;47;) (type 0) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    i32.const 1
    local.get 1
    call 10)
  (func (;48;) (type 3) (param i32)
    i32.const 1050884
    i32.const 43
    local.get 0
    call 26
    unreachable)
  (func (;49;) (type 0) (param i32 i32) (result i32)
    block (result i32)  ;; label = @1
      local.get 0
      i32.load
      i32.load8_u
      i32.eqz
      if  ;; label = @2
        local.get 1
        i32.const 1051288
        i32.const 5
        call 1
        br 1 (;@1;)
      end
      local.get 1
      i32.const 1051293
      i32.const 4
      call 1
    end)
  (func (;50;) (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.const 1048648
    local.get 1
    call 3)
  (func (;51;) (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.const 1048672
    local.get 1
    call 3)
  (func (;52;) (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.const 1048624
    local.get 1
    call 3)
  (func (;53;) (type 2) (param i32 i32)
    local.get 0
    local.get 1
    i32.const 1051544
    i32.load
    local.tee 0
    i32.const 6
    local.get 0
    select
    call_indirect (type 2)
    unreachable)
  (func (;54;) (type 1) (param i32 i32 i32) (result i32)
    local.get 2
    local.get 0
    local.get 1
    call 1)
  (func (;55;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 2
      local.tee 4
      i32.const 16
      i32.lt_u
      if  ;; label = @2
        local.get 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 3
      i32.add
      local.set 5
      local.get 3
      if  ;; label = @2
        local.get 0
        local.set 2
        local.get 1
        local.set 6
        loop  ;; label = @3
          local.get 2
          local.get 6
          i32.load8_u
          i32.store8
          local.get 6
          i32.const 1
          i32.add
          local.set 6
          local.get 2
          i32.const 1
          i32.add
          local.tee 2
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 5
      local.get 4
      local.get 3
      i32.sub
      local.tee 8
      i32.const -4
      i32.and
      local.tee 7
      i32.add
      local.set 2
      block  ;; label = @2
        local.get 1
        local.get 3
        i32.add
        local.tee 3
        i32.const 3
        i32.and
        if  ;; label = @3
          local.get 7
          i32.const 0
          i32.le_s
          br_if 1 (;@2;)
          local.get 3
          i32.const 3
          i32.shl
          local.tee 4
          i32.const 24
          i32.and
          local.set 9
          local.get 3
          i32.const -4
          i32.and
          local.tee 6
          i32.const 4
          i32.add
          local.set 1
          i32.const 0
          local.get 4
          i32.sub
          i32.const 24
          i32.and
          local.set 4
          local.get 6
          i32.load
          local.set 6
          loop  ;; label = @4
            local.get 5
            local.get 6
            local.get 9
            i32.shr_u
            local.get 1
            i32.load
            local.tee 6
            local.get 4
            i32.shl
            i32.or
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            local.get 2
            i32.lt_u
            br_if 0 (;@4;)
          end
          br 1 (;@2;)
        end
        local.get 7
        i32.const 0
        i32.le_s
        br_if 0 (;@2;)
        local.get 3
        local.set 1
        loop  ;; label = @3
          local.get 5
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 2
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 8
      i32.const 3
      i32.and
      local.set 4
      local.get 3
      local.get 7
      i32.add
      local.set 1
    end
    local.get 4
    if  ;; label = @1
      local.get 2
      local.get 4
      i32.add
      local.set 3
      loop  ;; label = @2
        local.get 2
        local.get 1
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const 1
        i32.add
        local.tee 2
        local.get 3
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func (;56;) (type 3) (param i32))
  (table (;0;) 35 35 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (export "memory" (memory 0))
  (export "foo" (func 28))
  (elem (;0;) (i32.const 1) func 31 42 41 7 47 21 56 49 22 23 13 52 33 24 11 50 18 8 51 38 40 25 43 32 14 19 56 39 35 44 36 45 56 39)
  (data (;0;) (i32.const 1048576) "Hello From Wasm!\0a\00\00\00\00\00\10\00\11\00\00\00\00\00\00\00\07\00\00\00\04\00\00\00\04\00\00\00\08\00\00\00\09\00\00\00\0c\00\00\00\04\00\00\00\0a\00\00\00\0b\00\00\00\0c\00\00\00\0d\00\00\00\0c\00\00\00\04\00\00\00\0e\00\00\00\0f\00\00\00\10\00\00\00\09\00\00\00\0c\00\00\00\04\00\00\00\11\00\00\00\12\00\00\00\13\00\00\00/rust/deps/dlmalloc-0.2.6/src/dlmalloc.rsassertion failed: psize >= size + min_overhead\00x\00\10\00)\00\00\00\a8\04\00\00\09\00\00\00assertion failed: psize <= size + max_overhead\00\00x\00\10\00)\00\00\00\ae\04\00\00\0d\00\00\00\01\00\00\00\00\00\00\00library/std/src/io/buffered/linewritershim.rsmid > len\00\00U\01\10\00\09\00\00\00(\01\10\00-\00\00\00\0a\01\00\00)\00\00\00entity not foundpermission deniedconnection refusedconnection resethost unreachablenetwork unreachableconnection abortednot connectedaddress in useaddress not availablenetwork downbroken pipeentity already existsoperation would blocknot a directoryis a directorydirectory not emptyread-only filesystem or storage mediumfilesystem loop or indirection limit (e.g. symlink loop)stale network file handleinvalid input parameterinvalid datatimed outwrite zerono storage spaceseek on unseekable filefilesystem quota exceededfile too largeresource busyexecutable file busydeadlockcross-device link or renametoo many linksinvalid filenameargument list too longoperation interruptedunsupportedunexpected end of fileout of memoryother erroruncategorized error (os error )\00\00\00\01\00\00\00\00\00\00\00e\04\10\00\0b\00\00\00p\04\10\00\01\00\00\00library/std/src/io/stdio.rs\00\8c\04\10\00\1b\00\00\00-\03\00\00\14\00\00\00failed printing to : \00\00\00\b8\04\10\00\13\00\00\00\cb\04\10\00\02\00\00\00\8c\04\10\00\1b\00\00\00^\04\00\00\09\00\00\00stdoutformatter error\00\00\00\f6\04\10\00\0f\00\00\00(\00\00\00cannot recursively acquire mutex\14\05\10\00 \00\00\00library/std/src/sys/sync/mutex/no_threads.rs<\05\10\00,\00\00\00\14\00\00\00\09\00\00\00library/std/src/sync/once.rsx\05\10\00\1c\00\00\00\d0\00\00\00\14\00\00\00x\05\10\00\1c\00\00\00\d0\00\00\001\00\00\00lock count overflow in reentrant mutexlibrary/std/src/sync/reentrant_lock.rs\da\05\10\00&\00\00\00\bc\00\00\00-\00\00\00memory allocation of  bytes failed\00\00\10\06\10\00\15\00\00\00%\06\10\00\0d\00\00\00library/std/src/alloc.rsD\06\10\00\18\00\00\00b\01\00\00\09\00\00\00library/std/src/panicking.rsl\06\10\00\1c\00\00\00\8b\02\00\00\1e\00\00\00\0d\00\00\00\0c\00\00\00\04\00\00\00\14\00\00\00\07\00\00\00\08\00\00\00\04\00\00\00\15\00\00\00\07\00\00\00\08\00\00\00\04\00\00\00\16\00\00\00\17\00\00\00\18\00\00\00\10\00\00\00\04\00\00\00\19\00\00\00\1a\00\00\00\1b\00\00\00\00\00\00\00\01\00\00\00\1c\00\00\00operation successfulone-time initialization may not be performed recursively\04\07\10\008\00\00\00\10\00\00\00\11\00\00\00\12\00\00\00\10\00\00\00\10\00\00\00\13\00\00\00\12\00\00\00\0d\00\00\00\0e\00\00\00\15\00\00\00\0c\00\00\00\0b\00\00\00\15\00\00\00\15\00\00\00\0f\00\00\00\0e\00\00\00\13\00\00\00&\00\00\008\00\00\00\19\00\00\00\17\00\00\00\0c\00\00\00\09\00\00\00\0a\00\00\00\10\00\00\00\17\00\00\00\19\00\00\00\0e\00\00\00\0d\00\00\00\14\00\00\00\08\00\00\00\1b\00\00\00\0e\00\00\00\10\00\00\00\16\00\00\00\15\00\00\00\0b\00\00\00\16\00\00\00\0d\00\00\00\0b\00\00\00\13\00\00\00x\01\10\00\88\01\10\00\99\01\10\00\ab\01\10\00\bb\01\10\00\cb\01\10\00\de\01\10\00\f0\01\10\00\fd\01\10\00\0b\02\10\00 \02\10\00,\02\10\007\02\10\00L\02\10\00a\02\10\00p\02\10\00~\02\10\00\91\02\10\00\b7\02\10\00\ef\02\10\00\08\03\10\00\1f\03\10\00+\03\10\004\03\10\00>\03\10\00N\03\10\00e\03\10\00~\03\10\00\8c\03\10\00\99\03\10\00\ad\03\10\00\b5\03\10\00\d0\03\10\00\de\03\10\00\ee\03\10\00\04\04\10\00\19\04\10\00$\04\10\00:\04\10\00G\04\10\00R\04\10\00capacity overflow\00\00\00\8c\08\10\00\11\00\00\00library/alloc/src/raw_vec.rs\a8\08\10\00\1c\00\00\00\19\00\00\00\05\00\00\00BorrowMutErroralready borrowed: \e2\08\10\00\12\00\00\00\01\00\00\00\00\00\00\00called `Option::unwrap()` on a `None` value\00!\00\00\00\00\00\00\00\01\00\00\00\22\00\00\00==!=matchesassertion `left  right` failed\0a  left: \0a right: \00K\09\10\00\10\00\00\00[\09\10\00\17\00\00\00r\09\10\00\09\00\00\00 right` failed: \0a  left: \00\00\00K\09\10\00\10\00\00\00\94\09\10\00\10\00\00\00\a4\09\10\00\09\00\00\00r\09\10\00\09\00\00\0000010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899falsetruelibrary/core/src/slice/memchr.rs\00\00\00\a1\0a\10\00 \00\00\00\83\00\00\00\1e\00\00\00\a1\0a\10\00 \00\00\00\9f\00\00\00\09\00\00\00range start index  out of range for slice of length \e4\0a\10\00\12\00\00\00\f6\0a\10\00\22\00\00\00range end index (\0b\10\00\10\00\00\00\f6\0a\10\00\22"))
