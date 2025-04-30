Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore
Descrição:
Este projeto implementa um sistema de busca otimizado para o catálogo de produtos da "MegaStore", uma gigante do varejo e-commerce. O objetivo é criar um mecanismo de busca eficiente e escalável que ajude a melhorar a experiência de compra dos clientes, oferecendo resultados rápidos e precisos.

A solução foi construída utilizando a linguagem de programação Rust, com foco em desempenho e segurança de memória. O sistema de busca é baseado em uma estrutura de dados HashMap, que indexa os produtos do catálogo por palavras-chave extraídas do nome e da categoria de cada item.

Tecnologias Utilizadas:
Rust: Linguagem de programação utilizada para a implementação do sistema de busca.

HashMap: Estrutura de dados utilizada para indexar os produtos e otimizar as buscas.

Cargo: Ferramenta de build do Rust, usada para gerenciar dependências e executar o projeto.

Como Executar:
Pré-requisitos
Para executar este projeto, você precisa ter o Rust instalado no seu sistema. Caso não tenha, siga as instruções em https://www.rust-lang.org/learn/get-started.

Passo a Passo:
Clone o repositório para o seu computador.

Navegue até a pasta do projeto.

Compile e execute o projeto utilizando a ferramenta Cargo.

Isso irá compilar o código e rodar a aplicação. O sistema irá adicionar alguns produtos ao índice e realizar uma busca por um termo específico, exibindo os resultados no terminal.

Executando os Testes
Para garantir a qualidade do código, o projeto inclui testes unitários. Para executá-los, utilize o comando do Cargo para rodar os testes.

Os testes vão verificar a funcionalidade principal do sistema de busca, garantindo que os produtos estão sendo indexados corretamente e as buscas retornem os resultados esperados.

Arquitetura do Sistema
O sistema de busca é composto pelos seguintes módulos principais:

Product: Estrutura que representa um produto no catálogo. Contém os campos id, name e category.

SearchEngine: Estrutura que contém um índice baseado em HashMap. Cada palavra-chave do nome ou categoria de um produto é associada a uma lista de produtos que correspondem a essa palavra-chave.

Função main: Executa a aplicação, adicionando produtos ao sistema de busca e realizando uma busca por um termo.

Algoritmos e Estruturas de Dados Utilizados
HashMap: A estrutura HashMap foi escolhida para indexar os produtos. A chave é uma palavra-chave (relacionada ao nome ou à categoria do produto), e o valor é um vetor de produtos correspondentes a essa palavra-chave. Essa estrutura permite buscas rápidas, com tempo médio de busca O(1).

Busca por Termo: Quando um termo de busca é fornecido, o sistema converte o termo para minúsculas e procura esse termo no índice. Se o termo for encontrado, retorna os produtos associados a ele; caso contrário, retorna uma lista vazia.

Considerações sobre Desempenho e Escalabilidade
Desempenho: O uso do HashMap garante que o sistema de busca seja eficiente, permitindo que a busca por termos seja feita em tempo constante em média (O(1)).

Escalabilidade: O sistema foi projetado para lidar com grandes volumes de dados, e a estrutura de indexação pode ser expandida para acomodar milhares ou milhões de produtos. A solução é escalável, pois novos produtos podem ser facilmente adicionados ao índice sem impacto significativo no desempenho.
