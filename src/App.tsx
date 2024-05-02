import "./App.css";
import ChatPage from "./messaging/ChatPage.tsx";

function App() {

    return (
        <div className="container">
            <h1>Welcome to P2P Messaging!</h1>
            <p>Send a message to start.</p>

            <ChatPage/>

        </div>
    );
}

export default App;
