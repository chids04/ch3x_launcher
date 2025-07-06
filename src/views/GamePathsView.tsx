import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { useState, useEffect } from "react";
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

import GameTable from "@/components/GameTable";

export interface GameDir {
  name: string
  path: string
}

const games: GameDir[] = [
  {
    name: "Game 1",
    path: "/path/to/game1.iso"
  },
  {
    name: "Game 2",
    path: "/path/to/game2.wbfs"
  },
  {
    name: "Game 3",
    path: "/path/to/game3.iso"
  }
]

export default function GamePathsView() {
  const [error, setError] = useState("")
  const [name, setName] = useState("")
  const [dolphPath, setDolphPath] = useState("")
  const [gameDir, setGameDir] = useState("")

  const [savedDirs, setSavedDirs] = useState<GameDir[]>([])

  const loadGameDirs = async () => {
    const dirs = await invoke<GameDir[]>("get_gamedirs");

    if(dirs){
      setSavedDirs(dirs)
    }
  }

  const loadDolphPath = async () => {
    const path = await invoke<string>("get_dolph_path");

    if(path){
      setDolphPath(path)
    }
  }

  useEffect(() => {
    loadGameDirs()
    loadDolphPath()
    
  }, [])

  const handleSetDolph = async (path: string) => {
    await invoke("set_dolph_path", {path: path.trim()})
  }

  const selectDolphPath = async () => {
    const path = await open({
      multiple: false,
      directory: false,
    });

    if(path){
      setDolphPath(path)
      handleSetDolph(path)
    }
  }

  const handleFileSelection = async ()  => {
    const dir = await open({
      multiple: false,
      directory: false,
    });

    if(dir){
      setGameDir(dir)
    }
  }

  const handleDirDelete = async (index: number) => {
    await invoke("remove_gamedir", {index})
    loadGameDirs()
  };

  const handleCreateGameDir = async ()  => {
    if(name == ""){
      handleError("name missing")
      return
    }
    else if (gameDir == ""){
      handleError("missing path")
      return
    }

    try {
      await invoke("create_gamedir", {name, path: gameDir})
    }
    catch(error){
      handleError(String(error))
      return
    }

    loadGameDirs()
  }

  const handleError = (msg: string) => {
    setError(msg)

    setTimeout(() => {
      setError("")
    }, 3000)
  }

  
  return (
    <>
    <div className="p-4">
      <div className="mb-4">
        <h1 className=" text-2xl font-bold">paths</h1>
      </div>

      <hr className="w-full my-1 mb-5 border-neutral-500" />

      <div className="flex flex-col items-center gap-5">

        <div className="flex w-full gap-2 mb-6">
          <p className="w-[200px]">dolphin path:</p>
          <Input defaultValue={dolphPath} onChange={(e) => (handleSetDolph(e.target.value))}/>
          <Button variant="outline" onClick={selectDolphPath}>select path</Button>
        </div>


        <div className="w-full">
          <h1 className="font-bold">games</h1>
        </div>

        <div className="flex gap-2 w-full">
          <Input 
            className="w-[150px] text-center" 
            placeholder="name"
            onChange={(e) => setName(e.target.value)}
            >
          </Input>
          <Input 
            className="text-center truncate" 
            value={gameDir}
            onChange={(e) => setGameDir(e.target.value)}
            placeholder="game path (.iso, .wbfs ..etc)">
          </Input>
          <Button  variant="outline" onClick={handleFileSelection}>select path</Button>
        </div>

        <Button className="mt-5" variant="outline" onClick={handleCreateGameDir}>add path</Button>

        {error && (
          <div
            className="mt-4 p-3 rounded-md text-sm w-full text-center bg-red-900/30 text-red-400">
            {error}
          </div>
        )}
      </div>

        <div className="mt-5">
          <GameTable games={savedDirs} handleDelete={handleDirDelete}/>
        </div>

    </div>
    </>
  );
}

