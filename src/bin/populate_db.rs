use std::{fs, path::PathBuf};

use clap::Parser;
use diesel::{r2d2, ExpressionMethods, SqliteConnection, Insertable, insert_into, RunQueryDsl};
use track_notes::schema;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    path: PathBuf,
}

fn main() {
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new("diesel_demo.sqlite");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Database url should be a valid path to a SQLite DB file");

    let mut conn = pool.get().expect("couldnt get db conn from pool");

    let cli = Cli::parse();

    let contents = fs::read_to_string(cli.path).unwrap();
    let rows = contents.lines().skip(3);

    let mut inserts = vec![];
    use schema::foods::dsl;

    for r in rows {
        let cells: Vec<&str> = r.split(";").map(|x| x.trim()).collect();
        println!("{} {}", cells[3], cells[6]);
        let name = cells[0][1..cells[0].len() - 1].to_owned();
        let cals: f32 = cells[3].parse().unwrap();
        let protein: f32 = cells[6].parse().unwrap();
        inserts.push((
            dsl::name.eq(name),
            dsl::calories.eq((cals * 100.0) as i32),
            dsl::protein.eq((protein * 100.0) as i32),
        ));
    }

    insert_into(dsl::foods).values(inserts).execute(&mut conn).unwrap();

    println!("Populating db");
}
