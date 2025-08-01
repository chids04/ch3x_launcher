import { open } from '@tauri-apps/plugin-dialog';
import { useState, useEffect } from "react"
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@/components/ui/button'


interface PresetModalProps {
  isOpen: boolean
  onClose: () => void
}

export default function PresetModal({ isOpen, onClose}: PresetModalProps) {
  const [name, setName] = useState("")
  const [xmlPath, setXmlPath] = useState("")
  const [error, setError] = useState("")


  useEffect(() => {
      if(isOpen) {
        setError("")
        setXmlPath("")
      }

  },[isOpen])

  const handleSave = async () => {
    const id = crypto.randomUUID();

    if(xmlPath == ""){
      setError("Please select an XML path")
      return
    }

    try {
      const create_preset = await invoke('create_preset', {id: id, name: name, xmlPath: xmlPath})
      onClose()
    }
    catch(error){
      setError(String(error));
    }

    // onSave({ name, id, xmlPath})
    // setName("")
  }

  const selectXML = async () => {

    const file = await open({
      multiple: false,
      directory: false,
      filters: [{
        name: 'XML Files',
        extensions: ['xml'] // Optionally filter for XML files
      }]
    });

    

    if(file) {
      setXmlPath(file)
    }
  }

  const getFileName = (path: string) => {
    return path.split("/").pop();
  }

  

  if (!isOpen) return null

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
      <div className="bg-neutral-700 p-6 rounded-lg flex flex-col gap-2">
        <h2 className="text-xl mb-4">Create New Preset</h2>
        <input
          type="text"
          placeholder="Preset Name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="w-full p-2 mb-4 border rounded"
        />

        <div className="flex gap-2 max-w-xs items-center">
          <Button variant="outline" onClick={selectXML}>select xml</Button>
          <p className="truncate">{getFileName(xmlPath)}</p>
        </div>
        
        {/* <input
          type="text"
          placeholder="Description"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          className="w-full p-2 mb-4 border rounded"
        /> */}

        <hr className="my-4 border-neutral-500" />
        
        <div className="flex gap-2 justify-center">
          <Button variant="outline" onClick={handleSave}>save</Button>
          <Button variant="outline" onClick={onClose}>close</Button>
        </div>

        {error && (
          <div className="bg-red-500 rounded-md p-2 mt-4 text-center">
            {error}
          </div>
        )}

      </div>

    </div>
  )
}