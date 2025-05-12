"use client";

import { useEffect, useRef, useState } from "react";

import init, { Chip8 } from "@/utils/wasm/chip_8";

const KEY_MAP: { [key: string]: number } = {
    Digit1: 0x1,
    Digit2: 0x2,
    Digit3: 0x3,
    Digit4: 0xC,
    KeyQ: 0x4,
    KeyW: 0x5,
    KeyE: 0x6,
    KeyR: 0xD,
    KeyA: 0x7,
    KeyS: 0x8,
    KeyD: 0x9,
    KeyF: 0xE,
    KeyZ: 0xA,
    KeyX: 0x0,
    KeyC: 0xB,
    KeyV: 0xF,
} as const;

type KeyMapType = keyof typeof KEY_MAP;

export default function Canvas() {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const [engine, setEngine] = useState<Chip8 | null>(null);
    const [audioContext, setAudioContext] = useState<AudioContext | null>(null);
    const [oscillator, setOscillator] = useState<OscillatorNode | null>(null);
    const [soundActive, setSoundActive] = useState(false);

    const initializeEngine = async (setEngine: (engine: Chip8) => void) => {
        try {
            await init();

            const engine = new Chip8();
            setEngine(engine);

            const response = await fetch("/games/PONG(1P)");
            if (!response.ok) throw new Error("Failed to fetch ROM");
            const rom = new Uint8Array(await response.arrayBuffer());
            engine.load_rom(rom);
        } catch (error) {
            console.error("Error initializing engine:", error);
        }
    };

    useEffect(() => {
        initializeEngine(setEngine);
    }, []);

    const handleSound = () => {
        if (soundActive) {
            if (!oscillator && audioContext) {
                const osc = audioContext.createOscillator();
                osc.type = "square";
                osc.frequency.setValueAtTime(440, audioContext.currentTime);
                osc.connect(audioContext.destination);
                osc.start();
                setOscillator(osc);
            }
        } else {
            if (oscillator) {
                oscillator.stop();
                oscillator.disconnect();
                setOscillator(null);
            }
        }
    }

    const handleUserInteractionAudio = () => {
        if (!audioContext && typeof AudioContext !== "undefined") {
            const audioCtx = new AudioContext();
            setAudioContext(audioCtx);
        }
    };

    useEffect(() => {
        if (engine) handleSound();
    }, [engine, soundActive]);

    useEffect(() => {
        const handleKeyDown = (event: KeyboardEvent) => {
            const key = event.code as KeyMapType;

            if (KEY_MAP[key] !== undefined) {
                engine?.key_down(KEY_MAP[key]);
            }
        };

        const handleKeyUp = (event: KeyboardEvent) => {
            const key = event.code as KeyMapType;

            if (KEY_MAP[key] !== undefined) {
                engine?.key_up(KEY_MAP[key]);
            }
        };

        window.addEventListener("keydown", handleKeyDown);
        window.addEventListener("keyup", handleKeyUp);

        return () => {
            window.removeEventListener("keydown", handleKeyDown);
            window.removeEventListener("keyup", handleKeyUp);
        };
    }, [engine]);

    useEffect(() => {
        const canvas = canvasRef.current;
        const context = canvas?.getContext("2d");

        if (!engine || !canvas || !context) return;

        const width = engine.get_width();
        const height = engine.get_height();

        canvas.width = width;
        canvas.height = height;

        context.fillStyle = "FFFFFF";
        context.fillRect(0, 0, width, height);

        const renderFrame = () => {
            const executeCycle = (steps: number) => {
                for (let i = 0; i < steps; i++) {
                    engine.execute_cycle();
                }

                engine.decrement_timer();
            };

            const drawDisplay = () => {
                const display = engine.get_display();
                const image = context.createImageData(width, height);

                for (let i = 0; i < width * height; i++) {
                    image.data[i * 4] = display[i] === 1 ? 0x33 : 0;
                    image.data[i * 4 + 1] = display[i] === 1 ? 0xff : 0;
                    image.data[i * 4 + 2] = display[i] === 1 ? 0x66 : 0;
                    image.data[i * 4 + 3] = 255;
                }

                context.putImageData(image, 0, 0);
            }

            executeCycle(12);
            drawDisplay();
            setSoundActive(engine.is_sound_active());
        };

        const interval = setInterval(renderFrame, 1000 / 60);

        return () => clearInterval(interval);
    }, [engine]);

    return (
        <canvas onClick={handleUserInteractionAudio} ref={canvasRef}></canvas>
    );
}
