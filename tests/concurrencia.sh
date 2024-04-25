#!/bin/bash


# Variables
url="http://localhost:4221/hello"
n=30

# Bucle de peticiones 

for i in $(seq 1 $n); do
    curl $url &
    echo "Petición $i lanzada"
done
