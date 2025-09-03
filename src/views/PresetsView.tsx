import PresetItem from "@/components/PresetItem"
import PresetModal from "@/components/PresetModal";
import { useState, useEffect } from "react";
import { invoke } from '@tauri-apps/api/core';
import { Button } from "@/components/ui/button";



import type { Preset } from "@/components/PresetItem"

export default function PresetsView() {
  const [presets, setPresets] = useState<Preset[]>([])
  const [isModalOpen, setIsModalOpen] = useState(false);
  

  useEffect(() => {
    const getPresets = async () => {
        const loaded_presets = await invoke<Preset[]>('get_presets');
        setPresets(loaded_presets);
    }
    getPresets();
  }, [isModalOpen])

  const removePreset = (id: string) => {
        const new_presets = presets.filter(p => p.id !== id)
        setPresets(new_presets)
    }

    return (
        <>
            <div className="p-4">
                <div className="flex justify-between items-center mb-4">
                    <h1 className="text-2xl font-bold">Presets</h1>
                    <Button 
                        onClick={() => setIsModalOpen(true)}
                        variant="outline"
                    >
                        add preset
                    </Button>
                </div>
                
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 auto-rows-min ">
                    {presets.map(preset => (
                        <PresetItem 
                            key={preset.id}
                            preset={preset}
                            onRemove={removePreset}
                        />
                    ))}
                </div>
            </div>

            <PresetModal
                isOpen={isModalOpen}
                onClose={() => setIsModalOpen(false)}
            />
        </>
    )
}
