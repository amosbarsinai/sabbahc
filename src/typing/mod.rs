#[derive(Clone, Debug)]
pub struct Type {
    size: usize,
}
pub static TYPE_INT32: Type = Type {
    size: 32,
};