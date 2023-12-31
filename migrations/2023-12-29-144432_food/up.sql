-- Your SQL goes here
CREATE TABLE foods (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    calories INTEGER NOT NULL,
    protein INTEGER NOT NULL,
    CONSTRAINT name_unique UNIQUE (name)
);

CREATE TABLE meals (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    CONSTRAINT name_unique UNIQUE (name)
);

CREATE TABLE meal_food_relations (
    food_id INTEGER NOT NULL,
    meal_id INTEGER NOT NULL,
    PRIMARY KEY(food_id, meal_id)
);
