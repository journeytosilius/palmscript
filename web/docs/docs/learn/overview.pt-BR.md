# Aprenda PalmScript

A documentacao publica do PalmScript esta organizada em torno de:

- a linguagem para escrever estrategias
- exemplos que mostram como os scripts sao escritos e usados

## O Que Voce Faz Com PalmScript

Fluxo tipico:

1. escrever um script `.ps`
2. declarar um `interval` base
3. declarar um ou mais bindings `source`
4. valida-lo no IDE do navegador
5. executa-lo sobre dados historicos na app

## Otimizacoes Longas

Para jobs longos de tuning pela CLI:

- use `palmscript run optimize ...` quando quiser o resultado em primeiro plano
- use `palmscript runs submit optimize ...` quando a busca deve continuar em estado local duravel e salvar cada candidato concluido
- volte depois com `palmscript runs status <run-id>`, `palmscript runs show <run-id>`, `palmscript runs tail <run-id>` ou `palmscript runs best <run-id> --preset-out best.json`

## O Que Ler Depois

- Primeiro fluxo executavel: [Inicio Rapido](quickstart.md)
- Primeiro walkthrough completo de estrategia: [Primeira Estrategia](first-strategy.md)
- Visao geral da linguagem: [Visao Geral Da Linguagem](language-overview.md)
- Regras e semantica exatas: [Visao Geral Da Referencia](../reference/overview.md)

## Papeis Da Documentacao

- `Aprenda` explica como usar PalmScript de forma eficaz.
- `Referencia` define o que PalmScript significa.
