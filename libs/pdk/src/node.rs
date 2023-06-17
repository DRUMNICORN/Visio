use std::collections::HashMap;

pub struct NodiumNode {
    pub name: String,
    pub description: String,
    pub input_params: HashMap<String, String>,
    pub output_params: HashMap<String, String>,
    // Other fields
}

impl NodiumNode {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            input_params: HashMap::new(),
            output_params: HashMap::new(),
        }
    }

    pub fn add_input_param(&mut self, key: &str, value: &str) {
        self.input_params.insert(String::from(key), String::from(value));
    }

    pub fn add_output_param(&mut self, key: &str, value: &str) {
        self.output_params.insert(String::from(key), String::from(value));
    }

    // Process the input parameters and generate output parameters
    pub fn process(&mut self) {
        // Implement the logic for processing the input parameters and generating the output parameters
        // For example, you could concatenate all input parameters
        let output_value = self.input_params.values().cloned().collect::<Vec<String>>().join(", ");
        self.output_params.insert(String::from("output"), output_value);
    }
}