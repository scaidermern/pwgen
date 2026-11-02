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

pwgen supports various options that allow to adjust the style of the generated passwords:
- `-f, --dict-file <FILE>`           Path to the dictionary [default: words.txt]
- `-w, --word-count <COUNT>`         Number of words per password (>0) [default: 4]
- `-c, --pw-count <COUNT>`           Number of passwords to generate (>0) [default: 10]
- `-m, --min-char-per-word <COUNT>`  Minimum number of characters per word (>0) [default: 4]
- `-n, --max-char-per-word <COUNT>`  Maximum number of characters per word (>0, >=min-char-per-word) [default: 5]
- `-s, --separator <SEP>`            Separator alphabet between words [default: !@$%^&*+=:|~]
- `-h, --help`                       Print help
- `-V, --version`                    Print version

## Word list

How to obtain a good word list? There are multiple options.

### EFF Large Wordlist for Passphrases

Download the [EFF large word list](https://www.eff.org/document/passphrase-wordlists]):
```
curl -s https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt | awk '{print $2}' > words.txt
```

This is a high quality list of many common words. It is the default word list for pwgen.

### Dictionaries

Generate a word list from a (spellcheck) dictionary, such as `wamerican`:
```
grep -E '^[a-zA-Z]{3,12}$' /usr/share/dict/words | tr '[:upper:]' '[:lower:]' | sort -u > words.txt
```

or `hunspell-en-us`:
```
cut -d'/' -f1 /usr/share/hunspell/en_US.dic | grep -E '^[a-zA-Z]{3,12}$' | tr '[:upper:]' '[:lower:]' | sort -u > words.txt
```

Both lead to rather large lists, also including very rare words.

### Manpages

Go crazy and turn manpages into word lists (you can also add more sections than just 1 and 8):
```
apropos -s 1,8 . | awk '{print $1}' | while read -r page; do man -P cat "$page" 2>/dev/null; done | grep -E '^[a-zA-Z]{3,12}$' | tr '[:upper:]' '[:lower:]' | sort -u > words.txt
```

This will take some time and lead to a more technical list. Some words will be less common and less memorable.

# License
[GPL v3](https://www.gnu.org/licenses/gpl-3.0.html)
(c) Alexander Heinlein
