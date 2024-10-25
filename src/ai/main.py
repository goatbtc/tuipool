import requests
from fastapi import FastAPI
import uvicorn

app = FastAPI()

# Função para obter as taxas recomendadas e limite de expulsão
def get_fee_recommendations():
    # Obter taxas da mempool.space
    fees = requests.get('https://mempool.space/api/v1/fees/recommended').json()

    # Obter transações recentes para calcular o limite de expulsão
    recent_txs = requests.get('https://mempool.space/api/mempool/recent').json()
    fee_per_vsize_list = [tx['fee_per_vsize'] for tx in recent_txs]
    expulsion_threshold = min(fee_per_vsize_list)

    # Adicionar o limite de expulsão ao dicionário de taxas
    fees['expulsion_threshold'] = expulsion_threshold

    return fees

# Endpoint para obter as taxas recomendadas e limite de expulsão
@app.get("/recommend_fees")
def recommend_fees():
    fees = get_fee_recommendations()
    return fees

# Função para estimar o tempo de confirmação com base na taxa por vByte
def estimate_confirmation_time(fee_per_vsize):
    fees = get_fee_recommendations()
    if fee_per_vsize >= fees['fastestFee']:
        return {'estimated_time_minutes': 10, 'estimated_blocks': 1}
    elif fee_per_vsize >= fees['halfHourFee']:
        return {'estimated_time_minutes': 30, 'estimated_blocks': 3}
    elif fee_per_vsize >= fees['hourFee']:
        return {'estimated_time_minutes': 60, 'estimated_blocks': 6}
    elif fee_per_vsize >= fees['economyFee']:
        return {'estimated_time_minutes': 120, 'estimated_blocks': 12}
    else:
        return {'estimated_time_minutes': 180, 'estimated_blocks': 18}

# Endpoint para estimar o tempo de confirmação
@app.get("/estimate_confirmation_time")
def estimate_time(fee_per_vsize: float):
    estimation = estimate_confirmation_time(fee_per_vsize)
    return estimation

# Função para verificar se um txid está no mempool
def is_tx_in_mempool(txid):
    response = requests.get(f'https://mempool.space/api/tx/{txid}/status')
    if response.status_code == 200:
        status = response.json()
        return not status['confirmed']
    else:
        # Se a transação não for encontrada, assumimos que não está no mempool
        return False

# Endpoint para verificar se um txid está no mempool
@app.get("/is_tx_in_mempool")
def check_tx(txid: str):
    in_mempool = is_tx_in_mempool(txid)
    return {"txid": txid, "in_mempool": in_mempool}

# Ponto de entrada para rodar a API
if __name__ == "__main__":
    uvicorn.run("main:app", host="0.0.0.0", port=8000)


