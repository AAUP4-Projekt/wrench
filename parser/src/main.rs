use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::ptr;

extern crate lalrpop;
lalrpop_mod!(pub grammar);  // Genereret parser fra LALRPOP

// Hjælpefunktion til at generere LLVM IR
fn generate_llvm_ir(expr_result: i32) -> String {
    unsafe {
        // Initialiser LLVM
        LLVM_InitializeNativeTarget();
        LLVM_InitializeNativeAsmPrinter();
        LLVM_InitializeNativeAsmParser();

        // Opret LLVM-modul
        let module = LLVMModuleCreateWithName(cstr!("my_module"));
        let builder = LLVMCreateBuilder();
        let context = LLVMContextCreate();

        // Opret funktionstype og funktion
        let function_type = LLVMFunctionType(LLVMInt32Type(), ptr::null_mut(), 0, 0);
        let function = LLVMAddFunction(module, cstr!("main"), function_type);

        // Opret en grundlæggende blok og tilføj den til funktionen
        let entry = LLVMAppendBasicBlock(function, cstr!("entry"));
        LLVMPositionBuilderAtEnd(builder, entry);

        // Generer koden til dit udtryk
        let value = LLVMConstInt(LLVMInt32Type(), expr_result as u64, 0);

        // Returner resultatet
        LLVMBuildRet(builder, value);

        // Opret strengen for at returnere LLVM IR
        let mut ir: *mut i8 = ptr::null_mut();
        let result = LLVMPrintModuleToString(module);
        format!("{:?}", result)
    }
}

fn main() {
    // Eksempel på at analysere et udtryk
    let expr = "3 + 5 * (2 ** 3)";
    let parsed_result = grammar::ExprParser::new().parse(expr).unwrap();

    // Konverter parsed resultat til LLVM IR
    let llvm_ir = generate_llvm_ir(parsed_result);
    
    // Print LLVM IR
    println!("{}", llvm_ir);
}
