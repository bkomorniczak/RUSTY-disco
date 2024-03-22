# Opis projektu
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
```

#### Monogramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt  --p src/resource/monogram_probabilities.txt --t
```
#### Bigramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt  --p src/resource/bigram_probabilities.txt --t
```
#### Trigramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt  --p src/resource/trigram_probabilities.txt --t
```
#### Quadgramy
```
cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k src/resource/dictionary.txt  --p src/resource/quadgram_probabilities.txt --t
```



