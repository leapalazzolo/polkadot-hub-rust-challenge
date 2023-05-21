use diesel::{Insertable, Queryable, Selectable};

use crate::schema::houses;
use crate::schema::houses_kind;
use crate::utils;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = houses_kind)]
pub struct HouseKind {
    pub id: i32,
    pub kind: String,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = houses)]
#[diesel(belongs_to(HouseKind))]
pub struct House {
    pub id: i32,
    pub street: String,
    pub street_number: i32,
    pub street_floor: String,
    pub postal_code: String,
    pub surface_square_meters: i32,
    pub bathrooms: i32,
    pub rooms: i32,
    pub kind_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = houses)]
#[diesel(belongs_to(HouseKind))]
pub struct NewHouse {
    pub street: String,
    pub street_number: i32,
    pub street_floor: String,
    pub postal_code: String,
    pub surface_square_meters: i32,
    pub bathrooms: i32,
    pub rooms: i32,
    pub kind_id: i32,
}

#[derive(Debug)]
pub struct HouseWithKind {
    pub id: i32,
    pub street: String,
    pub street_number: i32,
    pub street_floor: String,
    pub postal_code: String,
    pub surface_square_meters: i32,
    pub bathrooms: i32,
    pub rooms: i32,
    pub kind_id: i32,
    pub kind: String,
}

impl ToString for HouseWithKind {
    fn to_string(&self) -> String {
        let address = if utils::requires_floor(self.kind_id) {
            let floor = if self.street_floor != "" {
                format!(" (piso {}", self.street_floor)
            } else {
                format!("")
            };
            format!("{} {}{}", self.street, self.street_number, floor)
        } else {
            format!("{} {}", self.street, self.street_number)
        };
        format!(
            "#{}: {} CP: {}. Con {} baño/s ,{} habitación/es. Tipo \"{}\" ({}m2)",
            self.id,
            address,
            self.postal_code,
            self.bathrooms,
            self.rooms,
            self.kind,
            self.surface_square_meters,
        )
    }
}
