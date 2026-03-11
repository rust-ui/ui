use tw_merge::merge::{MergeOptions, tw_merge_options, tw_merge_override};

#[test]
fn test_collisions() {
    pub fn tw_merge(class: &str) -> String {
        tw_merge_override(&[class], Default::default(), collision_id_fn, get_collisions)
    }

    pub fn collision_id_fn(elements: &[&str], _: Option<&str>) -> Option<&'static str> {
        match elements {
            ["bg", "red", _] => Some("america"),
            ["bg", "white"] => Some("america"),
            ["bg", "blue", _] => Some("america"),
            ["flex"] => Some("florida"),
            _ => None,
        }
    }

    pub fn get_collisions(collision_id: &str) -> Option<Vec<&'static str>> {
        match collision_id {
            "florida" => Some(vec!["america"]),
            _ => None,
        }
    }

    let class = "bg-red-500 bg-black";
    let result = tw_merge(class);
    assert_eq!(class, result);

    let result = tw_merge("bg-white bg-red-500");
    assert_eq!("bg-red-500", result);

    let result = tw_merge("bg-white flex");
    assert_eq!("flex", result);
}

#[test]
fn test_override_config() {
    let config = MergeOptions { prefix: "tw-", separator: "|" };

    let class = "hover|lg|tw-bg-blue-100 hover|lg|tw-bg-red-500";
    let result = tw_merge_options(class, config);
    assert_eq!("hover|lg|tw-bg-red-500", result);

    let class = "tw-bg-blue-100 bg-red-500";
    let result = tw_merge_options(class, config);
    assert_eq!(class, result, "No conflict because non-prefix is not considered tailwind class")
}
