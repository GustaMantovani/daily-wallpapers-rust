import os
import subprocess

# Caminho do arquivo de configuração
config_path = os.path.join(os.getcwd(), "config/config.json")

# Verificar se o arquivo de configuração existe
if not os.path.isfile(config_path):
    print("Arquivo de configuração não encontrado:", config_path)
    exit(1)

# Executar o comando `next`
try:
    subprocess.run(["C:\\Users\\gusta\\OneDrive\\Documents\\pjct\\daily-wallpapers-rust\\target\\release\\daily-wallpapers-rust.exe", "next"], cwd=os.getcwd())
except subprocess.CalledProcessError as e:
    print("O comando `next` falhou com o código de saída:", e.returncode)
    print("Saída de erro:", e.stderr.decode("utf-8"))
    exit(1)

# Se o comando `next` for executado com sucesso, imprimir uma mensagem
print("O comando `next` foi executado com sucesso.")