import React, { useEffect, useRef, useState } from 'react';

// Types
interface Position {
  x: number;
  y: number;
}

interface Velocity {
  x: number;
  y: number;
}

interface AttackBox {
  position: Position;
  width: number;
  height: number;
  offset: Position;
}

class Fighter {
  position: Position;
  velocity: Velocity;
  width: number;
  height: number;
  color: string;
  attackBox: AttackBox;
  isAttacking: boolean;
  isDefending: boolean;
  health: number;
  facing: 'left' | 'right';
  canvasWidth: number;
  canvasHeight: number;
  onHealthChange: (health: number) => void;

  constructor(
    position: Position,
    color: string,
    facing: 'left' | 'right',
    canvasWidth: number,
    canvasHeight: number,
    onHealthChange: (health: number) => void
  ) {
    this.position = position;
    this.velocity = { x: 0, y: 0 };
    this.width = 50;
    this.height = 100;
    this.color = color;
    this.facing = facing;
    this.canvasWidth = canvasWidth;
    this.canvasHeight = canvasHeight;
    this.onHealthChange = onHealthChange;
    this.attackBox = {
      position: { x: this.position.x, y: this.position.y },
      width: 100,
      height: 50,
      offset: facing === 'right' ? { x: 0, y: 10 } : { x: -50, y: 10 }
    };
    this.isAttacking = false;
    this.isDefending = false;
    this.health = 100;
  }

  draw(ctx: CanvasRenderingContext2D) {
    // Mecha Body
    ctx.fillStyle = this.color;
    ctx.fillRect(this.position.x, this.position.y, this.width, this.height);

    // Mecha Details (Inner panels)
    ctx.fillStyle = 'rgba(0, 0, 0, 0.3)';
    ctx.fillRect(this.position.x + 10, this.position.y + 30, this.width - 20, this.height - 40);

    // Head/Visor area
    ctx.fillStyle = '#111';
    ctx.fillRect(this.position.x + 5, this.position.y + 5, this.width - 10, 25);

    // Eye/Visor
    ctx.fillStyle = '#0ff';
    const eyeX = this.facing === 'right' ? this.position.x + 25 : this.position.x + 10;
    ctx.fillRect(eyeX, this.position.y + 12, 15, 8);
    // Visor glow
    ctx.fillStyle = 'rgba(0, 255, 255, 0.5)';
    ctx.fillRect(eyeX - 2, this.position.y + 10, 19, 12);

    // Energy core in chest
    ctx.fillStyle = this.health > 30 ? '#0f0' : '#f00';
    ctx.fillRect(this.position.x + 20, this.position.y + 45, 10, 10);

    // Legs
    ctx.fillStyle = '#333';
    ctx.fillRect(this.position.x + 5, this.position.y + this.height - 15, 15, 15);
    ctx.fillRect(this.position.x + this.width - 20, this.position.y + this.height - 15, 15, 15);

    // Defense Shield
    if (this.isDefending) {
      ctx.strokeStyle = '#0ff';
      ctx.lineWidth = 4;
      ctx.beginPath();
      const shieldX = this.facing === 'right' ? this.position.x + this.width + 10 : this.position.x - 10;
      ctx.arc(shieldX, this.position.y + this.height / 2, 60, 0, Math.PI * 2);
      ctx.stroke();
      
      // Inner shield glow
      ctx.fillStyle = 'rgba(0, 255, 255, 0.2)';
      ctx.beginPath();
      ctx.arc(shieldX, this.position.y + this.height / 2, 60, 0, Math.PI * 2);
      ctx.fill();
    }

    // Attack Box (Energy Sword)
    if (this.isAttacking) {
      ctx.fillStyle = this.color === '#ff3366' ? '#ffeb3b' : '#33ffcc';
      
      // Main sword blade
      ctx.fillRect(
        this.attackBox.position.x,
        this.attackBox.position.y + 10,
        this.attackBox.width,
        this.attackBox.height - 20
      );
      
      // Sword glow
      ctx.fillStyle = this.color === '#ff3366' ? 'rgba(255, 235, 59, 0.4)' : 'rgba(51, 255, 204, 0.4)';
      ctx.fillRect(
        this.attackBox.position.x,
        this.attackBox.position.y,
        this.attackBox.width,
        this.attackBox.height
      );
    }
  }

  update(ctx: CanvasRenderingContext2D) {
    this.draw(ctx);
    
    // Attack box position follows player
    this.attackBox.position.x = this.position.x + this.attackBox.offset.x;
    this.attackBox.position.y = this.position.y + this.attackBox.offset.y;
    
    // Update facing offset dynamically if facing changes
    this.attackBox.offset.x = this.facing === 'right' ? 0 : -50;

    // Movement
    this.position.x += this.velocity.x;
    this.position.y += this.velocity.y;

    // Gravity
    const gravity = 0.7;
    if (this.position.y + this.height + this.velocity.y >= this.canvasHeight - 50) {
      this.velocity.y = 0;
      this.position.y = this.canvasHeight - 50 - this.height;
    } else {
      this.velocity.y += gravity;
    }

    // Border collision
    if (this.position.x <= 0) {
      this.position.x = 0;
    } else if (this.position.x + this.width >= this.canvasWidth) {
      this.position.x = this.canvasWidth - this.width;
    }
  }

  attack() {
    if (!this.isAttacking) {
      this.isAttacking = true;
      setTimeout(() => {
        this.isAttacking = false;
      }, 150); // attack duration
    }
  }

  defend() {
    this.isDefending = true;
  }

  stopDefend() {
    this.isDefending = false;
  }

  takeHit(damage: number) {
    if (this.isDefending) {
      // Shield absorbs most damage
      this.health -= Math.floor(damage * 0.1); 
    } else {
      this.health -= damage;
    }
    
    if (this.health < 0) this.health = 0;
    this.onHealthChange(this.health);
  }
}

export const GameCanvas: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  
  const [p1Health, setP1Health] = useState(100);
  const [p2Health, setP2Health] = useState(100);
  const [winner, setWinner] = useState<string | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const canvasWidth = 800;
    const canvasHeight = 450;
    canvas.width = canvasWidth;
    canvas.height = canvasHeight;

    // Entities
    const player = new Fighter(
      { x: 100, y: 0 },
      '#ff3366', // Cyberpunk red/pink
      'right',
      canvasWidth,
      canvasHeight,
      (h) => setP1Health(h)
    );

    const enemy = new Fighter(
      { x: 650, y: 0 },
      '#3366ff', // Cyberpunk blue
      'left',
      canvasWidth,
      canvasHeight,
      (h) => setP2Health(h)
    );

    const keys = {
      a: { pressed: false },
      d: { pressed: false },
      w: { pressed: false },
      ArrowRight: { pressed: false },
      ArrowLeft: { pressed: false },
      ArrowUp: { pressed: false }
    };

    let animationId: number;

    const handleKeyDown = (e: KeyboardEvent) => {
      // Player 1
      switch (e.key) {
        case 'd': keys.d.pressed = true; player.facing = 'right'; break;
        case 'a': keys.a.pressed = true; player.facing = 'left'; break;
        case 'w': if (player.velocity.y === 0) player.velocity.y = -15; break;
        case 'f': player.attack(); break;
        case 'g': player.defend(); break;
      }
      // Player 2
      switch (e.key) {
        case 'ArrowRight': keys.ArrowRight.pressed = true; enemy.facing = 'right'; break;
        case 'ArrowLeft': keys.ArrowLeft.pressed = true; enemy.facing = 'left'; break;
        case 'ArrowUp': if (enemy.velocity.y === 0) enemy.velocity.y = -15; break;
        case 'k': enemy.attack(); break;
        case 'l': enemy.defend(); break;
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      switch (e.key) {
        case 'd': keys.d.pressed = false; break;
        case 'a': keys.a.pressed = false; break;
        case 'g': player.stopDefend(); break;
        
        case 'ArrowRight': keys.ArrowRight.pressed = false; break;
        case 'ArrowLeft': keys.ArrowLeft.pressed = false; break;
        case 'l': enemy.stopDefend(); break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);

    // Collision Detection
    function rectangularCollision({ rectangle1, rectangle2 }: { rectangle1: Fighter, rectangle2: Fighter }) {
      return (
        rectangle1.attackBox.position.x + rectangle1.attackBox.width >= rectangle2.position.x &&
        rectangle1.attackBox.position.x <= rectangle2.position.x + rectangle2.width &&
        rectangle1.attackBox.position.y + rectangle1.attackBox.height >= rectangle2.position.y &&
        rectangle1.attackBox.position.y <= rectangle2.position.y + rectangle2.height
      );
    }

    const animate = () => {
      animationId = window.requestAnimationFrame(animate);
      
      // Draw background
      ctx.fillStyle = '#0f0f1b'; // Dark background
      ctx.fillRect(0, 0, canvasWidth, canvasHeight);
      
      // Draw floor
      ctx.fillStyle = '#1a1a3a';
      ctx.fillRect(0, canvasHeight - 50, canvasWidth, 50);

      // Grid effect for floor
      ctx.strokeStyle = '#333366';
      ctx.lineWidth = 2;
      for (let i = 0; i < canvasWidth; i += 40) {
        ctx.beginPath();
        ctx.moveTo(i, canvasHeight - 50);
        ctx.lineTo(i - 20, canvasHeight);
        ctx.stroke();
      }

      player.update(ctx);
      enemy.update(ctx);

      // Player Movement
      player.velocity.x = 0;
      if (keys.a.pressed && player.facing === 'left') {
        player.velocity.x = -5;
      } else if (keys.d.pressed && player.facing === 'right') {
        player.velocity.x = 5;
      }

      // Enemy Movement
      enemy.velocity.x = 0;
      if (keys.ArrowLeft.pressed && enemy.facing === 'left') {
        enemy.velocity.x = -5;
      } else if (keys.ArrowRight.pressed && enemy.facing === 'right') {
        enemy.velocity.x = 5;
      }

      // Detect Collision & Hit
      if (
        rectangularCollision({ rectangle1: player, rectangle2: enemy }) &&
        player.isAttacking
      ) {
        player.isAttacking = false; // Prevent multiple hits per attack
        enemy.takeHit(10);
      }

      if (
        rectangularCollision({ rectangle1: enemy, rectangle2: player }) &&
        enemy.isAttacking
      ) {
        enemy.isAttacking = false;
        player.takeHit(10);
      }

      // Win Condition Check
      if (enemy.health <= 0) {
        setWinner('Player 1 Wins!');
        window.cancelAnimationFrame(animationId);
      } else if (player.health <= 0) {
        setWinner('Player 2 Wins!');
        window.cancelAnimationFrame(animationId);
      }
    };

    animate();

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
      window.cancelAnimationFrame(animationId);
    };
  }, []); // Run once on mount

  const resetGame = () => {
    window.location.reload();
  };

  return (
    <div className="relative flex flex-col items-center w-full max-w-[800px] mx-auto font-pixel select-none">
      {/* HUD */}
      <div className="absolute top-0 flex justify-between items-center w-full p-4 pointer-events-none">
        {/* P1 Health Bar */}
        <div className="flex flex-col w-[40%]">
          <span className="text-white text-sm mb-2 text-shadow">P1 (WASD/F/G)</span>
          <div className="h-6 border-4 border-white bg-red-900 w-full rounded-sm overflow-hidden">
            <div 
              className="h-full bg-red-500 transition-all duration-200 ease-out"
              style={{ width: `${p1Health}%` }}
            ></div>
          </div>
        </div>

        {/* Timer / VS */}
        <div className="text-yellow-400 text-3xl font-bold px-4 text-shadow-glow">
          VS
        </div>

        {/* P2 Health Bar */}
        <div className="flex flex-col w-[40%] items-end">
          <span className="text-white text-sm mb-2 text-shadow">P2 (Arrows/K/L)</span>
          <div className="h-6 border-4 border-white bg-blue-900 w-full rounded-sm overflow-hidden flex justify-end">
            <div 
              className="h-full bg-blue-500 transition-all duration-200 ease-out"
              style={{ width: `${p2Health}%` }}
            ></div>
          </div>
        </div>
      </div>

      {/* Game Canvas */}
      <canvas 
        ref={canvasRef} 
        className="border-8 border-gray-800 rounded-lg shadow-[0_0_20px_rgba(0,0,0,0.5)] bg-black"
        style={{ imageRendering: 'pixelated' }}
      />

      {/* End Game Screen */}
      {winner && (
        <div className="absolute inset-0 flex flex-col items-center justify-center bg-black/80 backdrop-blur-sm z-10 animate-fade-in">
          <h1 className="text-5xl text-yellow-400 mb-8 animate-bounce text-shadow-glow text-center leading-tight">
            {winner}
          </h1>
          <button 
            onClick={resetGame}
            className="px-8 py-4 bg-gray-800 border-4 border-white text-white hover:bg-white hover:text-black transition-colors duration-200 uppercase text-xl shadow-[4px_4px_0px_0px_rgba(255,255,255,0.5)] active:shadow-none active:translate-y-1 active:translate-x-1"
          >
            RESTART
          </button>
        </div>
      )}
    </div>
  );
};
