import { GameCanvas } from './components/GameCanvas';

function App() {
  return (
    <div className="min-h-screen bg-gray-900 flex flex-col items-center justify-center p-4">
      <div className="text-center mb-8">
        <h1 className="text-4xl md:text-5xl text-white font-pixel mb-4 text-shadow-glow tracking-widest">
          MECHA KOMBAT
        </h1>
        <p className="text-gray-400 font-pixel text-sm">Retro Pixel Fighting Game</p>
      </div>
      
      <GameCanvas />
      
      <div className="mt-8 text-gray-500 font-pixel text-xs text-center max-w-2xl space-y-2">
        <p>Player 1: W/A/S/D to move, F to attack, G to defend.</p>
        <p>Player 2: Arrows to move, K to attack, L to defend.</p>
      </div>
    </div>
  );
}

export default App;
