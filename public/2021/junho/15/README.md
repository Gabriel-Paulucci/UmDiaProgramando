# Dia 15/07/2021 - Express

Olá pessoal tudo bem com todos ?

Hoje um amigo, meu veio pedir uma ajuda de como fazer um back end em node, pois ele nunca tinha feito isso na vida, então pensei em mostrar a todos um exemplo de como fazer uma api em Node, a lib mais simples que eu conheço para isso é o [Express](https://github.com/expressjs/express) então vou usar ele para exemplo.

Como sempre com node iniciamos um projeto simples, e para fazermos esse projeto vamos adicionar uma biblioteca que é o express

```cmd
npm i express
```

E bem agora fazer uma mini API simples que retorna um ola so que com o seu nome

```js
const express = require('express')

const app = express()

app.get('/:nome', (req, res) => {
    const nome = req.params.nome;

    res.send('Olá ' + nome)
})

app.listen(8000)
```

Bem caso seja chamado no navegador `http://localhost:8000/Gabriel` eu vou receber `Olá Gabriel`,
bem isso é so um exemplo bem simples, da para fazer coisas muito mais complexas, como receber Arquivos, acessar o banco de dados, e tudo mais.

E é isso, uma maneira facil de fazer uma mini API usando Express.

Por hoje é so isso, bye bye.
