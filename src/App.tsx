import {useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
    const [greetMsg, setGreetMsg] = useState("");
    const [ip, setIp] = useState("");
    const [message, setMessage] = useState("");

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        setGreetMsg(await invoke("message", {name: message}));
    }

    async function request_ip() {
        setIp(await invoke("request_ip"))
    }

    return (
        <div className="container">
            <h1>Welcome to P2P Messaging!</h1>
            <p>Send a message to start.</p>

            <form
                className="row"
                onSubmit={(e) => {
                    e.preventDefault();
                    greet()
                    request_ip();
                    invoke("register", {name: message})
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
            {ip !== "" && <p>*Note your ip has been tracked: {ip}</p>}
        </div>
    );
}

export default App;
