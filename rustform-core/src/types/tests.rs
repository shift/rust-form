use crate::types::{GeneratedFile, GeneratedProject};
use rstest::*;
use serde_json;

#[rstest]
fn test_generated_file_creation() {
    let file = GeneratedFile {
        path: "src/main.rs".to_string(),
        content: "fn main() { println!(\"Hello, world!\"); }".to_string(),
    };

    assert_eq!(file.path, "src/main.rs");
    assert!(file.content.contains("Hello, world!"));
}

#[rstest]
fn test_generated_file_serialization() {
    let file = GeneratedFile {
        path: "src/models.rs".to_string(),
        content: "#[derive(Debug)] pub struct User { pub id: i32 }".to_string(),
    };

    let json = serde_json::to_string(&file).expect("Should serialize to JSON");
    let deserialized: GeneratedFile =
        serde_json::from_str(&json).expect("Should deserialize from JSON");

    assert_eq!(file.path, deserialized.path);
    assert_eq!(file.content, deserialized.content);
}

#[rstest]
fn test_generated_project_creation() {
    let files = vec![
        GeneratedFile {
            path: "src/main.rs".to_string(),
            content: "fn main() {}".to_string(),
        },
        GeneratedFile {
            path: "Cargo.toml".to_string(),
            content: "[package]\nname = \"test\"".to_string(),
        },
    ];

    let project = GeneratedProject {
        name: "test_project".to_string(),
        files,
    };

    assert_eq!(project.name, "test_project");
    assert_eq!(project.files.len(), 2);
    assert_eq!(project.files[0].path, "src/main.rs");
    assert_eq!(project.files[1].path, "Cargo.toml");
}

#[rstest]
fn test_generated_project_serialization() {
    let project = GeneratedProject {
        name: "serialization_test".to_string(),
        files: vec![GeneratedFile {
            path: "src/lib.rs".to_string(),
            content: "// Library code".to_string(),
        }],
    };

    let json = serde_json::to_string(&project).expect("Should serialize to JSON");
    let deserialized: GeneratedProject =
        serde_json::from_str(&json).expect("Should deserialize from JSON");

    assert_eq!(project.name, deserialized.name);
    assert_eq!(project.files.len(), deserialized.files.len());
    assert_eq!(project.files[0].path, deserialized.files[0].path);
    assert_eq!(project.files[0].content, deserialized.files[0].content);
}

#[rstest]
fn test_generated_file_clone() {
    let original = GeneratedFile {
        path: "test.rs".to_string(),
        content: "test content".to_string(),
    };

    let cloned = original.clone();
    assert_eq!(original.path, cloned.path);
    assert_eq!(original.content, cloned.content);
}

#[rstest]
fn test_generated_project_clone() {
    let original = GeneratedProject {
        name: "clone_test".to_string(),
        files: vec![GeneratedFile {
            path: "file1.rs".to_string(),
            content: "content1".to_string(),
        }],
    };

    let cloned = original.clone();
    assert_eq!(original.name, cloned.name);
    assert_eq!(original.files.len(), cloned.files.len());
    assert_eq!(original.files[0].path, cloned.files[0].path);
}

mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_generated_file_property(
            path in "[a-zA-Z0-9/_.-]{1,100}",
            content in ".*"
        ) {
            let file = GeneratedFile {
                path: path.clone(),
                content: content.clone(),
            };

            prop_assert_eq!(file.path.clone(), path);
            prop_assert_eq!(file.content.clone(), content);

            let serialized = serde_json::to_string(&file).expect("Failed to serialize");
            let deserialized: GeneratedFile = serde_json::from_str(&serialized).expect("Failed to deserialize");

            prop_assert_eq!(file.path, deserialized.path);
            prop_assert_eq!(file.content, deserialized.content);
        }

        #[test]
        fn test_generated_project_property(
            name in "[a-zA-Z][a-zA-Z0-9_-]{0,49}",
            file_count in 0usize..10
        ) {
            let files: Vec<GeneratedFile> = (0..file_count).map(|i| {
                GeneratedFile {
                    path: format!("file{}.rs", i),
                    content: format!("content{}", i),
                }
            }).collect();

            let project = GeneratedProject {
                name: name.clone(),
                files,
            };

            prop_assert_eq!(&project.name, &name);
            prop_assert_eq!(project.files.len(), file_count);

            // Test serialization roundtrip
            let json = serde_json::to_string(&project).unwrap();
            let deserialized: GeneratedProject = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(&project.name, &deserialized.name);
            prop_assert_eq!(project.files.len(), deserialized.files.len());
        }
    }
}
