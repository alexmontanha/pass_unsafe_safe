# Exemplo de criptografia segura para senha

## Descrição

Este exemplo usa a biblioteca bcrypt para criar um hash seguro da senha. O algoritmo bcrypt é projetado para ser lento e resistente a ataques de força bruta. O algoritmo também adiciona um sal aleatório de 16 bytes ao hash. O sal é armazenado junto com o hash, portanto, não é necessário armazenar o sal separadamente.