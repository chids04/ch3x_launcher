import PresetsView from "./views/PresetsView";
import GamePathsView from "./views/GamePathsView";
import Wallets from "./views/Wallets"
import Layout from "./views/Layout";
import { useState, useEffect } from "react";
import { invoke } from '@tauri-apps/api/core';

interface Wallet {
  id: number
  publicKey: string
}

function App() {

  //this is my first react project so i am not too sure on best practices
  //will come clean this up

  
  useEffect(() => {
    document.documentElement.classList.add("dark");
  }, [])

  return (
    <Layout>
      {(section) => {
        if (section === "presets") return <PresetsView />;
        if (section === "paths") return <GamePathsView />;
        // if (section === "mods") return <Wallets wallet={wallet} genWallet={genWallet} />;
      }}
    </Layout>
  );
}

export default App;