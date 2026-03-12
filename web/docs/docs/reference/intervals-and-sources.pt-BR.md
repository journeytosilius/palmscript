# Intervalos E Fontes

Esta pagina define as regras normativas de intervalos e fontes no PalmScript.

## Intervalos Suportados

PalmScript aceita os literais de intervalo listados na
[Tabela De Intervalos](intervals.md). Os intervalos diferenciam maiusculas de
minusculas.

## Intervalo Base

Todo script declara exatamente um intervalo base:

```palmscript
interval 1m
```

O intervalo base define o clock de execucao.

## Fontes Nomeadas

Scripts executaveis declaram uma ou mais fontes nomeadas ligadas a exchanges:

```palmscript
interval 1m
source hl = hyperliquid.perps("BTC")
source bn = binance.spot("BTCUSDT")
use hl 1h

plot(bn.close - hl.1h.close)
```

Regras:

- pelo menos uma declaracao `source` e obrigatoria
- series de mercado precisam ser qualificadas por fonte
- cada fonte declarada contribui com um feed base no intervalo base do script
- `use <alias> <interval>` declara um intervalo adicional para aquela fonte
- `<alias>.<field>` referencia aquela fonte no intervalo base
- `<alias>.<interval>.<field>` referencia aquela fonte no intervalo nomeado
- referencias a intervalos inferiores ao intervalo base sao rejeitadas

## Templates De Source Suportados

PalmScript atualmente suporta estes templates de primeira classe:

- `binance.spot("<symbol>")`
- `binance.usdm("<symbol>")`
- `hyperliquid.spot("<symbol>")`
- `hyperliquid.perps("<symbol>")`

O suporte a intervalos depende do template:

- `binance.spot` aceita todos os intervalos PalmScript suportados
- `binance.usdm` aceita todos os intervalos PalmScript suportados
- `hyperliquid.spot` rejeita `1s` e `6h`
- `hyperliquid.perps` rejeita `1s` e `6h`

Restricoes operacionais de busca tambem dependem do template:

- o REST da Hyperliquid expoe apenas os `5000` candles mais recentes por feed
- o modo mercado rejeita qualquer requisicao de feed da Hyperliquid que exceda
  essa janela de retencao
- feeds da Binance sao paginados internamente e nao possuem o mesmo limite de
  retencao da janela inteira

## Conjunto De Campos De Source

Todos os templates de source sao normalizados para os mesmos campos canonicos
de mercado:

- `time`
- `open`
- `high`
- `low`
- `close`
- `volume`

Regras:

- `time` e o horario de abertura do candle em milissegundos Unix UTC
- campos de preco e volume sao numericos
- campos extras especificos do venue nao sao expostos na linguagem

## Intervalos Iguais, Superiores E Inferiores

PalmScript distingue tres casos para um intervalo referenciado em relacao ao
intervalo base:

- intervalo igual: valido
- intervalo superior: valido se declarado com `use <alias> <interval>`
- intervalo inferior: rejeitado

## Semantica De Runtime

No modo mercado:

- PalmScript busca diretamente dos venues os feeds `(source, interval)`
  necessarios
- a timeline de execucao base e a uniao dos tempos de abertura das barras de
  intervalo base de todas as fontes declaradas
- se uma fonte nao tiver barra base em um passo da timeline, ela contribui com
  `na` nesse passo
- intervalos de fonte mais lentos mantem o ultimo valor totalmente fechado ate
  o proximo limite de fechamento

## Garantia Sem Lookahead

PalmScript nao deve expor um candle de intervalo superior antes que ele esteja
totalmente fechado.

Isso se aplica a intervalos qualificados source-aware como `hl.1h.close`.

## Regras De Alinhamento Em Runtime

Feeds preparados precisam estar alinhados aos seus intervalos declarados.

O runtime rejeita feeds que estejam:

- desalinhados ao limite do intervalo
- fora de ordenacao
- duplicados no mesmo tempo de abertura do intervalo
