# Opis projektu

## Glowne funkcje programu
Program pozwala na szyfrowanie i deszyfrowanie tekstu przy użyciu szyfru podstawieniowego. Program pozwala na obliczenie monogramów, bigramów, trigramów, quadgramów oraz prawdopodobieństw ich wystąpienia w tekście. Program pozwala na obliczenie wartości T dla każdego z ngramów.

### Funkcja main
Funkcja main odpowiada za obsługę argumentów programu oraz wywołanie odpowiednich funkcji. 
Główne elementy funkcji main:
- Wczytanie argumentów programu. Wczytywanie komend z argumentów programu odbywa się przy użyciu biblioteki clap. Używane są następujące flagi:
  - -i - ścieżka do pliku z tekstem wejściowym
  - -o - ścieżka do pliku z tekstem wyjściowym
  - -k - ścieżka do pliku z kluczem
  - --g1 - ścieżka do pliku z monogramami
  - --g2 - ścieżka do pliku z bigramami
  - --g3 - ścieżka do pliku z trigramami
  - --g4 - ścieżka do pliku z quadgramami
  - --ri - ścieżka do pliku referencyjnego z ngramami
  - --ro - ścieżka do pliku referencyjnego z prawdopodobieństwami ngramów
  - --p - ścieżka do pliku z prawdopodobieństwami ngramów
  - --t1 - flaga oznaczająca obliczenie wartości T dla monogramów
  - --t2 - flaga oznaczająca obliczenie wartości T dla bigramów
  - --t3 - flaga oznaczająca obliczenie wartości T dla trigramów
  - --t4 - flaga oznaczająca obliczenie wartości T dla quadgramów
- Wczytanie tekstu z pliku
- Wczytanie klucza z pliku
- Wczytanie ngramów z pliku
- Wczytanie prawdopodobieństw ngramów z pliku
- Wywołanie odpowiednich funkcji w zależności od argumentów programu
- Zapisanie wyników do pliku
- Wypisanie wyników na ekran

## Komendy

### Szyfruj
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt
```
### Oblicz monogramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --g1 src/resource/monogram.txt
```
### Oblicz bigramy
``` 
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --g2 src/resource/bigram.txt
```
### Oblicz trigramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --g3 src/resource/trigram.txt
```
### Oblicz quadgramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --g4 src/resource/quadgram.txt
```
### Oblicz prawdopodobieństwa quadgramów
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --ri src/resource/quadgram.txt --ro src/resource/quadgram_probabilities.txt
```
### Oblicz prawdopodobieństwa trigramów
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --ri src/resource/trigram.txt --ro src/resource/trigram_probabilities.txt
```
### Oblicz prawdopodobieństwa bigramów
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --ri src/resource/bigram.txt --ro src/resource/bigram_probabilities.txt
```
### Oblicz prawdopodobieństwa monogramów
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt --ri src/resource/monogram.txt --ro src/resource/monogram_probabilities.txt
```
### Oblicz T dla wszystkich ngramow

#### Monogramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt  --p src/resource/monogram_probabilities.txt --t1
```
#### Bigramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt  --p src/resource/bigram_probabilities.txt --t2
```
#### Trigramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt  --p src/resource/trigram_probabilities.txt --t3
```
#### Quadgramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt  --p src/resource/quadgram_probabilities.txt --t4
```



