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

use crate::arquitetura::cache::{Cache, CacheType, DataCache, HaveCache, InstructionCache, SplitCache, UnifiedCache};
use crate::arquitetura::configuracoes::Arquitetura;
use crate::arquitetura::cpu::CPU;
use crate::arquitetura::main_memory::MainMemory;
use crate::arquitetura::processor::Processor;
use crate::arquitetura::replacement_algorithm::ReplacementAlgorithm;
use crate::arquitetura::tlb::{DataTLB, InstructionTLB, SplitTLB, TLBType, UnifiedTLB};
use crate::arquitetura::trace::Trace;
use crate::arquitetura::virtual_memory::{HaveVirtualMemory, VirtualMemory};
use crate::arquitetura::write_policy::WritePolicy;
use crate::fronteira::myio;

/// Cria uma nova arquitetura para o amnesia.
/// # Arguments
/// * arquitetura - Ponteiro para a instancia da struct Arquitetura a ser modificada.
pub fn criar_arquitetura(arquitetura: &mut Arquitetura)
{
    arquitetura.processor = obter_processor();
    arquitetura.trace = Trace::new();
    arquitetura.cpu = CPU::new();
    arquitetura.cache = obter_cache();
    arquitetura.main_memory = obter_main_memory();
    arquitetura.virtual_memory = obter_virtual_memory();
}

/// Obtem se tem memoria virtual.
/// # Return
/// * HaveVirtualMemory - Instancia da enum HaveVirtualMemory obtida.
pub fn obter_virtual_memory() -> HaveVirtualMemory {
    let mut virtual_memory = HaveVirtualMemory::No;
    println!("Virtual Memory:");

    print!("Tem Virtual Memory? (true/false)");
    io::stdout().flush().expect("flush failed!");
    let tem_memoria_virtual = myio::read_bool();

    if tem_memoria_virtual
    {
        virtual_memory = HaveVirtualMemory::Yes(obter_arquitetura_virtual_memory())
    }

    return virtual_memory;
}

/// Obtem a arquitetura da memoria virtual.
/// # Return
/// * VirtualMemory - Instancia da struct VirtualMemory obtida.
fn obter_arquitetura_virtual_memory() -> VirtualMemory
{
    let mut virtual_memory = VirtualMemory::new();

    print!("pageSize:");
    io::stdout().flush().expect("flush failed!");
    virtual_memory.set_page_size(myio::read_usize());

    print!("diskMemorySize:");
    io::stdout().flush().expect("flush failed!");
    virtual_memory.set_disk_memory_size(myio::read_usize());

    print!("diskCiclesPerAccess:");
    io::stdout().flush().expect("flush failed!");
    virtual_memory.set_disk_cicles_per_access(myio::read_usize());

    print!("diskCiclesPerAccessWrite:");
    io::stdout().flush().expect("flush failed!");
    virtual_memory.set_disk_cicles_per_access_write(myio::read_usize());

    print!("diskCiclesPerAccessRead:");
    io::stdout().flush().expect("flush failed!");
    virtual_memory.set_disk_cicles_per_access_read(myio::read_usize());

    print!("timeCicle:");
    io::stdout().flush().expect("flush failed!");
    virtual_memory.set_time_cicle(myio::read_usize());

    virtual_memory.set_page_table_replacement_algorithm(obter_replacement_algorithm("pageTableReplacementAlgorithm".to_string()));

    virtual_memory.set_tlb_type(obter_tlb_type());

    return virtual_memory;
}

/// Obtem a arquitetura da memoria principal.
/// # Return
/// * MainMemory - Instancia da struct MainMemory obtida.
fn obter_main_memory() -> MainMemory
{
    let mut main_memory = MainMemory::new();
    println!("Main Memory:");

    print!("blockSize:");
    io::stdout().flush().expect("flush failed!");
    main_memory.set_block_size(myio::read_usize());

    print!("memorySize:");
    io::stdout().flush().expect("flush failed!");
    main_memory.set_memory_size(myio::read_usize());

    print!("ciclesPerAccessRead:");
    io::stdout().flush().expect("flush failed!");
    main_memory.set_cicles_per_access_read(myio::read_usize());

    print!("ciclesPerAccessWrite:");
    io::stdout().flush().expect("flush failed!");
    main_memory.set_cicles_per_access_write(myio::read_usize());

    print!("timeCicle:");
    io::stdout().flush().expect("flush failed!");
    main_memory.set_time_cicle(myio::read_usize());

    print!("ciclesPerAccess:");
    io::stdout().flush().expect("flush failed!");
    main_memory.set_cicles_per_access(myio::read_usize());

    return main_memory;
}

/// Obtem a arquitetura do processador.
/// # Return
/// * Processor - Instancia da struct Processo obtida.
fn obter_processor() -> Processor
{
    let mut processor = Processor::new();
    println!("Processor:");

    print!("processorContains:");
    io::stdout().flush().expect("flush failed!");
    processor.set_processor_contains(myio::read_usize());

    print!("createTraceFile:");
    io::stdout().flush().expect("flush failed!");
    processor.set_create_trace_file(myio::read_usize());

    return processor;
}

/// Obtem se tem cache.
/// # Return
/// * HaveCache - Instancia da enum HaveCache obtida.
pub fn obter_cache() -> HaveCache
{
    let mut have_cache = HaveCache::No;
    println!("Cache");

    print!("Tem cache? (true/false): ");
    io::stdout().flush().expect("flush failed!");
    let tem_cache = myio::read_bool();

    if tem_cache
    {
        let mut cache = Cache::new();
        cache.set_cache_type(obter_cache_type());
        have_cache = HaveCache::Yes(cache);
    }

    return have_cache;
}

/// Obtem a arquitetura do TBL.
/// # Return
/// * TLBType - Instancia da struct TLBType obtida.
fn obter_tlb_type() -> TLBType
{
    let mut tlb_type = TLBType::None;
    let mut repetir = true;
    while repetir
    {
        //Obtendo o tipo de cache
        print!("TLB type (Unified/Split/none):");
        io::stdout().flush().expect("flush failed!");
        let entrada = myio::read_string().to_lowercase().chars().next().unwrap();

        //Validando entrada do usuario
        if entrada == 'u'
        {
            tlb_type = TLBType::Unified(obter_unified_tlb());
            repetir = false;
        } else if entrada == 's'
        {
            tlb_type = TLBType::Split(obter_tlb_split());
            repetir = false;
        } else if entrada == 'n'
        {
            tlb_type = TLBType::None;
            repetir = false;
        } else {
            eprintln!("cache type inválido");
        }
    }

    //Retornando o tipo de cache
    return tlb_type;
}

/// Obtem a arquitetura para a SplitTLB.
/// # Return
/// * SplitTLB - Instancia da struct SplitTLB obtida.
fn obter_tlb_split() -> SplitTLB {
    let mut split_tlb = SplitTLB::new();
    println!("Split TLB:");

    split_tlb.set_instruction_tlb(obter_instruction_tlb());
    split_tlb.set_data_tlb(obter_data_tlb());

    return split_tlb;
}

/// Obtem a arquitetura para a DataTLB.
/// # Return
/// * DataTLB - Instancia da struct DataTLB obtida.
fn obter_data_tlb() -> DataTLB {
    let mut data_tlb = DataTLB::new();
    println!("Data TLB");

    print!("ciclesPerAccess:");
    io::stdout().flush().expect("flush failed!");
    data_tlb.set_cicles_per_access(myio::read_usize());

    print!("ciclesPerAccessRead:");
    io::stdout().flush().expect("flush failed!");
    data_tlb.set_cicles_per_access_read(myio::read_usize());

    print!("ciclesPerAccessWrite:");
    io::stdout().flush().expect("flush failed!");
    data_tlb.set_cicles_per_access_write(myio::read_usize());

    print!("timeCicle:");
    io::stdout().flush().expect("flush failed!");
    data_tlb.set_time_cicle(myio::read_usize());

    print!("memorySize:");
    io::stdout().flush().expect("flush failed!");
    data_tlb.set_memory_size(myio::read_usize());

    data_tlb.set_replacement_algorithm(obter_replacement_algorithm("replacementAlgorithm".to_string()));

    return data_tlb;
}

/// Obtem a arquitetura para InstructionTLB.
/// # Return
/// * InstructionTLB - Instancia da struct InstructionTLB obtida.
fn obter_instruction_tlb() -> InstructionTLB {
    let mut instruction_tlb = InstructionTLB::new();
    println!("Instruction TLB:");

    print!("ciclesPerAccess:");
    io::stdout().flush().expect("flush failed!");
    instruction_tlb.set_cicles_per_access(myio::read_usize());

    print!("ciclesPerAccessRead:");
    io::stdout().flush().expect("flush failed!");
    instruction_tlb.set_cicles_per_access_read(myio::read_usize());

    print!("ciclesPerAccessWrite:");
    io::stdout().flush().expect("flush failed!");
    instruction_tlb.set_cicles_per_access_write(myio::read_usize());

    print!("timeCicle:");
    io::stdout().flush().expect("flush failed!");
    instruction_tlb.set_time_cicle(myio::read_usize());

    print!("memorySize:");
    io::stdout().flush().expect("flush failed!");
    instruction_tlb.set_memory_size(myio::read_usize());

    instruction_tlb.set_replacement_algorithm(obter_replacement_algorithm("replacementAlgorithm".to_string()));

    return instruction_tlb;
}

/// Obtem a arquitetura para a UnifiedTLB.
/// # Return
/// * UnifiedTLB - Instancia da struct UnifiedTLB obtida.
fn obter_unified_tlb() -> UnifiedTLB
{
    let mut unified_tlb = UnifiedTLB::new();
    println!("Unified TLB:");

    print!("ciclesPerAccess:");
    io::stdout().flush().expect("flush failed!");
    unified_tlb.set_cicles_per_access(myio::read_usize());

    print!("ciclesPerAccessRead:");
    io::stdout().flush().expect("flush failed!");
    unified_tlb.set_cicles_per_access_read(myio::read_usize());

    print!("ciclesPerAccessWrite:");
    io::stdout().flush().expect("flush failed!");
    unified_tlb.set_cicles_per_access_write(myio::read_usize());

    print!("timeCicle:");
    io::stdout().flush().expect("flush failed!");
    unified_tlb.set_time_cicle(myio::read_usize());

    print!("memorySize:");
    io::stdout().flush().expect("flush failed!");
    unified_tlb.set_memory_size(myio::read_usize());

    unified_tlb.set_replacement_algorithm(obter_replacement_algorithm("replacementAlgorithm:".to_string()));

    return unified_tlb;
}

/// Obtem a arquitetura da UnifiedCache.
/// # Return
/// * UnifiedCache - Instancia da struct UnifiedCache obtida.
fn obter_unified_cache() -> UnifiedCache
{
    let mut unified_cache = UnifiedCache::new();
    println!("unifiedCache:");

    print!("lineSize:");
    io::stdout().flush().expect("flush failed!");
    unified_cache.set_line_size(myio::read_usize());

    print!("ciclesPerAccess:");
    io::stdout().flush().expect("flush failed!");
    unified_cache.set_cicles_per_access(myio::read_usize());

    print!("ciclesPerAccessRead:");
    io::stdout().flush().expect("flush failed!");
    unified_cache.set_cicles_per_access_read(myio::read_usize());

    print!("ciclesPerAccessWrite:");
    io::stdout().flush().expect("flush failed!");
    unified_cache.set_cicles_per_access_write(myio::read_usize());

    print!("timeCicle:");
    io::stdout().flush().expect("flush failed!");
    unified_cache.set_time_cicle(myio::read_usize());

    print!("memorySize:");
    io::stdout().flush().expect("flush failed!");
    unified_cache.set_memory_size(myio::read_usize());

    print!("associativityLevel:");
    io::stdout().flush().expect("flush failed!");
    unified_cache.set_associativity_level(myio::read_usize());

    print!("writePolicy:");
    io::stdout().flush().expect("flush failed!");
    unified_cache.set_write_policy(obter_write_policy());

    unified_cache.set_replacement_algorithm(obter_replacement_algorithm("replacementAlgorithm".to_string()));

    return unified_cache;
}

/// Obtem a arquitetura da SplitCache.
/// # Return
/// * SplitCache - Instancia da struct SplitCache obtida.
fn obter_split_cache() -> SplitCache
{
    let mut split_cache = SplitCache::new();

    split_cache.set_instruction_cache(obter_instruction_cache());
    split_cache.set_data_cache(obter_data_cache());

    return split_cache;
}

/// Obtem a arquitetura da DataCache.
/// # Return
/// * DataCache - Instancia da struct DataCache obtida.
fn obter_data_cache() -> DataCache
{
    let mut data_cache = DataCache::new();
    println!("Instruction Cache:");

    print!("lineSize:");
    io::stdout().flush().expect("flush failed!");
    data_cache.set_line_size(myio::read_usize());

    print!("ciclesPerAccess:");
    io::stdout().flush().expect("flush failed!");
    data_cache.set_cicles_per_access(myio::read_usize());

    print!("ciclesPerAccessRead:");
    io::stdout().flush().expect("flush failed!");
    data_cache.set_cicles_per_access_read(myio::read_usize());

    print!("ciclesPerAccessWrite:");
    io::stdout().flush().expect("flush failed!");
    data_cache.set_cicles_per_access_write(myio::read_usize());

    print!("timeCicle:");
    io::stdout().flush().expect("flush failed!");
    data_cache.set_time_cicle(myio::read_usize());

    print!("memorySize:");
    io::stdout().flush().expect("flush failed!");
    data_cache.set_memory_size(myio::read_usize());

    print!("associativityLevel:");
    io::stdout().flush().expect("flush failed!");
    data_cache.set_associativity_level(myio::read_usize());

    print!("writePolicy:");
    io::stdout().flush().expect("flush failed!");
    data_cache.set_write_policy(obter_write_policy());

    data_cache.set_replacement_algorithm(obter_replacement_algorithm("replacementAlgorithm".to_string()));

    return data_cache;
}

/// Obtem a arquitetura da InstructionCache.
/// # Return
/// * InstructionCache - Instancia da InstructionCache obtida.
fn obter_instruction_cache() -> InstructionCache
{
    let mut instruction_cache = InstructionCache::new();
    println!("Instruction Cache:");

    print!("lineSize:");
    io::stdout().flush().expect("flush failed!");
    instruction_cache.set_line_size(myio::read_usize());

    print!("ciclesPerAccess:");
    io::stdout().flush().expect("flush failed!");
    instruction_cache.set_cicles_per_access(myio::read_usize());

    print!("ciclesPerAccessRead:");
    io::stdout().flush().expect("flush failed!");
    instruction_cache.set_cicles_per_access_read(myio::read_usize());

    print!("ciclesPerAccessWrite:");
    io::stdout().flush().expect("flush failed!");
    instruction_cache.set_cicles_per_access_write(myio::read_usize());

    print!("timeCicle:");
    io::stdout().flush().expect("flush failed!");
    instruction_cache.set_time_cicle(myio::read_usize());

    print!("memorySize:");
    io::stdout().flush().expect("flush failed!");
    instruction_cache.set_memory_size(myio::read_usize());

    print!("associativityLevel:");
    io::stdout().flush().expect("flush failed!");
    instruction_cache.set_associativity_level(myio::read_usize());

    print!("writePolicy:");
    io::stdout().flush().expect("flush failed!");
    instruction_cache.set_write_policy(obter_write_policy());

    instruction_cache.set_replacement_algorithm(obter_replacement_algorithm("replacementAlgorithm".to_string()));

    return instruction_cache;
}

/// Obtem a arquitetura da CacheType.
/// # Return
/// * CacheType - Instancia da struct CacheType obtida.
fn obter_cache_type() -> CacheType
{
    let mut cache_type = CacheType::Null;

    let mut repetir = true;
    while repetir
    {
        //Obtendo o tipo de cache
        print!("cache type (Unified/Split):");
        io::stdout().flush().expect("flush failed!");
        let entrada = myio::read_string().to_lowercase().chars().next().unwrap();

        //Validando entrada do usuario
        if entrada == 'u'
        {
            cache_type = CacheType::Unified(obter_unified_cache());
            repetir = false;
        } else if entrada == 's'
        {
            cache_type = CacheType::Split(obter_split_cache());
            repetir = false;
        } else {
            eprintln!("cache type inválido");
        }
    }

    //Retornando o tipo de cache
    return cache_type;
}

/// Obtem a politica de escrita.
/// # Return
/// * WritePolicy - Instancia da enum WritePolicy obtida.
fn obter_write_policy() -> WritePolicy
{
    let mut write_policy = WritePolicy::WriteBack;

    let mut repetir = true;
    while repetir
    {
        //Obtendo a politica de escrita
        print!("write policy (WriteThrough/WriteBack):");
        io::stdout().flush().expect("flush failed!");
        let entrada: Vec<char> = myio::read_string().to_lowercase().chars().collect();

        //Validando entrada do usuario
        if entrada.len() > 5 && entrada[5] == 't'
        {
            write_policy = WritePolicy::WriteThrough;
            repetir = false;
        } else if entrada.len() > 5 && entrada[5] == 'b'
        {
            write_policy = WritePolicy::WriteBack;
            repetir = false;
        } else {
            eprintln!("write policy inválido");
        }
    }

    //Retornando a politica de escrita
    return write_policy;
}

/// Obtem o algoritmo de substituicao.
/// # Return
/// * ReplacementAlgorithm - Instancia da enum ReplacementAlgorithm obtida.
fn obter_replacement_algorithm(mensagem: String) -> ReplacementAlgorithm
{
    let mut replacement_algorithm = ReplacementAlgorithm::LRU;

    let mut repetir = true;
    while repetir
    {
        //Obtendo algoritmo de substituicao
        print!("{} (FIFO/LRU):", mensagem);
        io::stdout().flush().expect("flush failed!");
        let entrada = myio::read_string().to_lowercase().chars().next().unwrap();

        //Validando entrada do usuario
        if entrada == 'f'
        {
            replacement_algorithm = ReplacementAlgorithm::FIFO;
            repetir = false;
        } else if entrada == 'l'
        {
            replacement_algorithm = ReplacementAlgorithm::LRU;
            repetir = false;
        } else {
            eprintln!("replacement algorithm inválido");
        }
    }

    //Retornando algoritmo de substituicao
    return replacement_algorithm;
}