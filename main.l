; ModuleID = 'main'
source_filename = "main"

@str_pointer = private unnamed_addr constant [13 x i8] c"value: %c %c\00", align 1

define [13 x i8] @name() {
entry:
  ret [13 x i8] c"Hello, world!"
}

define [4 x i32] @some() {
entry:
  ret [4 x i32] [i32 1, i32 2, i32 3, i32 4]
}

define i8 @w() {
entry:
  ret i8 99
}

define i32 @main() {
entry:
  %testing = alloca i32, align 4
  store i32 0, i32* %testing, align 4
  %values = alloca [4 x i32], align 4
  store [4 x i32] [i32 1, i32 2, i32 3, i32 4], [4 x i32]* %values, align 4
  %somstr = alloca [13 x i8], align 1
  store [13 x i8] c"Hello, world!", [13 x i8]* %somstr, align 1
  %wow = alloca i8, align 1
  store i8 97, i8* %wow, align 1
  %call = call i8 @getchar()
  %woooo = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @str_pointer, i32 0, i32 0), i8 %call, i8 10)
  %call1 = call i8 @getchar()
  %call2 = call i8 @putchar(i8 %call1)
  ret i32 0
}

declare i32 @printf(i8*, ...)

declare i8 @getchar()

declare i8 @putchar(i8)
