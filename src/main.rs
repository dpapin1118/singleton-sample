use std::collections::HashMap;
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

#[derive(Debug)]
pub struct Params {
    pub verbose: bool,
    pub config_file: &'static str,
}

lazy_static! {
    #[derive(Debug)]
    static ref SERVICES : Mutex<HashMap<&'static str, &'static Params>> = Mutex::new( {
         HashMap::new()
    });
}

// OK
pub fn get_params() -> Params {
    let serv_map = &SERVICES.lock().unwrap();

    let cf = serv_map.get("params").unwrap().config_file;
    let verbose = serv_map.get("params").unwrap().verbose;

    Params {
        verbose,
        config_file: cf,
    }
}

pub fn get_params_ref<'lt>() -> &'lt Params {
    let serv_map = &SERVICES.lock().unwrap();
    let pp  = serv_map.get("params").unwrap();
    pp
}

fn my_other_process(my_str : &'static str) {
    let ref_pp = get_params_ref();

    if ref_pp.verbose {
        dbg!(&ref_pp.config_file, my_str);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = Box::new(Params {
        verbose: true,
        config_file: Box::leak(String::from("toto.yml").into_boxed_str()),
    });

    let b_params = Box::<Params>::leak( params );

    {
        SERVICES.lock().unwrap().insert("params", b_params);
    }

    let pp = get_params();

    if pp.verbose {
        dbg!(&pp.config_file, pp.verbose);
    }

    let ref_pp = get_params_ref();

    if ref_pp.verbose {
        dbg!(&ref_pp.config_file, ref_pp.verbose);
    }

    my_other_process(&ref_pp.config_file);

    Ok(())
}