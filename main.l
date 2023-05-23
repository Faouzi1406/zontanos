; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %some = alloca [3 x i8], align 1
  store [3 x i8] c"abc", [3 x i8]* %some, align 1
  ret i32 0
}
