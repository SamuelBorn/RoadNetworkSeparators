# C++ Routenplanungs-Basis-Framework

In diesem GIT-Repository finden Sie eine kleine Codebasis die Sie zur Bearbeitung des Übungsblatts verwenden sollen.
Der Ihnen zur Verfügung gestellt Code besteht aus mehreren Dateien:
`constants.h`,
`timer.h`,
`id_queue.h`,
`vector_io.h`,
`decode_vector.cpp`,
`encode_vector.cpp`,
`compare_vector.cpp` und
`compile.sh`.

In `constants.h` werden zwei oft verwendete Konstanten definiert: `invalid_id` und `inf_weight`.
Erstere gibt eine ungültige ID an und letztere stellt eine unendliche Länge dar.
`inf_weight` ist so gewählt, dass die Konstante verdoppelt werden kann, ohne einen Überlauf zu verursachen, d.h., der Ausdruck `inf_weight < inf_weight+inf_weight` ist unproblematisch.
In der Datei `timer.h` gibt es eine Funktion die die aktuelle Zeit misst (Alternativ können Sie die Funktionen aus dem `<chrono>`-Header verwenden.).
Diese kann verwendet werden um die Laufzeit ihres Codes zu messen.
Die Datei `id_queue.h` enthält eine Prioritätswarteschlange (`std::priority_queue` ist problematisch für unseren Anwendungsfall, da sie keine `decrease_key` Operation besitzt).
Die restlichen Dateien dienen dem Einlesen und der Ausgabe von Daten.
Jede Datendatei ist das binäre Abbild eines `std::vector`s im Speicher, d.h., ein Vektor von 100 `int`s wird in einer Datei gespeichert die genau 400 Byte lang (Wir gehen stets davon aus, dass ein int 32 Bit hat.) ist.
In `vector_io.h` werden die Funktionen `load_vector` und `save_vector` zur Verfügung gestellt.
Diese können Sie wie folgt verwenden:

```C++
vector<unsigned> head = load_vector<unsigned>("arc_head_file_name");
vector<float> lat = load_vector<float>("node_latitude_file_name");
save_vector("my_new_file_name", head);
```

Die restlichen Dateien stellen Hilfsprogramme dar.
`encode_vector` und `decode_vector` konvertieren Vektoren von und zu textuellen Darstellungen.
Das Programm `compare_vector` vergleicht ob zwei Vektoren identisch sind und wenn sie es nicht sind gibt es eine Übersicht über die Unterschiede.
Die Datei `compile.sh` ist ein Shellskript das die drei Programme übersetzt.
Erweitern Sie `compile.sh` so, dass es auch die von Ihnen erstellten Programme übersetzt.

## Graphen

Knoten und Kanten werden durch numerische IDs identifiziert, die von `0` bis `n-1` bzw. `m-1` gehen, wobei `n` die Anzahl an Knoten und `m` die Anzahl an gerichteten Kanten ist.
Wir speichern gewichtete und gerichtete Graphen in einer Ajdazenzarraydarstellung.
Ein gerichteter und gewichteter Graph besteht aus 3 Vektoren.
Diese heißen `first_out`, `head` und `weight`.
Um über die ausgehenden Kanten eines Knoten zu iterieren können Sie den folgenden Code verwenden:

```C++
vector<unsigned> first_out = load_vector<unsigned>("first_out_file_name");
vector<unsigned> head = load_vector<unsigned>("head_file_name");
vector<unsigned> weight = load_vector<unsigned>("weight_file_name");

unsigned my_node = 42;
for(unsigned out_arc = first_out[my_node]; out_arc < first_out[my_node+1]; ++out_arc){
    cout<< "There is an arc from " << my_node
        << " to " << head[out_arc]
        << " with weight " << weight[out_arc]
        << endl;
}
```

**Hinweis**: `head` und `weight` haben so viel Elemente wie es Kanten gibt.
`first_out` hat ein Element mehr als es Knoten gibt.
Das erste Element von `first_out` ist immer 0 und das letzte ist die Anzahl an Kanten.
Für alle Graphen gibt es zwei unterschiedliche Kantengewichte: Reisezeit und Reiselänge.
Des Weiteren gibt es für manche Graphen zusätzliche für jeden Knoten die geographische Position.
Diese wird als zwei `float` Vektoren abgespeichert die für jeden Knoten den Längen- und Breitengrad angeben.

Im Verzeichnis `/algoDaten/praktikum/graph` liegen die Daten von mehreren Graphen in diesem Format.
Manche dienen nur zu Testzwecken während andere zur Aufgabenbewertung verwendet werden.
Die Testgraphen entsprechen ganz grob Stupferich, Karlsruhe\&Umgebung, Deutschland\&Umgebung und (West-)Europa.
Die Aufgabengraphen haben die Größe des Deutschlandgraphen.

**Achtung**: Der Europagraph könnte zu groß sein für den Hauptspeicher von manchen Poolraumrechnern.

## Hinweise zur Nutzung im Routenplanungspraktikum

Zu jeder Aufgabe sollen Sie den Quellcode ihrer Programme und die berechneten Lösungen abgeben.
Der Quellcode soll durch das Ausführen von `./compile.sh` auf einem der Poolraumrechner übersetzt werden können.
Auf den Poolraumrechner ist ein GCC 7.3.1 installiert.
Dieser unterstützt C++17 vollständig.
Neuere C++ Features die nicht vom Compiler unterstützt werden sind nicht erlaubt.
Das verwenden extern Bibliotheken ist nicht erlaubt.
Die C++-Standardbibliothek is nicht extern.
