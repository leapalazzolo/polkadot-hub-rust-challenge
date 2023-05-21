// @generated automatically by Diesel CLI.

diesel::table! {
    houses (id) {
        id -> Integer,
        street -> Text,
        street_number -> Integer,
        street_floor -> Text,
        postal_code -> Text,
        surface_square_meters -> Integer,
        bathrooms -> Integer,
        rooms -> Integer,
        kind_id -> Integer,
    }
}

diesel::table! {
    houses_kind (id) {
        id -> Integer,
        kind -> Text,
    }
}

diesel::joinable!(houses -> houses_kind (kind_id));

diesel::allow_tables_to_appear_in_same_query!(houses, houses_kind,);
