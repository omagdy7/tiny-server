use regex::{escape, Regex};

pub fn build_regex_from_path(path_template: &str) -> Regex {
    // Escape literal parts of the path to safely convert to regex
    let mut regex_string = "^".to_string();
    for component in path_template.split('/') {
        if component.starts_with(':') {
            // Replace placeholder with regex to capture alphanumeric, underscores, or hyphens
            regex_string.push_str("/([a-zA-Z0-9_-]+)");
        } else if !component.is_empty() {
            // Escape and add literal components to the regex
            regex_string.push('/');
            regex_string.push_str(&escape(component));
        }
    }
    regex_string.push_str("/?$");

    // Compile the regex
    Regex::new(&regex_string).unwrap()
}

// pub fn match_path(method: &Method) -> String {
//     use Method::*;
//     match method {
//         Get(route) => {
//             match re.captures(route) {
//                 Some(caps) => {
//                     println!("Matched route: {}", route);
//                     // Iterate over the captures to extract the path segments and parameters
//                     for cap in caps.iter().flatten().skip(1) {
//                         println!("Segment: {}", cap.as_str());
//                     }
//                     "empty".to_string()
//                 }
//                 None => {
//                     println!("No match for route: {}", route);
//                     "empty none".to_string()
//                 }
//             }
//         }
//         Post(_) => todo!(),
//         Put(_) => todo!(),
//     }
// }
