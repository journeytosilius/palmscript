# Aprende PalmScript

La documentacion publica de PalmScript se organiza alrededor de:

- el lenguaje para escribir estrategias
- ejemplos que muestran como se escriben y usan los scripts

## Que Haces Con PalmScript

Flujo tipico:

1. escribir un script `.ps`
2. declarar un `interval` base
3. declarar uno o mas bindings `source`
4. validarlo en el IDE del navegador
5. ejecutarlo sobre datos historicos en la app

## Optimizaciones Largas

Para trabajos largos de tuning por CLI:

- usa `palmscript run optimize ...` cuando quieras el resultado en primer plano
- usa `palmscript runs submit optimize ...` cuando la busqueda deba seguir en estado local durable y guardar cada candidato completado
- vuelve luego con `palmscript runs status <run-id>`, `palmscript runs show <run-id>`, `palmscript runs tail <run-id>` o `palmscript runs best <run-id> --preset-out best.json`

## Que Leer Despues

- Primer flujo ejecutable: [Inicio Rapido](quickstart.md)
- Primer recorrido completo de estrategia: [Primera Estrategia](first-strategy.md)
- Vista general del lenguaje: [Resumen Del Lenguaje](language-overview.md)
- Reglas y semantica exactas: [Resumen De Referencia](../reference/overview.md)

## Roles De La Documentacion

- `Aprende` explica como usar PalmScript de forma efectiva.
- `Referencia` define que significa PalmScript.
