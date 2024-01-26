use iso_20022_macros::AsjsonRequired;

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use super::*;

    #[derive(AsjsonRequired)]
    pub struct MyStruct {
        pub field_one: Option<String>,
        pub field_two: String,
        pub field_other: u64,
        pub eee: MyStructTwo,
    }

    #[derive(Serialize)]
    pub struct MyStructTwo {
        pub algodon: String,
        pub ttt: Option<String>,
    }

    #[test]
    fn test_to_json_required() {
        let my_struct = MyStruct {
            field_one: Some(String::new()),
            field_two: "Plomom".to_string(),
            field_other: 10000,
            eee: MyStructTwo {
                algodon: "tomates".to_string(),
                ttt: Some("pelo".to_string()),
            },
        };

        let result = my_struct.to_json_required();

        println!("{:#}", result);

        // let expected_result: Value = serde_json::from_str(r#"{"field_one":{}}"#).unwrap();
        assert!(false)
        // assert_eq!(result, expected_result);
    }
}
