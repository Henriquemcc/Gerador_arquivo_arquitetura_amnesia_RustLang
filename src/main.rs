use std::{io, fs::File};
use io::Write;
mod myio;
mod arquitetura;

///Esta funcao eh a funcao principal, que eh a primeira funcao chamada quando o programa inicia.
fn main()
{
    //Criando uma variavel para armazenar as configuracoes do Amnesia
    let mut configuracoes = arquitetura::Configuracoes::new();

    //Exibindo mensagem de bem vindo
    println!("Bem vindo ao gerador de arquitetura para o Amnesia.");

    //Obtendo e executando o comando do usuario
    loop
    {
        let comando = obter_comando();

        if comando == 0
        {
            break;
        }
        else if comando == 1
        {
            criar_configuracoes_arquitetura(&mut configuracoes);
        }
        else if comando == 2
        {
            exibir_configuracoes(&configuracoes);
        }
        else if comando == 3
        {
            alterar_configuracoes(&mut configuracoes)
        }
        else if comando == 4
        {
            salvar_configuracoes_arquivo(&configuracoes);
        }
    }
}

///Esta funcao serve para obter do usuario o comando que o programa deve executar
///Retorno: u8: Numero inteiro unsigned de 8 bits, contendo o numero do comando.
fn obter_comando() -> u8
{
    let mut comando: u8;
    loop
    {
        //Exibindo os comandos
        println!("O que deseja fazer?");
        println!("0 - Sair.");
        println!("1 - Criar nova configuração de arquitetura para o Amnesia.");
        println!("2 - Exibir atual configuração de arquitetura do Amnesia.");
        println!("3 - Alterar atual configuração de arquitetura do Amnesia.");
        println!("4 - Salvar configuração de arquitetura em um arquivo.");

        //Obtendo o comando
        comando = myio::read_u8();

        if comando > 4
        {
            eprint!("Comando inválido!");
        }
        else
        {
            break;
        }
    }
    return comando;
}

///Esta funcao serve para alterar os atributos das configuracoes da arquitetura do Amnesia.
///Parametro: configuracoes: Instancia mutavel da struct configuracoes.
fn alterar_configuracoes(configuracoes: &mut arquitetura::Configuracoes)
{
    let configuracao_a_ser_alterada = obter_configuracao_a_ser_alterada();

    if configuracao_a_ser_alterada == 0
    {
        alterar_configuracao_processor(configuracoes);
    }
    else if configuracao_a_ser_alterada == 1
    {
        alterar_configuracao_trace(configuracoes);
    }
    else if configuracao_a_ser_alterada == 2
    {
        alterar_configuracao_cpu(configuracoes);
    }
    else if configuracao_a_ser_alterada == 3
    {
        alterar_configuracao_cache(configuracoes);
    }
    else if configuracao_a_ser_alterada == 4
    {
        alterar_configuracao_main_memory(configuracoes);
    }
    else if configuracao_a_ser_alterada == 5
    {
        alterar_configuracao_virtual_memory(configuracoes);
    }
}

///Esta funcao serve para alterar os atributos do processador.
///Parametro: configuracoes: Instancia mutavel da struct configuracoes.
fn alterar_configuracao_processor(configuracoes: &mut arquitetura::Configuracoes)
{
    //Imprimindo a configuracao atual
    println!("Processor:");
    println!("");
    println!("Configuração atual:");
    println!("{}", configuracoes.processor.to_string());
    println!("-------------------");

    //Obtendo a nova configuracao
    println!("Nova configuração:");
    print!("processor contains:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.processor.set_processor_contains(myio::read_usize());

    print!("create trace file:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.processor.set_create_trace_file(myio::read_usize());

    println!("");

}

///Esta funcao serve para alterar os atributos da trace.
///Parametro: configuracoes: Instancia mutavel da struct configuracoes.
fn alterar_configuracao_trace(configuracoes: &mut arquitetura::Configuracoes)
{
    //Imprimindo a configuracao atual
    println!("Trace:");
    println!("");
    println!("Configuração atual:");
    println!("{}", configuracoes.trace.to_string());
    println!("-------------------");

    //Obtendo a nova configuracao
    println!("Nova configuração:");
    print!("word size:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.set_word_size(myio::read_usize());

    println!("");
}

///Esta funcao serve para alterar os atributos da cpu;.
///Parametro: configuracoes: Instancia mutavel da struct configuracoes.
fn alterar_configuracao_cpu(configuracoes: &mut arquitetura::Configuracoes)
{
    //Imprimindo a configuracao atual
    println!("CPU:");
    println!("");
    println!("Configuração atual:");
    println!("{}", configuracoes.cpu.to_string());
    println!("-------------------");

    //Obtendo a nova configuracao
    println!("Nova configuração:");
    print!("word size:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.set_word_size(myio::read_usize());
}

///Esta funcao serve para alterar os atributos da cache.
///Parametro: configuracoes: Instancia mutavel da struct configuracoes.
fn alterar_configuracao_cache(configuracoes: &mut arquitetura::Configuracoes)
{
    //Imprimindo a configuracao atual
    println!("Cache:");
    println!("");
    println!("Configuração atual:");
    println!("{}", configuracoes.cache.to_string());
    println!("-------------------");

    //Obtendo a nova configuracao
    println!("Nova configuração:");
    configuracoes.cache.set_cache_type(obter_cache_type());
    if configuracoes.cache.get_cache_type() == arquitetura::CacheType::Unified
    {
        print!("line size:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.cache.set_line_size(myio::read_usize());

        print!("cicles per access read:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.cache.set_cicles_per_access_read(myio::read_usize());

        print!("cicles_per_access_write:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.cache.set_cicles_per_access_write(myio::read_usize());

        print!("time cicle:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.cache.set_time_cicle(myio::read_usize());

        print!("memory size:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.cache.set_memory_size(myio::read_usize());

        print!("associativity level:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.cache.set_associativity_level(myio::read_usize());

        configuracoes.cache.set_write_policy(obter_write_policy());
        configuracoes.cache.set_replacement_algorithm(obter_replacement_algorithm("replacement algorithm".to_string()));
    }
}

///Esta funcao serve para alterar os atributos da memoria principal.
///Parametro: configuracoes: Instancia mutavel da struct configuracoes.
fn alterar_configuracao_main_memory(configuracoes: &mut arquitetura::Configuracoes)
{
    //Imprimindo a configuracao atual
    println!("Main Memory:");
    println!("");
    println!("Configuração atual:");
    println!("{}", configuracoes.main_memory.to_string());
    println!("-------------------");

    //Obtendo a nova configuracao
    println!("Nova configuração:");
    print!("block size:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.main_memory.set_block_size(myio::read_usize());

    print!("memory size:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.main_memory.set_memory_size(myio::read_usize());

    print!("cicles per access read:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.main_memory.set_cicles_per_access_read(myio::read_usize());

    print!("cicles per access write:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.main_memory.set_cicles_per_access_write(myio::read_usize());

    print!("time cicle:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.main_memory.set_time_cicle(myio::read_usize());
}

///Esta funcao serve para alterar os atributos da memoria virtual.
///Parametro: configuracoes: Instancia mutavel da struct configuracoes.
fn alterar_configuracao_virtual_memory(configuracoes: &mut arquitetura::Configuracoes)
{
    //Imprimindo a configuracao atual
    println!("Virtual Memory:");
    println!("");
    println!("Configuração atual:");
    println!("{}", configuracoes.virtual_memory.to_string());
    println!("-------------------");

    //Obtendo a nova configuracao
    println!("Nova configuração:");
    print!("page size:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.virtual_memory.set_page_size(myio::read_usize());

    print!("disk memory size:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.virtual_memory.set_disk_memory_size(myio::read_usize());

    print!("disk cicles per access read:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.virtual_memory.set_disk_cicles_per_access_read(myio::read_usize());

    print!("disk cicles per access write:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.virtual_memory.set_disk_cicles_per_access_write(myio::read_usize());

    print!("time cicle:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.virtual_memory.set_time_cicle(myio::read_usize());

    configuracoes.virtual_memory.set_page_table_replacement_algorithm(obter_replacement_algorithm("page table replacement algorithm".to_string()));
    configuracoes.virtual_memory.set_tlb_type(obter_tlb_type());

    if configuracoes.virtual_memory.get_tlb_type() == arquitetura::TLBType::Unified
    {
        print!("memory size:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.virtual_memory.set_memory_size(myio::read_usize());

        print!("cicles per access read:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.virtual_memory.set_cicles_per_access_read(myio::read_usize());

        print!("cicles per access write:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.virtual_memory.set_cicles_per_access_write(myio::read_usize());

        print!("time cicle unified tlb:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.virtual_memory.set_time_cicle_unified_tlb(myio::read_usize());

        configuracoes.virtual_memory.set_replacement_algorithm(obter_replacement_algorithm("replacement algorithm".to_string()));
    }
}

///Esta funcao serve para obter a configuracao a ser alterada do usuario.
///Retorno: u8: Configuracao a ser alterada.
fn obter_configuracao_a_ser_alterada() -> u8
{
    let mut configuracao: u8;

    loop
    {
        println!("Qual configuração deseja alterar?");
        println!("0 - processor");
        println!("1 - trace");
        println!("2 - cpu");
        println!("3 - cache");
        println!("4 - main_memory");
        println!("5 - virtual_memory");

        let entrada = myio::read_u8();

        if entrada > 5
        {
            eprintln!("Entrada inválida!");
        }
        else
        {
            configuracao=entrada;
            break;
        }
    }

    return configuracao;
}

///Esta funcao serve para exibir as configuracoes da arquitetura do amnesia, de forma interativa com o usuario.
///Parametro: configuracoes: Instancia da struct Configuracoes a ser exibida.
fn exibir_configuracoes(configuracoes: &arquitetura::Configuracoes)
{
    let exibir_tudo = obter_modo_de_exibir_configuracoes();

    if exibir_tudo == 1
    {
        println!("");
        println!("{}", configuracoes.to_string());
        println!("");
    }
    else
    {
        let configuracao_a_ser_exibida = obter_configuracao_a_ser_exibinda();

        println!("");

        if configuracao_a_ser_exibida == 0
        {
            println!("Processor:");
            println!("{}", configuracoes.processor.to_string());
        }
        else if configuracao_a_ser_exibida == 1
        {
            println!("Trace:");
            println!("{}", configuracoes.trace.to_string());
        }
        else if configuracao_a_ser_exibida == 2
        {
            println!("CPU:");
            println!("{}", configuracoes.cpu.to_string());
        }
        else if configuracao_a_ser_exibida == 3
        {
            println!("Cache:");
            println!("{}", configuracoes.cache.to_string());
        }
        else if configuracao_a_ser_exibida == 4
        {
            println!("Main Memory:");
            println!("{}", configuracoes.main_memory.to_string());
        }
        else if configuracao_a_ser_exibida == 5
        {
            println!("virtual Memory:");
            println!("{}", configuracoes.virtual_memory.to_string());
        }

        println!("");

    }
}

///Esta funcao serve para obter a configuracao do amnesia que sera exibida, de forma interativa com o usuario.
///Retorno: u8: Numero inteiro unsigned indicando qual configuracao sera exibida.
fn obter_configuracao_a_ser_exibinda() -> u8
{
    let mut configuracao: u8;

    loop
    {
        println!("Qual configuração deseja que seja exibida?");
        println!("0 - processor");
        println!("1 - trace");
        println!("2 - cpu");
        println!("3 - cache");
        println!("4 - main_memory");
        println!("5 - virtual_memory");

        let entrada = myio::read_u8();

        if entrada > 5
        {
            eprintln!("Entrada inválida!");
        }
        else
        {
            configuracao=entrada;
            break;
        }
    }

    return configuracao;
}

///Esta funcao serve para obter o modo de exibicao das configuracoes do amnesia, de forma interativa com o usuario.
///Retorno: u8: Numero inteiro indicando se o usuario deseja exibir todas as configuracoes 0=nao, 1=sim.
fn obter_modo_de_exibir_configuracoes() -> u8
{
    let mut comando: u8;

    loop
    {
        //Exibindo os comandos
        println!("Deseja exibir toda as configurações?");
        println!("1 - Sim.");
        println!("0 - Não");
        let entrada=myio::read_u8();

        if entrada > 1
        {
            eprintln!("Entrada inválida!");
        }
        else
        {
            comando = entrada;
            break;
        }
    }


    return comando;

}

///Esta funcao serve para executar o comando de salvar interativamente com o usuario.
///Parametro: configuracoes: Instancia da struct Configuracoes, contendo todos os dados da arquitetura do amnesia a ser salvos no arquivo.
fn salvar_configuracoes_arquivo(configuracoes: &arquitetura::Configuracoes)
{
    //Obtendo do usuario o nome do arquivo
    let nome_arquivo = obter_nome_arquivo();

    //Criando este arquivo
    let mut arquivo = File::create(nome_arquivo);

    //Caso algum erro ocorra, exibindo mensagem de erro e saindo da funcao.
    if arquivo.is_err()
    {
        eprint!("{}", arquivo.unwrap_err());
        return;
    }

    //Convertendo arquivo do tipo Result para o tipo File
    let mut arquivo = arquivo.unwrap();

    //Gravando configuracoes do amnesia para o arquivo.
    configuracoes.to_file(arquivo);
}

///Esta funcao serve para obter o nome do arquivo interativamente com o usuario.
///Retorno: String: Nome do arquivo.
fn obter_nome_arquivo() -> String
{
    let mut nome_arquivo = String::new();
    loop
    {
        println!("Nome do arquivo");
        let entrada = myio::read_string().trim().to_string();

        if !entrada.is_ascii()
        {
            eprintln!("O nome do arquivo deve estar em ascii")
        }
        else if !entrada.trim().ends_with(".xml")
        {
            eprintln!("O nome do arquivo deve terminar com a extensão .xml");
        }
        else
        {
            nome_arquivo=entrada;
            break;
        }
    }

    //Retornando o nome do arquivo
    return nome_arquivo;
}

///Esta funcao serve para gerar as configuracoes de arquitetura do amnesia de forma interativa com o usuario.
///Parametro: Instancia mutavel da struct Configuracoes a qual sera salvo os dados da arquitetura.
fn criar_configuracoes_arquitetura(configuracoes: &mut arquitetura::Configuracoes)
{
    //Obtendo configuracoes gerais
    println!("Geral:");

    print!("word size:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.set_word_size(myio::read_usize());

    println!("");

    //Obtendo dados do processador
    println!("Processor:");

    print!("processor contains:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.processor.set_processor_contains(myio::read_usize());

    print!("create trace file:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.processor.set_create_trace_file(myio::read_usize());

    println!("");

    //Obtendo dados da memoria cache
    println!("Cache:");

    configuracoes.cache.set_cache_type(obter_cache_type());

    //Obtendo dados para a cache do tipo unified
    if configuracoes.cache.get_cache_type() == arquitetura::CacheType::Unified
    {
        println!("cacheType: Unified");

        print!("line size:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.cache.set_line_size(myio::read_usize());

        print!("cicles per access read:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.cache.set_cicles_per_access_read(myio::read_usize());

        print!("cicles per access write:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.cache.set_cicles_per_access_write(myio::read_usize());

        print!("time cicle:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.cache.set_time_cicle(myio::read_usize());

        print!("memory size:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.cache.set_memory_size(myio::read_usize());

        print!("associativity level:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.cache.set_associativity_level(myio::read_usize());

        configuracoes.cache.set_write_policy(obter_write_policy());
        configuracoes.cache.set_replacement_algorithm(obter_replacement_algorithm("replacementAlgorithm".to_string()));
    }

    println!("");

    //Obtendo dados da memoria ram
    println!("Main Memory:");

    print!("block size:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.main_memory.set_block_size(myio::read_usize());

    print!("memory size:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.main_memory.set_memory_size(myio::read_usize());

    print!("cicles per access read:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.main_memory.set_cicles_per_access_read(myio::read_usize());

    print!("cicles per access write:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.main_memory.set_cicles_per_access_write(myio::read_usize());

    print!("time cicle:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.main_memory.set_time_cicle(myio::read_usize());

    println!("");

    //Obtendo dados da memoria virtual
    println!("Virtual Memory:");

    print!("page size:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.virtual_memory.set_page_size(myio::read_usize());

    print!("disk memory size:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.virtual_memory.set_disk_memory_size(myio::read_usize());

    print!("disk cicles per access read:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.virtual_memory.set_disk_cicles_per_access_read(myio::read_usize());

    print!("disk cicles per access write:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.virtual_memory.set_disk_cicles_per_access_write(myio::read_usize());

    print!("time cicle:");
    io::stdout().flush().expect("flush failed!");
    configuracoes.virtual_memory.set_time_cicle(myio::read_usize());

    configuracoes.virtual_memory.set_page_table_replacement_algorithm(obter_replacement_algorithm("pageTableReplacementAlgorithm".to_string()));

    configuracoes.virtual_memory.set_tlb_type(obter_tlb_type());

    if configuracoes.virtual_memory.get_tlb_type() == arquitetura::TLBType::Unified
    {
        println!("unified TLB:");

        print!("memory size:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.virtual_memory.set_memory_size(myio::read_usize());

        print!("cicles per access read:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.virtual_memory.set_cicles_per_access_read(myio::read_usize());

        print!("cicles per access write:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.virtual_memory.set_cicles_per_access_write(myio::read_usize());

        print!("time cicle:");
        io::stdout().flush().expect("flush failed!");
        configuracoes.virtual_memory.set_time_cicle_unified_tlb(myio::read_usize());

        configuracoes.virtual_memory.set_replacement_algorithm(obter_replacement_algorithm("replacementAlgorithm".to_string()));
    }
}

///Este metodo serve para obter o tipo de TLB de maneira interativa com o usuario.
///Retorno: Enum para o tipo de TLB.
fn obter_tlb_type() -> arquitetura::TLBType
{
    let mut cache_type:arquitetura::TLBType;
    loop
    {
        //Obtendo o tipo de cache
        print!("TLB type (Unified/none):");
        io::stdout().flush().expect("flush failed!");
        let entrada = myio::read_string().to_lowercase().chars().next().unwrap();

        //Validando entrada do usuario
        if entrada == 'u'
        {
            cache_type=arquitetura::TLBType::Unified;
            break;
        }
        else if entrada == 'n'
        {
            cache_type=arquitetura::TLBType::None;
            break;
        }
        else
        {
            eprintln!("cache type inválido");
        }
    }

    //Retornando o tipo de cache
    return cache_type;
}


///Esta funcao serve para obter o tipo da cache, de forma interativa com o usuario.
///Retorno: Enum para o tipo de Cache.
fn obter_cache_type() -> arquitetura::CacheType
{
    let mut cache_type:arquitetura::CacheType;
    loop
    {
        //Obtendo o tipo de cache
        print!("cache type (Unified/Split):");
        io::stdout().flush().expect("flush failed!");
        let entrada = myio::read_string().to_lowercase().chars().next().unwrap();

        //Validando entrada do usuario
        if entrada == 'u'
        {
            cache_type=arquitetura::CacheType::Unified;
            break;
        }
        else if entrada == 's'
        {
            cache_type=arquitetura::CacheType::Split;
            break;
        }
        else
        {
            eprintln!("cache type inválido");
        }
    }

    //Retornando o tipo de cache
    return cache_type;
}

///Esta funcao serve para obter a politica de escrita, de forma interativa com o usuario.
///Retorno: Enum para a politica de escrita.
fn obter_write_policy() -> arquitetura::WritePolicy
{
    let mut write_policy:arquitetura::WritePolicy;
    loop
    {
        //Obtendo a politica de escrita
        print!("write policy (WriteThrough/WriteBack):");
        io::stdout().flush().expect("flush failed!");
        let entrada: Vec<char> = myio::read_string().to_lowercase().chars().collect();

        //Validando entrada do usuario
        if entrada.len() > 5 && entrada[5] == 't'
        {
            write_policy=arquitetura::WritePolicy::WriteThrough;
            break;
        }
        else if entrada.len() > 5 && entrada[5] == 'b'
        {
            write_policy=arquitetura::WritePolicy::WriteBack;
            break;
        }
        else
        {
            eprintln!("write policy inválido");
        }
    }

    //Retornando a politica de escrita
    return write_policy;
}

///Esta funcao serve para obter o algoritmo de substituicao, de maneira interativa com o usuario.
///Parametro: Mensagem a ser exibida com instrucoes para o usuario digitar.
///Retorno: Enum para o algoritmo de substituicao.
fn obter_replacement_algorithm(mensagem: String) -> arquitetura::ReplacementAlgorithm
{
    let mut replacement_algorithm:arquitetura::ReplacementAlgorithm;
    loop
    {
        //Obtendo algoritmo de substituicao
        print!("{} (FIFO/LRU):", mensagem);
        io::stdout().flush().expect("flush failed!");
        let entrada = myio::read_string().to_lowercase().chars().next().unwrap();

        //Validando entrada do usuario
        if entrada == 'f'
        {
            replacement_algorithm = arquitetura::ReplacementAlgorithm::FIFO;
            break;
        }
        else if entrada == 'l'
        {
            replacement_algorithm = arquitetura::ReplacementAlgorithm::LRU;
            break;
        }
        else
        {
            eprintln!("replacement algorithm inválido");
        }
    }

    //Retornando algoritmo de substituicao
    return replacement_algorithm;
}