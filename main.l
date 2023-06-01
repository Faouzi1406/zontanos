; ModuleID = 'main'
source_filename = "main"

define [12 x i8] @name() {
entry:
  %some = alloca [2 x i8], align 1
  store [2 x i8] c"he", [2 x i8]* %some, align 1
  ret [12 x i8] c"hello world!"
}

define i32 @main() {
entry:
  %some = alloca [12 x i8], align 1
  store [12 x i8] c"Hello world!", [12 x i8]* %some, align 1
  ret i32 0
}
