# Dia 17/07/2021 - Em busca da lib

Olá pessoal tudo bem com todos ? Espero que sim!

Bem hoje eu passei o dia atras de libs para usar o Postgres no rust, tem algumas que eu conheço, mas não tenho muito conhecimento com elas então fui estudar um pouco em como usa-las.

## Postgres

Essa é a lib padrão para se usar mas quando tentei usa-la junto do [rocket](https://github.com/SergioBenitez/Rocket), que é o meu servidor http, houve um conflito de libs, ainda estou atras disso para resolver, mas ela é simples de utilizar, até por que é a padrão.

```rust
use postgres::{Client, NoTls};

let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

client.batch_execute("
    CREATE TABLE person (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        data    BYTEA
    )
")?;
```

[Link da lib](https://docs.rs/postgres/0.19.1/postgres/)

## Tokio Postgres

Após ler um pouco sobre, descobri que a própria lib Postgres usa ela internamente, mas caso seja usada ela te dá uma liberdade a mais para manipulação multithread.

Ele usa do [tokio](https://docs.rs/tokio/1.8.1/tokio/), para manipulação das threads.

```rust
use tokio_postgres::{NoTls, Error};

let (client, connection) = tokio_postgres::connect(
    "host=localhost user=postgres",
    NoTls
).await?;

tokio::spawn(async move {
    if let Err(e) = connection.await {
        eprintln!("connection error: {}", e);
    }
});

let rows = client
    .query("SELECT $1::TEXT", &[&"hello world"])
    .await?;
```

[Link da lib](https://docs.rs/tokio-postgres/0.7.2/tokio_postgres/)

## Diesel Postgres

Existe também uma forma usando um ORM em rust que é o [diesel](https://github.com/diesel-rs/diesel), ele aceita vários bancos de dados, ainda não tentei utilizar ele pois tem alguns passos a mais para poder ser usado, por que a maior parte dele depende de executar comandos CLI, so que no Windows nativamente eu não consegui por falta das libs do postgres.

E também não achei exemplos de como usá-lo, mas minha busca continua, continuarei atras de como farei a utilização do banco de dados no meu projeto.

Bem por hoje é so, caso alguém tenha alguma dica do que eu deva utilizar, mande uma mensagem na aba [Discussion](https://github.com/Gabriel-Paulucci/UmDiaProgramando/discussions)
