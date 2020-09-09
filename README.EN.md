[Versão em Português](README.md)

# Generator of architecture file for the Amnesia project

This repository contains the source code of a program that interactively generates the architecture file for the [Amnesia project](http://amnesia.lasdpc.icmc.usp.br), which was used in the subject Computer Architecture III of the Computer Science course at the Pontifical Catholic University of Minas Gerais.

## Download

The executable object can be downloaded from ["Releases"](https://github.com/Henriquemcc/Gerador_arquivo_arquitetura_amnesia_RustLang/releases).

## How to compile

### Prerequisite

1- Verify if RustLang's Cargo is installed on your computer, typing the following command on the terminal (on Windows, is the command prompt):
```
cargo --version
```

2- Verify if RustLang's compiler is installer on your computer, typing the following command on the terminal:
```
rustc --version
```

3- If RustLang's Cargo or RustLang's compiler is not installed, install through the RustLang website: https://www.rust-lang.org/tools/install.

### Compiling

1- Download this git repository's [zip file](https://github.com/Henriquemcc/Gerador_arquivo_arquitetura_amnesia_RustLang/archive/master.zip).

2- Unzip the zip file.

3- Open the terminal on the extracted folder from this zip file and type the following command:
```
cargo build --release
```

4- The binary executable object will be located on the 'release' folder inside the 'target' folder:
```
./target/release/
```
With the file name gerador_arquitetura_amnesia (gerador_arquitetura_amnesia.exe, on Windows).

## License

This program is licensed under [MIT License](LICENSE).