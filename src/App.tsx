import "./App.css";
import ApiTest from "./ApiTest";

function App() {
  return (
    <main className="container">
      <h1>Conduit API Test</h1>
      <div style={{ marginBottom: '20px' }}>
        <p>Test the OpenAI-compatible API endpoints and memory management</p>
      </div>
      <ApiTest />
    </main>
  );
}

export default App;
