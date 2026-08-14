# 🚀 Getting Started with Calyx

Welcome to Calyx! Calyx is an association-native database built in Rust that focuses on grounded intelligence, memory, and high-performance search.

Whether you are a developer, a data scientist, or just exploring, this guide assumes **zero prior knowledge** of setting up the environment. By the end of this guide, you will have Calyx downloaded, compiled, and running, and you will understand how to connect it to external tools and agents.

Calyx exposes three main entry points (applications) that you will use:
1. **`calyx` (CLI)**: A powerful command-line tool to manage your data vaults, run indexing tasks, test queries, and check the health of your database.
2. **`calyxd` (Daemon)**: A background server (daemon) configured via a TOML file, primarily used to serve data, enforce health checks, and expose metrics endpoints.
3. **`calyx-mcp` (MCP Server)**: A Model Context Protocol (MCP) server that communicates over standard input/output (stdio) using JSON-RPC, designed to seamlessly connect Calyx's tools and data directly to AI agents and LLM harnesses.

---

## 🛠️ Step 1: Setup and Installation

Calyx is written in Rust and needs to be compiled from source. Calyx primarily runs on the CPU out of the box using highly optimized mathematical operations, so **you do not need a GPU to get started**.

Follow the instructions for your specific Operating System below.

### Windows

1. **Install Git**: Go to [git-scm.com/download/win](https://git-scm.com/download/win) and download the 64-bit installer. Click "Next" on all default options.
2. **Install C++ Build Tools**:
   - Go to [visualstudio.microsoft.com/visual-cpp-build-tools/](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and click **Download Build Tools**.
   - Run the installer. Check the box for **Desktop development with C++**.
   - Ensure the default options on the right (like MSVC v143 and Windows 10/11 SDK) are checked. Click Install. (You may need to restart your computer afterward).
3. **Install Rust**: Go to [rustup.rs](https://rustup.rs/) and download `rustup-init.exe`. Run it, and when the black command window opens, type `1` and press Enter to proceed with the default installation.

### macOS

1. **Install Homebrew** (if you don't have it): Open your Terminal app and paste:
   ```bash
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```
2. **Install Git**: In your terminal, run `brew install git`.
3. **Install Build Tools**: Apple includes the required C compiler tools with Xcode Command Line Tools. Run:
   ```bash
   xcode-select --install
   ```
4. **Install Rust**: Run the official Rust installer script:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Press `1` and Enter to accept the defaults. Restart your terminal.

### Linux (Ubuntu/Debian)

1. **Install Git and Build Tools**: Open your terminal and run:
   ```bash
   sudo apt update
   sudo apt install -y git build-essential curl pkg-config libssl-dev
   ```
2. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Press `1` and Enter to accept the defaults. Run `source $HOME/.cargo/env` to update your current terminal session.

---

## 📁 Step 2: Download and Compile Calyx

Now that your system has Git and the Rust compiler, you can download and build Calyx.

1. **Clone the repository**: Open your terminal/PowerShell and download the code:
   ```bash
   git clone https://github.com/ChrisRoyse/Calyx.git
   cd Calyx
   ```

2. **Verify dependencies**: Ensure everything is downloaded and ready to compile:
   ```bash
   cargo check --workspace
   ```

3. **Build the project**: Compile Calyx in Release mode (optimized for CPU speed):
   ```bash
   cargo build --release --workspace
   ```
   *(Note: This step may take a few minutes as the compiler performs advanced optimizations.)*

You now have successfully installed Calyx! The compiled executable programs are located in the `target/release/` folder.

---

## 🚀 Step 3: Running the Applications

### 1. The CLI (`calyx`)

The CLI is your primary management interface. You can run it directly using `cargo run`.

**Check the help menu to see all available commands:**
```bash
cargo run --release -p calyx-cli -- --help
```

**Example: Check Vault Health**
If you have created a vault directory (e.g., at `/tmp/my_vault`), you can check its filesystem health:
```bash
cargo run --release -p calyx-cli -- fsv vault-health --vault /tmp/my_vault
```

### 2. The Daemon (`calyxd`)

The daemon is designed to run in the background, serving data and periodically verifying the integrity of your data chains. It **requires a configuration file** to run.

**Create a minimal configuration file:**
Create a new file named `calyx.toml` in your current directory with the following minimal content:

```toml
# calyx.toml - Minimal Daemon Configuration
vram_budget_mib = 8192
vault_path = "/tmp/calyx_vault"
log_dir = "/tmp/calyx_logs"

# The address the daemon will bind to for internal metrics and healthchecks
bind_addr = "127.0.0.1:7700"
```

**Validate your configuration:**
Before starting the daemon, it's good practice to validate the file to ensure there are no typos:
```bash
cargo run --release -p calyxd -- --config calyx.toml --validate-config
```

**Start the Daemon:**
Once validated, you can start the daemon:
```bash
cargo run --release -p calyxd -- --config calyx.toml
```
The daemon will now run continuously, serving internal metrics and monitoring the configured `vault_path`.

### 3. The MCP Server (`calyx-mcp`)

The Model Context Protocol (MCP) server allows AI agents (like Claude Desktop, Cursor, or custom LangChain setups) to invoke tools natively within Calyx. It communicates purely via **Standard Input and Output (stdio)** using the **JSON-RPC** protocol.

You start the MCP server just like the CLI:
```bash
cargo run --release -p calyx-mcp
```
When it starts, it will write a log to `stderr` indicating how many tools it registered (e.g., `calyx-mcp: registered 31 tools`), and then it will wait for JSON-RPC messages on `stdin`.

#### Connecting to Agentic Harnesses (Raw JSON-RPC Example)

Agentic harnesses connect to `calyx-mcp` by launching it as a subprocess and sending JSON strings to it. Because it is a generic stdio MCP server, you can interface with it directly via JSON-RPC.

Here is an example of what the raw communication looks like.

**Request (Sent by the Agent to `stdin`):**
To execute a tool, the agent sends a JSON-RPC request containing an ID, the method `tools/call`, and the parameters for the specific tool.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "example_tool_name",
    "arguments": {
      "param1": "value1",
      "param2": 42
    }
  }
}
```

**Response (Output by `calyx-mcp` to `stdout`):**
The server processes the request and replies with a JSON-RPC response containing the result or an error.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "The tool executed successfully and here is the output."
      }
    ]
  }
}
```

By pointing your agent's MCP configuration to execute `cargo run --release -p calyx-mcp` (or directly to the compiled executable at `target/release/calyx-mcp`), your agent can seamlessly query your Calyx data and manage the database using native, structured tool calls!
