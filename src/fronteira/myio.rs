/*
 * MIT License
 *
 * Copyright (c) 2020 Henrique Mendonça Castelar Campos
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::io;

/// Le (da entrada padrao) uma string.
/// # Return
/// * String - String lida.
pub fn read_string() -> String
{
    //Lendo entrada
    let mut input = String::new();

    let mut repetir = true;
    while repetir
    {
        if let Err(e) = io::stdin().read_line(&mut input)
        {
            eprintln!("{}", e);
        } else {
            repetir = false;
        }
    }

    return input;
}

/// Le (da entrada padrao) um numero inteiro unsigned de 8 bits.
/// # Return
/// * u8 - Numero inteiro unsigned de 8 bits lido.
pub fn read_u8() -> u8
{
    let mut unsigned_integer: u8 = 0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para u8
        let result = input.parse::<u8>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            unsigned_integer = result.unwrap();
            repetir = false;
        }
    }

    return unsigned_integer;
}

/// Le (da entrada padrao) um numero inteiro unsigned de 16 bits.
/// # Return
/// * u16 - Numero inteiro unsigned de 16 bits lido.
pub fn read_u16() -> u16
{
    let mut unsigned_integer: u16 = 0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para u16
        let result = input.parse::<u16>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            unsigned_integer = result.unwrap();
            repetir = false;
        }
    }

    return unsigned_integer;
}

/// Le (da entrada padrao) um numero inteiro unsigned de 32 bits.
/// # Return
/// * u32 - Numero inteiro unsigned de 32 bits lido.
pub fn read_u32() -> u32
{
    let mut unsigned_integer: u32 = 0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para u32
        let result = input.parse::<u32>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            unsigned_integer = result.unwrap();
            repetir = false;
        }
    }

    return unsigned_integer;
}

/// Le (da entrada padrao) um numero inteiro unsigned de 64 bits.
/// # Return
/// * u64 - Numero inteiro unsigned de 64 bits lido.
pub fn read_u64() -> u64
{
    let mut unsigned_integer: u64 = 0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para u64
        let result = input.parse::<u64>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            unsigned_integer = result.unwrap();
            repetir = false;
        }
    }

    return unsigned_integer;
}

/// Le (da entrada padrao) um numero inteiro unsigned de 128 bits.
/// # Return
/// * u128 - Numero inteiro unsigned de 128 bits lido.
pub fn read_u128() -> u128
{
    let mut unsigned_integer: u128 = 0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para u128
        let result = input.parse::<u128>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            unsigned_integer = result.unwrap();
            repetir = false;
        }
    }

    return unsigned_integer;
}

/// Le (da entrada padrao) um numero inteiro unsigned do tamanho da palavra do processador.
/// # Return
/// * usize - Numero inteiro unsigned do tamanho da palavra do processador lido.
pub fn read_usize() -> usize
{
    let mut unsigned_integer: usize = 0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para usize
        let result = input.parse::<usize>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            unsigned_integer = result.unwrap();
            repetir = false;
        }
    }

    return unsigned_integer;
}

/// Le (da entrada padrao) um numero inteiro de 8 bits.
/// # Return
/// * i8 - Numero inteiro de 8 bits lido.
pub fn read_i8() -> i8
{
    let mut integer: i8 = 0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para i8
        let result = input.parse::<i8>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            integer = result.unwrap();
            repetir = false;
        }
    }

    return integer;
}

/// Le (da entrada padrao) um numero inteiro de 16 bits.
/// # Return
/// * i16 - Numero inteiro de 16 bits lido.
pub fn read_i16() -> i16
{
    let mut integer: i16 = 0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para i16
        let result = input.parse::<i16>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            integer = result.unwrap();
            repetir = false;
        }
    }

    return integer;
}

/// Le (da entrada padrao) um numero inteiro de 32 bits.
/// # Return
/// * i32 - Numero inteiro de 32 bits lido.
pub fn read_i32() -> i32
{
    let mut integer: i32 = 0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para i32
        let result = input.parse::<i32>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            integer = result.unwrap();
            repetir = false;
        }
    }

    return integer;
}

/// Le (da entrada padrao) um numero inteiro de 64 bits.
/// # Return
/// * i64 - Numero inteiro de 64 bits lido.
pub fn read_i64() -> i64
{
    let mut integer: i64 = 0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para i64
        let result = input.parse::<i64>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            integer = result.unwrap();
            repetir = false;
        }
    }

    return integer;
}

/// Le (da entrada padrao) um numero inteiro de 128 bits.
/// # Return
/// * i128 - Numero inteiro de 128 bits lido.
pub fn read_i128() -> i128
{
    let mut integer: i128 = 0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para i128
        let result = input.parse::<i128>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            integer = result.unwrap();
            repetir = false;
        }
    }

    return integer;
}

/// Ler (da entrada padrao) um numero inteiro do tamanho da palavra do processador.
/// # Return
/// * isize - Numero inteiro do tamanho da palavra do processador lido.
pub fn read_isize() -> isize
{
    let mut integer: isize = 0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para i8
        let result = input.parse::<isize>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            integer = result.unwrap();
            repetir = false;
        }
    }

    return integer;
}

/// Le (da entrada padrao) um numero real de 32 bits.
/// # Return
/// * f32 - Numero real de 32 bits lido.
pub fn read_f32() -> f32
{
    let mut float: f32 = 0.0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para f32
        let result = input.parse::<f32>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            float = result.unwrap();
            repetir = false;
        }
    }

    return float;
}

/// Le (da entrada padrao) um numero real de 64 bits.
/// # Return
/// * f64: Numero real de 64 bits lido.
pub fn read_f64() -> f64
{
    let mut float: f64 = 0.0;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para f64
        let result = input.parse::<f64>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            float = result.unwrap();
            repetir = false;
        }
    }

    return float;
}

/// Ler (da entrada padrao) um caractere.
/// # Return
/// * char - Caractere lido.
pub fn read_char() -> char
{
    let mut character: char = ' ';

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para f64
        let result = input.parse::<char>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            character = result.unwrap();
            repetir = false;
        }
    }

    return character;
}

/// Le (da entrada padrao) um valor booleano.
/// # Return
/// * bool - Valor booleano lido.
pub fn read_bool() -> bool
{
    let mut boolean: bool = false;

    let mut repetir = true;
    while repetir
    {
        //Lendo entrada
        let input = read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para f64
        let result = input.parse::<bool>();

        if result.is_err()
        {
            eprintln!("{}", result.unwrap_err());
        } else {
            boolean = result.unwrap();
            repetir = false;
        }
    }

    return boolean;
}