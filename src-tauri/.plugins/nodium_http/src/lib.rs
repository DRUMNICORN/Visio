// use notime::{Extension, Node, Tab, Service, Message, Event};
// use reqwest::blocking::Client;

// struct HttpExtension;

// impl Extension for HttpExtension {
//     fn nodes(&self) -> Vec<Box<dyn Node>> {
//         vec![Box::new(HttpRequestNode)]
//     }

//     fn tabs(&self) -> Vec<Box<dyn Tab>> {
//         vec![]
//     }

//     fn services(&self) -> Vec<Box<dyn Service>> {
//         vec![]
//     }
// }

// struct HttpRequestNode;

// impl Node for HttpRequestNode {
//     fn process_event(&self, event: &Event) -> Option<Event> {
//         if let Some(url) = event.payload().get("url").and_then(|v| v.as_str()) {
//             let client = Client::new();
//             let response = client.get(url).send().unwrap();
//             let html_text = response.text().unwrap();
//             let output_event = Event::new("output", json!({
//                 "html_text": html_text,
//             }));
//             Some(output_event)
//         } else {
//             None
//         }
//     }

//     fn inputs(&self) -> Vec<String> {
//         vec!["url".to_string()]
//     }

//     fn outputs(&self) -> Vec<String> {
//         vec!["output".to_string()]
//     }
// }

// fn main() {
//     let extension = HttpExtension;
//     let mut nodes = vec![];

//     // Register the nodes with the list
//     for node in extension.nodes() {
//         nodes.push(node.clone());
//     }

//     // List the registered nodes
//     for node in nodes {
//         debug!("Registered node: {:?}", node);
//     }
// }
