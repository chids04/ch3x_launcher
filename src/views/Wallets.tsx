import { useState } from "react";
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"

export interface Wallet {
  publicKey: string
  id: number
}

interface WalletsProps {
  wallet: Wallet | undefined;
  genWallet: () => Promise<Wallet>;
}

export default function Wallets({ wallet, genWallet }: WalletsProps) {
  const [error, setError] = useState("");

  const handleGenWallet = async () => {
    try {
      setError(""); // Clear any previous errors
      await genWallet();
    } catch (error) {
      setError((error as Error).message);
    }
  };

  return (
    <div className="w-full">
      <div className="flex flex-col items-center justify-center gap-5">
        <Input 
          placeholder="Wallet ID" 
          value={wallet?.id?.toString() ?? ""} 
          readOnly 
          className=""
        />

        <div className="flex flex-row gap-4">
          <Input 
            placeholder="Public Key" 
            value={wallet?.publicKey ?? ""} 
            readOnly 
            className="flex-1"
          />

          <Button
            onClick={() => {
              if (wallet?.publicKey) {
                navigator.clipboard.writeText(wallet.publicKey);
              }
            }}
          >
            copy
          </Button>
        </div>
        
        <Button onClick={handleGenWallet} className="px-8 py-2">
          create Wallet
        </Button>
        {error && <div className="text-red-500 text-center mt-4">{error}</div>}
      </div>
    </div>
  )
}