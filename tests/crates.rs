use reqwest::{blocking::Client,StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_crates(){
    //setup
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);
    let b_crate = common::create_test_crate(&client, &rustacean);
    

    //test
    let response = client.get(format!("{}/crates", common::APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json:Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&a_crate));
    assert!(json.as_array().unwrap().contains(&b_crate));

    //clean
    common::delete_test_crate(&client, a_crate);
    common::delete_test_crate(&client, b_crate);
    common::delete_test_rustacean(&client, rustacean);
}


#[test]
fn test_create_crate(){
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    let response = client.post(format!("{}/crates", common::APP_HOST))
    .json(&json!({
        "rustacean_id": rustacean["id"],
        "code":"1999",
        "name":"Foo crate",
        "version":"0.1",
        "description":"1999 Foo crate description"
    }))
    .send()
    .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id":a_crate["id"],
        "code":"1999",
        "rustacean_id": rustacean["id"],
        "name":"Foo crate",
        "version":"0.1",
        "description":"1999 Foo crate description",
        "created_at":a_crate["created_at"]
    }));

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}


#[test]
fn test_view_crate(){
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client.get(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
    .send()
    .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id":a_crate["id"],
        "code":"1999",
        "name":"Foo crate",
        "version":"0.1",
        "description":"1999 Foo crate description",
        "rustacean_id": rustacean["id"],
        "created_at":a_crate["created_at"]  
    }));

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate(){
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client.put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
    .json(&json!({
        "code":"2000",
        "name":"updated crate",
        "version":"0.2",
        "description":"2000 updated crate description",
        "rustacean_id": rustacean["id"]
    }))
    .send()
    .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id":a_crate["id"],
        "code":"2000",
        "name":"updated crate",
        "version":"0.2",
        "description":"2000 updated crate description",
        "rustacean_id": rustacean["id"],
        "created_at":a_crate["created_at"]  
    }));

    // Test author switch
    let rustacean2 = common::create_test_rustacean(&client);
    let response = client.put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
    .json(&json!({
        "code":"2000",
        "name":"updated crate",
        "version":"0.2",
        "description":"2000 updated crate description,2000 updated crate description,2000 updated crate description,2000 updated crate description,2000 updated crate description",
        "rustacean_id": rustacean2["id"]
    }))
    .send()
    .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id":a_crate["id"],
        "code":"2000",
        "name":"updated crate",
        "version":"0.2",
        "description":"2000 updated crate description,2000 updated crate description,2000 updated crate description,2000 updated crate description,2000 updated crate description",
        "rustacean_id": rustacean2["id"],
        "created_at":a_crate["created_at"]  
    }));
  

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_crate(){
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client.delete(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
    .send()
    .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
   
    common::delete_test_rustacean(&client, rustacean);
}