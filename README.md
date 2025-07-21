# 🐦 Birds Evolution Simulation

A neural network-powered evolution simulation where birds learn to survive and find food using genetic algorithms. Watch as generations of birds evolve better survival strategies!

## 🚀 Getting Started

### Prerequisites
- Node.js (latest version)
- Rust (for building the WASM simulation engine)
- `wasm-pack` for compiling Rust to WebAssembly

### Installation & Setup

1. **Clone the repo**
   ```bash
   git clone https://github.com/m4jkiuwr/birds-evolution.git
   cd birds-evolution
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Build the WASM simulation engine**
   ```bash
   # Navigate to the simulation-wasm library
   cd libs/simulation-wasm
   
   # Build with wasm-pack
   wasm-pack build --target web
   
   # Go back to root
   cd ../..
   ```

4. **Start the development server**
   ```bash
   npm run dev
   ```

5. **Open your browser** and go to `http://localhost:5173`

## 🎮 How to Use

- **Canvas**: Watch the birds (blue triangles) move around and hunt for food (green circles)
- **Yellow birds**: Birds that are currently performing the best are marked yellow
- **Play/Pause**: Click the canvas or use the play button to control the simulation
- **Speed Control**: Adjust how fast the simulation runs
- **Generation Info**: See live stats about the current generation
- **History Chart**: Track performance across generations
- **Restart Panel**: Start a new simulation with custom population and food counts
- **Elite View**: Use the vision button to highlight only the best-performing birds

## 🛠️ Tech Stack

### Frontend
- **React 19** - UI framework
- **TypeScript** - Type safety
- **Vite** - Build tool and dev server
- **Tailwind CSS** - Styling

### Simulation Engine
- **Rust** - High-performance simulation logic
- **WebAssembly (WASM)** - Running Rust in the browser
- **Neural Networks** - Bird "brains" for decision making
- **Genetic Algorithms** - Evolution and learning mechanism

### Libraries Structure
- `libs/network/` - Neural network implementation
- `libs/genetic-algorithm/` - GA with selection, crossover, mutation
- `libs/simulation/` - Core simulation logic (animals, food, world)
- `libs/simulation-wasm/` - WASM bindings for the browser

## 🧠 How It Works

### The Strategy
1. **Birds have neural network "brains"** that take sensory input (what they can "see")
2. **Each generation**, birds try to survive and collect as much food as possible
3. **After each generation**, the best-performing birds are selected for breeding
4. **Genetic crossover and mutation** create the next generation with slightly modified neural networks
5. **Over time**, birds evolve better strategies for finding food and avoiding death

### Neural Network Input
- Eye sensors that detect food and other birds in different directions
- Current rotation and position information
- Distance to nearest food sources

### Neural Network Output
- Rotation adjustments (turn left/right)
- Movement speed
- Decision making for food collection

## 🎯 What You'll See

- **Early generations**: Birds move randomly, often missing food entirely
- **Later generations**: Birds develop hunting strategies, efficient movement patterns
- **Elite birds**: The best performers turn yellow, making it easy to spot the "smart" ones
- **Statistics tracking**: Min/max/average scores showing improvement over time
- **Real-time evolution**: Watch intelligence emerge from simple rules!
- **Elite filtering**: Toggle the view to see only the top performers and observe their superior strategies

## 🔧 Development Notes

### Architecture Decisions
- **WASM for performance**: Simulation runs in Rust for speed, UI in React for ease
- **State management**: Simulation object in `useRef`, world state in `useState`
- **Canvas rendering**: Custom drawing functions with high-DPI support
- **Responsive design**: Adapts to different screen sizes

