; ModuleID = 'main'
source_filename = "main"

@call_str = private unnamed_addr constant [31 x i8] c"Other just got called %s %d %c\00", align 1
@call_str.1 = private unnamed_addr constant [13 x i8] c"hello wolrd!\00", align 1
@call_str.2 = private unnamed_addr constant [8 x i8] c"testing\00", align 1

define i32 @other(i8* %0, i32 %1) {
entry:
  %some_print = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([31 x i8], [31 x i8]* @call_str, i32 0, i32 0), i8* %0, i32 %1, i8 10)
  ret i32 25
}

declare i32 @printf(i8*, ...)

define i32 @main() {
entry:
  %some_value = call i32 @other(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @call_str.1, i32 0, i32 0), i32 10)
  %return = call i32 @other(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @call_str.2, i32 0, i32 0), i32 20)
  ret i32 %return
}
