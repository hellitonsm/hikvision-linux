#!/bin/bash
# ============================================
# HCNetSDKV6 JavaDemo - Script de Execução
# ============================================

# Diretório do projeto (raiz do script)
PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Java 8 (requerido pelo projeto)
JAVA="/usr/lib/jvm/java-8-openjdk-amd64/bin/java"
JAVAC="/usr/lib/jvm/java-8-openjdk-amd64/bin/javac"

# Bibliotecas
CP="$PROJECT_DIR/bin:$PROJECT_DIR/jna.jar:$PROJECT_DIR/examples.jar"
LIBS_PATH="$PROJECT_DIR/libs:$PROJECT_DIR/libs/HCNetSDKCom"

# Limpa o terminal
clear

# Compila se necessário
if [ ! -f "$PROJECT_DIR/bin/test/JavaDemo.class" ]; then
    echo "Compilando..."
    SRCDIR="$PROJECT_DIR/src/test"
    BINDIR="$PROJECT_DIR/bin"
    mkdir -p "$BINDIR"
    find "$SRCDIR" -name "*.java" > /tmp/sources_$$.txt
    "$JAVAC" -cp "$PROJECT_DIR/jna.jar:$PROJECT_DIR/examples.jar" -d "$BINDIR" -sourcepath "$SRCDIR" @/tmp/sources_$$.txt
    rm -f /tmp/sources_$$.txt
    echo ""
fi

echo "Java: $($JAVA -version 2>&1 | head -1)"
echo "JAVA_HOME: $(dirname $(dirname $JAVA))"
echo "libs: $LIBS_PATH"
echo "==========================================="
echo ""

# Executa
export LD_LIBRARY_PATH="$LIBS_PATH:$LD_LIBRARY_PATH"
"$JAVA" -cp "$CP" -Djava.library.path="$LIBS_PATH" test.JavaDemo
