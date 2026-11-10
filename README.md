 **yorq is:**
A minimalist CLI tool for managing, automatically deploying (soon), and scaling Docker containers.
```bash
let help = r#"yorq - lightweight Docker container CLI manager

USAGE:
    yorq <COMMAND> [OPTIONS]

COMMANDS:
    ls                 List all containers with status and timestamps
    inspect <id/name>  Show container state and configuration
    start   <id/name>  Start specified container
    stop    <id/name>  Stop specified container
    sweep              Interactively clean containers older than 24h"#;
```
