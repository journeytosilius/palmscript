# Semantica De Avaliacao

Esta pagina define como expressoes e instrucoes PalmScript sao avaliadas em
runtime.

## Modelo De Execucao

PalmScript compila um script uma vez e o avalia uma vez por passo do clock
base.

Em cada passo:

1. o runtime materializa as amostras atuais de series de mercado para o passo
2. feeds de intervalos mais lentos avancam apenas se seus candles ja tiverem
   fechado completamente naquele passo
3. o programa em bytecode e executado
4. saidas `plot`, `export` e `trigger` sao coletadas para o passo

Diferentes feeds em modo mercado podem construir as entradas do passo de formas
diferentes, mas a avaliacao de expressoes e a mesma depois que o passo comeca.

## Categorias De Expressoes

Expressoes se avaliam como a amostra atual de um valor escalar ou de serie.

Para uma expressao de serie:

- o resultado da expressao em um passo e uma unica amostra atual
- a indexacao endereca amostras anteriores no proprio clock de atualizacao
  daquela expressao

## Precedencia De Operadores

PalmScript avalia operadores nesta ordem, da menor para a maior precedencia:

1. `or`
2. `and`
3. `==`, `!=`, `<`, `<=`, `>`, `>=`
4. `+`, `-`
5. `*`, `/`
6. unario `-`, unario `!`
7. postfixes de chamada, indexacao e qualificacao

Operadores do mesmo nivel de precedencia associam da esquerda para a direita.

## Aritmetica

Operadores aritmeticos sao `+`, `-`, `*` e `/`.

Regras:

- ambos os operandos devem ser numericos, series numericas ou `na`
- se qualquer operando for `na`, o resultado e `na`
- se qualquer operando for `series<float>`, o resultado e `series<float>`
- caso contrario, o resultado e `float`

## Comparacoes

Operadores de comparacao sao `==`, `!=`, `<`, `<=`, `>`, `>=`.

Regras:

- `<`, `<=`, `>`, `>=` exigem operandos numericos
- `==` e `!=` sao definidos para quaisquer operandos nao-`na`
- igualdade entre tipos mistos resulta em desigualdade
- se qualquer operando for `na`, o resultado e `na`
- se qualquer operando for uma serie, o resultado e booleano de serie
- caso contrario, o resultado e `bool`

## Operadores Unarios

PalmScript suporta:

- unario `-` para operandos numericos
- unario `!` para operandos booleanos

Regras:

- operadores unarios propagam `na`
- unario `-` sobre `series<float>` produz `series<float>`
- unario `!` sobre `series<bool>` produz `series<bool>`

## Operadores Logicos

`and` e `or` exigem `bool`, `series<bool>` ou `na`.

Eles usam logica deterministica de tres valores:

### `and`

| Left | Right | Result |
| --- | --- | --- |
| `false` | `false` | `false` |
| `false` | `true` | `false` |
| `false` | `na` | `false` |
| `true` | `false` | `false` |
| `true` | `true` | `true` |
| `true` | `na` | `na` |
| `na` | `false` | `false` |
| `na` | `true` | `na` |
| `na` | `na` | `na` |

### `or`

| Left | Right | Result |
| --- | --- | --- |
| `true` | `true` | `true` |
| `true` | `false` | `true` |
| `true` | `na` | `true` |
| `false` | `true` | `true` |
| `false` | `false` | `false` |
| `false` | `na` | `na` |
| `na` | `true` | `true` |
| `na` | `false` | `na` |
| `na` | `na` | `na` |

PalmScript avalia ambos os operandos antes de aplicar o operador logico. A
linguagem nao garante short-circuit, entao expressoes logicas sao analisadas e
executadas eager dentro das regras normais da linguagem.

## Semantica De `if`

`if` e uma forma de instrucao, nao uma forma de expressao.

Regras:

- a condicao deve se avaliar como `bool`, `series<bool>` ou `na`
- `na` em uma condicao `if` e tratado como falso para selecao do ramo
- exatamente um ramo executa em cada passo
- ambos os ramos devem estar presentes sintaticamente porque `else` e
  obrigatorio

## Avaliacao De Funcoes

Funcoes definidas pelo usuario tem corpo de expressao e sao compiladas por
especializacao em vez de dispatch dinamico em runtime.

Regras:

- a quantidade de argumentos deve corresponder a quantidade de parametros
  declarados
- funcoes sao especializadas por tipo de argumento e clock de atualizacao
- grafos de funcoes recursivos e ciclicos sao rejeitados em tempo de
  compilacao
- um corpo de funcao nao pode chamar `plot`

## Regra Sem Lookahead

PalmScript nao deve expor candles de intervalos superiores parcialmente
formados.

Consequencias:

- uma serie de intervalo superior muda apenas depois que aquele candle fecha
  completamente
- indexacao em series de intervalo superior percorre o historico de amostras
  completamente fechadas naquele intervalo superior
- intervalos suplementares source-aware seguem a mesma regra

## Semantica De Helpers Builtin

Formulas de helpers builtin, contratos de indicadores, regras de janela e
comportamento de `na` sao definidos em [Builtins](builtins.md) e na secao
[Indicadores](indicators.md).

Regras:

- helpers builtin seguem os clocks de atualizacao de suas entradas de serie
- saidas de helpers participam de `if`, indexacao e outras chamadas builtin
  pelas mesmas regras de valor e `na` definidas nesta pagina
- `if` continua tratando `na` como falso para selecao de ramo, mesmo quando a
  condicao vem de um helper como `crossover(...)`

## Determinismo

A avaliacao de expressoes e deterministica.

Durante a execucao de estrategia, a semantica da linguagem depende apenas de:

- o programa compilado
- os feeds de entrada preparados
- os limites de VM configurados

Ela nao depende de relogio de parede, acesso ao sistema de arquivos, aleatoriedade
ou acesso a rede.
