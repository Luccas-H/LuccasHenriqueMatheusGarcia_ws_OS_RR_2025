# 💭 Simulador do Jantar dos Filósofos

Este projeto tem como objetivo simular o clássico **Problema dos Filósofos**, uma abstração em ciência da computação usada para demonstrar conceitos de **concorrência**, **sincronização** e **recursos compartilhados**.

Além da implementação da simulação, o projeto inclui um **diagrama de redes de Petri** modelado com a ferramenta **Snoopy**, representando visualmente o comportamento e os estados possíveis do sistema.

## 📘 Sobre o Problema

O problema envolve **N filósofos** sentados ao redor de uma mesa circular. Entre cada par de filósofos, há um **garfo**. Cada filósofo pode estar em um dos três estados:

- **Pensando**
- **Com Fome**
- **Comendo**

Para comer, um filósofo precisa de **dois garfos** (o da esquerda e o da direita). O desafio é garantir que não ocorra **deadlock (impasse)** ou **starvation (inanição)**.

## 🧠 Objetivos

- Simular o comportamento dos filósofos em tempo real.
- Garantir que a solução seja livre de impasses e inanição.
- Modelar o sistema usando **Redes de Petri** no **Snoopy**.

## 🛠️ Tecnologias Utilizadas

- Rust
- Threading / Semáforos / Monitores
- Snoopy (para modelagem de Redes de Petri)
- Graphviz (opcional para visualização gráfica)

## 📁 Estrutura do Repositório

