import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [message, setMessage] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("message", { name: message }));
  }

  return (
    <div className="container">
      <h1>Welcome to P2P Messaging!</h1>

      <p>Send a message to start.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setMessage(e.currentTarget.value)}
          placeholder="Enter a Message..."
        />
        <button type="submit">Send</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
