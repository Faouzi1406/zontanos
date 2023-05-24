; ModuleID = 'main'
source_filename = "main"

define i32 @other(i32 %0) {
entry:
  ret i32 0
}

define i32 @main() {
entry:
  %some = alloca [16 x i8], align 1
  store [16 x i8] c"dit is een test!", ptr %some, align 1
  %call_to_func = call i32 @other(i32 10)
  ret i32 0
}
