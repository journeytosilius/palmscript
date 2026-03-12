# Serien Und Indexierung

Serienwerte repraesentieren zeitindizierte Samples mit begrenzter Historie.

## Marktserien-Formen

PalmScript stellt Marktserien nur ueber quellqualifizierte Formen bereit:

```palmscript
bn.close
hl.1h.close
```

Regeln:

- `<alias>.<field>` verweist auf diese Quelle im Basisintervall des Skripts
- `<alias>.<interval>.<field>` verweist auf diese Quelle im benannten Intervall
- nackte Marktbezeichner wie `close` werden abgelehnt
- Referenzen auf hoehere Quellintervalle erfordern eine passende
  `use <alias> <interval>`-Deklaration

## Aktuelle-Sample-Semantik

Wenn eine Serie ohne Indexierung verwendet wird, evaluiert der Ausdruck zum
aktuellen Sample dieser Serie auf dem aktuellen Ausfuehrungsschritt.

## Indexierung

Indexierung hat die Form:

```palmscript
x[n]
```

Regeln:

- `n` muss ein nicht-negatives Integer-Literal sein
- dynamische Indexierung wird abgelehnt
- nur Serienwerte duerfen indiziert werden
- `x[0]` verweist auf das aktuelle Sample
- `x[1]` verweist auf das vorherige Sample
- `x[n]` verweist auf das Sample von vor `n` Updates auf dem eigenen
  Aktualisierungstakt dieser Serie

Wenn nicht genug Historie vorhanden ist, evaluiert der indizierte Ausdruck zu
`na`.

## Besitz Des Aktualisierungstakts

Jede Serie schreitet auf ihrem eigenen Aktualisierungstakt voran.

Beispiele:

- `bn.close[1]` folgt dem Basisintervall
- `hl.1h.close[1]` folgt Quelle `hl` auf dem Ein-Stunden-Takt

Abgeleitete Serien uebernehmen die Aktualisierungstakte ihrer Eingaben. Eine
langsamere Serie wird auf schnelleren Takten nicht neu gezaehlt, wenn sie nicht
fortgeschritten ist.

## Fehlende Samples

Serien koennen fuer das aktuelle Sample `na` liefern, wenn:

- nicht genug Historie vorhanden ist
- der Quell-Feed auf einem Basis-Taktschritt aus der Vereinigung der
  Basis-Zeitstempel der deklarierten Quellen fehlt
- die Serie ein Hoeherintervall-Feed ist, der noch kein einziges Mal geschlossen
  hat
- ein Indikator noch in der Aufwaermphase ist

## Zeitserien

`time` ist eine numerische Serie, deren Sample die Kerzen-Oeffnungszeit in
Unix-Millisekunden UTC ist.

Regeln:

- Basis-`time` stellt die Oeffnungszeit der Basisintervall-Kerze bereit
- Hoeherintervall-`time` stellt die Oeffnungszeit dieser Hoeherintervall-Kerze
  bereit
- quellqualifiziertes `time` folgt denselben Quell- und Intervall-
  Selektionsregeln wie Preis- und Volumenfelder
