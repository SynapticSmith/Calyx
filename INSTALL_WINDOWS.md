# 🌸 Installing Calyx Natively on Windows (CPU-only Guide)

Welcome! This guide is written specifically for Windows users. We assume **absolutely zero prior programming or terminal experience**—we will walk you through every single download, button click, and command from scratch.

By the end of this guide, you will have Calyx compiled and running natively on your Windows computer using your computer's processor (CPU-only mode).

---

## 🛠️ Step 1: Install Git (to download Calyx)

Calyx's source code is hosted on GitHub. To download it, you need a free tool called **Git**.

1. **Download**: Go to [git-scm.com/download/win](https://git-scm.com/download/win) and click on **64-bit Git for Windows Setup**.
2. **Install**: Open the downloaded `.exe` file. Click **Next** on all the default options—you don't need to change anything!
3. **Verify**:
   - Click your Windows Start menu, type `PowerShell`, and open **Windows PowerShell**.
   - Type the following command and press **Enter**:
     ```powershell
     git --version
     ```
   - If it prints something like `git version 2.x.x`, Git is successfully installed!

---

## 🔨 Step 2: Install C++ Build Tools (C compiler for SQLite)

Calyx uses an embedded database (SQLite) for state and migrations, which requires a **C/C++ compiler** to build. On Windows, we install this via the official Microsoft Visual Studio Build Tools.

1. **Download**: Go to [visualstudio.microsoft.com/visual-cpp-build-tools/](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and click **Download Build Tools**.
2. **Install**:
   - Run the downloaded installer (`vs_buildtools.exe`).
   - It will open a window with various "Workloads".
   - Check the box for **Desktop development with C++** (it is usually the first box under the "Desktop & Mobile" section).
   - On the right side under "Installation details", ensure that the default options (specifically **MSVC v143...** and **Windows 11 SDK** or **Windows 10 SDK**) are checked.
   - Click **Install** (this may take a few gigabytes and a few minutes to download and install).
   - You may be prompted to restart your computer once it's finished.

---

## 🦀 Step 3: Install Rust (the Rust Compiler)

Calyx is written entirely in the Rust programming language. We need the Rust compiler to turn the source code into an executable program.

1. **Download**: Go to [rustup.rs](https://rustup.rs/) and click **download rustup-init.exe (64-bit)**.
2. **Install**:
   - Run `rustup-init.exe`.
   - A black command window will open. It will ask you how you want to install.
   - Simply press **1** and hit **Enter** to proceed with the default installation (option `1) Proceed with installation (default)`).
   - Once it finishes, it will say "Rust is installed now. Great!". Press **Enter** to close the window.
3. **Verify**:
   - Open a **brand new** PowerShell window (so that the new tools are recognized in your path).
   - Type the following command and press **Enter**:
     ```powershell
     cargo --version
     ```
   - If it prints something like `cargo 1.x.x`, Rust is successfully installed!

---

## 📁 Step 4: Download (Clone) the Calyx Code

Now, we will download the Calyx files onto your computer.

1. In your PowerShell window, decide where you want to download the code. For example, to go to your **Documents** folder, type:
   ```powershell
   cd ~/Documents
   ```
2. Clone the repository by typing this command and pressing **Enter**:
   ```powershell
   git clone https://github.com/ChrisRoyse/Calyx.git
   ```
3. Move into the downloaded `Calyx` directory:
   ```powershell
   cd Calyx
   ```

---

## 🚀 Step 5: Build and Compile Calyx (CPU-only)

Now, we will compile Calyx! By default, Calyx builds in CPU-only mode, which uses highly optimized CPU SIMD math.

1. **Run the check**: To verify that your system is fully ready to compile, run:
   ```powershell
   cargo check --workspace
   ```
   *This will download all necessary library dependencies and verify that everything compiles perfectly.*

2. **Build the executable**: Compile the database and CLI tool by running:
   ```powershell
   cargo build --release --workspace
   ```
   *This may take 2 to 5 minutes as it performs advanced compiler optimizations to ensure Calyx runs as fast as possible on your CPU.*

3. **Verify the builds**: Once complete, the compiled executables will be located in your `target\release` folder. You can test running the main Calyx command-line utility by typing:
   ```powershell
   cargo run -p calyx-cli -- --help
   ```
   If you see the help output for the Calyx command-line tool, **your installation was 100% successful!**

---

## 🧪 Step 6: Running the Tests (Optional)

If you want to ensure that every mathematical and database engine component is working perfectly on your specific CPU architecture, you can run the test suite:

```powershell
cargo test -p calyx-core
```

---

## 🎉 Troubleshooting & Tips

* **Error: `link.exe` not found**: This means the Visual Studio C++ Build Tools (Step 2) were not installed correctly, or you did not restart your computer/PowerShell after installing them. Make sure **Desktop development with C++** is fully installed.
* **CPU vs GPU**: If you have an NVIDIA graphics card and want to use GPU acceleration in the future, you will need to install the NVIDIA CUDA Toolkit (version 13.2+) and compile with `--features cuda`. However, the CPU-only build is highly recommended and portable!
