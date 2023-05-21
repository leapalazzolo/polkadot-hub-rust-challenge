use crate::models::{House, HouseKind, HouseWithKind, NewHouse};
use crate::repository::HouseRepository;
use crate::utils::{self, APARTMENT};

pub struct HouseService {
    pub repository: HouseRepository,
}

impl HouseService {
    pub fn new(house_repository: HouseRepository) -> Self {
        HouseService {
            repository: house_repository,
        }
    }
    pub fn create_house(
        &mut self,
        street: &str,
        street_number: &str,
        street_floor: &str,
        postal_code: &str,
        surface_square_meters: &str,
        bathrooms: &str,
        rooms: &str,
        kind_id: i32,
    ) -> Result<usize, &str> {
        let street_number = street_number
            .parse::<i32>()
            .map_err(|_| "Error convritiendo el número de la calle")?;
        let surface_square_meters = surface_square_meters
            .parse::<i32>()
            .map_err(|_| "Error convritiendo la superficie")?;
        let bathrooms = bathrooms
            .parse::<i32>()
            .map_err(|_| "Error convritiendo los baños")?;
        let rooms = rooms
            .parse::<i32>()
            .map_err(|_| "Error convritiendo las habitaciones")?;
        let street_floor = if kind_id == APARTMENT {
            street_floor
        } else {
            ""
        };
        if kind_id < 0 || kind_id > 2 {
            return Err("Error convirtiendo el tipo de casa");
        }
        let new_house = NewHouse {
            street: street.to_string(),
            street_number,
            street_floor: street_floor.to_string(),
            postal_code: postal_code.to_string(),
            surface_square_meters,
            bathrooms,
            rooms,
            kind_id,
        };
        self.repository
            .create(&new_house)
            .map_err(|_| "Error guardando en la DB")
    }

    pub fn get_houses(&mut self) -> Result<Vec<HouseWithKind>, diesel::result::Error> {
        self.repository.find_all()
    }
    pub fn get_houses_kind(&mut self) -> Result<Vec<HouseKind>, diesel::result::Error> {
        self.repository.find_all_kinds()
    }
    pub fn update_house(
        &mut self,
        id: &str,
        street: &str,
        street_number: &str,
        street_floor: &str,
        postal_code: &str,
        surface_square_meters: &str,
        bathrooms: &str,
        rooms: &str,
        kind_id: i32,
    ) -> Result<usize, &str> {
        let id = id.parse::<i32>().map_err(|_| "Error convritiendo el id")?;
        let street_number = street_number
            .parse::<i32>()
            .map_err(|_| "Error convritiendo el número de la calle")?;
        let surface_square_meters = surface_square_meters
            .parse::<i32>()
            .map_err(|_| "Error convritiendo la superficie")?;
        let bathrooms = bathrooms
            .parse::<i32>()
            .map_err(|_| "Error convritiendo los baños")?;
        let rooms = rooms
            .parse::<i32>()
            .map_err(|_| "Error convritiendo las habitaciones")?;
        let street_floor = if utils::requires_floor(kind_id) {
            street_floor
        } else {
            ""
        };

        let house = House {
            id,
            street: street.to_string(),
            street_number,
            street_floor: street_floor.to_string(),
            postal_code: postal_code.to_string(),
            surface_square_meters,
            bathrooms,
            rooms,
            kind_id,
        };
        self.repository
            .update(house)
            .map_err(|_| "Error guardando en la DB")
    }

    pub fn delete_house(&mut self, id: i32) -> Result<usize, diesel::result::Error> {
        self.repository.delete(id)
    }
}
