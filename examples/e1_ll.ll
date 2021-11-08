define i32 @main(i32, i32) {
    %3 = alloca i32
    %4 = alloca i32
    store i32 %0, i32* %3
    store i32 %1, i32* %4
    %5 = load i32, i32* %3
    %6 = load i32, i32* %4
    %7 = add i32 %5, %6
    ret i32 %7
}