; ModuleID = 'probe4.28e7d3ccde49b5ce-cgu.0'
source_filename = "probe4.28e7d3ccde49b5ce-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

@alloc_67f5413620faf71265770e304a4128ee = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/c469197b19d53a6c45378568f73c00986b20a5a5/library/core/src/num/mod.rs" }>, align 1
@alloc_cae19efaa9898aefc8b55272e8bf44a1 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_67f5413620faf71265770e304a4128ee, [16 x i8] c"K\00\00\00\00\00\00\00w\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17hab7b2e0a5be6efbfE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h15342409585817f1E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hdb666576d5c5fa9fE(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_cae19efaa9898aefc8b55272e8bf44a1) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h15342409585817f1E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17hdb666576d5c5fa9fE(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="all" "probe-stack"="inline-asm" "target-cpu"="core2" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="all" "probe-stack"="inline-asm" "target-cpu"="core2" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.74.0-nightly (c469197b1 2023-08-22)"}