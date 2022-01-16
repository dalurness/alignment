pub enum Direction {Unset, Left, Up, Diagonal}
pub enum ValueType {Static, Dynamic, Unknown}

pub struct Cell{
    pub value: i64,
    pub direction: Direction,
    pub value_type: ValueType,
}