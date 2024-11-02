use msfx::{JSON, Entry};

fn main() {
    let json_data = r#"
        {
            "is_active": true,
            "balance": 1234.5678,
            "profile": {
                "created_date": "2024-11-02",
                "created_time": "14:30:00",
                "last_login": "2024-11-02T14:30:00Z",
                "username": "user123"
            },
            "notes": "This is a test",
            "file_data": "SGVsbG8gd29ybGQ="
        }
    "#;

    let mut json_obj = JSON::deserialize(json_data).unwrap();

    if let Some(entry) = json_obj.get("balance") {
        println!("Balance entry: {:?}", entry);
    }

    if let Some(entry) = json_obj.get("file_data") {
        println!("File data entry: {:?}", entry);
    }

    json_obj.set("notes".to_string(), Entry::String("Updated note".to_string()));

    let serialized = json_obj.serialize().unwrap();
    println!("Serialized JSON: {}", serialized);
}
