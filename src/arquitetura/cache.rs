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

use crate::arquitetura::replacement_algorithm::ReplacementAlgorithm;
use crate::arquitetura::write_policy::WritePolicy;

/// Define o tipo de cache da arquitetura do Amnesia.
#[derive(Copy, Clone, PartialEq)]
pub enum CacheType
{
    Unified(UnifiedCache),
    Split(SplitCache),
    Null,
}

/// Implementa de metodos na enum CacheType.
impl CacheType
{
    /// Converte uma enum do tipo CacheType em uma String.
    /// # Return
    /// * String - Uma string contendo os atributos da enum CacheType e todos os valores de seus atributos.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        match self
        {
            CacheType::Unified(unified_cache_struct) => dados += &unified_cache_struct.to_string(),
            CacheType::Split(split_cache_struct) => dados += &split_cache_struct.to_string(),
            _ => {}
        }

        return dados;
    }

    /// Converte uma enum do tipo CacheType em uma string sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();

        match self
        {
            CacheType::Unified(unified_cache_struct) => dados_para_arquivo += &unified_cache_struct.to_string_arquivo(),
            CacheType::Split(split_cache_struct) => dados_para_arquivo += &split_cache_struct.to_string_arquivo(),
            _ => {}
        }

        return dados_para_arquivo;
    }
}

/// Define se na arquitetura contem memoria cache.
#[derive(Copy, Clone, PartialEq)]
pub enum HaveCache
{
    Yes(Cache),
    No,
}

/// Implementacao de metodos da enum HaveCache.
impl HaveCache
{
    /// Converte uma enum do tipo HaveCache em uma String.
    /// # Return
    /// * String - Uma string contendo os atributos da enum HaveCache.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        match self
        {
            HaveCache::Yes(cache_struct) =>
                {
                    dados += &cache_struct.to_string();
                }
            _ => {}
        }

        return dados;
    }

    /// Converte uma enum do tipo HaveCache em em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();

        match self
        {
            HaveCache::Yes(cache_struct) =>
                {
                    dados_para_arquivo += &cache_struct.to_string_arquivo();
                }
            _ => {}
        }

        return dados_para_arquivo;
    }
}

/// Define a struct do tipo Cache da arquitetura do Amnesia.
#[derive(Copy, Clone, PartialEq)]
pub struct Cache
{
    cache_type: CacheType
}

/// Implementacao de metodos para a struct Cache.
impl Cache
{
    /// Altera o valor do atributo cache_type.
    /// # Arguments
    /// * 'cache_type' - Novo valor para o atributo cache_type.
    pub fn set_cache_type(&mut self, cache_type: CacheType)
    {
        self.cache_type = cache_type;
    }

    /// Obtem o valor do atributo cache_type.
    /// # Return
    /// * CacheType - Valor do atributo cache_type.
    pub fn get_cache_type(&self) -> CacheType
    {
        return self.cache_type;
    }

    /// Cria uma nova instancia da struct Cache.
    /// # Return
    /// * Cache - Nova instancia da struct Cache.
    pub fn new() -> Cache
    {
        Cache
        {
            cache_type: CacheType::Null,
        }
    }

    /// Converte uma struct do tipo Cache em uma String.
    /// # Return
    /// * String - Uma String contendo os atributos da struct Cache.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados += "cache type:";
        dados += &self.get_cache_type().to_string();

        return dados;
    }

    /// Converte uma struct do tipo Cache em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();
        dados_para_arquivo += "\t<Cache>\n";
        dados_para_arquivo += "\t<cacheType>";
        match self.get_cache_type()
        {
            CacheType::Unified(..) => dados_para_arquivo += "Unified",
            CacheType::Split(..) => dados_para_arquivo += "Split",
            _ => {}
        }
        dados_para_arquivo += "</cacheType>\n";
        match self.get_cache_type()
        {
            CacheType::Unified(unified_cache_struct) => dados_para_arquivo += &unified_cache_struct.to_string_arquivo(),
            CacheType::Split(split_cache_struct) => dados_para_arquivo += &split_cache_struct.to_string_arquivo(),
            _ => {}
        }
        dados_para_arquivo += "\t</Cache>\n";

        dados_para_arquivo += &self.get_cache_type().to_string_arquivo();

        return dados_para_arquivo;
    }
}

/// Define uma struct do tipo UnifiedCache da arquitetura do Amnesia.
#[derive(Copy, Clone, PartialEq)]
pub struct UnifiedCache
{
    line_size: usize,
    cicles_per_access: usize,
    cicles_per_access_read: usize,
    cicles_per_access_write: usize,
    time_cicle: usize,
    memory_size: usize,
    associativity_level: usize,
    write_policy: WritePolicy,
    replacement_algorithm: ReplacementAlgorithm,
}

/// Implementacao de metodos para struct UnifiedCache.
impl UnifiedCache
{
    /// Altera o valor do atributo line_size.
    /// # Arguments
    /// * line_size - Novo valor para o atributo line_size.
    pub fn set_line_size(&mut self, line_size: usize)
    {
        self.line_size = line_size;
    }

    /// Obtem o valor do atributo line_size.
    /// # Return
    /// * usize - Valor do atributo line_size.
    pub fn get_line_size(&self) -> usize
    {
        return self.line_size;
    }

    /// Altera o valor do atributo cicles_per_access.
    /// # Arguments
    /// * cicles_per_access - Novo valor para o atributo cicles_per_access.
    pub fn set_cicles_per_access(&mut self, cicles_per_access: usize)
    {
        self.cicles_per_access = cicles_per_access;
    }

    /// Obtem o valor do atributo cicles_per_access.
    /// # Return
    /// * usize - Valor do atributo cicles_per_access.
    pub fn get_cicles_per_access(&self) -> usize
    {
        return self.cicles_per_access;
    }

    /// Altera o valor do atributo cicles_per_access_read.
    /// # Arguments
    /// * cicles_per_access_read - Novo valor para o atributo cicles_per_access_read.
    pub fn set_cicles_per_access_read(&mut self, cicles_per_access_read: usize)
    {
        self.cicles_per_access_read = cicles_per_access_read;
    }

    /// Obtem o valor do atributo cicles_per_access_read.
    /// # Return
    /// * usize - Valor do atributo cicles_per_access_read.
    pub fn get_cicles_per_access_read(&self) -> usize
    {
        return self.cicles_per_access_read;
    }

    /// Altera o valor do atributo cicles_per_access_write.
    /// # Arguments
    /// * cicles_per_access_write - Novo valor para o atributo cicles_per_access_write.
    pub fn set_cicles_per_access_write(&mut self, cicles_per_access_write: usize)
    {
        self.cicles_per_access_write = cicles_per_access_write;
    }

    /// Obtem o valor do atributo cicles_per_access_write.
    /// # Return
    /// * usize - Valor do atributo cicles_per_access_write.
    pub fn get_cicles_per_access_write(&self) -> usize
    {
        return self.cicles_per_access_write;
    }

    /// Altera o valor do atributo time_cicle.
    /// # Arguments
    /// * time_cicle - Novo valor para o atributo time_cicle.
    pub fn set_time_cicle(&mut self, time_cicle: usize)
    {
        self.time_cicle = time_cicle;
    }

    /// Obtem o valor do atributo time_cicle.
    /// # Return
    /// * usize - Valor do atributo time_cicle.
    pub fn get_time_cicle(&self) -> usize
    {
        return self.time_cicle;
    }

    /// Altera o valor do atributo memory_size.
    /// # Arguments
    /// * memory_size - Novo valor para o atributo memory_size.
    pub fn set_memory_size(&mut self, memory_size: usize)
    {
        self.memory_size = memory_size;
    }

    /// Obtem o valor do atributo memory_size.
    /// # Return
    /// * usize - Valor do atributo memory_size.
    pub fn get_memory_size(&self) -> usize
    {
        return self.memory_size;
    }

    /// Altera o valor do atributo associativity_level.
    /// # Arguments
    /// * associativity_level - Novo valor para o atributo associativity_level.
    pub fn set_associativity_level(&mut self, associativity_level: usize)
    {
        self.associativity_level = associativity_level;
    }

    /// Obtem o valor do atributo associativity_level.
    /// # Return
    /// * usize - Valor do atributo associativity_level.
    pub fn get_associativity_level(&self) -> usize
    {
        return self.associativity_level;
    }

    /// Altera o valor do atributo write_policy.
    /// # Arguments
    /// * write_policy - Novo valor para o atributo write_policy.
    pub fn set_write_policy(&mut self, write_policy: WritePolicy)
    {
        self.write_policy = write_policy;
    }

    /// Obtem o valor do atributo write_policy.
    /// # Return
    /// * WritePolicy - Valor do atributo write_policy.
    pub fn get_write_policy(&self) -> WritePolicy
    {
        return self.write_policy;
    }

    /// Altera o valor do atributo replacement_algorithm.
    /// # Arguments
    /// * replacement_algorithm - Novo valor para o atributo replacement_algorithm.
    pub fn set_replacement_algorithm(&mut self, replacement_algorithm: ReplacementAlgorithm)
    {
        self.replacement_algorithm = replacement_algorithm
    }

    /// Obtem o valor do atributo replacement_algorithm.
    /// # Return
    /// * ReplacementAlgorithm - Valor do atributo replacement_algorithm.
    pub fn get_replacement_algorithm(&self) -> ReplacementAlgorithm
    {
        return self.replacement_algorithm;
    }

    /// Converte uma struct do tipo UnifiedCache em uma String.
    /// # Return
    /// * String - Uma String contendo os atributos da struct UnifiedCache.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados += "unified";
        dados += "\n";

        dados += "line size:";
        dados += &self.get_line_size().to_string();
        dados += "\n";

        dados += "cicles per access:";
        dados += &self.get_cicles_per_access().to_string();
        dados += "\n";

        dados += "cicles per access read:";
        dados += &self.get_cicles_per_access_read().to_string();
        dados += "\n";

        dados += "cicles per access write:";
        dados += &self.get_cicles_per_access_write().to_string();
        dados += "\n";

        dados += "time cicle:";
        dados += &self.get_time_cicle().to_string();
        dados += "\n";

        dados += "memory size:";
        dados += &self.get_memory_size().to_string();
        dados += "\n";

        dados += "associativity level:";
        dados += &self.get_associativity_level().to_string();
        dados += "\n";

        dados += "write policy:";
        match self.get_write_policy()
        {
            WritePolicy::WriteBack => dados += "WriteBack",
            WritePolicy::WriteThrough => dados += "WriteThrough",
            _ => {}
        }
        dados += "\n";

        dados += "replacement algorithm:";
        match self.get_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados += "FIFO",
            ReplacementAlgorithm::LRU => dados += "LRU",
            _ => {}
        }

        return dados;
    }

    /// Cria uma nova instancia da struct UnifiedCache.
    /// # Return
    /// * UnifiedCache - Nova instancia da struct UnifiedCache.
    pub fn new() -> UnifiedCache
    {
        UnifiedCache
        {
            line_size: 0,
            cicles_per_access: 0,
            cicles_per_access_read: 0,
            cicles_per_access_write: 0,
            time_cicle: 0,
            memory_size: 0,
            associativity_level: 0,
            write_policy: WritePolicy::Null,
            replacement_algorithm: ReplacementAlgorithm::Null,
        }
    }

    /// Converte uma struct do tipo UnifiedCache em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();
        dados_para_arquivo += "\t\t<unifiedCache>\n";

        dados_para_arquivo += "\t\t\t<lineSize>";
        dados_para_arquivo += &self.get_line_size().to_string();
        dados_para_arquivo += "</lineSize>\n";

        dados_para_arquivo += "\t\t\t<ciclesPerAccessRead>";
        dados_para_arquivo += &self.get_cicles_per_access_read().to_string();
        dados_para_arquivo += "</ciclesPerAccessRead>\n";

        dados_para_arquivo += "\t\t\t<ciclesPerAccessWrite>";
        dados_para_arquivo += &self.get_cicles_per_access_write().to_string();
        dados_para_arquivo += "</ciclesPerAccessWrite>\n";

        dados_para_arquivo += "\t\t\t<timeCicle>";
        dados_para_arquivo += &self.get_time_cicle().to_string();
        dados_para_arquivo += "</timeCicle>\n";

        dados_para_arquivo += "\t\t\t<memorySize>";
        dados_para_arquivo += &self.get_memory_size().to_string();
        dados_para_arquivo += "</memorySize>\n";

        dados_para_arquivo += "\t\t\t<associativityLevel>";
        dados_para_arquivo += &self.get_associativity_level().to_string();
        dados_para_arquivo += "</associativityLevel>\n";

        dados_para_arquivo += "\t\t\t<writePolicy>";
        match self.get_write_policy()
        {
            WritePolicy::WriteBack => dados_para_arquivo += "WriteBack",
            WritePolicy::WriteThrough => dados_para_arquivo += "WriteThrough",
            _ => {}
        }
        dados_para_arquivo += "</writePolicy>\n";

        dados_para_arquivo += "\t\t\t<replacementAlgorithm>";
        match self.get_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados_para_arquivo += "FIFO",
            ReplacementAlgorithm::LRU => dados_para_arquivo += "LRU",
            _ => {}
        }
        dados_para_arquivo += "</replacementAlgorithm>\n";

        dados_para_arquivo += "\t\t</unifiedCache>\n";

        return dados_para_arquivo;
    }
}

/// Define uma struct do tipo SplitCache para a arquitetura do Amnesia.
#[derive(Copy, Clone, PartialEq)]
pub struct SplitCache
{
    instruction_cache: InstructionCache,
    data_cache: DataCache,
}

/// Implementacao de metodos para a struct SplitCache.
impl SplitCache
{
    /// Altera o valor do atributo instruction_cache.
    /// # Arguments
    /// * instruction_cache - Novo valor para o atributo instruction_cache.
    pub fn set_instruction_cache(&mut self, instruction_cache: InstructionCache)
    {
        self.instruction_cache = instruction_cache;
    }

    /// Obtem o valor do atributo instruction_cache.
    /// # Return
    /// * InstructionCache - Valor do atributo instruction_cache.
    pub fn get_instruction_cache(&self) -> InstructionCache
    {
        return self.instruction_cache;
    }

    /// Altera o valor do atributo data_cache.
    /// # Arguments
    /// * data_cache - Novo valor para o atributo data_cache.
    pub fn set_data_cache(&mut self, data_cache: DataCache)
    {
        self.data_cache = data_cache;
    }

    /// Obtem o valor do atributo data_cache.
    /// # Return
    /// * DataCache - Valor do atributo data_cache.
    pub fn get_data_cache(&self) -> DataCache
    {
        return self.data_cache;
    }

    /// Converte uma struct do tipo SplitCache em uma String.
    /// # Return
    /// * String - Uma String contendo os atributos da struct SplitCache.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados += &self.get_instruction_cache().to_string();
        dados += &self.get_data_cache().to_string();

        return dados;
    }

    /// Converte uma struct do tipo SplitCache em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo XML.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();
        dados_para_arquivo += &self.get_instruction_cache().to_string_arquivo();
        dados_para_arquivo += &self.get_data_cache().to_string_arquivo();

        return dados_para_arquivo;
    }

    /// Cria uma nova instancia da struct SplitCache.
    /// # Return
    /// * SplitCache - Nova instancia da struct SplitCache.
    pub fn new() -> SplitCache
    {
        SplitCache
        {
            instruction_cache: InstructionCache::new(),
            data_cache: DataCache::new(),
        }
    }
}
/// Define uma struct do tipo InstructionCache para a arquitetura do Amnesia.
#[derive(Copy, Clone, PartialEq)]
pub struct InstructionCache
{
    line_size: usize,
    cicles_per_access: usize,
    cicles_per_access_read: usize,
    cicles_per_access_write: usize,
    time_cicle: usize,
    memory_size: usize,
    associativity_level: usize,
    write_policy: WritePolicy,
    replacement_algorithm: ReplacementAlgorithm,
}

/// Implementacao de metodos para a struct InstructionCache.
impl InstructionCache
{
    /// Altera o valor do atributo line_size.
    /// # Arguments
    /// * line_size - Novo valor para o atributo line_size.
    pub fn set_line_size(&mut self, line_size: usize)
    {
        self.line_size = line_size;
    }

    /// Obtem o valor do atributo line_size.
    /// # Return
    /// * usize - Valor do atributo line_size.
    pub fn get_line_size(&self) -> usize
    {
        return self.line_size;
    }

    /// Altera o valor do atributo cicles_per_access.
    /// # Arguments
    /// * cicles_per_access - Novo valor para o atributo cicles_per_access.
    pub fn set_cicles_per_access(&mut self, cicles_per_access: usize)
    {
        self.cicles_per_access = cicles_per_access;
    }

    /// Obtem o valor do atributo cicles_per_access.
    /// # Return
    /// * usize - Valor do atributo cicles_per_access.
    pub fn get_cicles_per_access(&self) -> usize
    {
        return self.cicles_per_access;
    }

    /// Altera o valor do atributo cicles_per_access_read.
    /// # Arguments
    /// * cicles_per_access_read - Novo valor para o atributo cicles_per_access_read.
    pub fn set_cicles_per_access_read(&mut self, cicles_per_access_read: usize)
    {
        self.cicles_per_access_read = cicles_per_access_read;
    }

    /// Obtem o valor do atributo cicles_per_access_read.
    /// # Return
    /// * usize - Valor do atributo cicles_per_access_read.
    pub fn get_cicles_per_access_read(&self) -> usize
    {
        return self.cicles_per_access_read;
    }

    /// Altera o valor do atributo cicles_per_access_write.
    /// # Arguments
    /// * cicles_per_access_write - Novo valor para o atributo cicles_per_access_write.
    pub fn set_cicles_per_access_write(&mut self, cicles_per_access_write: usize)
    {
        self.cicles_per_access_write = cicles_per_access_write;
    }

    /// Obtem o valor do atributo cicles_per_access_write.
    /// # Return
    /// * usize - Valor do atributo cicles_per_access_write.
    pub fn get_cicles_per_access_write(&self) -> usize
    {
        return self.cicles_per_access_write;
    }

    /// Altera o valor do atributo time_cicle.
    /// # Arguments
    /// * time_cicle - Novo valor para o atributo time_cicle.
    pub fn set_time_cicle(&mut self, time_cicle: usize)
    {
        self.time_cicle = time_cicle;
    }

    /// Obtem o valor do atributo time_cicle.
    /// # Return
    /// * usize - Valor do atributo time_cicle.
    pub fn get_time_cicle(&self) -> usize
    {
        return self.time_cicle;
    }

    /// Altera o valor do atributo memory_size.
    /// # Arguments
    /// * memory_size - Novo valor para o atributo memory_size.
    pub fn set_memory_size(&mut self, memory_size: usize)
    {
        self.memory_size = memory_size;
    }

    /// Obtem o valor do atributo memory_size.
    /// # Return
    /// * usize - Valor do atributo memory_size.
    pub fn get_memory_size(&self) -> usize
    {
        return self.memory_size;
    }

    /// Altera o valor do atributo associativity_level.
    /// # Arguments
    /// * associativity_level - Novo valor para o atributo associativity_level.
    pub fn set_associativity_level(&mut self, associativity_level: usize)
    {
        self.associativity_level = associativity_level;
    }

    /// Obtem o valor do atributo associativity_level.
    /// # Return
    /// * usize - Valor do atributo associativity_level.
    pub fn get_associativity_level(&self) -> usize
    {
        return self.associativity_level;
    }

    /// Altera o valor do atributo write_policy.
    /// # Arguments
    /// * write_policy - Novo valor para o atributo write_policy.
    pub fn set_write_policy(&mut self, write_policy: WritePolicy)
    {
        self.write_policy = write_policy;
    }

    /// Obtem o valor do atributo write_policy.
    /// # Return
    /// * WritePolicy - Valor do atributo write_policy.
    pub fn get_write_policy(&self) -> WritePolicy
    {
        return self.write_policy;
    }

    /// Altera o valor do atributo replacement_algorithm.
    /// # Arguments
    /// * replacement_algorithm - Novo valor para o atributo replacement_algorithm.
    pub fn set_replacement_algorithm(&mut self, replacement_algorithm: ReplacementAlgorithm)
    {
        self.replacement_algorithm = replacement_algorithm
    }

    /// Obtem o valor do atributo replacement_algorithm.
    /// # Return
    /// * ReplacementAlgorithm - Valor do atributo replacement_algorithm.
    pub fn get_replacement_algorithm(&self) -> ReplacementAlgorithm
    {
        return self.replacement_algorithm;
    }

    /// Converte uma struct do tipo InstructionCache em uma String.
    /// # Return
    /// * String - Uma String contendo os atributos da struct InstructionCache.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados += "instruction cache";
        dados += "\n";

        dados += "line size:";
        dados += &self.get_line_size().to_string();
        dados += "\n";

        dados += "cicles per access:";
        dados += &self.get_cicles_per_access().to_string();
        dados += "\n";

        dados += "cicles per access read:";
        dados += &self.get_cicles_per_access_read().to_string();
        dados += "\n";

        dados += "cicles per access write:";
        dados += &self.get_cicles_per_access_write().to_string();
        dados += "\n";

        dados += "time cicle:";
        dados += &self.get_time_cicle().to_string();
        dados += "\n";

        dados += "memory size:";
        dados += &self.get_memory_size().to_string();
        dados += "\n";

        dados += "associativity level:";
        dados += &self.get_associativity_level().to_string();
        dados += "\n";

        dados += "write policy:";
        match self.get_write_policy()
        {
            WritePolicy::WriteBack => dados += "WriteBack",
            WritePolicy::WriteThrough => dados += "WriteThrough",
            _ => {}
        }
        dados += "\n";

        dados += "replacement algorithm:";
        match self.get_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados += "FIFO",
            ReplacementAlgorithm::LRU => dados += "LRU",
            _ => {}
        }

        return dados;
    }

    /// Converte uma struct do tipo InstructionCache em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo XML.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();
        dados_para_arquivo += "\t\t<instructionCache>\n";

        dados_para_arquivo += "\t\t\t<lineSize>";
        dados_para_arquivo += &self.get_line_size().to_string();
        dados_para_arquivo += "</lineSize>\n";

        dados_para_arquivo += "\t\t\t<ciclesPerAccessRead>";
        dados_para_arquivo += &self.get_cicles_per_access_read().to_string();
        dados_para_arquivo += "</ciclesPerAccessRead>\n";

        dados_para_arquivo += "\t\t\t<ciclesPerAccessWrite>";
        dados_para_arquivo += &self.get_cicles_per_access_write().to_string();
        dados_para_arquivo += "</ciclesPerAccessWrite>\n";

        dados_para_arquivo += "\t\t\t<timeCicle>";
        dados_para_arquivo += &self.get_time_cicle().to_string();
        dados_para_arquivo += "</timeCicle>\n";

        dados_para_arquivo += "\t\t\t<memorySize>";
        dados_para_arquivo += &self.get_memory_size().to_string();
        dados_para_arquivo += "</memorySize>\n";

        dados_para_arquivo += "\t\t\t<associativityLevel>";
        dados_para_arquivo += &self.get_associativity_level().to_string();
        dados_para_arquivo += "</associativityLevel>\n";

        dados_para_arquivo += "\t\t\t<writePolicy>";
        match self.get_write_policy()
        {
            WritePolicy::WriteBack => dados_para_arquivo += "WriteBack",
            WritePolicy::WriteThrough => dados_para_arquivo += "WriteThrough",
            _ => {}
        }
        dados_para_arquivo += "</writePolicy>\n";

        dados_para_arquivo += "\t\t\t<replacementAlgorithm>";
        match self.get_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados_para_arquivo += "FIFO",
            ReplacementAlgorithm::LRU => dados_para_arquivo += "LRU",
            _ => {}
        }
        dados_para_arquivo += "</replacementAlgorithm>\n";

        dados_para_arquivo += "\t\t</instructionCache>\n";

        return dados_para_arquivo;
    }

    /// Cria uma nova instancia da struct InstructionCache.
    /// # Return
    /// * SplitCache - Nova instancia da struct InstructionCache.
    pub fn new() -> InstructionCache
    {
        InstructionCache
        {
            line_size: 0,
            cicles_per_access: 0,
            cicles_per_access_read: 0,
            cicles_per_access_write: 0,
            time_cicle: 0,
            memory_size: 0,
            associativity_level: 0,
            write_policy: WritePolicy::Null,
            replacement_algorithm: ReplacementAlgorithm::Null,
        }
    }
}

/// Define uma struct do tipo DataCache para a arquitetura do Amnesia.
#[derive(Copy, Clone, PartialEq)]
pub struct DataCache
{
    line_size: usize,
    cicles_per_access: usize,
    cicles_per_access_read: usize,
    cicles_per_access_write: usize,
    time_cicle: usize,
    memory_size: usize,
    associativity_level: usize,
    write_policy: WritePolicy,
    replacement_algorithm: ReplacementAlgorithm,
}

/// Implementacao de metodos para a struct DataCache.
impl DataCache
{
    /// Altera o valor do atributo line_size.
    /// # Arguments
    /// * line_size - Novo valor para o atributo line_size.
    pub fn set_line_size(&mut self, line_size: usize)
    {
        self.line_size = line_size;
    }

    /// Obtem o valor do atributo line_size.
    /// # Return
    /// * usize - Valor do atributo line_size.
    pub fn get_line_size(&self) -> usize
    {
        return self.line_size;
    }

    /// Altera o valor do atributo cicles_per_access.
    /// # Arguments
    /// * cicles_per_access - Novo valor para o atributo cicles_per_access.
    pub fn set_cicles_per_access(&mut self, cicles_per_access: usize)
    {
        self.cicles_per_access = cicles_per_access;
    }

    /// Obtem o valor do atributo cicles_per_access.
    /// # Return
    /// * usize - Valor do atributo cicles_per_access.
    pub fn get_cicles_per_access(&self) -> usize
    {
        return self.cicles_per_access;
    }

    /// Altera o valor do atributo cicles_per_access_read.
    /// # Arguments
    /// * cicles_per_access_read - Novo valor para o atributo cicles_per_access_read.
    pub fn set_cicles_per_access_read(&mut self, cicles_per_access_read: usize)
    {
        self.cicles_per_access_read = cicles_per_access_read;
    }

    /// Obtem o valor do atributo cicles_per_access_read.
    /// # Return
    /// * usize - Valor do atributo cicles_per_access_read.
    pub fn get_cicles_per_access_read(&self) -> usize
    {
        return self.cicles_per_access_read;
    }

    /// Altera o valor do atributo cicles_per_access_write.
    /// # Arguments
    /// * cicles_per_access_write - Novo valor para o atributo cicles_per_access_write.
    pub fn set_cicles_per_access_write(&mut self, cicles_per_access_write: usize)
    {
        self.cicles_per_access_write = cicles_per_access_write;
    }

    /// Obtem o valor do atributo cicles_per_access_write.
    /// # Return
    /// * usize - Valor do atributo cicles_per_access_write.
    pub fn get_cicles_per_access_write(&self) -> usize
    {
        return self.cicles_per_access_write;
    }

    /// Altera o valor do atributo time_cicle.
    /// # Arguments
    /// * time_cicle - Novo valor para o atributo time_cicle.
    pub fn set_time_cicle(&mut self, time_cicle: usize)
    {
        self.time_cicle = time_cicle;
    }

    /// Obtem o valor do atributo time_cicle.
    /// # Return
    /// * usize - Valor do atributo time_cicle.
    pub fn get_time_cicle(&self) -> usize
    {
        return self.time_cicle;
    }

    /// Altera o valor do atributo memory_size.
    /// # Arguments
    /// * memory_size - Novo valor para o atributo memory_size.
    pub fn set_memory_size(&mut self, memory_size: usize)
    {
        self.memory_size = memory_size;
    }

    /// Obtem o valor do atributo memory_size.
    /// # Return
    /// * usize - Valor do atributo memory_size.
    pub fn get_memory_size(&self) -> usize
    {
        return self.memory_size;
    }

    /// Altera o valor do atributo associativity_level.
    /// # Arguments
    /// * associativity_level - Novo valor para o atributo associativity_level.
    pub fn set_associativity_level(&mut self, associativity_level: usize)
    {
        self.associativity_level = associativity_level;
    }

    /// Obtem o valor do atributo associativity_level.
    /// # Return
    /// * usize - Valor do atributo associativity_level.
    pub fn get_associativity_level(&self) -> usize
    {
        return self.associativity_level;
    }

    /// Altera o valor do atributo write_policy.
    /// # Arguments
    /// * write_policy - Novo valor para o atributo write_policy.
    pub fn set_write_policy(&mut self, write_policy: WritePolicy)
    {
        self.write_policy = write_policy;
    }

    /// Obtem o valor do atributo write_policy.
    /// # Return
    /// * WritePolicy - Valor do atributo write_policy.
    pub fn get_write_policy(&self) -> WritePolicy
    {
        return self.write_policy;
    }

    /// Altera o valor do atributo replacement_algorithm.
    /// # Arguments
    /// * replacement_algorithm - Novo valor para o atributo replacement_algorithm.
    pub fn set_replacement_algorithm(&mut self, replacement_algorithm: ReplacementAlgorithm)
    {
        self.replacement_algorithm = replacement_algorithm
    }

    /// Obtem o valor do atributo replacement_algorithm.
    /// # Return
    /// * ReplacementAlgorithm - Valor do atributo replacement_algorithm.
    pub fn get_replacement_algorithm(&self) -> ReplacementAlgorithm
    {
        return self.replacement_algorithm;
    }

    /// Converte uma struct do tipo DataCache em uma String.
    /// # Return
    /// * String - Uma String contendo os atributos da struct DataCache.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados += "data cache";
        dados += "\n";

        dados += "line size:";
        dados += &self.get_line_size().to_string();
        dados += "\n";

        dados += "cicles per access:";
        dados += &self.get_cicles_per_access().to_string();
        dados += "\n";

        dados += "cicles per access read:";
        dados += &self.get_cicles_per_access_read().to_string();
        dados += "\n";

        dados += "cicles per access write:";
        dados += &self.get_cicles_per_access_write().to_string();
        dados += "\n";

        dados += "time cicle:";
        dados += &self.get_time_cicle().to_string();
        dados += "\n";

        dados += "memory size:";
        dados += &self.get_memory_size().to_string();
        dados += "\n";

        dados += "associativity level:";
        dados += &self.get_associativity_level().to_string();
        dados += "\n";

        dados += "write policy:";
        match self.get_write_policy()
        {
            WritePolicy::WriteBack => dados += "WriteBack",
            WritePolicy::WriteThrough => dados += "WriteThrough",
            _ => {}
        }
        dados += "\n";

        dados += "replacement algorithm:";
        match self.get_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados += "FIFO",
            ReplacementAlgorithm::LRU => dados += "LRU",
            _ => {}
        }

        return dados;
    }

    /// Converte uma struct do tipo DataCache em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo XML.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();
        dados_para_arquivo += "\t\t<dataCache>\n";

        dados_para_arquivo += "\t\t\t<lineSize>";
        dados_para_arquivo += &self.get_line_size().to_string();
        dados_para_arquivo += "</lineSize>\n";

        dados_para_arquivo += "\t\t\t<ciclesPerAccessRead>";
        dados_para_arquivo += &self.get_cicles_per_access_read().to_string();
        dados_para_arquivo += "</ciclesPerAccessRead>\n";

        dados_para_arquivo += "\t\t\t<ciclesPerAccessWrite>";
        dados_para_arquivo += &self.get_cicles_per_access_write().to_string();
        dados_para_arquivo += "</ciclesPerAccessWrite>\n";

        dados_para_arquivo += "\t\t\t<timeCicle>";
        dados_para_arquivo += &self.get_time_cicle().to_string();
        dados_para_arquivo += "</timeCicle>\n";

        dados_para_arquivo += "\t\t\t<memorySize>";
        dados_para_arquivo += &self.get_memory_size().to_string();
        dados_para_arquivo += "</memorySize>\n";

        dados_para_arquivo += "\t\t\t<associativityLevel>";
        dados_para_arquivo += &self.get_associativity_level().to_string();
        dados_para_arquivo += "</associativityLevel>\n";

        dados_para_arquivo += "\t\t\t<writePolicy>";
        match self.get_write_policy()
        {
            WritePolicy::WriteBack => dados_para_arquivo += "WriteBack",
            WritePolicy::WriteThrough => dados_para_arquivo += "WriteThrough",
            _ => {}
        }
        dados_para_arquivo += "</writePolicy>\n";

        dados_para_arquivo += "\t\t\t<replacementAlgorithm>";
        match self.get_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados_para_arquivo += "FIFO",
            ReplacementAlgorithm::LRU => dados_para_arquivo += "LRU",
            _ => {}
        }
        dados_para_arquivo += "</replacementAlgorithm>\n";

        dados_para_arquivo += "\t\t</dataCache>\n";

        return dados_para_arquivo;
    }

    /// Cria uma nova instancia da struct DataCache.
    /// # Return
    /// * SplitCache - Nova instancia da struct DataCache.
    pub fn new() -> DataCache
    {
        DataCache
        {
            line_size: 0,
            cicles_per_access: 0,
            cicles_per_access_read: 0,
            cicles_per_access_write: 0,
            time_cicle: 0,
            memory_size: 0,
            associativity_level: 0,
            write_policy: WritePolicy::Null,
            replacement_algorithm: ReplacementAlgorithm::Null,
        }
    }
}