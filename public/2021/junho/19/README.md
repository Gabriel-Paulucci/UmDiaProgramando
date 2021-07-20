# Dia 19/07/2021 - Mudando a lib e criando users

Após alguns testes que fiz ontem tentando utilizar o rocket_db_pool, notei que eu não conseguia criar o banco de dados da maneira que eu queria, pois eu não conseguia a conexão inicial, então fui para a segunda opção que o takasaki me deu que foi usando o [Rbatis](https://github.com/rbatis/rbatis), então fiz a configuração inicial.

```rust
use rbatis::{executor::Executor, rbatis::Rbatis};

use crate::types::GenericResult;

lazy_static! {
    static ref DATABASE: Rbatis = Rbatis::new();
}

pub async fn init_rbatis() -> GenericResult<()> {
    DATABASE
        .link("postgres://<user>:<pass>@<host>/<db>")
        .await?;

    DATABASE.exec(crate_structure(), &vec![]).await?;

    Ok(())
}

fn crate_structure() -> &'static str {
    r"
    CREATE TABLE IF NOT EXISTS users (
        id BIGSERIAL NOT NULL PRIMARY KEY,
        username VARCHAR(50) NOT NULL,
        password VARCHAR(128) NOT NULL,
        create_at TIMESTAMP DEFAULT now()
    );
    "
}
```

Com isso consegui ter uma conexão com o banco de dados na inicialização do banco de dados.

Foi mais facil do que a maneira anterior é o rbatis também é um ORM, que facilita para fazer os comandos SQL.

Fiz um simples so para testar fazer o registro do usuario no meu banco de dados.

```rust
use chrono::{DateTime, Utc};
use rbatis::crud::{Skip, CRUD};

use crate::{database::get_database, errors::DbError};

#[crud_table(table_name: "users")]
pub struct User {
    pub id: Option<u64>,
    pub username: String,
    pub password: String,
    pub create_at: Option<DateTime<Utc>>,
}

impl User {
    pub async fn register(&self) {
        let result = || async move {
            let db = get_database();
            let result = db
                .save(self, &[Skip::Column("id"), Skip::Column("create_at")])
                .await
                .map_err(|err| DbError::FailCreate(err.into()))?;

            Ok(result.rows_affected) as Result<u64, DbError>
        };

        if let Err(DbError::FailCreate(err)) = result().await {
            println!("{:?}", err)
        };
    }
}
```

Bem estou evoluindo com o projeto, como eu estou fazendo com calma pois estou resolvendo coisas relacionado a trabalho esse projeto estou fazendo um pouco devagar mas espero ter a parte de login pelo menos até o fim dessa semana.

Por hoje é isso pessoal, espero que tenham uma boa noite é obrigado pela atenção, bye bye.
