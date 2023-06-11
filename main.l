; ModuleID = 'main'
source_filename = "main"

@str_pointer = private unnamed_addr constant [5 x i8] c"%s%c\00", align 1
@str_pointer.1 = private unnamed_addr constant [17 x i8] c"Welkom my friend\00", align 1
@str_pointer.2 = private unnamed_addr constant [5 x i8] c"%d%c\00", align 1

define i32 @println(i8* %0) {
entry:
  %chars = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @str_pointer, i32 0, i32 0), i8* %0, i8 10)
  ret i32 %chars
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
  %call = call i32 @println(i8* getelementptr inbounds ([17 x i8], [17 x i8]* @str_pointer.1, i32 0, i32 0))
  ret void
}

define i32 @main() {
entry:
  %some = alloca i32, align 4
  store i32 10, i32* %some, align 4
  store i32 20, i32* %some, align 4
  %loaded = load i32, i32* %some, align 4
  %call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @str_pointer.2, i32 0, i32 0), i32 %loaded, i8 10)
  call void @welkom()
  ret i32 0
}
