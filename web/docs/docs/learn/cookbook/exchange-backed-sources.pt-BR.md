# Cookbook: Fontes Ligadas A Exchanges

Use fontes nomeadas quando a estrategia precisar buscar candles historicos
diretamente de exchanges suportadas.

```palmscript
interval 1m

source bn = binance.spot("BTCUSDT")
source hl = hyperliquid.perps("BTC")
use hl 1h

plot(bn.close)
plot(hl.1h.close)
```

## Teste No IDE Do Navegador

Abra [https://palmscript.dev/app/](https://palmscript.dev/app/), cole o exemplo
no editor e execute-o sobre o historico BTCUSDT disponivel na app.

## O Que Observar

- scripts source-aware precisam usar series de mercado qualificadas por fonte
- `use hl 1h` e obrigatorio antes de `hl.1h.close`
- o script ainda tem um unico `interval` base global
- o runtime resolve cada feed `(source, interval)` necessario antes da
  execucao

Referencia:

- [Intervalos e Fontes](../../reference/intervals-and-sources.md)
