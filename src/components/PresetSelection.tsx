import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

import type { Options } from "@/components/PresetItem"

interface PresetSelectionArgs {
    options: Options,
    handleSelection: (selection: string, opt_name: string) => void
}

export default function PresetSelection({options, handleSelection} : PresetSelectionArgs){

    return (
        <div className="flex flex-row">
            <div className="w-[150px] break-words mr-3">{options.name}</div>
            <Select  defaultValue={options.selected}  onValueChange={(value: string) => handleSelection(value, options.name)}>
                <SelectTrigger className="w-full">
                    <SelectValue placeholder="select an option"/>
                </SelectTrigger>
                <SelectContent>
                    {options.choices?.map((choice: string, index: number) => (
                        <SelectItem value={choice} key={index.toString()}>
                            {choice}
                        </SelectItem>

                    )) || <div>no options found</div>}
                </SelectContent>
            </Select>
        </div>
    )
        


}