# Dia 18/07/2021 - Config inicial da DB

Olá a todos, boa noite pessoas lindas tudo bem com todos ?

Hoje eu peguei pra fazer a configuração inicial do meu banco de dados, após um amigo meu me explicar como fazer essa configuração inicial na aba [Discussions](https://github.com/Gabriel-Paulucci/UmDiaProgramando/discussions/8), agradeço muito a ele.

Para fazer isso eu escolhei a primeira opção, usando o [rocket_sync_db_pools](https://api.rocket.rs/master/rocket_sync_db_pools/index.html).

Fiz a importação do pacote com a feature para usar o postgres, e após isso fiz a criação da struct para usar o banco de dados.

```rust
use rocket::fairing::{Fairing, Info, Kind};
use rocket_sync_db_pools::{database, postgres};

// deve ser passado o mesmo nome que foi deixado na config
#[database("postgres_db")]
pub struct AppDatabase(postgres::Client);

impl Fairing for AppDatabase {
    fn info(&self) -> rocket::fairing::Info {
        // aqui tenho acesso para fazer a criação inicial da estrutura
        Info {
            name: "'postgres_db' Database Pool",
            kind: Kind::Ignite,
        }
    }
}
```

Para o banco de dados fazer a conexão a maneira mais facil é definindo no arquivo de configuração do rocket.

```toml
# Rocket.toml
[default]
address = "0.0.0.0"
port = 8000

[global.databases]
# o `postgres_db` é um nome a sua escolha
postgres_db = { url = "postgres://<user>:<pass>@<host>/<dbname>" }
```

E para finalizar fiz a vinculação de todo essa estrutura com o rocket no momento da construção do rocket.

```rust
use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().attach(AppDatabase::fairing());
}
```

Com isso eu tenho a conexão com o banco de dados e posso construí-lo caso não exista a estrutura, logo vou trazer mais exemplo de como utilizar esse método, mas agradeço ao Takasaki por ter me ajudado com essa parte.

Bem por hoje é isso, estarei criando a estrutura inicial para registrar os usuários agora, boa noite a todos e muito obrigado pela atenção.
