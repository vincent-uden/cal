// @generated automatically by Diesel CLI.

diesel::table! {
    foods (id) {
        id -> Integer,
        name -> Text,
        calories -> Integer,
        protein -> Integer,
    }
}

diesel::table! {
    meal_food_relations (food_id, meal_id) {
        food_id -> Integer,
        meal_id -> Integer,
    }
}

diesel::table! {
    meals (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(foods, meal_food_relations, meals, users,);
