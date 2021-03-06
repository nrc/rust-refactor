
extern crate refactor;

use std::fs::File;
use std::io::prelude::*;
use refactor::refactor::Response;

fn read_to_string(filename: &str) -> String {
    let mut file = match File::open(filename) {
        Err(why) => panic!("couldn't open file {} {}", filename, why),
        Ok(file) => file,
    };

    let mut output = String::new();
    let _ = file.read_to_string(&mut output);

    return output;
}

fn read_analysis(filename: &str) -> refactor::refactor::AnalysisData {
    refactor::refactor::init(&read_to_string(filename))
}

#[test]
fn working_variable_1() {
    let file = "tests/variable/basic_rename.rs";
    let output = read_to_string("tests/variable/working_rename_1_out.rs");
    let analysis = read_analysis("tests/variable/basic_rename.csv");

    match refactor::refactor::rename_variable(&"tests/variable/basic_rename.rs", &analysis, "hello", "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_variable_2() {
    let file = "tests/variable/basic_rename.rs";
    let output = read_to_string("tests/variable/working_rename_2_out.rs");
    let analysis = read_analysis("tests/variable/basic_rename.csv");

    match refactor::refactor::rename_variable(&"tests/variable/basic_rename.rs", &analysis, "hello", "17") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_variable_3() {
    let file = "tests/variable/alex_var_test.rs";
    let output = read_to_string("tests/variable/alex_var_test_out.rs");
    let analysis = read_analysis("tests/variable/alex_var_test.csv");

    match refactor::refactor::rename_variable(&"tests/variable/alex_var_test.rs", &analysis, "bar", "14") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_variable_4() {
    let file = "tests/variable/alex_var_test.rs";
    let output = read_to_string("tests/variable/alex_var_test_out2.rs");
    let analysis = read_analysis("tests/variable/alex_var_test.csv");

    match refactor::refactor::rename_variable(&"tests/variable/alex_var_test.rs", &analysis, "bar", "4") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_variable_5() {
    let file = "tests/variable/const_rename.rs";
    let output = read_to_string("tests/variable/const_rename_out.rs");
    let analysis = read_analysis("tests/variable/const_rename.csv");

    match refactor::refactor::rename_variable(&"tests/variable/const_rename.rs", &analysis, "BAZ", "8") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_variable_6() {
    let file = "tests/variable/working_fn_local.rs";
    let output = read_to_string("tests/variable/working_fn_local_out.rs");
    let analysis = read_analysis("tests/variable/working_fn_local.csv");

    match refactor::refactor::rename_variable(&"tests/variable/working_fn_local.rs", &analysis, "Foo", "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_variable_7() {
    let file = "tests/variable/working_nested.rs";
    let output = read_to_string("tests/variable/working_nested_out.rs");
    let analysis = read_analysis("tests/variable/working_nested.csv");

    match refactor::refactor::rename_variable(&"tests/variable/working_nested.rs", &analysis, "b", "16") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_variable_8() {
    let file = "tests/variable/working_tuple_let.rs";
    let output = read_to_string("tests/variable/working_tuple_let_out.rs");
    let analysis = read_analysis("tests/variable/working_tuple_let.csv");

    match refactor::refactor::rename_variable(file, &analysis, "x", "10") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_variable_9() {
    let file = "tests/variable/working_mut_tuple_let.rs";
    let output = read_to_string("tests/variable/working_mut_tuple_let_out.rs");
    let analysis = read_analysis("tests/variable/working_mut_tuple_let.csv");

    match refactor::refactor::rename_variable(file, &analysis, "x", "10") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_variable_10() {
    let file = "tests/variable/working_mut_tuple_let2.rs";
    let output = read_to_string("tests/variable/working_mut_tuple_let2_out.rs");
    let analysis = read_analysis("tests/variable/working_mut_tuple_let2.csv");

    match refactor::refactor::rename_variable(file, &analysis, "x", "11") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_variable_11() {
    let file = "tests/variable/working_mut_tuple_let3.rs";
    let output = read_to_string("tests/variable/working_mut_tuple_let3_out.rs");
    let analysis = read_analysis("tests/variable/working_mut_tuple_let3.csv");

    match refactor::refactor::rename_variable(file, &analysis, "x", "11") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_variable_12() {
    let file = "tests/variable/conflict_var_use_mod.rs";
    let output = read_to_string("tests/variable/conflict_var_use_mod_out.rs");
    let analysis = read_analysis("tests/variable/conflict_var_use_mod.csv");

    match refactor::refactor::rename_variable(file, &analysis, "BIT3", "6") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn prevented_variable_1() {
    let file = "tests/variable/basic_rename.rs";
    let analysis = read_analysis("tests/variable/basic_rename.csv");

    match refactor::refactor::rename_variable(&file, &analysis, "j", "36") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_variable_2() {
    let file = "tests/variable/basic_rename.rs";
    let analysis = read_analysis("tests/variable/basic_rename.csv");

    match refactor::refactor::rename_variable(&file, &analysis, "x", "36") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_variable_3() {
    let file = "tests/variable/override.rs";
    let analysis = read_analysis("tests/variable/override.csv");

    match refactor::refactor::rename_variable(&file, &analysis, "v", "9") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_variable_4() {
    let file = "tests/variable/name_conflict_method.rs";
    let analysis = read_analysis("tests/variable/name_conflict_method.csv");
    match refactor::refactor::rename_variable(file, &analysis, "foo", "12") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_variable_5() {
    let file = "tests/variable/name_conflict_type.rs";
    let analysis = read_analysis("tests/variable/name_conflict_type.csv");
    match refactor::refactor::rename_variable(file, &analysis, "Foo", "12") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_variable_6() {
    let file = "tests/variable/name_conflict_type_local.rs";
    let analysis = read_analysis("tests/variable/name_conflict_type_local.csv");
    match refactor::refactor::rename_variable(file, &analysis, "Foo", "13") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_variable_7() {
    let file = "tests/variable/name_conflict_type_local2.rs";
    let analysis = read_analysis("tests/variable/name_conflict_type_local2.csv");
    match refactor::refactor::rename_variable(file, &analysis, "Foo", "9") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_variable_8() {
    let file = "tests/variable/name_conflict_method_local.rs";
    let analysis = read_analysis("tests/variable/name_conflict_method_local.csv");
    match refactor::refactor::rename_variable(file, &analysis, "foo", "13") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_variable_9() {
    let file = "tests/variable/name_conflict_method_local2.rs";
    let analysis = read_analysis("tests/variable/name_conflict_method_local2.csv");
    match refactor::refactor::rename_variable(file, &analysis, "Foo", "9") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }

    // fn main() {
    //     let a = 2;
    //     fn foo() {}
    //     foo();
    // }
    //
    // Unlike the type case, this is not detected by the resolve_path
    // This test is slightly modified, using a, to make sure only resolving occurs
    // (Rather than a full run)

}

#[test]
fn prevented_variable_10() {
    let file = "tests/variable/name_conflict_global.rs";
    let analysis = read_analysis("tests/variable/name_conflict_global.csv");
    match refactor::refactor::rename_variable(file, &analysis, "FOO", "12") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_variable_11() {
    let file = "tests/variable/name_conflict_type_global.rs";
    let analysis = read_analysis("tests/variable/name_conflict_type_global.csv");
    match refactor::refactor::rename_variable(file, &analysis, "Foo", "7") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_variable_12() {
    let file = "tests/variable/name_conflict_method_global.rs";
    let analysis = read_analysis("tests/variable/name_conflict_method_global.csv");
    match refactor::refactor::rename_variable(file, &analysis, "foo", "4") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_variable_13() {
    // Broken destructure
    // Point {x, y} = Point{x:1, x:2}
    // => Point {foo, y} = Point{x:1, x:2}
    let file = "tests/variable/destructure_conflict.rs";
    let analysis = read_analysis("tests/variable/destructure_conflict.csv");
    match refactor::refactor::rename_variable(file, &analysis, "bar", "16") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_variable_14() {
    let file = "tests/variable/conflict_var_use_mod.rs";
    let analysis = read_analysis("tests/variable/conflict_var_use_mod.csv");
    match refactor::refactor::rename_variable(file, &analysis, "BIT2", "6") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_variable_15() {
    let file = "tests/variable/conflict_var_use_mod.rs";
    let analysis = read_analysis("tests/variable/conflict_var_use_mod.csv");
    match refactor::refactor::rename_variable(file, &analysis, "BIT1", "11") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn working_argument_1() {
    let file = "tests/variable/fn_args_1.rs";
    let output = read_to_string("tests/variable/fn_args_1_out.rs");
    let analysis = read_analysis("tests/variable/fn_args_1.csv");
    match refactor::refactor::rename_variable(file, &analysis, "z", "6") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_argument_2() {
    let file = "tests/variable/fn_args_2.rs";
    let output = read_to_string("tests/variable/fn_args_2_1_out.rs");
    let analysis = read_analysis("tests/variable/fn_args_2.csv");
    match refactor::refactor::rename_variable(file, &analysis, "z", "10") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_argument_3() {
    let file = "tests/variable/fn_args_2.rs";
    let output = read_to_string("tests/variable/fn_args_2_2_out.rs");
    let analysis = read_analysis("tests/variable/fn_args_2.csv");
    match refactor::refactor::rename_variable(file, &analysis, "z", "15") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn prevented_argument_1() {
    let file = "tests/variable/fn_args_1.rs";
    let analysis = read_analysis("tests/variable/fn_args_1.csv");
    match refactor::refactor::rename_variable(file, &analysis, "c", "6") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_argument_2() {
    let file = "tests/variable/fn_args_1.rs";
    let analysis = read_analysis("tests/variable/fn_args_1.csv");
    match refactor::refactor::rename_variable(file, &analysis, "foo", "6") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn working_struct_1() {
    let file = "tests/type/basic_struct.rs";
    let output = read_to_string("tests/type/working_struct_1_out.rs");
    let analysis = read_analysis("tests/type/basic_struct.csv");

    match refactor::refactor::rename_type(&"tests/type/basic_struct.rs", &analysis, "Pointer", "4") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_struct_2() {
    // ::Point mentioned instead of Point
    let file = "tests/type/scoped_struct.rs";
    let output = read_to_string("tests/type/working_struct_1_out.rs");
    let analysis = read_analysis("tests/type/scoped_struct.csv");

    match refactor::refactor::rename_type(&"tests/type/basic_struct.rs", &analysis, "Pointer", "4") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_struct_3() {
    // Tuple struct
    let file = "tests/type/tuple_struct.rs";
    let output = read_to_string("tests/type/tuple_struct_out.rs");
    let analysis = read_analysis("tests/type/tuple_struct.csv");

    match refactor::refactor::rename_type(&file, &analysis, "Pointer", "4") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_enum_1() {
    let file = "tests/type/basic_enum.rs";
    let output = read_to_string("tests/type/working_enum_1_out.rs");
    let analysis = read_analysis("tests/type/basic_enum.csv");

    match refactor::refactor::rename_type(&"tests/type/basic_enum.rs", &analysis, "YesNo", "4") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_enum_2() {
    let file = "tests/type/modref_enum.rs";
    let output = read_to_string("tests/type/working_enum_2_out.rs");
    let analysis = read_analysis("tests/type/modref_enum.csv");

    match refactor::refactor::rename_type(&"tests/type/modref_enum.rs", &analysis, "YesNo", "7") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn prevented_struct_1() {
    let file = "tests/type/conflict_struct.rs";
    let analysis = read_analysis("tests/type/conflict_struct.csv");

    match refactor::refactor::rename_type(&file, &analysis, "P", "4") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_struct_2() {
    let file = "tests/type/conflict_mod_struct.rs";
    let analysis = read_analysis("tests/type/conflict_mod_struct.csv");

    match refactor::refactor::rename_type(&file, &analysis, "B", "6") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn prevented_struct_3() {
    let file = "tests/type/conflict_use_mod_struct.rs";
    let analysis = read_analysis("tests/type/conflict_use_mod_struct.csv");

    match refactor::refactor::rename_type(&file, &analysis, "B", "6") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn working_method_1() {
    let file = "tests/function/basic_default_method.rs";
    let output = read_to_string("tests/function/working_method_1_out.rs");
    let analysis = read_analysis("tests/function/basic_default_method.csv");

    match refactor::refactor::rename_function(&file, &analysis, "func", "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_method_2() {
    let file = "tests/function/impl_override_method.rs";
    let output = read_to_string("tests/function/working_method_2_out.rs");
    let analysis = read_analysis("tests/function/impl_override_method.csv");

    match refactor::refactor::rename_function(&file, &analysis, "func", "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_method_3() {
    let file = "tests/function/alex_override_method.rs";
    let output = read_to_string("tests/function/alex_override_method_out2.rs");
    let analysis = read_analysis("tests/function/alex_override_method.csv");

    match refactor::refactor::rename_function(&file, &analysis, "grue", "74") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn not_working_method_1() {
    let file = "tests/function/alex_override_method.rs";
    let analysis = read_analysis("tests/function/alex_override_method.csv");

    match refactor::refactor::rename_function(&file, &analysis, "foo", "74") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn working_fn_1() {
    let file = "tests/function/basic_function.rs";
    let output = read_to_string("tests/function/basic_function_out.rs");
    let analysis = read_analysis("tests/function/basic_function.csv");

    match refactor::refactor::rename_function(&file, &analysis, "bar", "4") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_fn_2() {
    let file = "tests/function/basic_module_function.rs";
    let output = read_to_string("tests/function/basic_module_function_out.rs");
    let analysis = read_analysis("tests/function/basic_module_function.csv");

    match refactor::refactor::rename_function(&file, &analysis, "bar", "6") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_fn_3() {
    let file = "tests/function/basic_generic_function.rs";
    let output = read_to_string("tests/function/basic_generic_function_out.rs");
    let analysis = read_analysis("tests/function/basic_generic_function.csv");

    match refactor::refactor::rename_function(&file, &analysis, "bar", "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_fn_4() {
    let file = "tests/function/basic_trait_function.rs";
    let output = read_to_string("tests/function/basic_trait_function_out.rs");
    let analysis = read_analysis("tests/function/basic_trait_function.csv");

    match refactor::refactor::rename_function(&file, &analysis, "bar", "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_fn_5() {
    let file = "tests/function/basic_use_function.rs";
    let output = read_to_string("tests/function/basic_use_function_out.rs");
    let analysis = read_analysis("tests/function/basic_use_function.csv");

    match refactor::refactor::rename_function(&file, &analysis, "bar", "6") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_fn_6() {
    let file = "tests/function/extern_function.rs";
    let output = read_to_string("tests/function/extern_function_out.rs");
    let analysis = read_analysis("tests/function/extern_function.csv");

    match refactor::refactor::rename_function(&file, &analysis, "bar", "4") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_fn_7() {
    let file = "tests/function/extern_stdcall_function.rs";
    let output = read_to_string("tests/function/extern_stdcall_function_out.rs");
    let analysis = read_analysis("tests/function/extern_stdcall_function.csv");

    match refactor::refactor::rename_function(&file, &analysis, "bar", "4") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_fn_8() {
    let file = "tests/function/fn_local_mod.rs";
    let output = read_to_string("tests/function/fn_local_mod_out.rs");
    let analysis = read_analysis("tests/function/fn_local_mod.csv");

    match refactor::refactor::rename_function(&file, &analysis, "bar", "10") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_fn_9() {
    let file = "tests/function/fn_local_mod_after.rs";
    let output = read_to_string("tests/function/fn_local_mod_after_out.rs");
    let analysis = read_analysis("tests/function/fn_local_mod_after.csv");

    match refactor::refactor::rename_function(&file, &analysis, "bar", "13") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn not_working_fn_1() {
    let file = "tests/function/basic_function.rs";
    let analysis = read_analysis("tests/function/basic_function.csv");
    match refactor::refactor::rename_function(&file, &analysis, "main", "4") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn not_working_fn_2() {
    let file = "tests/function/conflict_module_function.rs";
    let analysis = read_analysis("tests/function/conflict_module_function.csv");
    match refactor::refactor::rename_function(&file, &analysis, "foo", "9") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn not_working_fn_3() {
    let file = "tests/function/conflict_module_function.rs";
    let analysis = read_analysis("tests/function/conflict_module_function.csv");
    match refactor::refactor::rename_function(&file, &analysis, "bar", "6") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn not_working_fn_4() {
    let file = "tests/function/conflict_fn_with_var.rs";
    let analysis = read_analysis("tests/function/conflict_fn_with_var.csv");
    match refactor::refactor::rename_function(&file, &analysis, "a", "8") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn multi_file_1() {
    let file = "tests/multi-file/simple_function_1/main.rs";
    let changed1 = file;
    let changed2 = "tests/multi-file/simple_function_1/foo.rs";
    let output1 = read_to_string("tests/multi-file/simple_function_1/main_out.rs");
    let output2 = read_to_string("tests/multi-file/simple_function_1/foo_out.rs");
    let analysis = read_analysis("tests/multi-file/simple_function_1/main.csv");

    match refactor::refactor::rename_function(&file, &analysis, "boo", "6") {
        Ok(x) => {
            assert_eq!(output1.trim(), x.get(changed1).unwrap().trim());
            assert_eq!(output2.trim(), x.get(changed2).unwrap().trim());
        },
        Err(_) => assert!(false)
    }

}

#[test]
fn multi_file_2() {
    let file = "tests/multi-file/simple_function_2/main.rs";
    let changed1 = file;
    let changed2 = "tests/multi-file/simple_function_2/foo/mod.rs";
    let output1 = read_to_string("tests/multi-file/simple_function_2/main_out.rs");
    let output2 = read_to_string("tests/multi-file/simple_function_2/foo/mod_out.rs");
    let analysis = read_analysis("tests/multi-file/simple_function_2/main.csv");

    match refactor::refactor::rename_function(&file, &analysis, "boo", "6") {
        Ok(x) => {
            assert_eq!(output1.trim(), x.get(changed1).unwrap().trim());
            assert_eq!(output2.trim(), x.get(changed2).unwrap().trim());
        },
        Err(_) => assert!(false)
    }

}

#[test]
fn working_inline_1() {
    let file = "tests/inline/inline_single.rs";
    let output = read_to_string("tests/inline/inline_single_out.rs");
    let analysis = read_analysis("tests/inline/inline_single.csv");

    match refactor::refactor::inline_local(&file, &analysis, "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_inline_2() {
    let file = "tests/inline/inline3.rs";
    let output = read_to_string("tests/inline/inline3_out.rs");
    let analysis = read_analysis("tests/inline/inline3.csv");

    match refactor::refactor::inline_local(&file, &analysis, "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_inline_3() {
    let file = "tests/inline/inline4.rs";
    let output = read_to_string("tests/inline/inline4_out.rs");
    let analysis = read_analysis("tests/inline/inline4.csv");

    match refactor::refactor::inline_local(&file, &analysis, "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_inline_4() {
    let file = "tests/inline/inline5.rs";
    let output = read_to_string("tests/inline/inline5_out.rs");
    let analysis = read_analysis("tests/inline/inline5.csv");

    match refactor::refactor::inline_local(&file, &analysis, "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_inline_5() {
    let file = "tests/inline/inline6.rs";
    let output = read_to_string("tests/inline/inline6_out.rs");
    let analysis = read_analysis("tests/inline/inline6.csv");

    match refactor::refactor::inline_local(&file, &analysis, "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_inline_6() {
    let file = "tests/inline/inline7.rs";
    let output = read_to_string("tests/inline/inline7_out.rs");
    let analysis = read_analysis("tests/inline/inline7.csv");

    match refactor::refactor::inline_local(&file, &analysis, "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_inline_7() {
    let file = "tests/inline/inline8.rs";
    let output = read_to_string("tests/inline/inline8_out.rs");
    let analysis = read_analysis("tests/inline/inline8.csv");

    match refactor::refactor::inline_local(&file, &analysis, "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_inline_8() {
    // Multi-file
    let file = "tests/multi-file/simple_inline_1/main.rs";
    let changed = "tests/multi-file/simple_inline_1/foo.rs";
    let output = read_to_string("tests/multi-file/simple_inline_1/foo_out.rs");
    let analysis = read_analysis("tests/multi-file/simple_inline_1/main.csv");

    match refactor::refactor::inline_local(&file, &analysis, "11") {
        Ok(x) => {
            assert_eq!(output.trim(), x.get(changed).unwrap().trim());
        },
        Err(_) => assert!(false)
    }

}

#[test]
fn working_field_1() {
    let file = "tests/field/simple_field.rs";
    let output = read_to_string("tests/field/simple_field_out.rs");
    let analysis = read_analysis("tests/field/simple_field.csv");

    match refactor::refactor::rename_variable(&file, &analysis, "z", "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn prevented_field_1() {
    let file = "tests/field/simple_field.rs";
    let analysis = read_analysis("tests/field/simple_field.csv");

    match refactor::refactor::rename_variable(&file, &analysis, "y", "5") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn working_reify_1() {
    let file = "tests/lifetime/reify_single_in.rs";
    let output = read_to_string("tests/lifetime/reify_single_in_out.rs");
    let analysis = read_analysis("tests/lifetime/reify_single_in.csv");

    match refactor::refactor::restore_fn_lifetime(&file, &analysis, "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_reify_2() {
    let file = "tests/lifetime/reify_single_in_ret.rs";
    let output = read_to_string("tests/lifetime/reify_single_in_ret_out.rs");
    let analysis = read_analysis("tests/lifetime/reify_single_in_ret.csv");

    match refactor::refactor::restore_fn_lifetime(&file, &analysis, "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_reify_3() {
    let file = "tests/lifetime/reify_single_in_anon_ret.rs";
    let output = read_to_string("tests/lifetime/reify_single_in_anon_ret_out.rs");
    let analysis = read_analysis("tests/lifetime/reify_single_in_anon_ret.csv");

    match refactor::refactor::restore_fn_lifetime(&file, &analysis, "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_reify_4() {
    let file = "tests/lifetime/reify_multi_in.rs";
    let output = read_to_string("tests/lifetime/reify_multi_in_out.rs");
    let analysis = read_analysis("tests/lifetime/reify_multi_in.csv");

    match refactor::refactor::restore_fn_lifetime(&file, &analysis, "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_reify_5() {
    let file = "tests/lifetime/reify_multi_named_in.rs";
    let output = read_to_string("tests/lifetime/reify_multi_named_in_out.rs");
    let analysis = read_analysis("tests/lifetime/reify_multi_named_in.csv");

    match refactor::refactor::restore_fn_lifetime(&file, &analysis, "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_reify_6() {
    let file = "tests/lifetime/reify_multi_named_self_ret.rs";
    let output = read_to_string("tests/lifetime/reify_multi_named_self_ret_out.rs");
    let analysis = read_analysis("tests/lifetime/reify_multi_named_self_ret.csv");

    match refactor::refactor::restore_fn_lifetime(&file, &analysis, "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_reify_7() {
    let file = "tests/lifetime/reify_multi_self_ret.rs";
    let output = read_to_string("tests/lifetime/reify_multi_self_ret_out.rs");
    let analysis = read_analysis("tests/lifetime/reify_multi_self_ret.csv");

    match refactor::refactor::restore_fn_lifetime(&file, &analysis, "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_elide_1() {
    let file = "tests/lifetime/elide_single_in.rs";
    let output = read_to_string("tests/lifetime/elide_single_in_out.rs");
    let analysis = read_analysis("tests/lifetime/elide_single_in.csv");

    match refactor::refactor::elide_fn_lifetime(&file, &analysis, "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_elide_2() {
    // You can't elide something that has a lifetime that isn't in the input
    let file = "tests/lifetime/elide_single_anon_static_ret.rs";
    let analysis = read_analysis("tests/lifetime/elide_single_anon_static_ret.csv");

    match refactor::refactor::elide_fn_lifetime(&file, &analysis, "5") {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!(Response::Conflict, x)
    }
}

#[test]
fn working_elide_3() {
    let file = "tests/lifetime/elide_single_in_anon_ret.rs";
    let output = read_to_string("tests/lifetime/elide_single_in_anon_ret_out.rs");
    let analysis = read_analysis("tests/lifetime/elide_single_in_anon_ret.csv");

    match refactor::refactor::elide_fn_lifetime(&file, &analysis, "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_elide_4() {
    let file = "tests/lifetime/elide_single_in_ret.rs";
    let output = read_to_string("tests/lifetime/elide_single_in_ret_out.rs");
    let analysis = read_analysis("tests/lifetime/elide_single_in_ret.csv");

    match refactor::refactor::elide_fn_lifetime(&file, &analysis, "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_elide_5() {
    // TODO this should be fixed
    // At the moment, don't bother eliding even input if output is invalid
    let file = "tests/lifetime/elide_single_static_ret.rs";
    let output = read_to_string("tests/lifetime/elide_single_static_ret_out.rs");
    let analysis = read_analysis("tests/lifetime/elide_single_static_ret.csv");

    match refactor::refactor::elide_fn_lifetime(&file, &analysis, "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_elide_6() {
    let file = "tests/lifetime/elide_multi_in.rs";
    let output = read_to_string("tests/lifetime/elide_multi_in_out.rs");
    let analysis = read_analysis("tests/lifetime/elide_multi_in.csv");

    match refactor::refactor::elide_fn_lifetime(&file, &analysis, "5") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_elide_7() {
    let file = "tests/lifetime/elide_multi_named_self_ret.rs";
    let output = read_to_string("tests/lifetime/elide_multi_named_self_ret_out.rs");
    let analysis = read_analysis("tests/lifetime/elide_multi_named_self_ret.csv");

    match refactor::refactor::elide_fn_lifetime(&file, &analysis, "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_elide_8() {
    let file = "tests/lifetime/elide_multi_anon_self_ret.rs";
    let output = read_to_string("tests/lifetime/elide_multi_anon_self_ret_out.rs");
    let analysis = read_analysis("tests/lifetime/elide_multi_anon_self_ret.csv");

    match refactor::refactor::elide_fn_lifetime(&file, &analysis, "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}

#[test]
fn working_elide_9() {
    // TODO needs to be fixed, elide when there are non-elidables in input
    // e.g. self: 'a, 'b, 'static -> 'a
    // ===> self, _, 'static -> _
    let file = "tests/lifetime/elide_multi_static_self_ret.rs";
    let output = read_to_string("tests/lifetime/elide_multi_static_self_ret_out.rs");
    let analysis = read_analysis("tests/lifetime/elide_multi_static_self_ret.csv");

    match refactor::refactor::elide_fn_lifetime(&file, &analysis, "9") {
        Ok(x) => assert_eq!(output.trim(), x.get(file).unwrap().trim()),
        Err(_) => assert!(false)
    }
}
