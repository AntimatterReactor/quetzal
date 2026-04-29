#include "llvm_bridge.hpp"
#include "llvm/Support/TargetSelect.h"

void quetzal_init_llvm() {
    llvm::InitializeNativeTarget();
    llvm::InitializeNativeTargetAsmPrinter();
    llvm::InitializeNativeTargetAsmParser();
}
