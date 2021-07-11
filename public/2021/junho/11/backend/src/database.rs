use rbatis::{executor::Executor, rbatis::Rbatis};

use crate::errors::DatabaseError;

lazy_static::lazy_static! {
    pub static ref DATABASE: Rbatis = Rbatis::new();
}

pub async fn init_database() -> Result<(), DatabaseError> {
    DATABASE
        .link("sqlite://database.db")
        .await
        .map_err(|why| DatabaseError::ConnectionFailed(why.into()))?;

    DATABASE
        .exec(
            r"
        create table if not exists user (
            username varchar(32) not null,
            password varchar(255) not null,
            jwt varchar(255) not null,
            primary key(username)
        );

        create index if not exists jwt on user(jwt);

        insert or ignore into user values ('vulcan', '123', 'NULL');
        insert or ignore into user values ('takasaki', '123', 'NULL');
        insert or ignore into user values ('shiba', '123', 'NULL');
        insert or ignore into user values ('pitas', '123', 'NULL');
        insert or ignore into user values ('fardragi', '123', 'NULL');
    ",
            &vec![],
        )
        .await
        .map_err(|why| DatabaseError::CreateDatabaseFailed(why.into()))?;

    Ok(())
}

#[crud_table(table_name: user)]
pub struct Usuario {
    pub username: String,
    pub password: String,
    pub jwt: Option<String>,
}
