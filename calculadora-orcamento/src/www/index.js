// www/index.js
import init, { add_transaction, get_monthly_balance, get_forecast } from '../pkg/calculadora_orcamento.js';

async function run() {
    await init();

    const form = document.getElementById('transaction-form');
    const transactionsTable = document.getElementById('transactions-table').getElementsByTagName('tbody')[0];
    const monthlyBalanceElement = document.getElementById('monthly-balance');
    const forecastElement = document.getElementById('forecast');

    form.addEventListener('submit', event => {
        event.preventDefault();

        const transaction = {
            description: document.getElementById('description').value,
            category: document.getElementById('category').value,
            amount: parseFloat(document.getElementById('amount').value),
            date: document.getElementById('date').value,
            transaction_type: document.getElementById('transaction-type').value,
        };

        add_transaction(JSON.stringify(transaction));

        // Atualize a tabela de transações
        const row = transactionsTable.insertRow();
        row.insertCell(0).innerText = transaction.description;
        row.insertCell(1).innerText = transaction.category;
        row.insertCell(2).innerText = transaction.amount.toFixed(2);
        row.insertCell(3).innerText = transaction.date;
        row.insertCell(4).innerText = transaction.transaction_type === 'Income' ? 'Receita' : 'Despesa';

        // Atualize o saldo mensal e previsão
        const today = new Date();
        const month = today.getMonth() + 1;
        const year = today.getFullYear();

        const monthlyBalance = get_monthly_balance(month, year);
        monthlyBalanceElement.innerText = monthlyBalance.toFixed(2);

        const forecast = get_forecast(month, year);
        forecastElement.innerText = forecast.toFixed(2);

        form.reset();
    });
}

run();
