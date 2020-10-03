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
use std::io::Write;

use crate::arquitetura::configuracoes::Arquitetura;
use crate::fronteira::{criar_arquitetura, myio};

/// Altera a arquitetura do Amnesia.
/// # Arguments
/// * arquitetura - Ponteiro para a instancia da struct Arquitetura a ser modificada.
pub fn alterar_arquitetura(arquitetura: &mut Arquitetura)
{
    let arquitetura_a_ser_alterada = obter_arquitetura_a_ser_alterada();

    if arquitetura_a_ser_alterada == 0
    {
        alterar_arquitetura_processor(arquitetura);
    } else if arquitetura_a_ser_alterada == 1
    {
        alterar_arquitetura_cache(arquitetura);
    } else if arquitetura_a_ser_alterada == 2
    {
        alterar_arquitetura_main_memory(arquitetura);
    } else if arquitetura_a_ser_alterada == 3
    {
        alterar_arquitetura_virtual_memory(arquitetura);
    }
}

/// Altera a arquitetura do processador.
/// # Arguments
/// * arquitetura - Ponteiro para a instancia da struct Arquitetura a ser modificada.
fn alterar_arquitetura_processor(arquitetura: &mut Arquitetura)
{
    //Imprimindo a configuracao atual
    println!("Processor:");
    println!();
    println!("Configuração atual:");
    println!("{}", arquitetura.processor.to_string());
    println!("-------------------");

    //Obtendo a nova configuracao
    println!("Nova configuração:");
    print!("processor contains:");
    io::stdout().flush().expect("flush failed!");
    arquitetura.processor.set_processor_contains(myio::read_usize());

    print!("create trace file:");
    io::stdout().flush().expect("flush failed!");
    arquitetura.processor.set_create_trace_file(myio::read_usize());

    println!();
}

/// Altera a arquitetura da cache.
/// # Arguments
/// * arquitetura - Ponteiro para a instancia da struct Arquitetura a ser modificada.
fn alterar_arquitetura_cache(arquitetura: &mut Arquitetura)
{
    arquitetura.cache = criar_arquitetura::obter_cache();
}

/// Altera a arquitetura da memoria principal.
/// # Arguments
/// * arquitetura - Ponteiro para a instancia da struct Arquitetura a ser modificada.
fn alterar_arquitetura_main_memory(arquitetura: &mut Arquitetura)
{
    //Imprimindo a configuracao atual
    println!("Main Memory:");
    println!();
    println!("Configuração atual:");
    println!("{}", arquitetura.main_memory.to_string());
    println!("-------------------");

    //Obtendo a nova configuracao
    println!("Nova configuração:");
    print!("block size:");
    io::stdout().flush().expect("flush failed!");
    arquitetura.main_memory.set_block_size(myio::read_usize());

    print!("memory size:");
    io::stdout().flush().expect("flush failed!");
    arquitetura.main_memory.set_memory_size(myio::read_usize());

    print!("cicles per access read:");
    io::stdout().flush().expect("flush failed!");
    arquitetura.main_memory.set_cicles_per_access_read(myio::read_usize());

    print!("cicles per access write:");
    io::stdout().flush().expect("flush failed!");
    arquitetura.main_memory.set_cicles_per_access_write(myio::read_usize());

    print!("time cicle:");
    io::stdout().flush().expect("flush failed!");
    arquitetura.main_memory.set_time_cicle(myio::read_usize());
}

/// Altera a arquitetura da memoria virtual.
/// # Arguments
/// * arquitetura - Ponteiro para a instancia da struct Arquitetura a ser modificada.
fn alterar_arquitetura_virtual_memory(arquitetura: &mut Arquitetura)
{
    arquitetura.virtual_memory = criar_arquitetura::obter_virtual_memory();
}

/// Obtem a arquitetura a ser alterada.
/// # Return
/// * u8 - Arquitetura a ser alterada.
fn obter_arquitetura_a_ser_alterada() -> u8
{
    let mut configuracao: u8 = 0;

    let mut repetir = true;
    while repetir
    {
        println!("Qual configuração deseja alterar?");
        println!("0 - processor");
        println!("1 - cache");
        println!("2 - main_memory");
        println!("3 - virtual_memory");

        let entrada = myio::read_u8();

        if entrada > 3
        {
            eprintln!("Entrada inválida!");
        } else {
            configuracao = entrada;
            repetir = false;
        }
    }

    return configuracao;
}