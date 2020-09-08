use std::io;

///Esta funcao serve para ler (da entrada padrao) uma string.
///Retorno: String: String lida.
pub fn read_string() -> String
{
    //Lendo entrada
    let mut input: String;
    loop
    {
        input = String::new();
        if let Err(e)=io::stdin().read_line(&mut input)
        {
            eprintln!("{}", e);
        }
        else
        {
            break;
        }
    }

    return input;
}

///Esta funcao serve para ler (da entrada padrao) um numero inteiro unsigned de 8 bits.
///Retorno: u8: Numero inteiro lido.
pub fn read_u8() -> u8
{
    let unsigned_integer: u8;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para u8
        let result=input.parse::<u8>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            unsigned_integer=result.unwrap();
            break;
        }

    }

    return unsigned_integer
}

///Esta funcao serve para ler (da entrada padrao) um numero inteiro unsigned de 16 bits.
///Retorno: u16: Numero inteiro lido.
pub fn read_u16() -> u16
{
    let unsigned_integer: u16;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para u16
        let result=input.parse::<u16>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            unsigned_integer=result.unwrap();
            break;
        }

    }

    return unsigned_integer
}

///Esta funcao serve para ler (da entrada padrao) um numero inteiro unsigned de 32 bits.
///Retorno: u32: Numero inteiro lido.
pub fn read_u32() -> u32
{
    let unsigned_integer: u32;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para u32
        let result=input.parse::<u32>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            unsigned_integer=result.unwrap();
            break;
        }

    }

    return unsigned_integer
}

///Esta funcao serve para ler (da entrada padrao) um numero inteiro unsigned de 64 bits.
///Retorno: u64: Numero inteiro lido.
pub fn read_u64() -> u64
{
    let unsigned_integer: u64;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para u64
        let result=input.parse::<u64>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            unsigned_integer=result.unwrap();
            break;
        }

    }

    return unsigned_integer
}

///Esta funcao serve para ler (da entrada padrao) um numero inteiro unsigned de 128 bits.
///Retorno: u128: Numero inteiro lido.
pub fn read_u128() -> u128
{
    let unsigned_integer: u128;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para u128
        let result=input.parse::<u128>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            unsigned_integer=result.unwrap();
            break;
        }

    }

    return unsigned_integer
}

///Esta funcao serve para ler (da entrada padrao) um numero inteiro unsigned do tamanho da palavra do processador.
///Retorno: usize: Numero inteiro lido.
pub fn read_usize() -> usize
{
    let unsigned_integer: usize;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para usize
        let result=input.parse::<usize>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            unsigned_integer=result.unwrap();
            break;
        }

    }

    return unsigned_integer
}

///Esta funcao serve para ler (da entrada padrao) um numero inteiro de 8 bits.
///Retorno: i8: Numero inteiro lido.
pub fn read_i8() -> i8
{
    let integer: i8;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para i8
        let result=input.parse::<i8>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            integer=result.unwrap();
            break;
        }

    }

    return integer
}

///Esta funcao serve para ler (da entrada padrao) um numero inteiro de 16 bits.
///Retorno: i16: Numero inteiro lido.
pub fn read_i16() -> i16
{
    let integer: i16;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para i16
        let result=input.parse::<i16>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            integer=result.unwrap();
            break;
        }

    }

    return integer
}

///Esta funcao serve para ler (da entrada padrao) um numero inteiro de 32 bits.
///Retorno: i32: Numero inteiro lido.
pub fn read_i32() -> i32
{
    let integer: i32;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para i32
        let result=input.parse::<i32>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            integer=result.unwrap();
            break;
        }

    }

    return integer
}

///Esta funcao serve para ler (da entrada padrao) um numero inteiro de 64 bits.
///Retorno: i64: Numero inteiro lido.
pub fn read_i64() -> i64
{
    let integer: i64;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para i64
        let result=input.parse::<i64>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            integer=result.unwrap();
            break;
        }

    }

    return integer
}

///Esta funcao serve para ler (da entrada padrao) um numero inteiro de 128 bits.
///Retorno: i128: Numero inteiro lido.
pub fn read_i128() -> i128
{
    let integer: i128;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para i128
        let result=input.parse::<i128>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            integer=result.unwrap();
            break;
        }

    }

    return integer
}

///Esta funcao serve para ler (da entrada padrao) um numero inteiro do tamanho da palavra do processador.
///Retorno: isize: Numero inteiro lido.
pub fn read_isize() -> isize
{
    let integer: isize;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para i8
        let result=input.parse::<isize>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            integer=result.unwrap();
            break;
        }

    }

    return integer
}

///Esta funcao serve para ler (da entrada padrao) um numero real de 32 bits.
///Retorno: f32: Numero real lido.
pub fn read_f32() -> f32
{
    let float: f32;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para f32
        let result=input.parse::<f32>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            float=result.unwrap();
            break;
        }

    }

    return float
}

///Esta funcao serve para ler (da entrada padrao) um numero real de 64 bits.
///Retorno: f64: Numero real lido.
pub fn read_f64() -> f64
{
    let float: f64;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para f64
        let result=input.parse::<f64>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            float=result.unwrap();
            break;
        }

    }

    return float
}

///Esta funcao serve para ler (da entrada padrao) um caractere.
///Retorno: char: Caractere lido.
pub fn read_char() -> char
{
    let character: char;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para f64
        let result=input.parse::<char>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            character=result.unwrap();
            break;
        }

    }

    return character;

}

///Esta funcao serve para ler (da entrada padrao) um valor booleano.
///Retorno: bool: Valor booleano lido.
pub fn read_bool() -> bool
{
    let boolean: bool;
    loop
    {
        //Lendo entrada
        let input=read_string();

        //Removendo espacos
        let input = input.trim();

        //Convertendo para f64
        let result=input.parse::<bool>();

        if result.is_err()
        {
            eprintln!("{}",result.unwrap_err());
        }
        else
        {
            boolean=result.unwrap();
            break;
        }

    }

    return boolean;

}