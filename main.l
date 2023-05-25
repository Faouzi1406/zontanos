; ModuleID = 'main'
source_filename = "main"

@call_str = private unnamed_addr constant [25 x i8] c"Other just got called %c\00", align 1
@call_str.1 = private unnamed_addr constant [45 x i8] c"%c hello world! %c see this is very cool! %c\00", align 1
@call_str.2 = private unnamed_addr constant [64 x i8] c"%c hello world! %c see this is very cool! %c some_value = %d \\n\00", align 1

define i32 @other(i32 %0) {
entry:
  %some_print = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([25 x i8], [25 x i8]* @call_str, i32 0, i32 0), i8 10)
  ret i32 25
}

declare i32 @printf(i8*, ...)

define i32 @main() {
entry:
  %call_to_func = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([45 x i8], [45 x i8]* @call_str.1, i32 0, i32 0), i8 9, i8 10, i8 10)
  %some_value = call i32 @other(i32 10)
  %other_call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([64 x i8], [64 x i8]* @call_str.2, i32 0, i32 0), i8 9, i8 10, i8 10, i32 %some_value)
  ret i32 0
}
