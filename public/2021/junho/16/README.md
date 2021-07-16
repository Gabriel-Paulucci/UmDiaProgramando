# Dia 16/07/2021 - Links simbólicos

Olá pessoal de boas ?

Eu estava ontem fuçando na internet sobre coisas aleatórias até que descobri a existência de links simbólicos no Windows, ja sabia da existência deles no Linux, mas não sabia que dava pra fazer isso no Windows, e é facil igualmente do no Linux.

Suponhamos que temos um arquivo dentro de uma pasta fotos, e queremos fazer com que uma das fotos tenha um link para a pasta externa, então fazemos.

```cmd
mklink "C:\foto-01.png" "C:\fotos\foto-01.png"
```

Lembrando que o arquivo não foi copiado, mas sim so é um link para o arquivo.

Para fazer isso com pastas você precisa utilizar uma flag o `\D`.

```cmd
mklink \D "C:\outras fotos" "C:\fotos"
```

Com isso será criada uma pasta nova, mas ela é a pasta fotos, ou seja, toda modificação feita em `outras fotos` será feita em `fotos`

Existem outras tags que podem ser usadas, mas essas são as mais usadas.

Eu quis mostrar isso pois achei interessante isso, pois so sabia da existência disso no Linux.

E bem por hoje é isso, desejo um boa noite a todos.
