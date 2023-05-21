pub const APARTMENT: i32 = 1;

pub fn requires_floor(kind_id: i32) -> bool {
    kind_id == APARTMENT
}
