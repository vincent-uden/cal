use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::foods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub calories: i32,
    pub protein: i32,
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::meals)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Meal {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(table_name = crate::schema::meal_food_relations)]
#[diesel(belongs_to(Food))]
#[diesel(belongs_to(Meal))]
#[diesel(primary_key(food_id, meal_id))]
pub struct MealFood {
    pub food_id: i32,
    pub meal_id: i32,
}
