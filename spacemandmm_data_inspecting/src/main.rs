use console::Key;
use console::Term;
use dm::annotation::AnnotationTree;
use dm::indents::IndentProcessor;
use dm::objtree::ProcValue;
use dm::parser::Parser;
use dm::preprocessor::Preprocessor;
use dm::Context;
use dmc;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

fn main() {
    match env::var("USERPROFILE") {
        Ok(val) => {
            /*use regex::Regex;
            let loc_trim = Regex::new(r"location: Location \{ .*? \}").unwrap();
            */
            let mut requiem_path = PathBuf::from(&val);
            requiem_path.push("code/git/requiem/tgstation.dme");

            let req_context = Context::default();
            let mut req_annotation_tree = AnnotationTree::default();

            let pp = Box::new(Preprocessor::new(&req_context, requiem_path.to_owned()).unwrap());
            let ip = IndentProcessor::new(&req_context, *pp);
            let mut req_parser = Parser::new(&req_context, ip);
            let anno_mut = &mut req_annotation_tree;
            req_parser.annotate_to(anno_mut);

            let terminal = Term::stdout();

            let req_obj_tree = req_parser.parse_object_tree();

            dmc::run(&req_context, &req_obj_tree);
            /*
            let req_errors = req_context.errors().to_owned();
            let types: Vec<&dm::objtree::Type> = req_obj_tree.iter_types().map(|t| t.get()).collect();
            let annotations: Vec<&Annotation> = req_annotation_tree.iter().map(|(_, a)| a).collect();
            */
            let CONVENIENT_TUPLE = &(&req_annotation_tree, &req_obj_tree, &req_context);
            let mut input;
            loop {
                println!("Waiting for input...");
                println!(
                    "'_file' to inspect files, 'type/path' to inspect type, or 'exit' to quit."
                );
                input = terminal.read_line().expect("Failed to read line");

                let _exit = input == "exit" && break;
                if input == "_file" {
                    file_experiment(CONVENIENT_TUPLE, &terminal);
                    continue;
                }
                let Some(type_search) = req_obj_tree.find(&input) else {
                    println!("Type not found: {}", input);
                    continue;
                };
                loop {
                    println!("Inspecting type: {}", type_search.name());
                    println!("Options: (p)rocs  (v)ars (e)xit (q)uit");
                    if let Ok(Key::Char(c)) = terminal.read_key() {
                        match c {
                            'p' => {
                                display_procs(&type_search, &terminal, CONVENIENT_TUPLE);
                            }
                            /*'v' => {
                                display_vars(&got_type);
                            }*/
                            'e' => {
                                break;
                            }
                            'q' => {
                                println!("Quitting...");
                                return;
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                }
                println!("Exiting...");
            }
        }
        Err(e) => {
            println!("USERPROFILE is not set: {}", e)
        }
    }
}

fn file_experiment(
    CONVENIENT_TUPLE: &(&AnnotationTree, &dm::objtree::ObjectTree, &Context),
    terminal: &console::Term,
) {
    let (anno_tree, obj_tree, context) = *CONVENIENT_TUPLE;
    loop {
        println!("'s'earch 'e'xit 'b'uffer");
        if let Ok(Key::Char(input)) = terminal.read_key() {
            match input {
                's' => {
                    loop {
                        println!("search by 'p' or 'f'ile ID")
                        let mut search_type = terminal.read_key().unwrap()
                        
                    }

                }
                _ => {
                    println!("Invalid input, try again.");
                    continue;
                }
            }
        }
    }
}

fn display_procs(
    got_type: &dm::objtree::TypeRef,
    terminal: &Term,
    CONVENIENT_TUPLE: &(&AnnotationTree, &dm::objtree::ObjectTree, &Context),
) {
    println!("Procs for type: {}", got_type.name());
    let mut proc_ref_iter = got_type.iter_self_procs().enumerate();
    let mut proc_groups: HashMap<
        String,
        Vec<(usize, dm::objtree::ProcRef)>,
        std::hash::RandomState,
    > = HashMap::new();
    let mut grouping_idx = 0;
    loop {
        let Some((iter_idx, proc_ref)) = proc_ref_iter.next() else {
            break;
        };
        if !proc_groups.contains_key(proc_ref.name()) {
            proc_groups.insert(
                proc_ref.name().to_owned(),
                vec![(iter_idx, proc_ref.to_owned())],
            );
        } else {
            proc_groups
                .get_mut(proc_ref.name())
                .unwrap()
                .push((iter_idx, proc_ref).to_owned());
        }
    }
    let sorted_map: BTreeMap<String, Vec<(usize, dm::objtree::ProcRef)>> =
        proc_groups.into_iter().collect();
    for (proc_name, ref_tuples) in sorted_map {
        if ref_tuples.len() > 1 {
            grouping_idx = 0;
            print!("\n\n");
            print!("&&&&&  ");
            for tuple in ref_tuples {
                print!(
                    "t_idx[{}] o_idx:[{}] name: {} & ",
                    tuple.0,
                    tuple.1.index(),
                    proc_name
                );
            }
            print!("\n\n");
            continue;
        }
        grouping_idx += 1;
        if grouping_idx >= 5 {
            println!();
            grouping_idx = 0;
        }
        print!(" = ");
        for tuple in ref_tuples {
            print!("t_idx[{}] name: {}", tuple.0, proc_name);
        }
    }
    let mut input = String::new();
    println!();
    loop {
        println!("Analyze specific proc? Proc name or exit.");
        input = terminal.read_line().expect("Failed to read line");
        if input == "exit" {
            break;
        }
        let Some(matched_proc) = got_type.get_proc(&input) else {
            println!("No proc found with {}", input);
            continue;
        };
        let mut proc_val_vec: Vec<&ProcValue> = Vec::new();
        matched_proc.recurse_children(&mut |p| {
            proc_val_vec.push(p.get());
        });
        println!("Found {} proc values for {}", proc_val_vec.len(), input);
        println!("")
    }
}
