import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"

import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/context-menu"
import { GameDir } from "@/views/GamePathsView"
import { useState } from "react"

interface GameTableArgs{
    games: GameDir[],
    handleDelete: (index: number) => void;
}

export default function GameTable({games, handleDelete}: GameTableArgs) {
    
    return (
        
        <ContextMenu>
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead className="w-[150px] font-bold">name</TableHead>
                        <TableHead>path</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                {games.map((game, index) => (
                    <ContextMenu key={game.path}>
                        <ContextMenuTrigger asChild>
                            <TableRow>
                                <TableCell className="font-medium">{game.name}</TableCell>
                                <TableCell>{game.path}</TableCell>
                            </TableRow>
                        </ContextMenuTrigger>

                        <ContextMenuContent className="w-52">
                            <ContextMenuItem className="text-red-500" onSelect={(e) => handleDelete(index)}>
                                delete
                            </ContextMenuItem>
                        </ContextMenuContent>
                    </ContextMenu>
                ))}
                </TableBody>
            </Table>
        </ContextMenu>
        
    )
}
