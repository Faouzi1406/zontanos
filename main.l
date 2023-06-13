; ModuleID = 'main'
source_filename = "main"

@str_pointer = private unnamed_addr constant [5 x i8] c"%s%c\00", align 1
@str_pointer.1 = private unnamed_addr constant [17 x i8] c"Welkom my friend\00", align 1
@str_pointer.2 = private unnamed_addr constant [11 x i8] c"oaky %d %c\00", align 1
@str_pointer.3 = private unnamed_addr constant [15 x i8] c"limit reached.\00", align 1
@str_pointer.4 = private unnamed_addr constant [17 x i8] c"Value is now: %d\00", align 1
@str_pointer.5 = private unnamed_addr constant [2 x i8] c";\00", align 1
@str_pointer.6 = private unnamed_addr constant [7 x i8] c"faouzi\00", align 1
@str_pointer.7 = private unnamed_addr constant [10 x i8] c"fact = %d\00", align 1

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

define i32 @recurse(i32 %0, i32 %1) {
entry:
  %call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([11 x i8], [11 x i8]* @str_pointer.2, i32 0, i32 0), i32 %0, i8 10)
  %comp = icmp sge i32 %0, %1
  br i1 %comp, label %if_do, label %else_do
  ret i32 18

if_do:                                            ; preds = %entry
  %call1 = call i32 @println(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @str_pointer.3, i32 0, i32 0))
  ret i32 %0

else_do:                                          ; preds = %entry
  %v = alloca i32, align 4
  %load_val = load i32, i32* %v, align 4
  %add_op = add i32 %load_val, 1
  store i32 %add_op, i32* %v, align 4
  %loaded = load i32, i32* %v, align 4
  %return = call i32 @recurse(i32 %loaded, i32 %1)
  ret i32 %return
}

define i32 @factorial(i32 %0) {
entry:
  %comp = icmp sge i32 %0, 1
  br i1 %comp, label %if_do, label %else_do

if_do:                                            ; preds = %entry
  ret i32 1

else_do:                                          ; preds = %entry
  %b = alloca i32, align 4
  %c = alloca i32, align 4
  %load_val = load i32, i32* %b, align 4
  %add_op = sub i32 %load_val, 1
  store i32 %add_op, i32* %b, align 4
  %loaded = load i32, i32* %b, align 4
  %fact = call i32 @factorial(i32 %loaded)
  %load_val1 = load i32, i32* %c, align 4
  %add_op2 = mul i32 %load_val1, %fact
  store i32 %add_op2, i32* %c, align 4
  %ret_laod = load i32, i32* %c, align 4
  ret i32 %ret_laod
}

define void @testTimesIs() {
entry:
  %value = alloca i32, align 4
  store i32 10, i32* %value, align 4
  %load_val = load i32, i32* %value, align 4
  %add_op = sub i32 %load_val, 200
  store i32 %add_op, i32* %value, align 4
  %loaded = load i32, i32* %value, align 4
  %call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([17 x i8], [17 x i8]* @str_pointer.4, i32 0, i32 0), i32 %loaded)
  %call1 = call i32 @println(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @str_pointer.5, i32 0, i32 0))
  ret void
}

define void @main() {
entry:
  %fun = call i32 @recurse(i32 0, i32 1000)
  %call = call i32 @println(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @str_pointer.6, i32 0, i32 0))
  call void @testTimesIs()
  %fact = call i32 @factorial(i32 10)
  %call1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([10 x i8], [10 x i8]* @str_pointer.7, i32 0, i32 0), i32 %fact)
  ret void
}
