import * as React from "react"

import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

import { useEffect, useState} from "react"
import { invoke } from "@tauri-apps/api/core"

interface GameDir {
    name: string,
    path: string
}

interface SelectionProps {
    id: string
}


export default function GameDirSelection( { id }: SelectionProps ) {
    const [gameDirs, setGameDirs] = useState<GameDir[]>([])
    const [name, setName] = useState("")
    
    const getDirName = async () => {
        const dir_name = await invoke<string | null>("get_path_name", {id});

        console.log("dir name", dir_name)

        if(dir_name !== null){
            setName(dir_name)
        }
        else{
            setName("")
        }
    }

    useEffect(() => {
        const loadGameDirs = async () => {

            const dirs = await invoke<GameDir[]>("get_gamedirs");

            if(dirs){
                setGameDirs(dirs)
            }
        }

        loadGameDirs()
        getDirName()
    }, [])


    const handlePath = async (path: string) => {
        await invoke("set_game_path", {id, path})

        await getDirName()
    }

    return (
        <Select  value={name}  onValueChange={(value: string) => handlePath(value)}>
            <SelectTrigger>
                <SelectValue placeholder="select an option">
                    {name}
                </SelectValue>
            </SelectTrigger>
            <SelectContent>
                {gameDirs.map((gameDir,index) => (
                    <SelectItem value={gameDir.path} key={index.toString()}>
                        <div className="flex">
                            <p className="w-[100px]">{gameDir.name}</p>
                            <p>{gameDir.path}</p>
                        </div>
                    </SelectItem>

                )) || <div>no options found</div>}
            </SelectContent>
        </Select>
    )
}
