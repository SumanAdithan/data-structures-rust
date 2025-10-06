use std::collections::HashMap;
use serde_json::Value;

fn main() {
    let mut obj: HashMap<String, Value> = HashMap::new();
    obj.insert("name".to_string(), Value::String("hello".to_string()));
    obj.insert("age".to_string(), Value::Number((25).into()));
    obj.insert("key-three".to_string(), Value::Bool(true));

    obj.insert("hobby".to_string(), Value::String("football".to_string()));
    obj.remove("hobby");

    println!("{}", obj["name"]);
    if let Some(Value::Num(age)) = obj.get("age") {
        println!("Age: {}", age);
    }
    println!("{:?}", obj);

    // Iterate over key-value pairs
    for (key, value) in &obj {
        println!("{} => {}", key, value);
    }

    // Object.keys()
    let keys: Vec<&String> = obj.keys().collect();
    println!("keys = {:?}", keys);

    // Object.values()
    let values: Vec<&Value> = obj.values().collect();
    println!("values = {:?}", values);

    // Object.entries()
    let entries: Vec<(&String, &Value)> = obj.iter().collect();
    println!("entries = {:?}", entries);
}
