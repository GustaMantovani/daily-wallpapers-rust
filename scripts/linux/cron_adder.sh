#!/bin/bash

CRON_ENTRY="$1"

# Verificar se o crontab existe
crontab -l > /tmp/mycron
CRON_STATUS=$?

# Se o crontab não existe (código de saída 1), criar um novo arquivo de cron
if [ $CRON_STATUS -ne 0 ]; then
    echo "No crontab for this user. Creating a new one."
    touch /tmp/mycron
fi

# Adicionar a nova entrada
echo "$CRON_ENTRY" >> /tmp/mycron

# Instalar o novo crontab
crontab /tmp/mycron

# Remover o arquivo temporário
rm /tmp/mycron

