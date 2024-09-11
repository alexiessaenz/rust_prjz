C:\workspace5\rust_prjz\structs_and_enums [master ≡]> cargo run 
   Compiling structs_and_enums v0.1.0 (C:\workspace5\rust_prjz\structs_and_enums)
warning: unused variable: `point_a`                            
  --> src/main.rs:42:9
   |
42 |     let point_a = Point(12, 33, 66);
   |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_point_a`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: fields `0`, `1`, and `2` are never read
  --> src/main.rs:41:18
   |
41 |     struct Point(i32,i32,i32);
   |            ----- ^^^ ^^^ ^^^
   |            |
   |            fields in this struct
   |
   = help: consider removing these fields
   = note: `#[warn(dead_code)]` on by default

warning: `structs_and_enums` (bin "structs_and_enums") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.20s
     Running `target\debug\structs_and_enums.exe`
Usuario Julio, 
edad 32

Usuario Ana,
edad 32,
email some@email.com,
activo true

People:
nombre Julio,
edad 31,
email something@email.com,
activo true
C:\workspace5\rust_prjz\structs_and_enums [master ≡ +0 ~1 -0 !]