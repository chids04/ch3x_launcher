import PresetSelection from "@/components/PresetSelection"
import GameDirSelection from "@/components/GameDirSelection";
import { useState, useEffect } from "react";
import { invoke } from '@tauri-apps/api/core';
import { Button } from "@/components/ui/button"


export interface Preset {
  id: string,
  name: string,
  options: Options[]
  created_at: string
  game_path: string

}

export interface Options {
    name: string;
    selected: string;
    choices: string[];
}

interface PresetItemProps {
    preset: Preset;
    onRemove: (id: string) => void;
}

export default function PresetItem({preset, onRemove} : PresetItemProps) {
    const [error, setError] = useState("")
    const [gamePath, setGamePath] = useState("")
    const [dirName, setDirName] = useState("")
    
    const [isHovered, setHovered] = useState(false)

    const handleSelection = async (value: string, name: string) => {
        try {
            await invoke("set_selection", {id: preset.id, name, selection: value})
        }
        catch(error){
            setError(String(error))
        }
    }

    const handleDeletePreset = async () => {
        try {
            await invoke("remove_preset", {id: preset.id})
            onRemove(preset.id)
        }
        catch (error: any) {
            showError(String(error))

        }
        
    }

    
    const runGame = async () => {
        try{
            await invoke("run_game", {id: preset.id})

        } catch (error){
            showError(String(error))
        }
        
    }
    

    const showError = (msg: string) => {
        setError(msg);

        setTimeout(() => {
            setError("")
        }, 5000)
    }

    return (
        <>
            <div className="flex flex-col relative w-full border-2 p-4 border-dashed border-amber-50"
                onMouseEnter={() => setHovered(true)}
                onMouseLeave={() => setHovered(false)}
            >
                {isHovered && 
                    <div className="absolute top-1 left-1 select-none cursor-pointer"
                        onClick={() => handleDeletePreset()}>
                        X   
                    </div>
                }
                <div className="flex flex-col gap-2">
                    <p className="text-center">{preset.name}</p>
                    <hr className="my-1 border-neutral-500" />
                    {preset.options.map((option, index) => (
                        <PresetSelection
                            key={preset.id + index.toString()}
                            options={option}
                            handleSelection={handleSelection}
                        />
                    ))}
                </div>

                <div className="mt-5 mb-5 flex gap-2 items-center justify-center">
                    <p>game dir:</p>
                    <GameDirSelection id={preset.id}/>
                    
                </div>

                <Button className="mt-5" variant='outline' onClick={runGame}>play</Button>

                {error && (
                    <div className="mt-4 p-3 rounded-md text-sm bg-red-900/30 text-red-400">
                    {error}
                    </div>
                )}
            </div>


        </>
    )
}