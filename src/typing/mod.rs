use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Type {
    size: Option<u64>, // If a type is stored on the stack, it will have a size
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "type with size {:?}", self.size)
    }
}

pub static UINT8: LazyLock<Type> = LazyLock::new(|| Type {
    size: Some(1),
});

pub static BUILTIN_TYPES: LazyLock<HashMap<String, Type>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "uint8".to_string(),
        Type {
            size: Some(4),
        },
    );
    map
});
