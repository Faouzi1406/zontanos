; ModuleID = 'main'
source_filename = "main"

@str_pointer = private unnamed_addr constant [5 x i8] c"%s%c\00", align 1
@str_pointer.1 = private unnamed_addr constant [17 x i8] c"Welkom my friend\00", align 1

define [13 x i8] @println(i8* %0) {
entry:
  %call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @str_pointer, i32 0, i32 0), i8* %0, i8 10)
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

define void @welkom() {
entry:
  %call = call [13 x i8] @println(i8* getelementptr inbounds ([17 x i8], [17 x i8]* @str_pointer.1, i32 0, i32 0))
  ret void
}

define i32 @main() {
entry:
  call void @welkom()
  ret i32 0
}
