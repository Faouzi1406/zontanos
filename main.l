; ModuleID = 'main'
source_filename = "main"

@str_pointer = private unnamed_addr constant [5 x i8] c"%s%c\00", align 1
@str_pointer.1 = private unnamed_addr constant [17 x i8] c"Welkom my friend\00", align 1
@str_pointer.2 = private unnamed_addr constant [5 x i8] c"%d%c\00", align 1
@str_pointer.3 = private unnamed_addr constant [5 x i8] c"%d%c\00", align 1
@str_pointer.4 = private unnamed_addr constant [3 x i8] c"%c\00", align 1
@str_pointer.5 = private unnamed_addr constant [5 x i8] c"done\00", align 1
@str_pointer.6 = private unnamed_addr constant [21 x i8] c"First try... {%d} %c\00", align 1
@str_pointer.7 = private unnamed_addr constant [19 x i8] c"10 is more then 20\00", align 1
@str_pointer.8 = private unnamed_addr constant [23 x i8] c"10 is not more then 20\00", align 1

define i32 @println(i8* %0) {
entry:
  %chars = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @str_pointer, i32 0, i32 0), i8* %0, i8 10)
  ret i32 %chars
}

declare i32 @printf(i8*, ...)

define i32 @readChar() {
entry:
  %value = call i32 @getchar()
  ret i32 %value
}

declare i32 @getchar()

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

define i32 @recurse(i32 %0, i32 %1) {
entry:
  %call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @str_pointer.2, i32 0, i32 0), i32 %0, i8 10)
  %comp = icmp sge i32 %0, %1
  br i1 %comp, label %if_then_do, label %else_do

if_then_do:                                       ; preds = %entry
  ret i32 %0

else_do:                                          ; preds = %entry
  %call1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @str_pointer.3, i32 0, i32 0), i32 %0, i8 10)
  %v = alloca i32, align 4
  %load_val = load i32, i32* %v, align 4
  %add_op = add i32 %load_val, 1
  store i32 %add_op, i32* %v, align 4
  %loaded = load i32, i32* %v, align 4
  %return = call i32 @recurse(i32 %loaded, i32 %1)
  ret i32 %return
}

define void @print_all_chars() {
entry:
  %value = call i32 @getchar()
  %call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @str_pointer.4, i32 0, i32 0), i32 %value)
  %comp = icmp eq i32 %value, -1
  br i1 %comp, label %if_then_do, label %else_do

if_then_do:                                       ; preds = %entry
  %call1 = call i32 @println(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @str_pointer.5, i32 0, i32 0))
  ret void

else_do:                                          ; preds = %entry
  call void @print_all_chars()
  ret void
}

define i32 @testing(i32 %0) {
entry:
  %"0" = add i32 %0, 10
  ret i32 %"0"
}

define void @main() {
entry:
  %testing = alloca i32, align 4
  store i32 7, i32* %testing, align 4
  %some = alloca i32, align 4
  %"1" = load i32, i32* %testing, align 4
  %"2" = mul i32 10, %"1"
  %"14" = load i32, i32* %testing, align 4
  %"16" = call i32 @testing(i32 20)
  %"0" = sub i32 %"2", 20
  %"11" = add i32 %"0", 800
  %"22" = add i32 %"11", 10
  %"3" = add i32 %"22", %"14"
  %"4" = add i32 %"3", %"16"
  store i32 %"4", i32* %some, align 4
  %loaded = load i32, i32* %some, align 4
  %call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([21 x i8], [21 x i8]* @str_pointer.6, i32 0, i32 0), i32 %loaded, i8 10)
  %call3 = call i32 @testing(i32 30)
  br i1 false, label %if_then_do, label %else_do

if_then_do:                                       ; preds = %entry
  %call4 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([19 x i8], [19 x i8]* @str_pointer.7, i32 0, i32 0))
  ret void

else_do:                                          ; preds = %entry
  %call5 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([23 x i8], [23 x i8]* @str_pointer.8, i32 0, i32 0))
  ret void
}
