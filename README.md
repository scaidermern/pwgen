# pwgen

Password generator for secure, memorable passwords, written in Rust.

The password style is highly inspired by https://www.xkpasswd.net/

## Usage

Simply call the program to generate a list of secure, memorable passwords:
```
$ ./pwgen 
34|wheel!PERK!vans!NOWAY|90
88~flute:SMUT:puck:DRAWS~62
92!fail$SIRES$besot$BLAB!78
37~klutz=PENAL=ewes=MOORS~40
60:rondo+YORE+weft+SPIFF:67
04*shams+ALIGN+lube+PESTO*53
33^sedgy=GOLLY=scamp=SNUBS^33
96+fitly%HOBBY%ferry%ESTER+54
28~manly^MOREL^mane^LEAPS~46
85:bark^SHADS^writ^MILCH:41
```

## Options

There are various options that allow to adjust the style of the generated passwords:
- `-f, --dict-file <FILE>`           Path to the dictionary [default: words.txt]
- `-w, --word-count <COUNT>`         Number of words per password (>0) [default: 4]
- `-c, --pw-count <COUNT>`           Number of passwords to generate (>0) [default: 10]
- `-m, --min-char-per-word <COUNT>`  Minimum number of characters per word (>0) [default: 4]
- `-n, --max-char-per-word <COUNT>`  Maximum number of characters per word (>0, >=min-char-per-word) [default: 5]
- `-s, --separator <SEP>`            Separator alphabet between words [default: !@$%^&*+=:|~]
- `-h, --help`                       Print help
- `-V, --version`                    Print version

# License
[GPL v3](https://www.gnu.org/licenses/gpl-3.0.html)
(c) Alexander Heinlein
