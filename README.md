# foundry

## Overview

Font building toolkit

## Usage

### Commands

Create a new font project

```
foundry init
```

Build your font

```
foundry build
```

View your font on a webpage

```
foundry server
```

Help

```
foundry help
```

## Development

### Building

```
cargo build
```

### Optimizing

#### Tools

#### Profiling

```sh
cargo build --profile profiling
samply record .\target\profiling\foundry.exe

cargo build --profile profiling; samply record .\target\profiling\foundry.exe
```
