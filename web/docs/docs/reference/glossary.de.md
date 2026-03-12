# Glossar

## Basisintervall

Das Ausfuehrungsintervall, das mit `interval <...>` deklariert wird.

## Deklariertes Intervall

Ein Intervall, das explizit ueber `use <alias> <...>` fuer eine benannte Quelle
eingefuehrt wird.

## Quellenbewusstes Skript

Ein Skript, das mindestens eine `source` deklariert.

## Quell-Template

Ein eingebauter Exchange- oder Venue-Konstruktor wie `binance.spot` oder
`hyperliquid.perps`.

## Marktmodus

Ausfuehrung gegen historische, marktgestuetzte Quell-Feeds.

## Kein Lookahead

Die Garantie, dass ein Hoeherintervall-Sample erst sichtbar wird, nachdem
diese Kerze vollstaendig geschlossen ist.

## Ausgabeserie

Ein benanntes Ergebnis pro Schritt, das durch `export` oder `trigger`
emittiert wird.

## Vereinigung Der Basis-Zeitstempel

Die Ausfuehrungs-Zeitleiste, die aus der Vereinigung aller Basisintervall-
Oeffnungszeiten der deklarierten Quellen aufgebaut wird.
