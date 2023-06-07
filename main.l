; ModuleID = 'main'
source_filename = "main"

@str_pointer = private unnamed_addr constant [10 x i8] c"value: %s\00", align 1
@str_pointer.1 = private unnamed_addr constant [3 x i8] c"%c\00", align 1

define [13 x i8] @print(i8* %0) {
entry:
  %call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([10 x i8], [10 x i8]* @str_pointer, i32 0, i32 0), i8* %0)
  ret [13 x i8] c"Hello, world!"
}

declare i32 @printf(i8*, ...)

define i8 @readChar() {
entry:
  %value = call i8 @getchar()
  ret i8 %value
}

declare i8 @getchar()

define i8 @putChar(i8 %0) {
entry:
  %call = call i8 @putchar(i8 %0)
  ret i8 %0
}

declare i8 @putchar(i8)

define i32 @main() {
entry:
  %call = call i8 @readChar()
  %call1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @str_pointer.1, i32 0, i32 0), i8 %call)
  %call2 = call i8 @putChar(i8 97)
  ret i32 0
}
