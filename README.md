# Hikvision HCNetSDK Linux

SDK de rede Hikvision (HCNetSDK v6.1.7.x) para Linux com demos em **C++ (Qt4/Qt5/console)**, **Rust (Slint)** e **Java (Swing/JNA)**.

## Estrutura

```
hikvision-linux/
├── lib/                 # SDK principal (Linux x86_64)
│   ├── libhcnetsdk.so   # SDK principal
│   ├── libhpr.so        # Platform Runtime
│   ├── libHCCore.so     # Core
│   ├── libPlayCtrl.so   # Controle de playback
│   ├── libSuperRender.so
│   ├── libAudioRender.so
│   ├── libNPQos.so      # QoS de rede
│   ├── libopenal.so.1   # Áudio OpenAL
│   ├── libcrypto.so.1.1 # OpenSSL
│   ├── libssl.so.1.1
│   ├── libz.so
│   └── HCNetSDKCom/     # 13 plugins (alarme, preview, playback, voz, etc.)
├── incEn/               # Headers C/C++
│   ├── HCNetSDK.h       # ~2.4 MB, structs e funções da SDK
│   ├── DataType.h
│   ├── DecodeCardSdk.h
│   └── plaympeg4.h
├── doc/                 # 19+ guias do desenvolvedor
│   ├── Device Network SDK (General)_Developer Guide_V6.1.7.X_20220310/
│   ├── Device Network SDK Programming Manual.chm
│   └── readme.txt
├── consoleDemo/         # Demo C++ (terminal)
├── QtDemo/              # Demo C++ Qt4 (GUI completa)
├── Qt5demo/             # Demo C++ Qt5
├── rustdemo/            # Demo Rust + Slint
├── LinuxJavaDemo/       # Demo Java + Swing + JNA
└── psdatacall_demo/     # Captura de stream PS (C++ mínimo)
```

## Demos

| Demo | Linguagem | UI | Build | Completude |
|------|-----------|----|-------|------------|
| QtDemo | C++ | Qt4.7 | qmake / QtCreator | Completa (referência oficial) |
| Qt5demo | C++ | Qt5 | qmake / QtCreator | Completa (port Qt5) |
| consoleDemo | C++ | Terminal | make | Completa |
| **rustdemo** | **Rust** | **Slint** | **cargo build** | **Funcional** (alguns módulos em esqueleto) |
| LinuxJavaDemo | Java | Swing | ant / run.sh | Completa |
| psdatacall_demo | C++ | N/A | make | Mínima (captura PS) |

### QtDemo / Qt5demo

A demo de referência oficial da Hikvision. Funcionalidades completas:

- Login/logout em dispositivos
- Preview ao vivo (real play)
- PTZ (controle, presets, cruises)
- Playback por tempo/arquivo
- Configuração remota de parâmetros (alarme, rede, serial, usuário, etc.)
- Gerenciamento de dispositivo (status, formato, log, reboot, upgrade)
- Áudio (broadcast, intercom, transfer)
- Árvore de dispositivos persistente (`device_tree.txt`)
- Suporte a alarmes e logs

**Build:**
```bash
# Copiar as .so para o diretório da demo
cp -r lib/* QtDemo/Linux64/lib/

# Compilar com qmake
cd QtDemo/Linux64/QtCreator
qmake QtClientDemo.pro
make

# Executar
cd ../lib
export LD_LIBRARY_PATH=.:./HCNetSDKCom
./QtClientDemo
```

### rustdemo (Rust + Slint)

Port moderna do QtDemo em Rust, usando [Slint](https://slint.dev/) (UI declarativa) e `libloading` (carregamento dinâmico da SDK).

**Status dos módulos:**

| Módulo | Status |
|--------|--------|
| SDK FFI, login, callbacks | Funcional |
| Árvore de dispositivos | Funcional |
| Painel de alarmes/logs | Funcional |
| Preview ao vivo | Esqueleto |
| Playback | Esqueleto |
| Configuração | Esqueleto |
| PTZ | Não implementado |

**Build:**
```bash
cd rustdemo
cargo build
./run.sh
```

Requer Rust 1.75+ e as `.so` em `Linux64/lib/`.

### consoleDemo

Menu interativo no terminal com acesso a todas as funções básicas:

```
1 - GetStream         5 - PlayBack
2 - Config Params     6 - Voice
3 - Alarm             7 - SDK ability
4 - Capture Picture   8 - Tool interface
```

**Build:**
```bash
cd consoleDemo/linux64/proj
make
cd ../lib
export LD_LIBRARY_PATH=.:./HCNetSDKCom
./sdkTest
```

### LinuxJavaDemo

Interface Swing acessada via JNA (Java Native Access).

**Pré-requisitos:** JDK 8, JNA (já incluso).

**Build/execução:**
```bash
cd LinuxJavaDemo
./run.sh
```

### psdatacall_demo

Demo mínima para captura de stream PS (Program Stream) de um canal para arquivo.

Configure o dispositivo em `Device.ini` e compile:
```bash
cd psdatacall_demo
cp -r ../lib/* .
make
./getpsdata
```

## SDK Libraries

As bibliotecas em `lib/` devem estar acessíveis em tempo de execução. Opções:

1. Copiar para o diretório do executável
2. `export LD_LIBRARY_PATH=/caminho/lib:/caminho/lib/HCNetSDKCom`
3. Adicionar ao `/etc/ld.so.conf` e rodar `ldconfig`
4. Usar `-Wl,-rpath` na linkagem

```bash
export LD_LIBRARY_PATH=$PWD/lib:$PWD/lib/HCNetSDKCom
```

## Compatibilidade

### Sistemas testados

| Sistema | Status |
|---------|--------|
| Ubuntu 20.04+ (x86_64) | Testado |
| Debian 11+ (x86_64) | Provável |
| Fedora 36+ (x86_64) | Provável |
| Arch Linux (x86_64) | Provável |
| Windows (via VS2008) | Projetos incluídos, DLLs não inclusas |

### Dependências de build

- **C++:** g++ (C++11), make
- **Qt4:** Qt 4.7+, libqt4-dev
- **Qt5:** Qt 5.12+, qtbase5-dev, libqt5widgets5
- **Rust:** Rust 1.75+, cargo
- **Java:** JDK 8, Apache Ant (opcional)

### Estrutura de dados do dispositivo

Todas as demos compartilham o formato `device_tree.txt`:

```xml
<device>
nome
ip
porta
usuario
senha
secret_key
<channel>
nome
numero
protocolo (0=Tcp,1=Udp,2=Mcast,3=Rtp,4=Rtsp,5=Https)
stream (0=Main,1=Sub,2=Third,3=Trans,4=Fourth)
</channel>
</device>
```

## Licença

Este repositório contém software licenciado sob GPL/LGPL/BSD/MIT (OpenSSL, libiconv, libsrtp, cJSON, TinyXML). Consulte `doc/Open Source Software Licenses-*.txt` para detalhes. A SDK da Hikvision possui licença proprietária — consulte a Hikvision para termos de uso.
