#!/bin/bash

ENTRY_TO_REMOVE="$1"

# Obter o crontab atual
crontab -l > /tmp/mycron
CRON_STATUS=$?

if [ $CRON_STATUS -ne 0 ]; then
    echo "No crontab for this user."
    exit 1
fi

# Remover a entrada específica do crontab
sed -i "/$ENTRY_TO_REMOVE/d" /tmp/mycron

# Instalar o novo crontab sem a entrada removida
crontab /tmp/mycron

# Remover o arquivo temporário
rm /tmp/mycron

echo "Entry '$ENTRY_TO_REMOVE' removed from crontab."
