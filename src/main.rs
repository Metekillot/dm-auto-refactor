use dm::annotation;
use dm::annotation::Annotation;
use dm::annotation::AnnotationTree;
use dm::indents::IndentProcessor;
use dm::objtree::ObjectTree;
use dm::parser::Parser;
use dm::preprocessor::Preprocessor;
use dm::Context;
use dm::DMError;
use dmc;
use std::env;
use std::path::PathBuf;

fn main() {
    match env::var("USERPROFILE") {
        Ok(val) => {
            let mut requiem_path = PathBuf::from(&val);
            requiem_path.push("code/git/requiem/tgstation.dme");
            //let mut tgstation_path = PathBuf::from(&val);
            //tgstation_path.push("code/git/tgstation/tgstation.dme");
            let req_context = Context::default();
            let mut req_annotation_tree = AnnotationTree::default();
            let pp = Preprocessor::new(&req_context, requiem_path.to_owned()).unwrap();
            let ip = IndentProcessor::new(&req_context, pp);
            let mut req_parser = Parser::new(&req_context, ip);
            req_parser.annotate_to(&mut req_annotation_tree);
            //let tg_context = Context::default();
            let req_obj_tree = req_parser.parse_object_tree();
            dmc::run(&req_context, &req_obj_tree);
            //let tg_obj_tree = tg_context.parse_environment(tgstation_path.as_path()).unwrap();
            let req_errors = req_context.errors().to_owned();
            let types: Vec<&dm::objtree::Type> =
                req_obj_tree.iter_types().map(|t| t.get()).collect();
        }

        Err(e) => {
            println!("USERPROFILE is not set: {}", e)
        }
    }
}
