# ch3x

ch3x is a Tauri-based desktop application for managing and launching Dolphin emulator game presets with mod support. It provides a graphical interface to manage Riivolution XML files to launch with Dolphin emulator

## Features
- Import Riivolution XML files and select mod options
- Create and edit mod options
- Save and load application state using JSON file persistence
- Launch Dolphin emulator with selected preset and mod configuration

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) (for the frontend)
- [Dolphin Emulator](https://dolphin-emu.org/) (for running games)
- [npm](https://www.npmjs.com/) (for frontend dependencies)

## Building the Project

1. **Install Rust and Node.js**
   - Make sure you have Rust and Node.js installed on your system.

2. **Install Frontend Dependencies**
   - Navigate to the project root (where `package.json` is located):
     ```sh
     npm install
     ```

3. **Build the Frontend**
   - Run:
     ```sh
     pnpm run build
     # or
     npm run build
     ```

4. **Build the Tauri Backend**
   - From the `src-tauri` directory, run:
     ```sh
     cargo build --release
     ```

## Running the Application

### Development Mode
To run the app in development mode (with hot reload):

1. Start the frontend dev server:
   ```sh
   pnpm run dev
   # or
   npm run dev
   ```
2. In another terminal, run the Tauri dev process:
   ```sh
   cd src-tauri
   cargo tauri dev
   ```

### Production Mode
To run the built application:

1. Build the frontend and backend as described above.
2. Run the Tauri app:
   ```sh
   cd src-tauri
   cargo tauri build
   # The built app will be in the `src-tauri/target/release/bundle` directory
   ```

## Notes
- The application stores its data in a JSON file (`app_data.json`) in the same directory as `Cargo.toml`.
- All application state (presets, game directories, settings) is automatically saved when changes are made.
- Make sure Dolphin Emulator is installed and its path is set in the app settings.
- On first run, the app will create the necessary JSON data file automatically.



## todo
- hotkey to auto run specific mods
- include mod downloader
