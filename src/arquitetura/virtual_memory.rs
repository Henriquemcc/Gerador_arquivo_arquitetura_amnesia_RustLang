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
use crate::arquitetura::tlb::TLBType;

/// Define a struct do tipo VirtualMemory para a tag VirtualMemory, que representa a memória virtual da arquitetura do amnesia.
pub struct VirtualMemory
{
    page_size: usize,
    disk_memory_size: usize,
    disk_cicles_per_access: usize,
    disk_cicles_per_access_read: usize,
    disk_cicles_per_access_write: usize,
    time_cicle: usize,
    page_table_replacement_algorithm: ReplacementAlgorithm,
    tlb_type: TLBType,
}

///Implementacao de metodos para a struct VirtualMemory.
impl VirtualMemory
{
    /// Altera o valor do atributo page_size.
    /// # Arguments
    /// * page_size - Novo valor para o atributo page_size.
    pub fn set_page_size(&mut self, page_size: usize)
    {
        self.page_size = page_size;
    }

    /// Obtem o valor do atributo page_size.
    /// # Return
    /// * usize - Novo valor para o atributo page_size.
    pub fn get_page_size(&self) -> usize
    {
        return self.page_size;
    }

    /// Altera o valor do atributo disk_memory_size.
    /// # Arguments
    /// * disk_memory_size - Novo valor para o atributo disk_memory_size.
    pub fn set_disk_memory_size(&mut self, disk_memory_size: usize)
    {
        self.disk_memory_size = disk_memory_size;
    }

    /// Obtem o valor do atributo disk_memory_size.
    /// # Return
    /// * usize - Valor do atributo disk_memory_size.
    pub fn get_disk_memory_size(&self) -> usize
    {
        return self.disk_memory_size;
    }

    /// Altera o valor do atributo disk_cicles_per_access.
    /// # Arguments
    /// * disk_cicles_per_access - Novo valor para o atributo disk_cicles_per_access.
    pub fn set_disk_cicles_per_access(&mut self, disk_cicles_per_access: usize)
    {
        self.disk_cicles_per_access = disk_cicles_per_access;
    }

    /// Obtem o valor do atributo disk_cicles_per_access.
    /// # Return
    /// * usize - Valor do atributo disk_cicles_per_access.
    pub fn get_disk_cicles_per_access(&self) -> usize
    {
        return self.disk_cicles_per_access;
    }

    /// Altera o valor do atributo disk_cicles_per_access_read.
    /// # Arguments
    /// * disk_cicles_per_access_read - Novo valor para o atributo disk_cicles_per_access_read.
    pub fn set_disk_cicles_per_access_read(&mut self, disk_cicles_per_access_read: usize)
    {
        self.disk_cicles_per_access_read = disk_cicles_per_access_read;
    }

    /// Obtem o valor do atributo disk_cicles_per_access_read.
    /// # Return
    /// * usize - Valor do atributo disk_cicles_per_access_read.
    pub fn get_disk_cicles_per_access_read(&self) -> usize
    {
        return self.disk_cicles_per_access_read;
    }

    /// Altera o valor do atributo disk_cicles_per_access_write.
    /// # Arguments
    /// * disk_cicles_per_access_write - Novo valor para o atributo disk_cicles_per_access_write.
    pub fn set_disk_cicles_per_access_write(&mut self, disk_cicles_per_access_write: usize)
    {
        self.disk_cicles_per_access_write = disk_cicles_per_access_write;
    }

    /// Obtem o valor do atributo disk_cicles_per_access_write.
    /// # Return
    /// * usize - Valor do atributo disk_cicles_per_access_write.
    pub fn get_disk_cicles_per_access_write(&self) -> usize
    {
        return self.disk_cicles_per_access_write;
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

    /// Altera o valor do atributo page_table_replacement_algorithm.
    /// # Arguments
    /// * page_table_replacement_algorithm - Novo valor para o atributo page_table_replacement_algorithm.
    pub fn set_page_table_replacement_algorithm(&mut self, page_table_replacement_algorithm: ReplacementAlgorithm)
    {
        self.page_table_replacement_algorithm = page_table_replacement_algorithm;
    }

    /// Obtem o valor do atributo page_table_replacement_algorithm.
    /// # Return
    /// * ReplacementAlgorithm - Valor do atributo page_table_replacement_algorithm.
    pub fn get_page_table_replacement_algorithm(&self) -> ReplacementAlgorithm
    {
        return self.page_table_replacement_algorithm;
    }

    /// Altera o valor do atributo tlb_type.
    /// # Arguments
    /// * tlb_type - Novo valor para o atributo tlb_type.
    pub fn set_tlb_type(&mut self, tlb_type: TLBType)
    {
        self.tlb_type = tlb_type;
    }

    /// Obtem o valor do atributo tlb_type.
    /// # Return
    /// * TLBType - Valor do atributo tlb_type.
    pub fn get_tlb_type(&self) -> TLBType
    {
        return self.tlb_type;
    }

    /// Cria uma nova instancia da struct VirtualMemory.
    /// # Return
    /// * VirtualMemory - Nova instancia da struct VirtualMemory.
    pub fn new() -> VirtualMemory
    {
        VirtualMemory
        {
            page_size: 0,
            disk_memory_size: 0,
            disk_cicles_per_access: 0,
            disk_cicles_per_access_read: 0,
            disk_cicles_per_access_write: 0,
            time_cicle: 0,
            page_table_replacement_algorithm: ReplacementAlgorithm::FIFO,
            tlb_type: TLBType::None,
        }
    }

    /// Converte uma struct do tipo VirtualMemory em uma String.
    /// # Return
    /// * String - Uma String contendo os atributos da struct VirtualMemory.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        dados += "page size:";
        dados += &self.get_page_size().to_string();
        dados += "\n";

        dados += "disk memory size:";
        dados += &self.get_disk_memory_size().to_string();
        dados += "\n";

        dados += "disk cicles per access:";
        dados += &self.get_disk_cicles_per_access().to_string();
        dados += "\n";

        dados += "disk cicles per access read:";
        dados += &self.get_disk_cicles_per_access_read().to_string();
        dados += "\n";

        dados += "disk cicles per access write:";
        dados += &self.get_disk_cicles_per_access_write().to_string();
        dados += "\n";

        dados += "time cicle:";
        dados += &self.get_time_cicle().to_string();
        dados += "\n";

        dados += "page table replacement algorithm:";
        match self.get_page_table_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados += "FIFO",
            ReplacementAlgorithm::LRU => dados += "LRU",
            _ => {}
        }
        dados += "\n";

        dados += "tlb type:";
        dados += &self.get_tlb_type().to_string();

        return dados;
    }

    /// Converte uma struct do tipo VirtualMemory em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();

        dados_para_arquivo += "\t<VirtualMemory>\n";

        dados_para_arquivo += "\t\t<pageSize>";
        dados_para_arquivo += &self.get_page_size().to_string();
        dados_para_arquivo += "</pageSize>\n";

        dados_para_arquivo += "\t\t<diskMemorySize>";
        dados_para_arquivo += &self.get_disk_memory_size().to_string();
        dados_para_arquivo += "</diskMemorySize>\n";

        dados_para_arquivo += "\t\t<diskCiclesPerAccessRead>";
        dados_para_arquivo += &self.get_disk_cicles_per_access_read().to_string();
        dados_para_arquivo += "</diskCiclesPerAccessRead>\n";

        dados_para_arquivo += "\t\t<diskCiclesPerAccessWrite>";
        dados_para_arquivo += &self.get_disk_cicles_per_access_write().to_string();
        dados_para_arquivo += "</diskCiclesPerAccessWrite>\n";

        dados_para_arquivo += "\t\t<timeCicle>";
        dados_para_arquivo += &self.get_time_cicle().to_string();
        dados_para_arquivo += "</timeCicle>\n";

        dados_para_arquivo += "\t\t<pageTableReplacementAlgorithm>";
        match self.get_page_table_replacement_algorithm()
        {
            ReplacementAlgorithm::FIFO => dados_para_arquivo += "FIFO",
            ReplacementAlgorithm::LRU => dados_para_arquivo += "LRU",
            _ => {}
        }
        dados_para_arquivo += "</pageTableReplacementAlgorithm>\n";

        dados_para_arquivo += "\n";

        dados_para_arquivo += "\t\t<TLBType>";
        match self.get_tlb_type()
        {
            TLBType::Unified(..) => dados_para_arquivo += "unified",
            TLBType::Split(..) => dados_para_arquivo += "split",
            TLBType::None => dados_para_arquivo += "none",
        }
        dados_para_arquivo += "</TLBType>\n";

        dados_para_arquivo += &self.get_tlb_type().to_string_arquivo();

        dados_para_arquivo += "\t</VirtualMemory>\n";

        return dados_para_arquivo;
    }
}
/// Define se tem virtual memory na arquitetura do Amnesia.
pub enum HaveVirtualMemory
{
    Yes(VirtualMemory),
    No,
}

/// Implementacao de metodos na enum HaveVirtualMemory.
impl HaveVirtualMemory
{
    /// Converte uma enum do tipo HaveVirtualMemory em uma String.
    /// # Return
    /// * String - Uma String contendo os atributos da enum HaveVirtualMemory e todos os valores de seus atributos.
    pub fn to_string(&self) -> String
    {
        let mut dados = String::new();

        match self
        {
            HaveVirtualMemory::Yes(virtual_memory_struct) => dados += &virtual_memory_struct.to_string(),
            _ => {}
        }

        return dados;
    }

    /// Converte uma enum do tipo HaveVirtualMemory em uma String sendo esta parte do conteudo de um arquivo xml.
    /// # Return
    /// * String - Uma String contendo parte do conteudo de um arquivo xml.
    pub fn to_string_arquivo(&self) -> String
    {
        let mut dados_para_arquivo = String::new();

        match self
        {
            HaveVirtualMemory::Yes(virtual_memory_struct) => dados_para_arquivo += &virtual_memory_struct.to_string_arquivo(),
            _ => {}
        }

        return dados_para_arquivo;
    }
}