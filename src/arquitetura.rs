use std::{fs::File, fmt, io::Write};

///Implementacao do Option para a excecao ErroParametroBloqueado.
pub type Option<ErroAtributoBloqueado>= std::option::Option<ErroAtributoBloqueado>;

//Implementacao do Result para a excecao ErroParametroBloqueado.
pub type Result<T, ErroAtributoBloqueado>=std::result::Result<T, ErroAtributoBloqueado>;

///Excecao para quando algum campo de uma struct nao poder ser acessado ou alterado.
#[derive(Debug, Clone)]
pub struct ErroAtributoBloqueado;

///Implementacao da struct ErroAtributoBloqueado.
impl fmt::Display for ErroAtributoBloqueado
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Não é possível obter/alterar o valor deste parametro.")
    }
}

//Enums

///Enum para o tipo de cache.
#[derive(Copy, Clone, PartialEq)]
pub enum CacheType
{
    Unified,
    Split
}

///Enum para a política de escrita.
#[derive(Copy, Clone, PartialEq)]
pub enum WritePolicy
{
    WriteBack,
    WriteThrough
}

///Enum para algoritmo de substituicao.
#[derive(Copy, Clone, PartialEq)]
pub enum ReplacementAlgorithm
{
    LRU,
    FIFO
}

///Enum para o tipo de TLB.
#[derive(Copy, Clone, PartialEq)]
pub enum TLBType
{
    None,
    Unified
}

//Structs

///Struct para a tag Processor, que representa a configuração do processador da maquina do Amnesia.
pub struct Processor
{
    processor_contains: usize,
    create_trace_file: usize
}

///Struct para a tag Trace.
pub struct Trace
{
    word_size: usize
}

///Struct para a tag CPU, que representa a configuração do CPU.
pub struct CPU
{
    word_size:usize
}

///Struct para a tag Cache, que representa a memória cache.
pub struct Cache
{
    cache_type: CacheType,

    //Se cacheType for Unified
    line_size: usize,
    cicles_per_access_read: usize,
    cicles_per_access_write: usize,
    time_cicle: usize,
    memory_size: usize,
    associativity_level: usize,
    write_policy: WritePolicy,
    replacement_algorithm: ReplacementAlgorithm
}

///Struct para a tag MainMemory, que representa as configurações da memória principal.
pub struct MainMemory
{
    block_size: usize,
    memory_size: usize,
    cicles_per_access_read: usize,
    cicles_per_access_write: usize,
    time_cicle: usize
}

///Struct para a tag VirtualMemory, que representa as configurações da memória virtual.
pub struct VirtualMemory
{
    page_size: usize,
    disk_memory_size: usize,
    disk_cicles_per_access_read: usize,
    disk_cicles_per_access_write: usize,
    time_cicle: usize,
    page_table_replacement_algorithm: ReplacementAlgorithm,
    tlb_type: TLBType,

    //Se tlb_type for unified
    memory_size: usize,
    cicles_per_access_read: usize,
    cicles_per_access_write: usize,
    time_cicle_unified_tlb: usize,
    replacement_algorithm: ReplacementAlgorithm
}

///Struct para armazenar todas as configuracoes da maquina do amnesia.
pub struct Configuracoes
{
    pub processor:Processor,
    pub trace:Trace,
    pub cpu:CPU,
    pub cache:Cache,
    pub main_memory:MainMemory,
    pub virtual_memory:VirtualMemory
}

//Implementation

///Métodos para a struct Processor.
impl Processor
{
    ///Este metodo serve para alterar o valor da variavel processor_contains.
    ///Parametro: self: Instancia mutavel da struct Processor.
    ///Parametro: processor_contains: Novo valor para o atributo processor_contains da struct.
    pub fn set_processor_contains(&mut self, processor_contains:usize)
    {
        self.processor_contains = processor_contains;
    }

    ///Este metodo serve para obter o valor da variavel processor_contains.
    ///Parametro: self: Instancia da struct Processor.
    ///Retorno: usize: Valor do atributo processor_contains da struct.
    pub fn get_processor_contains(&self) -> usize
    {
        return self.processor_contains;
    }

    ///Este metodo serve para alterar o valor da variavel create_trace_file.
    ///Parametro: self: Instancia mutavel da struct Processor.
    ///Parametro: create_trace_file: Novo valor para o atributo create_trace_file.
    pub fn set_create_trace_file(&mut self, create_trace_file:usize)
    {
        self.create_trace_file = create_trace_file;
    }

    ///Este metodo serve para obter o valor da variavel get_create_trace_file.
    ///Parametro: self: Instancia da struct Processor.
    ///Retorno: usize: Valor do atributo create_trace_file da struct.
    pub fn get_create_trace_file(&self) -> usize
    {
        return self.create_trace_file;
    }

    ///Este metodo serve para criar uma nova instancia da struct Processor.
    ///Retorno: Processor: Nova instancia da struct Processor.
    pub fn new() -> Processor
    {
        Processor
        {
            processor_contains: 0,
            create_trace_file: 0
        }

    }

    ///Este metodo serve para converter uma instancia da struct Processor em uma String.
    ///Parametro: self: Instancia da struct Processor.
    ///Retorno: String: String contendo os dados da instancia da struct.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados+="processor contains:";
        dados+=&self.get_processor_contains().to_string();
        dados+="\n";
        dados+="create trace file:";
        dados+=&self.get_create_trace_file().to_string();

        return dados;

    }
}

///Métodos para a struct Trace.
impl Trace
{
    ///Este metodo serve para alterar o valor do atributo word_size.
    ///Parametro: self: Instancia mutavel da struct Trace.
    ///Parametor: word_size: Novo valor para o atributo word_size da struct.
    pub fn set_word_size(&mut self, word_size: usize)
    {
        self.word_size = word_size;
    }

    ///Este metodo serve para obter o valor do atributo word_size.
    ///Parametro: self: Instancia da struct Trace.
    ///Retorno: usize: Valor do atributo word_size da struct Trace.
    pub fn get_word_size(&self) -> usize
    {
        return self.word_size;
    }

    ///Este metodo serve para criar uma nova instancia da struct Trace.
    ///Retorno: Trace: Nova instancia da struct Trace.
    pub fn new() -> Trace
    {
        Trace
        {
            word_size: 0
        }
    }

    ///Este metodo serve para converter uma instancia da struct Trace em uma String
    ///Parametro: self: Instancia da struct Trace.
    ///Retorno: String: String contendo os dados da instancia da Struct.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados+="word size:";
        dados+=&self.get_word_size().to_string();

        return dados;
    }
}

///Métodos para a struct CPU.
impl CPU
{
    ///Este metodo serve para alterar o valor do atributo word_size da struct CPU.
    ///Parametro: self: Instancia mutável da struct CPU.
    ///Parametro: word_size: Novo valor para o atributo word_size da struct CPU.
    pub fn set_word_size(&mut self, word_size: usize)
    {
        self.word_size = word_size;
    }

    ///Este metodo serve para obter o valor do atributo word_size da struct CPU.
    ///Parametro: self: Instancia da struct CPU.
    ///Retorno: usize: Valor do atributo word_size da struct CPU.
    pub fn get_word_size(&self) -> usize
    {
        return self.word_size;
    }

    ///Este metodo serve para criar uma nova instancia da struct CPU.
    ///Retorno: CPU: Nova instancia da struct CPU.
    pub fn new() -> CPU
    {
        CPU
        {
            word_size: 0
        }
    }

    ///Este metodo serve para converter uma instancia da struct CPU em uma String.
    ///Parametro: self: Instancia da struct CPU.
    ///Retorno: String: String contendo os dados da instancia da Struct.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados+="word size:";
        dados+=&self.get_word_size().to_string();

        return dados;

    }
}

///Métodos para a struct Cache
impl Cache
{
    ///Este metodo serve para alterar o valor do atributo cache_type da struct Cache.
    ///Parametro: self: Instancia mutável da struct Cache.
    ///Parametro: cache_type: Novo valor para o atributo cache_type da struct Cache.
    pub fn set_cache_type(&mut self, cache_type:CacheType)
    {
        self.cache_type = cache_type;
    }

    ///Este metodo serve para obter o valor do atributo cache_type da struct Cache.
    ///Parametro: self: Instancia da struct Cache.
    ///Retorno: CacheType: Valor do atributo cache_type da struct Cache.
    pub fn get_cache_type(&self) -> CacheType
    {
        return self.cache_type;
    }

    ///Este metodo serve para alterar o valor do atributo line_size da struct Cache.
    ///Parametro: self: Instancia mutável da struct Cache.
    ///Parametro: line_size: Novo valor para o atributo line_size da struct Cache.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn set_line_size(&mut self, line_size: usize) -> Option<ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Some(ErroAtributoBloqueado);
        }

        //Alterando o valor do atributo line_size.
        self.line_size = line_size;

        //Retornando nenhum erro.
        return None;
    }

    ///Este metodo serve para obter o valor do atributo line_size da struct Cache.
    ///Parametro: self: Instancia da struct Cache.
    ///Retorno: Result<usize, ErroAtributoBloqueado>: Valor do atributo line_size da struct Cache.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn get_line_size(&self) -> Result<usize, ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Err(ErroAtributoBloqueado);
        }

        //Caso a cache seja do tipo Unified, retornando o valor do atributo line_size da struct Cache.
        return Ok(self.line_size);
    }

    ///Este metodo serve para alterar o valor do atributo cicles_per_access_read da struct Cache.
    ///Parametro: self: Instancia mutável da struct Cache.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn set_cicles_per_access_read(&mut self, cicles_per_access_read: usize) -> Option<ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Some(ErroAtributoBloqueado);
        }

        //Alterando o valor do atributo.
        self.cicles_per_access_read = cicles_per_access_read;

        //Retornando nenhum erro.
        return None;
    }

    ///Este metodo serve para obter o valor do atributo cicles_per_access_read da struct Cache
    ///Parametro: self: Instancia da struct Cache.
    ///Retorno:
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn get_cicles_per_access_read(&self) -> Result<usize, ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Err(ErroAtributoBloqueado);
        }

        //Caso a cache seja do tipo Unified, retornando o valor do atributo cicles_per_access_read.
        return Ok(self.cicles_per_access_read);
    }

    ///Este metodo serve para alterar o valor do atributo cicles_per_access_write da struct Cache.
    ///Parametro: self: Instancia mutável da struct Cache.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn set_cicles_per_access_write(&mut self, cicles_per_access_write: usize) -> Option<ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Some(ErroAtributoBloqueado);
        }

        //Alterando o valor do atributo.
        self.cicles_per_access_write = cicles_per_access_write;

        //Retornando nenhum erro.
        return None;
    }

    ///Este metodo serve para obter o valor do atributo cicles_per_access_write da struct Cache.
    ///Parametro: self: Instancia da struct Cache.
    ///Retorno: Caso nenhum erro ocorra, valor do atributo cicles_per_access_write da struct Cache.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn get_cicles_per_access_write(&self) -> Result<usize, ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Err(ErroAtributoBloqueado);
        }

        //Caso a cache seja do tipo Unified, retornando o valor do atributo cicles_per_access_read
        return Ok(self.cicles_per_access_write);
    }

    ///Este metodo serve para alterar o valor do atributo time_cicle da struct Cache.
    ///Parametro: self: Instancia mutável da struct Cache.
    ///Parametro: time_cicle: Novo valor para o atributo time_cicle da struct Cache.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn set_time_cicle(&mut self, time_cicle: usize) -> Option<ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Some(ErroAtributoBloqueado);
        }

        //Alterando o valor do atributo.
        self.time_cicle=time_cicle;


        //Retornando nenhum erro.
        return None;
    }

    ///Este metodo serve para obter o valor do atributo time_cicle da struct Cache.
    ///Parametro: self: Instancia da struct Cache.
    ///Retorno: Caso nenhum erro ocorra, o valor do atributo time_cicle da struct Cache.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn get_time_cicle(&self) -> Result<usize, ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Err(ErroAtributoBloqueado);
        }

        //Caso a cache seja do tipo Unified, retornando atributo time_cicle
        return Ok(self.time_cicle);
    }

    ///Este metodo serve para alterar o valor do atributo memory_size da struct Cache.
    ///Parametro: self: Instancia mutável da struct Cache.
    ///Parametro: memory_size: Novo valor para o atributo memory_size da struct Cache.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn set_memory_size(&mut self, memory_size: usize) -> Option<ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Some(ErroAtributoBloqueado);
        }

        //Alterando o valor do atributo.
        self.memory_size=memory_size;


        //Retornando nenhum erro.
        return None;
    }

    ///Este metodo serve para obter o valor do atributo memory_size da struct Cache.
    ///Parametro: self: Instancia da struct Cache.
    ///Retorno: Result<usize, ErroAtributoBloqueado>: O valor do atributo memory_size.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn get_memory_size(&self) -> Result<usize, ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Err(ErroAtributoBloqueado);
        }

        //Caso a cache seja do tipo Unified, retornando atributo memory_size
        return Ok(self.memory_size);
    }

    ///Este metodo serve para alterar o valor do atributo memory_size da struct Cache.
    ///Parametro: self: Instancia mutavel da struct Cache.
    ///Paramtero: associativity_level: Novo valor para o atributo associativity_level da struct Cache.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn set_associativity_level(&mut self, associativity_level: usize) -> Option<ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Some(ErroAtributoBloqueado);
        }

        self.associativity_level = associativity_level;

        //Retornando nenhum erro.
        return None;
    }

    ///Este metodo serve para obter o valor do atributo associativity_level da struct Cache
    ///Parametro: self: Instancia da struct Cache.
    ///Retorno: Result<usize, ErroAtributoBloqueado>: O valor do atributo associativity_level.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn get_associativity_level(&self) -> Result<usize, ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Err(ErroAtributoBloqueado);
        }

        //Caso a cache seja do tipo Unified, retornando atributo associativity_level
        return Ok(self.associativity_level);
    }

    ///Este metodo serve para alterar o valor do atributo write_policy da struct Cache.
    ///Parametro: self: Instancia mutavel da struct Cache.
    ///Parametro: write_policy: Novo valor para o atributo write_policy da struct Cache.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn set_write_policy(&mut self, write_policy: WritePolicy) -> Option<ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Some(ErroAtributoBloqueado);
        }

        //Alterando valor do atributo
        self.write_policy = write_policy;

        //Retornando nenhum erro.
        return None;
    }

    ///Este metodo serve para obter o valor do atributo write_policy da struct Cache.
    ///Parametro: self: Instancia da struct Cache.
    ///Retorno: Result<WritePolicy, ErroAtributoBloqueado>: O valor do atributo write_policy.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn get_write_policy(&self) -> Result<WritePolicy, ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Err(ErroAtributoBloqueado);
        }

        //Caso a cache seja do tipo Unified, retornando atributo write_policy
        return Ok(self.write_policy);
    }

    ///Este metodo serve para alterar o valor do atributo replacement_algorithm da struct Cache.
    ///Parametro: self: Instancia mutavel da struct Cache.
    ///Parametro: replacement_algorithm: Novo valor para o atributo replacement_algorithm da struct Cache.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn set_replacement_algorithm(&mut self, replacement_algorithm: ReplacementAlgorithm) -> Option<ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Some(ErroAtributoBloqueado);
        }

        //Alterando valor do atributo
        self.replacement_algorithm=replacement_algorithm;

        //Retornando nenhum erro.
        return None;
    }

    ///Este metodo serve para obter o valor do atributo replacement_algorithm da struct cache.
    ///Parametro: self: Instancia da struct Cache.
    ///Retorno: Result<ReplacementAlgorithm, ErroAtributoBloqueado>: O valor do atributo replacement_algorithm.
    ///Erro: Caso a cache nao seja Unified, sera retornado um erro.
    pub fn get_replacement_algorithm(&self) -> Result<ReplacementAlgorithm, ErroAtributoBloqueado>
    {
        //Caso a cache não seja do tipo Unified, este metodo retornara um erro.
        if self.cache_type!=CacheType::Unified
        {
            return Err(ErroAtributoBloqueado);
        }

        //Caso a cache seja do tipo Unified, retornando atributo replacement_algorithm
        return Ok(self.replacement_algorithm);
    }

    ///Este metodo serve para criar uma nova instancia da struct cache.
    ///Retorno: Cache: Nova instancia da struct cache.
    pub fn new() -> Cache
    {
        Cache
        {
            cache_type: CacheType::Unified,
            line_size: 0,
            cicles_per_access_read: 0,
            cicles_per_access_write: 0,
            time_cicle: 0,
            memory_size: 0,
            associativity_level: 0,
            write_policy: WritePolicy::WriteThrough,
            replacement_algorithm: ReplacementAlgorithm::FIFO
        }
    }

    ///Este metodo serve para converter uma instancia da struct Cache em uma String.
    ///Parametro: self: Instancia da struct Cache.
    ///Retorno: String: String contendo os dados da instancia da Struct.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados+= "cache type:";
        match self.get_cache_type()
        {
            CacheType::Unified => dados+="unified",
            CacheType::Split => dados+="split"
        }

        //Se cacheType for Unified
        if self.get_cache_type() == CacheType::Unified
        {
            dados+="\n";

            dados+="line size:";
            dados+=&self.get_line_size().unwrap().to_string();
            dados+="\n";

            dados+="cicles per access read:";
            dados+=&self.get_cicles_per_access_read().unwrap().to_string();
            dados+="\n";

            dados+="cicles per access write:";
            dados+=&self.get_cicles_per_access_write().unwrap().to_string();
            dados+="\n";

            dados+="time cicle:";
            dados+=&self.get_time_cicle().unwrap().to_string();
            dados+="\n";

            dados+="memory size:";
            dados+=&self.get_memory_size().unwrap().to_string();
            dados+="\n";

            dados+="associativity level:";
            dados+=&self.get_associativity_level().unwrap().to_string();
            dados+="\n";

            dados+="write policy:";
            match self.get_write_policy().unwrap()
            {
                WritePolicy::WriteBack => dados+="WriteBack",
                WritePolicy::WriteThrough => dados+="WriteThrough"
            }
            dados+="\n";

            dados+="replacement algorithm:";
            match self.get_replacement_algorithm().unwrap()
            {
                ReplacementAlgorithm::FIFO => dados+="FIFO",
                ReplacementAlgorithm::LRU => dados+="LRU"
            }

        }

        return dados;
    }
}

///Metodos para a struct MainMemory
impl MainMemory
{
    ///Este metodo serve para alterar o valor do atributo block_size da struct MainMemory.
    ///Parametro: self: Instancia mutavel da struct MainMemory.
    ///Parametro: block_size: Novo valor para o atributo block_size.
    pub fn set_block_size(&mut self, block_size:usize)
    {
        self.block_size = block_size;
    }

    ///Este metodo serve para obter o valor do atributo block_size da struct MainMemeory.
    ///Parametro: self: Instancia da struct MainMemory.
    ///Retorno: usize: O valor do atributo block_size.
    pub fn get_block_size(&self) -> usize
    {
        return self.block_size;
    }

    ///Este metodo serve para alterar o valor do atributo memory_size da struct MainMemory.
    ///Parametro: self: Instancia mutavel da struct MainMemory.
    ///Parametro:
    pub fn set_memory_size(&mut self, memory_size:usize)
    {
        self.memory_size = memory_size;
    }

    ///Este metodo serve para obter o valor do atributo memory_size da struct MainMemeory.
    ///Parametro: self: Instancia da struct MainMemory.
    ///Retorno: usize: O valor do atributo memory_size.
    pub fn get_memory_size(&self) -> usize
    {
        return self.memory_size;
    }

    ///Este metodo serve para alterar o valor do atributo cicles_per_access_read da struct MainMemory.
    ///Parametro: self: Instancia mutavel da struct MainMemory.
    ///Parametro: usize: cicles_per_access_read: Novo valor para o atributo cicles_per_access_write.
    pub fn set_cicles_per_access_read(&mut self, cicles_per_access_read:usize)
    {
        self.cicles_per_access_read = cicles_per_access_read;
    }

    ///Este metodo serve para obter o valor do atributo cicles_per_access_read da struct MainMemeory.
    ///Parametro: self: Instancia da struct MainMemory.
    ///Retorno: usize: O valor do atributo cicles_per_access_read.
    pub fn get_cicles_per_access_read(&self) -> usize
    {
        return self.cicles_per_access_read;
    }

    ///Este metodo serve para alterar o valor do atributo cicles_per_access_write da struct MainMemory.
    ///Parametro: self: Instancia mutavel da struct MainMemory.
    ///Parametro: cicles_per_access_write: Novo valor para o atributo cicles_per_access_write.
    pub fn set_cicles_per_access_write(&mut self, cicles_per_access_write:usize)
    {
        self.cicles_per_access_write = cicles_per_access_write;
    }

    ///Este metodo serve para obter o valor do atributo cicles_per_access_write da struct MainMemeory.
    ///Parametro: self: Instancia da struct MainMemory.
    ///Retorno: usize: O valor do atributo cicles_per_access_write.
    pub fn get_cicles_per_access_write(&self) -> usize
    {
        return self.cicles_per_access_write;
    }

    ///Este metodo serve para alterar o valor do atributo time_cicle da struct MainMemory.
    ///Parametro: self: Instancia mutavel da struct MainMemory.
    ///Parametro: time_cicle: Novo valor para o atributo time_cicle.
    pub fn set_time_cicle(&mut self, time_cicle:usize)
    {
        self.time_cicle = time_cicle;
    }

    ///Este metodo serve para obter o valor do atributo time_cicle da struct MainMemeory.
    ///Parametro: self: Instancia da struct MainMemory.
    ///Retorno: usize: O valor do atributo time_cicle.
    pub fn get_time_cicle(&self) -> usize
    {
        return self.time_cicle;
    }

    ///Este metodo serve para criar uma nova instancia da struct MainMemory.
    ///Retorno: MainMemory: Nova instancia da struct MainMemory.
    pub fn new() -> MainMemory
    {
        MainMemory
        {
            block_size: 0,
            memory_size: 0,
            cicles_per_access_read: 0,
            cicles_per_access_write: 0,
            time_cicle: 0
        }
    }

    ///Este metodo serve para converter uma instancia da struct MainMemory em uma String.
    ///Parametro: self: Instancia da struct MainMemory.
    ///Retorno: String: String contendo os dados da instancia da struct.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados+="block size:";
        dados+=&self.get_block_size().to_string();
        dados+="\n";

        dados+="memory size:";
        dados+=&self.get_memory_size().to_string();
        dados+="\n";

        dados+="cicles per access read:";
        dados+=&self.get_cicles_per_access_read().to_string();
        dados+="\n";

        dados+="cicles per access write:";
        dados+=&self.get_cicles_per_access_write().to_string();
        dados+="\n";

        dados+="time cicle:";
        dados+=&self.get_time_cicle().to_string();

        return dados;
    }

}

///Metodos para a struct VirtualMemory
impl VirtualMemory
{
    ///Este metodo serve para alterar o valor do atributo page_size da struct VirtualMemory.
    ///Parametro: self: Instancia mutavel da struct VirtualMemory.
    ///Parametro: page_size: Novo valor para o atributo page_size.
    pub fn set_page_size(&mut self, page_size:usize)
    {
        self.page_size = page_size;
    }

    ///Este metodo serve para obter o valor do atributo page_size da struct VirtualMemory.
    ///Parametro: self: Instancia da struct VirtualMemory.
    ///Retorno: usize: Valor do atributo page_size.
    pub fn get_page_size(&self) -> usize
    {
        return self.page_size;
    }

    ///Este metodo serve para alterar o valor do atributo disk_memory_size da struct VirtualMemory.
    ///Parametro: self: Instancia mutavel da struct VirtualMemory.
    ///Parametro: disk_memory_size: Novo valor para o atributo disk_memory_size.
    pub fn set_disk_memory_size(&mut self, disk_memory_size: usize)
    {
        self.disk_memory_size = disk_memory_size;
    }

    ///Este metodo serve para obter o valor do atributo disk_memory_size da struct VirtualMemory.
    ///Parametro: self: Instancia da struct VirtualMemory.
    ///Retorno: usize: Valor do atributo disk_memory_size.
    pub fn get_disk_memory_size(&self) -> usize
    {
        return self.disk_memory_size;
    }

    ///Este metodo serve para alterar o valor do atributo disk_cicles_per_access_read da struct VirtualMemory.
    ///Parametro: self: Instancia mutavel da struct VirtualMemory.
    ///Parametro: disk_cicles_per_access_read: Novo valor para o atributo disk_cicles_per_access_read.
    pub fn set_disk_cicles_per_access_read(&mut self, disk_cicles_per_access_read: usize)
    {
        self.disk_cicles_per_access_read = disk_cicles_per_access_read;
    }

    ///Este metodo serve para obter o valor do atributo disk_cicles_per_access_read da struct VirtualMemory.
    ///Parametro: self: Instancia da struct VirtualMemory.
    ///Retorno: usize: Valor do atributo disk_cicles_per_access_read.
    pub fn get_disk_cicles_per_access_read(&self) -> usize
    {
        return self.disk_cicles_per_access_read;
    }

    ///Este metodo serve para alterar o valor do atributo disk_cicles_per_access_write da struct VirtualMemory.
    ///Parametro: self: Instancia mutavel da struct VirtualMemory.
    ///Parametro: disk_cicles_per_access_write: Novo valor para o atributo disk_cicles_per_access_write.
    pub fn set_disk_cicles_per_access_write(&mut self, disk_cicles_per_access_write: usize)
    {
        self.disk_cicles_per_access_write=disk_cicles_per_access_write;
    }

    ///Este metodo serve para obter o valor do atributo disk_cicles_per_access_write da struct VirtualMemory.
    ///Parametro: self: Instancia da struct VirtulMemory.
    ///Retorno: usize: Valor do atributo disk_cicles_per_access_write.
    pub fn get_disk_cicles_per_access_write(&self) -> usize
    {
        return self.disk_cicles_per_access_write;
    }

    ///Este metodo serve para alterar o valor do atributo time_cicle da struct VirtualMemory.
    ///Parametro: self: Instancia mutavel da struct VirtualMemory.
    ///Parametro: time_cicle: Novo valor para o atributo time_cicle.
    pub fn set_time_cicle(&mut self, time_cicle:usize)
    {
        self.time_cicle = time_cicle;
    }

    ///Este metodo serve para obter o valor do atributo time_cicle da struct VirtualMemory.
    ///Parametro: self: Instancia da struct VirtualMemory.
    ///Retorno: usize: Valor do atributo time_cicle.
    pub fn get_time_cicle(&self) -> usize
    {
        return self.time_cicle;
    }

    ///Este metodo serve para alterar o valor do atributo page_table_replacement_algorithm.
    ///Parametro: self: Instancia mutavel da struct VirtualMemory.
    ///Parametro: page_table_replacement_algorithm: Novo valor para o atributo page_table_replacement_algorithm.
    pub fn set_page_table_replacement_algorithm(&mut self, page_table_replacement_algorithm:ReplacementAlgorithm)
    {
        self.page_table_replacement_algorithm = page_table_replacement_algorithm;
    }

    ///Este metodo serve para obter o valor do atributo page_table_replacement_algorithm.
    ///Parametro: self: Instancia da struct VirtualMemory.
    ///Retorno: ReplacementAlgorithm: Valor do atributo page_table_replacement_algorithm.
    pub fn get_page_table_replacement_algorithm(&self) -> ReplacementAlgorithm
    {
        return self.page_table_replacement_algorithm;
    }

    ///Este metodo serve para alterar o valor do atributo tlb_type.
    ///Parametro: self: Instancia mutavel da struct VirtualMemory.
    ///Parametro: tlb_type: Novo valor para o atributo tlb_type.
    pub fn set_tlb_type(&mut self, tlb_type: TLBType)
    {
        self.tlb_type = tlb_type;
    }

    ///Este metodo serve para obter o valor do atributo tlb_type.
    ///Parametro: self: Instancia da struct VirtualMemory.
    ///Retorno: TLBType: Valor do atributo tlb_type.
    pub fn get_tlb_type(&self) -> TLBType
    {
        return self.tlb_type;
    }

    ///Este metodo serve para alterar o valor do atributo memory_size.
    ///Parametro: self: Instancia mutavel da struct VirtualMemory.
    ///Parametro: memory_size: Novo valor para o atributo memory_size.
    ///Erro: Caso a tlb_type nao seja Unified, sera retornado um erro.
    pub fn set_memory_size(&mut self, memory_size: usize) -> Option<ErroAtributoBloqueado>
    {
        //Caso o tlb_type for diferente de Unified, sera retornado um erro.
        if self.tlb_type!=TLBType::Unified
        {
            return Some(ErroAtributoBloqueado);
        }

        //Alterando o valor do atributo.
        self.memory_size = memory_size;

        //Retornando nenhum erro.
        return None;
    }

    ///Este metodo serve para obter o valor do atributo memory_size.
    ///Parametro: self: Instancia da struct VirtualMemory.
    ///Retorno: Result<usize, ErroAtributoBloqueado>: Valor do atributo memory_size.
    ///Erro: Caso a tlb_type nao seja Unified, sera retornado um erro.
    pub fn get_memory_size(&self) -> Result<usize, ErroAtributoBloqueado>
    {
        //Caso o tlb_type for diferente de Unified, sera retornado um erro.
        if self.tlb_type!=TLBType::Unified
        {
            return Err(ErroAtributoBloqueado);
        }

        //Retornando o valor do atributo.
        return Ok(self.memory_size);
    }

    ///Este metodo serve para alterar o valor do atributo cicles_per_access_read.
    ///Parametro: self: Instancia mutavel da struct VirtualMemory.
    ///Parametro: cicles_per_access_read: Novo valor para o atributo cicles_per_access_read.
    ///Erro: Caso a tlb_type nao seja Unified, sera retornado um erro.
    pub fn set_cicles_per_access_read(&mut self, cicles_per_access_read: usize) -> Option<ErroAtributoBloqueado>
    {
        //Caso o tlb_type for diferente de Unified, sera retornado um erro.
        if self.tlb_type!=TLBType::Unified
        {
            return Some(ErroAtributoBloqueado);
        }

        //Alterando o valor do atributo.
        self.cicles_per_access_read = cicles_per_access_read;

        //Retornando nenhum erro.
        return None;
    }

    ///Este metodo serve para obter o valor do atributo cicles_per_access_read.
    ///Parametro: self: Instancia da struct VirtualMemory.
    ///Retorno: Result<usize, ErroAtributoBloqueado>: Valor do atributo cicles_per_access_read.
    ///Erro: Caso a tlb_type nao seja Unified, sera retornado um erro.
    pub fn get_cicles_per_access_read(&self) -> Result<usize, ErroAtributoBloqueado>
    {
        //Caso o tlb_type for diferente de Unified, sera retornado um erro.
        if self.tlb_type!=TLBType::Unified
        {
            return Err(ErroAtributoBloqueado);
        }

        //Retornando o valor do atributo.
        return Ok(self.cicles_per_access_read);
    }

    ///Este metodo serve para alterar o valor do atributo cicles_per_access_write.
    ///Parametro: self: Instancia mutavel da struct VirtualMemory.
    ///Parametro: cicles_per_access_write: Novo valor para o atributo cicles_per_access_write.
    ///Erro: Caso a tlb_type nao seja Unified, sera retornado um erro.
    pub fn set_cicles_per_access_write(&mut self, cicles_per_access_write:usize) -> Option<ErroAtributoBloqueado>
    {
        //Caso o tlb_type for diferente de Unified, sera retornado um erro.
        if self.tlb_type!=TLBType::Unified
        {
            return Some(ErroAtributoBloqueado);
        }

        //Alterando o valor do atributo.
        self.cicles_per_access_write = cicles_per_access_write;

        //Retornando nenhum erro.
        return None;
    }

    ///Este metodo serve para obter o valor do atributo cicles_per_access_write.
    ///Parametro: self: Instancia da struct VirtualMemory.
    ///Retorno: Result<usize, ErroAtributoBloqueado>: Valor do atributo cicles_per_access_write.
    ///Erro: Caso a tlb_type nao seja Unified, sera retornado um erro.
    pub fn get_cicles_per_access_write(&self) -> Result<usize, ErroAtributoBloqueado>
    {
        //Caso o tlb_type for diferente de Unified, sera retornado um erro.
        if self.tlb_type!=TLBType::Unified
        {
            return Err(ErroAtributoBloqueado);
        }

        //Retornando o valor do atributo.
        return Ok(self.cicles_per_access_write);
    }

    ///Este metodo serve para alterar o valor do atributo time_cicle_unified_tlb.
    ///Parametro: self: Instancia mutavel da struct VirtualMemory.
    ///Parametro: time_cicle_unified_tlb: Novo valor para o atributo time_cicle_unified_tlb.
    ///Erro: Caso a tlb_type nao seja Unified, sera retornado um erro.
    pub fn set_time_cicle_unified_tlb(&mut self, time_cicle_unified_tlb:usize) -> Option<ErroAtributoBloqueado>
    {
        //Caso o tlb_type for diferente de Unified, sera retornado um erro.
        if self.tlb_type!=TLBType::Unified
        {
            return Some(ErroAtributoBloqueado);
        }

        //Alterando o valor do atributo.
        self.time_cicle_unified_tlb = time_cicle_unified_tlb;

        //Retornando nenhum erro.
        return None;
    }

    ///Este metodo serve para obter o valor do atributo time_cicle_unified_tlb.
    ///Parametro: self: Instancia da struct VirtualMemory.
    ///Retorno: Result<usize, ErroAtributoBloqueado>: Valor do atributo time_cicle_unified_tlb .
    ///Erro: Caso a tlb_type nao seja Unified, sera retornado um erro.
    pub fn get_time_cicle_unified_tlb(&self) -> Result<usize, ErroAtributoBloqueado>
    {
        //Caso o tlb_type for diferente de Unified, sera retornado um erro.
        if self.tlb_type!=TLBType::Unified
        {
            return Err(ErroAtributoBloqueado);
        }

        //Retornando o valor do atributo.
        return Ok(self.time_cicle_unified_tlb);
    }

    ///Este metodo serve para alterar o valor do atributo replacement_algorithm.
    ///Parametro: self: Instancia mutavel da struct VirtualMemory.
    ///Parametro: replacement_algorithm: Novo valor para o atributo replacement_algorithm.
    ///Erro: Caso a tlb_type nao seja Unified, sera retornado um erro.
    pub fn set_replacement_algorithm(&mut self, replacement_algorithm:ReplacementAlgorithm) -> Option<ErroAtributoBloqueado>
    {
        //Caso o tlb_type for diferente de Unified, sera retornado um erro.
        if self.tlb_type!=TLBType::Unified
        {
            return Some(ErroAtributoBloqueado);
        }

        //Alterando valor do atributo.
        self.replacement_algorithm = replacement_algorithm;

        //Retornando nenhum erro.
        return None;
    }

    ///Este metodo serve para obter o valor do atributo replacement_algorithm.
    ///Parametro: self: Instancia da struct VirtualMemory.
    ///Retorno: Result<ReplacementAlgorithm, ErroAtributoBloqueado>: Novo valor para o atributo replacement_algorithm.
    ///Erro: Caso a tlb_type nao seja Unified, sera retornado um erro.
    pub fn get_replacement_algorithm(&self) -> Result<ReplacementAlgorithm, ErroAtributoBloqueado>
    {
        //Caso o tlb_type for diferente de Unified, sera retornado um erro.
        if self.tlb_type!=TLBType::Unified
        {
            return Err(ErroAtributoBloqueado);
        }

        //Retornando o valor do atributo.
        return Ok(self.replacement_algorithm);
    }

    ///Este metodo serve para criar uma nova instancia da struct VirtualMemory.
    ///Retorno: VirtualMemory: Nova instancia da struct VirtualMemory.
    pub fn new() -> VirtualMemory
    {
        VirtualMemory
        {
            page_size: 0,
            disk_memory_size: 0,
            disk_cicles_per_access_read: 0,
            disk_cicles_per_access_write: 0,
            time_cicle: 0,
            page_table_replacement_algorithm: ReplacementAlgorithm::FIFO,
            tlb_type: TLBType::None,

            //Se tlb_type for unified
            memory_size: 0,
            cicles_per_access_read: 0,
            cicles_per_access_write: 0,
            time_cicle_unified_tlb: 0,
            replacement_algorithm: ReplacementAlgorithm::FIFO
        }
    }

    ///Este metodo serve para converter uma instancia da struct VirtualMemory em uma String.
    ///Parametro: self: Instancia da struct VirtualMemory.
    ///Retorno: String: String contendo os dados da instancia da struct.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados+="page size:";
        dados+=&self.get_page_size().to_string();
        dados+="\n";

        dados+="disk memory size:";
        dados+=&self.get_disk_memory_size().to_string();
        dados+="\n";

        dados+="disk cicles per access read:";
        dados+=&self.get_disk_cicles_per_access_read().to_string();
        dados+="\n";

        dados+="disk cicles per access write:";
        dados+=&self.get_disk_cicles_per_access_write().to_string();
        dados+="\n";

        dados+="time cicle:";
        dados+=&self.get_time_cicle().to_string();
        dados+="\n";

        dados+="page table replacement algorithm:";
        match self.get_page_table_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados+="FIFO",
            ReplacementAlgorithm::LRU => dados+="LRU"
        }
        dados+="\n";

        dados+="tlb type:";
        match self.get_tlb_type()
        {
            TLBType::Unified => dados+="unified",
            TLBType::None => dados+="none"
        }

        //Se tlb_type for unified
        if self.get_tlb_type() == TLBType::Unified
        {
            dados+="\n";

            dados+="memory size:";
            dados+=&self.get_memory_size().unwrap().to_string();
            dados+="\n";

            dados+="cicles per access read:";
            dados+=&self.get_cicles_per_access_read().unwrap().to_string();
            dados+="\n";

            dados+="cicles per access write:";
            dados+=&self.get_cicles_per_access_write().unwrap().to_string();
            dados+="\n";

            dados+="time cicle unified tlb:";
            dados+=&self.get_time_cicle_unified_tlb().unwrap().to_string();
            dados+="\n";

            dados+="replacement algorithm:";
            match self.get_replacement_algorithm().unwrap()
            {
                ReplacementAlgorithm::FIFO => dados+="FIFO",
                ReplacementAlgorithm::LRU => dados+="LRU"
            }
        }

        return dados;
    }
}

///Metodos da struct Configuracoes
impl Configuracoes
{
    ///Este metodo serve para criar uma nova instancia da struct Configuracoes.
    ///Retorno: Nova instancia da struct Configuracoes.
    pub fn new()-> Configuracoes
    {
        Configuracoes
        {
            processor:Processor::new(),
            trace:Trace::new(),
            cpu:CPU::new(),
            cache:Cache::new(),
            main_memory:MainMemory::new(),
            virtual_memory:VirtualMemory::new()
        }
    }

    ///Este metodo serve para escrever as configuracoes do amnesia em um arquivo.
    ///Parametro: self: Instancia da struct Configuracoes.
    ///Parametro: arquivo: Instancia mutavel do objeto arquivo.
    pub fn to_file(&self, mut arquivo: File)
    {
        //Gerando a string
        let mut dados_para_arquivo = String::new();

        //Gerando o cabecalho
        dados_para_arquivo+= "<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n";
        dados_para_arquivo+="\n";
        dados_para_arquivo+="<!DOCTYPE AmnesiaConfiguration SYSTEM \"Configuration/amnesia.dtd\">\n";
        dados_para_arquivo+="<?xml-stylesheet type=\"text/css\" href=\"teste.css\"?>\n";
        dados_para_arquivo+="<AmnesiaConfiguration>\n";

        //Gerando as informacoes do processador
        dados_para_arquivo+="\t<Processor>\n";
        dados_para_arquivo+="\t\t<processorContains>";
        dados_para_arquivo+=&self.processor.get_processor_contains().to_string();
        dados_para_arquivo+="</processorContains>\n";
        dados_para_arquivo+="\t\t<createTraceFile>";
        dados_para_arquivo+=&self.processor.get_create_trace_file().to_string();
        dados_para_arquivo+="</createTraceFile>\n";
        dados_para_arquivo+="\t</Processor>\n";

        //Gerando as informacoes do trace
        dados_para_arquivo+="\t<Trace>\n";
        dados_para_arquivo+="\t\t<wordSize>";
        dados_para_arquivo+=&self.trace.get_word_size().to_string();
        dados_para_arquivo+="</wordSize>\n";
        dados_para_arquivo+="\t</Trace>\n";

        //Gerando as informacoes do cpu
        dados_para_arquivo+="\t<CPU>\n";
        dados_para_arquivo+="\t\t<wordSize>";
        dados_para_arquivo+=&self.cpu.get_word_size().to_string();
        dados_para_arquivo+="</wordSize>\n";
        dados_para_arquivo+="\t</CPU>\n";

        //Gerando as informacoes do memoria ram
        dados_para_arquivo+="\t<MainMemory>\n";
        dados_para_arquivo+="\t\t<blockSize>";
        dados_para_arquivo+=&self.main_memory.get_block_size().to_string();
        dados_para_arquivo+="</blockSize>\n";
        dados_para_arquivo+="\t\t<memorySize>";
        dados_para_arquivo+=&self.main_memory.get_memory_size().to_string();
        dados_para_arquivo+="</memorySize>\n";
        dados_para_arquivo+="\t\t<ciclesPerAccessRead>";
        dados_para_arquivo+=&self.main_memory.get_cicles_per_access_read().to_string();
        dados_para_arquivo+="</ciclesPerAccessRead>\n";
        dados_para_arquivo+="\t\t<ciclesPerAccessWrite>";
        dados_para_arquivo+=&self.main_memory.get_cicles_per_access_write().to_string();
        dados_para_arquivo+="</ciclesPerAccessWrite>\n";
        dados_para_arquivo+="\t\t<timeCicle>";
        dados_para_arquivo+=&self.main_memory.get_time_cicle().to_string();
        dados_para_arquivo+="</timeCicle>\n";
        dados_para_arquivo+="\t</MainMemory>\n";

        //Gerando as informacoes da memoria cache
        dados_para_arquivo+="\t<Cache>\n";
        dados_para_arquivo+="\t\t<cacheType>";
        match self.cache.get_cache_type()
        {
            CacheType::Unified => dados_para_arquivo+="Unified",
            CacheType::Split=> dados_para_arquivo+="Split"
        }
        dados_para_arquivo+="</cacheType>\n";
        if self.cache.get_cache_type() == CacheType::Unified
        {
            dados_para_arquivo+="\t\t<unifiedCache>\n";
            dados_para_arquivo+="\t\t\t<lineSize>";
            dados_para_arquivo+=&self.cache.get_line_size().unwrap().to_string();
            dados_para_arquivo+="</lineSize>\n";
            dados_para_arquivo+="\t\t\t<ciclesPerAccessRead>";
            dados_para_arquivo+=&self.cache.get_cicles_per_access_read().unwrap().to_string();
            dados_para_arquivo+="</ciclesPerAccessRead>\n";
            dados_para_arquivo+="\t\t\t<ciclesPerAccessWrite>";
            dados_para_arquivo+=&self.cache.get_cicles_per_access_write().unwrap().to_string();
            dados_para_arquivo+="</ciclesPerAccessWrite>\n";
            dados_para_arquivo+="<timeCicle>";
            dados_para_arquivo+=&self.cache.get_time_cicle().unwrap().to_string();
            dados_para_arquivo+="</timeCicle>\n";
            dados_para_arquivo+="\t\t\t<memorySize>";
            dados_para_arquivo+=&self.cache.get_memory_size().unwrap().to_string();
            dados_para_arquivo+="</memorySize>\n";
            dados_para_arquivo+="\t\t\t<associativityLevel>";
            dados_para_arquivo+=&self.cache.get_associativity_level().unwrap().to_string();
            dados_para_arquivo+="</associativityLevel>\n";
            dados_para_arquivo+="\t\t\t<writePolicy>";
            match self.cache.get_write_policy().unwrap()
            {
                WritePolicy::WriteThrough=>dados_para_arquivo+="WriteThrough",
                WritePolicy::WriteBack=>dados_para_arquivo+="WriteBack"
            }
            dados_para_arquivo+="</writePolicy>\n";
            dados_para_arquivo+="\t\t\t<replacementAlgorithm>";
            match self.cache.get_replacement_algorithm().unwrap()
            {
                ReplacementAlgorithm::FIFO => dados_para_arquivo+="FIFO",
                ReplacementAlgorithm::LRU => dados_para_arquivo+="LRU"
            }
            dados_para_arquivo+="</replacementAlgorithm>\n";
            dados_para_arquivo+="\t\t</unifiedCache>\n";
        }
        dados_para_arquivo+="\t</Cache>\n";

        //Gerando as informacoes da memoria virtual
        dados_para_arquivo+="\t<VirtualMemory>\n";
        dados_para_arquivo+="\t\t<pageSize>";
        dados_para_arquivo+=&self.virtual_memory.get_page_size().to_string();
        dados_para_arquivo+="</pageSize>\n";
        dados_para_arquivo+="\t\t<diskMemorySize>";
        dados_para_arquivo+=&self.virtual_memory.get_disk_memory_size().to_string();
        dados_para_arquivo+="</diskMemorySize>\n";
        dados_para_arquivo+="\t\t<diskCiclesPerAccessRead>";
        dados_para_arquivo+=&self.virtual_memory.get_disk_cicles_per_access_read().to_string();
        dados_para_arquivo+="</diskCiclesPerAccessRead>\n";
        dados_para_arquivo+="\t\t<diskCiclesPerAccessWrite>";
        dados_para_arquivo+=&self.virtual_memory.get_disk_cicles_per_access_write().to_string();
        dados_para_arquivo+="</diskCiclesPerAccessWrite>\n";
        dados_para_arquivo+="\t\t<timeCicle>";
        dados_para_arquivo+=&self.virtual_memory.get_time_cicle().to_string();
        dados_para_arquivo+="</timeCicle>\n";
        dados_para_arquivo+="\t\t<pageTableReplacementAlgorithm>";
        match self.virtual_memory.get_page_table_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados_para_arquivo+="FIFO",
            ReplacementAlgorithm::LRU => dados_para_arquivo+="LRU"
        }
        dados_para_arquivo+="</pageTableReplacementAlgorithm>\n";
        dados_para_arquivo+="\n";
        dados_para_arquivo+="\t\t<TLBType>";
        match self.virtual_memory.get_tlb_type()
        {
            TLBType::None => dados_para_arquivo+="none",
            TLBType::Unified => dados_para_arquivo+="unified"
        }
        dados_para_arquivo+="</TLBType>\n";
        if self.virtual_memory.get_tlb_type() == TLBType::Unified
        {
            dados_para_arquivo+="\t\t<unifiedTLB>\n";
            dados_para_arquivo+="\t\t\t<memorySize>";
            dados_para_arquivo+=&self.virtual_memory.get_memory_size().unwrap().to_string();
            dados_para_arquivo+="</memorySize>\n";
            dados_para_arquivo+="\t\t\t<ciclesPerAccessRead>";
            dados_para_arquivo+=&self.virtual_memory.get_cicles_per_access_read().unwrap().to_string();
            dados_para_arquivo+="</ciclesPerAccessRead>\n";
            dados_para_arquivo+="\t\t\t<ciclesPerAccessWrite>";
            dados_para_arquivo+=&self.virtual_memory.get_cicles_per_access_write().unwrap().to_string();
            dados_para_arquivo+="</ciclesPerAccessWrite>\n";
            dados_para_arquivo+="\t\t\t<timeCicle>";
            dados_para_arquivo+=&self.virtual_memory.get_time_cicle_unified_tlb().unwrap().to_string();
            dados_para_arquivo+="</timeCicle>\n";
            dados_para_arquivo+="\t\t\t<replacementAlgorithm>";
            match self.virtual_memory.get_replacement_algorithm().unwrap()
            {
                ReplacementAlgorithm::FIFO => dados_para_arquivo+="FIFO",
                ReplacementAlgorithm::LRU => dados_para_arquivo+="LRU"
            }
            dados_para_arquivo+="</replacementAlgorithm>\n";
            dados_para_arquivo+="\t\t</unifiedTLB>\n";
        }
        dados_para_arquivo+="\t</VirtualMemory>\n";

        //Imprimindo o cabecalho final
        dados_para_arquivo+="</AmnesiaConfiguration>";


        //Escrevendo dados no arquivo
        let sucesso = arquivo.write(dados_para_arquivo.as_bytes());

        if sucesso.is_err()
        {
            eprint!("{}", sucesso.unwrap_err());
        }

        //Fechando arquivo
        drop(arquivo);
    }

    ///Este metodo serve para converter uma instancia da struct Configuracoes em uma String.
    ///Parametro: self: Instancia da struct Configuracoes.
    ///Retorno: String: String contendo os dados da instancia da struct.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados+="Processor:";
        dados+="\n";
        dados+=&self.processor.to_string();
        dados+="\n\n";

        dados+="Trace:";
        dados+="\n";
        dados+=&self.trace.to_string();
        dados+="\n\n";

        dados+="CPU:";
        dados+="\n";
        dados+=&self.cpu.to_string();
        dados+="\n\n";

        dados+="Cache:";
        dados+="\n";
        dados+=&self.cache.to_string();
        dados+="\n\n";

        dados+="Main Memory:";
        dados+="\n";
        dados+=&self.main_memory.to_string();
        dados+="\n\n";

        dados+="Virtual Memory:";
        dados+="\n";
        dados+=&self.virtual_memory.to_string();

        return dados;
    }

    ///Este metodo serve para alterar o valor do atributo word_size de todas as configuracoes.
    ///Parametro: self: Instancia da struct Configuracoes mutavel.
    ///Parametro: word_size: Novo valor para o atributo word_size.
    pub fn set_word_size(&mut self, word_size: usize)
    {
        self.cpu.set_word_size(word_size);
        self.trace.set_word_size(word_size);
    }
}