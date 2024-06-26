import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";

function ChatPage() {
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

    /**
     * Runs when this component is finished rendering
     */
    useEffect( () => {
        invoke("p2p_start")
    }, []);

    return (
        <div>
            <form
                className="row"
                onSubmit={(e) => {
                    e.preventDefault();
                    greet()
                    request_ip();
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
            {
                ip !== "" && <p>*Note your ip has been tracked: {ip}*</p>
            }
        </div>
    )
}

export default ChatPage;