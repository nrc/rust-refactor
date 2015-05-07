extern crate csv;

use rustc::session::Session;
use rustc::session::config::{self, Input};
use rustc_driver::{driver, CompilerCalls, Compilation, RustcDefaultCalls};
use rustc::metadata::creader::CrateReader;
use rustc_resolve as resolve;
use rustc::middle::lang_items;
use rustc::middle::ty;
use syntax::ast_map;
use syntax::{ast, attr, diagnostics, visit};
use std::collections::HashMap;
use strings::src_rope::Rope;
use std::path::PathBuf;
use getopts;

#[derive(Debug, PartialEq)]
pub enum Response {
    Error,
    Conflict
}

pub fn rename_variable(input: &str, analysis: &str, new_name: &str, rename_var: &str) -> Result<String, Response> {
    let analyzed_data = init(analysis);
    
    //for (key, value) in analyzed_data.type_map.iter() {
    //    println!("{}: \"{}\"", *key, value.get("id").unwrap());
    //}

    //for (key, value) in analyzed_data.type_ref_map.iter() {
    //  println!("{}: \"{:?}\"", *key, value);
    //}

    let dec_map = analyzed_data.var_map;
    let ref_map = analyzed_data.var_ref_map;

    // Check if renaming will cause conflicts
    match dec_map.get(rename_var) {
        Some(x) => {
            for (key, value) in dec_map.iter() {
                let name = value.get("name").unwrap();
                if (x.get("scopeid") == value.get("scopeid") &&
                    name == &new_name) {
                    // Conflict present:
                    // May still be ok if there is no references to it
                    // However, standalone blocks won't be detected + macros
                    // Will also restrict if reference is on same line as renaming
                    let id = value.get("id").unwrap();
                    let line_no: i32 = value.get("file_line").unwrap().parse().unwrap();
                    if let Some(refs) = ref_map.get(id) {
                        for record in refs.iter() {
                            let line: i32 = record.get("file_line").unwrap().parse().unwrap();
                            if line >= line_no {
                                // Affected reference
                                return Err(Response::Conflict); //input.to_string();
                            }
                        }
                    }
                }
            }
        },
        _ => { return Err(Response::Conflict); } //input.to_string(); }
    }

    Ok(rename_dec_and_ref(input, new_name, rename_var, dec_map, ref_map))
}

pub fn rename_type(input: &str, analysis: &str, new_name: &str, rename_var: &str) -> Result<String, Response> {
    let analyzed_data = init(analysis);

    //for (key, value) in analyzed_data.type_map.iter() {
    //  println!("{}: \"{:?}\"", *key, value);
    //}

    let dec_map = analyzed_data.type_map;
    let ref_map = analyzed_data.type_ref_map;

    Ok(rename_dec_and_ref(input, new_name, rename_var, dec_map, ref_map))
}

pub fn rename_function(input: &str, analysis: &str, new_name: &str, rename_var: &str) -> Result<String, Response> {
    let analyzed_data = init(analysis);

    // method calls refer to top level trait function in declid

    // rename original function 

    // then rename all statically dispatched with refid = id
    // then rename all dynamically dispatched with declid = id
    // then rename all functions with declid = id
    // assuming mutual exclusion, these should all be covered in func_ref_map

    let dec_map = analyzed_data.func_map;
    let ref_map = analyzed_data.func_ref_map;

    Ok(rename_dec_and_ref(input, new_name, rename_var, dec_map, ref_map))
}

struct AnalysisData {
    var_map: HashMap<String, HashMap<String, String>>,
    var_ref_map: HashMap<String, Vec<HashMap<String, String>>>,
    type_map: HashMap<String, HashMap<String, String>>,
    type_ref_map: HashMap<String, Vec<HashMap<String, String>>>,
    func_map: HashMap<String, HashMap<String, String>>,
    func_ref_map: HashMap<String, Vec<HashMap<String, String>>>,
}

fn init(analysis: &str) -> AnalysisData {
    let mut var_map = HashMap::new();
    let mut var_ref_map = HashMap::new();
    let mut type_map = HashMap::new();
    let mut type_ref_map = HashMap::new();
    let mut ctor_map = HashMap::new();
    let mut qual_type_map = HashMap::new();
    let mut func_map = HashMap::new();
    let mut func_ref_map = HashMap::new();

    for line in analysis.lines() {
        //println!("{}", line);
        let mut rdr = csv::Reader::from_string(line).has_headers(false);
        for row in rdr.records() {
            let row = row.unwrap();
            let mut map_record = HashMap::new();
            //println!("{:?}", row);

            let mut it = row.iter();
            it.next(); // discard first value
            while let Some(key) = it.next() {
                if let Some(val) = it.next() {
                    // has pair of values as expected
                    if key.to_string() == "qualname" {
                        let new_val = val.trim_left_matches(':');
                        map_record.insert(key.clone(), new_val.to_string());
                        if !map_record.contains_key("name") {
                            let name: Vec<&str> = new_val.split("::").collect();
                            map_record.insert("name".to_string(), name[name.len()-1].to_string());
                        }
                    } else {
                        map_record.insert(key.clone(), val.clone());
                    }
                } else {
                    break;
                }
            }

            match &row[0][..] {
                "crate" => {},
                "external_crate" => {},
                "end_external_crates" => {},
                "function" | "function_impl" | "method_decl" => {
                    let rec = map_record.clone();
                    let copy = map_record.clone();
                    let key = rec.get("id").unwrap();
                    func_map.insert(key.clone(), map_record);

                    // Treat method impl as a function ref
                    let declid = rec.get("declid");
                    match declid {
                        Some(x) if *x != "" => {
                            if !func_ref_map.contains_key(x) {
                                let v = vec![copy];
                                func_ref_map.insert(x.clone(), v);
                            } else {
                                let vec = func_ref_map.get_mut(x);
                                vec.unwrap().push(copy);
                            }
                        },
                        _ => {}
                    }
                },
                "fn_ref" | "fn_call" | "method_call" => {
                    let rec = map_record.clone();
                    let refid = rec.get("refid");
                    let declid = rec.get("declid");
                    let mut key = "".to_string();

                    match refid {
                        Some(x) if *x != "" && *x != "0" => {
                            key = x.clone();
                        },
                        _ => {
                            match declid {
                                Some(x) if *x != "" => {
                                    key = x.clone();
                                },
                                None | _ => {}
                            }
                        }
                    }

                    if !func_ref_map.contains_key(&key) {
                        let v = vec![map_record];
                        func_ref_map.insert(key, v);
                    } else {
                        let vec = func_ref_map.get_mut(&key);
                        vec.unwrap().push(map_record);
                    
                    }
                },
                "variable" => {
                    let key = map_record.get("id").unwrap().clone();
                    var_map.insert(key, map_record);
                },
                "var_ref" => {
                    let key = map_record.get("refid").unwrap().clone();

                    if !var_ref_map.contains_key(&key) {
                        let v = vec![map_record];
                        var_ref_map.insert(key, v);
                    } else {
                        let vec = var_ref_map.get_mut(&key);
                        vec.unwrap().push(map_record);
                    
                    }
                },
                "enum" => {
                    let rec = map_record.clone();
                    let key = rec.get("id").unwrap();
                    let q_key = rec.get("qualname").unwrap();
                    type_map.insert(key.clone(), map_record);
                    qual_type_map.insert(q_key.clone(), key.clone());
                },
                "struct"  => {
                    let rec = map_record.clone();
                    let key = rec.get("id").unwrap();
                    let c_key = rec.get("ctor_id").unwrap();
                    let q_key = rec.get("qualname").unwrap();
                    type_map.insert(key.clone(), map_record);
                    ctor_map.insert(c_key.clone(), key.clone());
                    qual_type_map.insert(q_key.clone(), key.clone());
                },
                "type_ref" | "struct_ref" | "mod_ref" => {
                    let key = map_record.get("refid").unwrap().clone();

                    if !type_ref_map.contains_key(&key) {
                        let v = vec![map_record];
                        type_ref_map.insert(key, v);
                    } else {
                        let vec = type_ref_map.get_mut(&key);
                        vec.unwrap().push(map_record);
                    
                    }
                },
                "module" => {},
                "module_alias" => {},
                "unknown_ref" => {},
                _ => {}
            }
        }
        
    }

    // Fixup type_refs with refid = 0 and ctor_id references
    let mut to_add = Vec::new();
    for (key, value) in type_ref_map.iter() {
        if *key == "0" {
            for i in value.iter() {
                // must check qualname
                let name = i.get("qualname").unwrap();
                if qual_type_map.contains_key(name) {
                    let mut modified = i.clone();
                    modified.insert("refid".to_string(), qual_type_map.get(name).unwrap().clone());
                    to_add.push(modified);
                }
            }
        } else if let Some(ctor) = ctor_map.get(key) {
            for i in value.iter() {
                let mut modified = i.clone();
                modified.insert("refid".to_string(), ctor.clone());
                to_add.push(modified);
            }
        }
    }

    for add in to_add.iter() {
        let key = add.get("refid").unwrap().clone();
        if !type_ref_map.contains_key(&key) {
            let v = vec![add.clone()];
            type_ref_map.insert(key, v);
        } else {
            let vec = type_ref_map.get_mut(&key);
            vec.unwrap().push(add.clone());
        
        }
    }

    return AnalysisData{ var_map: var_map, var_ref_map: var_ref_map, type_map: type_map,
                         type_ref_map: type_ref_map, func_map: func_map, func_ref_map: func_ref_map }
}

fn rename_dec_and_ref(input: &str, new_name: &str, rename_var: &str,
                      dec_map: HashMap<String, HashMap<String, String>>, 
                      ref_map: HashMap<String, Vec<HashMap<String, String>>>) -> String {
    let mut ropes: Vec<Rope> = input.lines().map(|x| Rope::from_string(String::from_str(x))).collect();

    // TODO Failed an attempt to chain the declaration to the other iterator...
    let map = dec_map.get(rename_var).unwrap();
    let file_col: usize = map.get("file_col").unwrap().parse().unwrap();
    let file_line: usize = map.get("file_line").unwrap().parse().unwrap();
    let file_col_end: usize = map.get("file_col_end").unwrap().parse().unwrap();
    let file_line_end: usize = map.get("file_line_end").unwrap().parse().unwrap();
    rename(&mut ropes, file_col, file_line, file_col_end, file_line_end, new_name);

    if let Some(references) = ref_map.get(rename_var) {
        for map in references.iter() {
            let file_col: usize = map.get("file_col").unwrap().parse().unwrap();
            let file_line: usize = map.get("file_line").unwrap().parse().unwrap();
            let file_col_end: usize = map.get("file_col_end").unwrap().parse().unwrap();
            let file_line_end: usize = map.get("file_line_end").unwrap().parse().unwrap();

            println!("{} {} {} {}", file_col, file_line, file_col_end, file_line_end);
            rename(&mut ropes, file_col, file_line, file_col_end, file_line_end, new_name);
        }
    }

    let mut answer = String::new();
    let mut count = ropes.len();
    for rope in &ropes {
        answer.push_str(&rope.to_string());
        if count > 1 {
            answer.push_str("\n");
            count -= 1;
        }
    }

    answer
}

fn rename(ropes: &mut Vec<Rope>, file_col:usize , file_line:usize,
          file_col_end: usize, file_line_end: usize, new_name: &str) {
    let to_change = &mut ropes[file_line-1..file_line_end];
    let length = to_change.len();

    if file_line == file_line_end {
        to_change[0].src_remove(file_col, file_col_end);
    } else {
        for i in 0..length {
            let len = to_change[i].len();
            let line = &mut to_change[i];
            match i {
                0 => line.src_remove(file_col, len),
                x if x == length => line.src_remove(0, file_col_end),
                _ => line.src_remove(0, len)
            }
        }
    }

    to_change[0].src_insert(file_col, new_name.to_string());
}

// TODO more efficient, perhaps better indexed and given type of node as arg
// Factor out the init.
pub fn identify_id(input_filename: &str, analysis: &str, rename_var: &str, 
               row: i32, col: i32) -> String {
    let analyzed_data = init(analysis);

    println!("{} {} {}", rename_var, row, col);
    for (key, value) in analyzed_data.var_map {
        if check_match(rename_var, input_filename, row, col, value) {
            return key;
        }
    }

    for (key, value) in analyzed_data.type_map {
        if check_match(rename_var, input_filename, row, col, value) {
            return key;
        }
    }

    for (key, value) in analyzed_data.func_map {
        if check_match(rename_var, input_filename, row, col, value) {
            return key;
        }
    }

    "".to_string()
}

fn check_match(name: &str, input_filename: &str, row: i32, col: i32, 
               record: HashMap<String, String>) -> bool {

    let c: i32 = record.get("file_col").unwrap().parse().unwrap();
    let r: i32 = record.get("file_line").unwrap().parse().unwrap();
    let r_end: i32 = record.get("file_line_end").unwrap().parse().unwrap();
    let c_end: i32 = record.get("file_col_end").unwrap().parse().unwrap();
    let filename = record.get("file_name").unwrap();
    let n = record.get("name").unwrap();

    if &name == n { //&& filename == &input_filename {
        if !(row < 0) {
            if row >= r && row <= r_end {
                if !(col < 0) {
                    if col >= c && col < c_end {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        } else {
            return true;
        }
    }

    false
}

struct RefactorCalls {
    default_calls: RustcDefaultCalls,
}

impl RefactorCalls {
    fn new() -> RefactorCalls {
        RefactorCalls { default_calls: RustcDefaultCalls }
    }
}

impl<'a> CompilerCalls<'a> for RefactorCalls {
    fn early_callback(&mut self,
                      _: &getopts::Matches,
                      _: &diagnostics::registry::Registry)
                      -> Compilation {
        Compilation::Continue
    }

    fn late_callback(&mut self,
                     m: &getopts::Matches,
                     s: &Session,
                     i: &Input,
                     odir: &Option<PathBuf>,
                     ofile: &Option<PathBuf>)
                     -> Compilation {
        self.default_calls.late_callback(m, s, i, odir, ofile);
        Compilation::Continue
    }

    fn some_input(&mut self, input: Input, input_path: Option<PathBuf>) -> (Input, Option<PathBuf>) {
        (input, input_path)
    }

    fn no_input(&mut self,
                m: &getopts::Matches,
                o: &config::Options,
                odir: &Option<PathBuf>,
                ofile: &Option<PathBuf>,
                r: &diagnostics::registry::Registry)
                -> Option<(Input, Option<PathBuf>)> {
        self.default_calls.no_input(m, o, odir, ofile, r);
        // This is not optimal error handling.
        panic!("No input supplied");
    }

    fn build_controller(&mut self, _: &Session) -> driver::CompileController<'a> {
        let mut control = driver::CompileController::basic();
        control.after_write_deps.stop = Compilation::Stop;
        control.after_write_deps.callback = box |state| {
            let krate =  state.expanded_crate.unwrap().clone();
            let ast_map = state.ast_map.unwrap();
            let krate = ast_map.krate();
            CrateReader::new(&state.session).read_crates(krate);
            let lang_items = lang_items::collect_language_items(krate, &state.session);
            let resolve::CrateMap {
                def_map,
                freevars,
                export_map,
                trait_map,
                external_exports,
                glob_map,
            } = resolve::resolve_crate(&state.session, &ast_map, &lang_items, krate, resolve::MakeGlobMap::No);
            println!("{:?}", def_map);
        };

        control
    }
}
