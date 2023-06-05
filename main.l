; ModuleID = 'main'
source_filename = "main"

define i32 @name(i32 %0, [12 x i8] %1) {
entry:
  %some = alloca [2 x i8], align 1
  store [2 x i8] c"he", [2 x i8]* %some, align 1
  ret i32 10
}

define void @main() {
entry:
  %some = call i32 @name(i32 20, [12 x i8] c"Hello world!")
  %wicked = call i32 @name(i32 10, [12 x i8] c"Hello world!")
  ret void
}
