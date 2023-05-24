; ModuleID = 'main'
source_filename = "main"

@call_str = private unnamed_addr constant [40 x i8] c"hello world!%csee this is very cool! %c\00", align 1

define i32 @other(i32 %0) {
entry:
  ret i32 0
}

define i32 @main() {
entry:
  %some = alloca [16 x i8], align 1
  store [16 x i8] c"dit is een test!", [16 x i8]* %some, align 1
  %call_to_func = call i32 @other(i32 10)
  %print_something = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([40 x i8], [40 x i8]* @call_str, i32 0, i32 0), i8 10, i8 10)
  ret i32 1
}

declare i32 @printf(i8*, ...)
