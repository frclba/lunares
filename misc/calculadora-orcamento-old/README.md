Projeto de Rust

Calculadora de Orçamento Pessoal

Objetivo:

Desenvolver uma aplicação simples de gerenciamento financeiro
pessoal que permita ao usuário registrar suas despesas e receitas,
calcular saldos mensais e fazer previsões de orçamento. O objetivo
deste projeto é proporcionar uma introdução prática ao uso de Rust
e WebAssembly para criar aplicações que possam ser executadas
no navegador e serem úteis para organização financeira.

Descrição do Projeto:

Você foi contratado para criar uma calculadora de orçamento
pessoal que rode no navegador, permitindo ao usuário inserir suas
receitas e despesas de cada mês e ver um resumo financeiro de
forma prática e intuitiva. O sistema será utilizado para registrar
entradas e saídas de dinheiro, com a possibilidade de prever
saldos para meses futuros com base nos dados de receita e
despesa média.

Requisitos Funcionais
1. Registro de Receitas e Despesas:
• O sistema deve permitir que o usuário insira transações
de receita (ex.: salário, vendas) e de despesa (ex.: contas, lazer).
• Cada transação deve conter uma descrição, categoria
(ex.: salário, aluguel, alimentação), valor e data.

2. Resumo Mensal:
• O sistema deve calcular automaticamente o saldo
mensal com base nas receitas e despesas registradas.
• Exibir o saldo total ao final de cada mês e uma média
dos últimos três meses.

3. Previsão de Orçamento:
• Com base na média dos três meses anteriores, o sistema
deve calcular uma previsão do saldo para o mês seguinte,
permitindo ao usuário avaliar seu orçamento futuro.

4. Interface Simples para Visualização:
• A interface deve mostrar as transações do mês em uma
tabela com detalhes como categoria, descrição, valor e data.
• Exibir também o saldo final do mês e a previsão para o
próximo mês.

Requisitos Técnicos:
1. Uso de Rust e WebAssembly:
• A lógica de cálculo do orçamento e previsão financeira
deve ser implementada em Rust, enquanto a interface e interações
com o navegador serão feitas em WebAssembly.

2. Estrutura de Dados:
• Use structs em Rust para representar transações e
organizar receitas e despesas.
• A aplicação deve ser capaz de armazenar
temporariamente as transações para cada mês.

3. Persistência Temporária:
• As transações registradas devem permanecer na
memória enquanto a aplicação está aberta, permitindo ao usuário
verificar seu histórico até que a página seja fechada ou
recarregada.

Orientações de Implementação:
1. Estrutura do Código:
• Crie módulos de Rust para organizar as funcionalidades
principais: transaction, budget_calculator e wasm_interface.
• A wasm_interface deve fornecer funções acessíveis ao
WebAssembly para manipulação das transações e cálculos.
2. Cálculo de Média e Previsão:
• No módulo budget_calculator, implemente funções que
calculem a média das receitas e despesas dos últimos três meses
e que façam a previsão do saldo futuro com base nesses dados.
3. Interface do Usuário (Opcional):
• Uma interface simples que permita ao usuário adicionar
receitas e despesas, e visualize as transações e o saldo mensal.
• Deve incluir botões para adicionar receitas e despesas e
exibir automaticamente o saldo e previsão.