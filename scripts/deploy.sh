#!/usr/bin/env bash

mkdir -p -m 775 ./terraform/uploads 

source ./scripts/copy-files.sh

cd terraform

if [[ `uname -m` == 'arm64' ]]; then
    FILE=./terraform
    if [ -f "$FILE" ]; then
        source $FILE apply -auto-approve
    else
        echo "make sure to have the $FILE m1 binary of terraform installed placed in the terraform folder."
        echo "view https://github.com/hashicorp/terraform/issues/27257 for instructions on how."
    fi
else 
  terraform apply -auto-approve
fi

ssh -oStrictHostKeyChecking=no terraform@$(terraform output -raw public_ip) -i ./tf-cloud-init "sh /home/terraform/build/boot.sh"

cd ../