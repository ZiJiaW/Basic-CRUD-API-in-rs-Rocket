use crate::schema::hero;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Hero {
    pub id: i32,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="hero"]
pub struct NewHero {
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}
