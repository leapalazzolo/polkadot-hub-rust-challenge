use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::SqliteConnection;

use crate::models::{House, HouseKind, HouseWithKind, NewHouse};
use crate::schema::houses::dsl::*;
use crate::schema::houses_kind::dsl::{houses_kind, id as houses_kind_id};

pub struct HouseRepository {
    pub conn: SqliteConnection,
}

impl HouseRepository {
    pub fn find_all(&mut self) -> Result<Vec<HouseWithKind>, Error> {
        let houses_and_kinds: Vec<(House, HouseKind)> = houses::table()
            .inner_join(houses_kind::table())
            .select((House::as_select(), HouseKind::as_select()))
            .load::<(House, HouseKind)>(&mut self.conn)?;
        let mut houses_with_kind: Vec<HouseWithKind> = vec![];

        for (house, kind) in houses_and_kinds {
            houses_with_kind.push(HouseWithKind {
                id: house.id,
                street: house.street,
                street_number: house.street_number,
                street_floor: house.street_floor,
                postal_code: house.postal_code,
                surface_square_meters: house.surface_square_meters,
                bathrooms: house.bathrooms,
                rooms: house.rooms,
                kind_id: kind.id,
                kind: kind.kind,
            })
        }
        Ok(houses_with_kind)
    }

    pub fn find_all_kinds(&mut self) -> Result<Vec<HouseKind>, Error> {
        houses_kind
            .order(houses_kind_id)
            .load::<HouseKind>(&mut self.conn)
    }

    pub fn create(&mut self, new_house: &NewHouse) -> Result<usize, Error> {
        diesel::insert_into(houses)
            .values(new_house)
            .execute(&mut self.conn)
    }

    pub fn update(&mut self, house: House) -> Result<usize, Error> {
        diesel::update(houses.find(house.id))
            .set((
                id.eq(&house.id),
                street.eq(&house.street),
                street_number.eq(&house.street_number),
                street_floor.eq(&house.street_floor),
                postal_code.eq(&house.postal_code),
                surface_square_meters.eq(&house.surface_square_meters),
                bathrooms.eq(&house.bathrooms),
                rooms.eq(&house.rooms),
                kind_id.eq(&house.kind_id),
            ))
            .execute(&mut self.conn)
    }
    pub fn delete(&mut self, uniq_id: i32) -> Result<usize, Error> {
        diesel::delete(houses.find(uniq_id)).execute(&mut self.conn)
    }
}
