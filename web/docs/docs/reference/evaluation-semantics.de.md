# Auswertungssemantik

Diese Seite definiert, wie PalmScript-Ausdruecke und Anweisungen zur Laufzeit
ausgewertet werden.

## Ausfuehrungsmodell

PalmScript kompiliert ein Skript einmal und wertet es einmal pro
Basis-Taktschritt aus.

Bei jedem Schritt:

1. materialisiert die Runtime die aktuellen Marktserien-Samples fuer diesen
   Schritt
2. schreiten langsamere Intervall-Feeds nur fort, wenn ihre Kerzen bis zu
   diesem Schritt vollstaendig geschlossen sind
3. wird das Bytecode-Programm ausgefuehrt
4. werden `plot`-, `export`- und `trigger`-Ausgaben fuer diesen Schritt
   gesammelt

Unterschiedliche Marktmodus-Feeds koennen die Schritt-Eingaenge unterschiedlich
aufbauen, aber die Ausdrucksauswertung ist identisch, sobald der Schritt
beginnt.

## Ausdruckskategorien

Ausdruecke werden zum aktuellen Sample eines skalaren oder Serienwerts
ausgewertet.

Fuer einen Serienausdruck gilt:

- das Ausdrucksergebnis in einem Schritt ist genau ein aktuelles Sample
- Indexierung adressiert fruehere Samples auf dem eigenen Aktualisierungstakt
  dieses Ausdrucks

## Operatorpraezedenz

PalmScript wertet Operatoren in dieser Reihenfolge aus, von niedriger nach
hoeherer Praezedenz:

1. `or`
2. `and`
3. `==`, `!=`, `<`, `<=`, `>`, `>=`
4. `+`, `-`
5. `*`, `/`
6. unäres `-`, unäres `!`
7. Aufruf-, Indexierungs- und Qualifizierungs-Postfixe

Operatoren derselben Praezedenzstufe sind linksassoziativ.

## Arithmetik

Arithmetische Operatoren sind `+`, `-`, `*` und `/`.

Regeln:

- beide Operanden muessen numerisch, numerische Serie oder `na` sein
- wenn einer der Operanden `na` ist, ist das Ergebnis `na`
- wenn einer der Operanden `series<float>` ist, ist das Ergebnis
  `series<float>`
- andernfalls ist das Ergebnis `float`

## Vergleiche

Vergleichsoperatoren sind `==`, `!=`, `<`, `<=`, `>` und `>=`.

Regeln:

- `<`, `<=`, `>` und `>=` erfordern numerische Operanden
- `==` und `!=` sind fuer alle nicht-`na`-Operanden definiert
- Gleichheit ueber gemischte Typen vergleicht als ungleich
- wenn einer der Operanden `na` ist, ist das Ergebnis `na`
- wenn einer der Operanden eine Serie ist, ist das Ergebnis eine boolesche
  Serie
- andernfalls ist das Ergebnis `bool`

## Unäre Operatoren

PalmScript unterstuetzt:

- unäres `-` fuer numerische Operanden
- unäres `!` fuer boolesche Operanden

Regeln:

- unäre Operatoren propagieren `na`
- unäres `-` ueber `series<float>` ergibt `series<float>`
- unäres `!` ueber `series<bool>` ergibt `series<bool>`

## Logische Operatoren

`and` und `or` erfordern `bool`, `series<bool>` oder `na`.

Sie verwenden deterministische dreiwertige Logik:

### `and`

| Links | Rechts | Ergebnis |
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

| Links | Rechts | Ergebnis |
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

PalmScript wertet beide Operanden aus, bevor der logische Operator angewendet
wird. Die Sprache garantiert keine Kurzschlussauswertung, daher werden logische
Ausdruecke innerhalb der normalen Sprachregeln eager analysiert und ausgefuehrt.

## `if`-Semantik

`if` ist eine Anweisungsform, keine Ausdrucksform.

Regeln:

- die Bedingung muss zu `bool`, `series<bool>` oder `na` ausgewertet werden
- `na` in einer `if`-Bedingung wird fuer die Zweigauswahl als `false`
  behandelt
- genau ein Zweig wird pro Schritt ausgefuehrt
- beide Zweige muessen syntaktisch vorhanden sein, weil `else` verpflichtend
  ist

## Funktionsauswertung

Benutzerdefinierte Funktionen sind ausdrucksbasiert und werden ueber
Spezialisierung kompiliert, nicht ueber dynamischen Runtime-Dispatch.

Regeln:

- die Argumentanzahl muss zur Anzahl der deklarierten Parameter passen
- Funktionen werden nach Argumenttyp und Aktualisierungstakt spezialisiert
- rekursive und zyklische Funktionsgraphen werden zur Compile-Zeit abgelehnt
- ein Funktionskoerper darf `plot` nicht aufrufen

## Kein-Lookahead-Regel

PalmScript darf keine teilweise gebildeten Hoeherintervall-Kerzen sichtbar
machen.

Folgen:

- eine Hoeherintervall-Serie aendert sich erst, nachdem die Kerze vollstaendig
  geschlossen ist
- Indexierung auf Hoeherintervall-Serien laeuft ueber die Historie voll
  geschlossener Hoeherintervall-Samples
- quellenbewusste Zusatzintervalle folgen derselben Regel

## Builtin-Helper-Semantik

Builtin-Helper-Formeln, Indikator-Vertraege, Fensterregeln und `na`-Verhalten
sind in [Builtins](builtins.md) und im Abschnitt [Indikatoren](indicators.md)
definiert.

Regeln:

- Helper-Builtins folgen den Aktualisierungstakten ihrer Serien-Eingaenge
- Helper-Ausgaben nehmen mit denselben Wert- und `na`-Regeln wie auf dieser
  Seite an `if`, Indexierung und weiteren Builtin-Aufrufen teil
- `if` behandelt `na` weiterhin als `false` fuer die Zweigauswahl, auch wenn
  die Bedingung von einem Helper wie `crossover(...)` kommt

## Determinismus

Die Ausdrucksauswertung ist deterministisch.

Während der Strategieausfuehrung haengt die Sprachsemantik nur von Folgendem
ab:

- dem kompilierten Programm
- den vorbereiteten Eingabe-Feeds
- den konfigurierten VM-Limits

Sie haengt nicht von Systemzeit, Dateisystemzugriff, Zufall oder Netzwerkzugriff
ab.
