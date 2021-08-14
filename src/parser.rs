use serde_json::Value;

#[derive(Debug)]
struct Circuit {
    name: String,
    inputs: Vec<(String, u64)>,
    outputs: Vec<(String, u64)>,
    command: String,
    meaning: (Vec<String>, Vec<u64>),
    output_values: Vec<i32>
}

impl Circuit {
    // pub fn new(
    //     name: String,
    //     inputs: Vec<(String, u64)>,
    //     outputs: Vec<&'a str>,
    //     command: String,
    //     meaning: &'a str,
    //     output_values: Vec<i32>
    // ) -> Circuit<'a> {
    //     return Circuit { name, inputs, outputs, command, meaning, output_values }
    // }
    pub fn empty() -> Circuit {
        return Circuit { 
            name: "".to_string(), 
            inputs: vec![], 
            outputs: vec![], 
            command: "".to_string(), 
            meaning: (vec![], vec![]),
            output_values: vec![0]
        }
    }
}

pub fn parse(file: &str) {
    let mut circuits: Vec<Circuit> = vec![];
    // Parse Json
    let json: Value = serde_json::from_str(file).unwrap();

    match json {
        Value::Object(map) => {
            for (k,x) in map {
                let mut new_circuit = Circuit::empty();
                new_circuit.name = k;
                new_circuit.inputs = get_io(&x, "in");
                new_circuit.outputs = get_io(&x, "out");
                new_circuit.command = x["cmd"].as_str().unwrap().to_string();

                circuits.push(new_circuit);
            }
        },
        _ => (),
    }

    println!("{:#?}", circuits);
} 

fn get_io(x: &Value, port_type: &str) -> Vec<(String, u64)> {
    match &x[port_type] {
        Value::Object(map2) => {
            let mut output = vec![];
            for (l, c) in map2 {
                output.push((l.clone(), c.as_u64().unwrap()));
            }
            output
        },
        Value::Array(v) => {
            let mut output = vec![];
            for i in v.iter() {
                output.push((i.as_str().unwrap().to_string(), 1));
            }
            output
        }
        _ if port_type == "out" => panic!("INVALID OUTPUT {}", x[port_type]),
        _ if port_type == "in" => panic!("INVALID INPUT {}", x[port_type]),
        _ => panic!("unknown type {}", port_type)
    }
}

fn _replace_terms(input: String) -> String {
    let mut _checked = "".to_string();
    let mut index = 0;
    while index != input.len() {
        

        index += 1;
    }

    return input
}

