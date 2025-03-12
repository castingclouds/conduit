import { useState } from 'react';

interface Model {
  id: string;
  object: string;
  created: number;
  owned_by: string;
}

interface ModelList {
  object: string;
  data: Model[];
}

interface Memory {
  id: string;
  title: string;
  content: string;
  tags: string[];
  created_at: string;
  updated_at: string;
}

const ApiTest = () => {
  const [models, setModels] = useState<Model[]>([]);
  const [memories, setMemories] = useState<Memory[]>([]);
  const [chatResponse, setChatResponse] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [testMemory, setTestMemory] = useState<Memory | null>(null);

  // Test the /v1/models endpoint
  const testModelsEndpoint = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch('http://localhost:3000/v1/models');
      const data: ModelList = await response.json();
      setModels(data.data);
      console.log('Models:', data);
    } catch (err) {
      console.error('Error testing models endpoint:', err);
      setError(`Error testing models endpoint: ${err instanceof Error ? err.message : String(err)}`);
    } finally {
      setLoading(false);
    }
  };

  // Test the /v1/chat/completions endpoint
  const testChatEndpoint = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch('http://localhost:3000/v1/chat/completions', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          model: 'gpt-3.5-turbo',
          messages: [
            { role: 'system', content: 'You are a helpful assistant.' },
            { role: 'user', content: 'Hello, how are you?' }
          ],
          temperature: 0.7,
          max_tokens: 100
        }),
      });
      const data = await response.json();
      setChatResponse(data.choices?.[0]?.message?.content || JSON.stringify(data, null, 2));
      console.log('Chat response:', data);
    } catch (err) {
      console.error('Error testing chat endpoint:', err);
      setError(`Error testing chat endpoint: ${err instanceof Error ? err.message : String(err)}`);
    } finally {
      setLoading(false);
    }
  };

  // Test the /v1/memories endpoint (create, list, get, delete)
  const testMemoriesEndpoint = async () => {
    setLoading(true);
    setError(null);
    try {
      // Create a memory
      const createResponse = await fetch('http://localhost:3000/v1/memories', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          title: 'Test Memory',
          content: 'This is a test memory created by the API test component.',
          tags: ['test', 'api']
        }),
      });
      const memory = await createResponse.json();
      setTestMemory(memory);
      console.log('Created memory:', memory);

      // List memories
      const listResponse = await fetch('http://localhost:3000/v1/memories');
      const memories = await listResponse.json();
      setMemories(memories);
      console.log('All memories:', memories);

      // Get specific memory
      if (memory && memory.id) {
        const getResponse = await fetch(`http://localhost:3000/v1/memories/${memory.id}`);
        const retrievedMemory = await getResponse.json();
        console.log('Retrieved memory:', retrievedMemory);
      }
    } catch (err) {
      console.error('Error testing memories endpoint:', err);
      setError(`Error testing memories endpoint: ${err instanceof Error ? err.message : String(err)}`);
    } finally {
      setLoading(false);
    }
  };

  // Delete the test memory
  const deleteTestMemory = async () => {
    if (!testMemory) return;
    
    setLoading(true);
    setError(null);
    try {
      const deleteResponse = await fetch(`http://localhost:3000/v1/memories/${testMemory.id}`, {
        method: 'DELETE',
      });
      
      // Check if the delete was successful (204 No Content)
      if (deleteResponse.status === 204) {
        console.log('Memory deleted successfully');
        setTestMemory(null);
        
        // Refresh the memories list
        const listResponse = await fetch('http://localhost:3000/v1/memories');
        const memories = await listResponse.json();
        setMemories(memories);
      } else {
        // Handle error response
        const errorText = await deleteResponse.text();
        console.error('Error deleting memory:', errorText);
        setError(`Error deleting memory: ${errorText}`);
      }
    } catch (err) {
      console.error('Error deleting memory:', err);
      setError(`Error deleting memory: ${err instanceof Error ? err.message : String(err)}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="api-test-container" style={{ padding: '20px', maxWidth: '800px', margin: '0 auto' }}>
      <h2>OpenAI-Compatible API Test</h2>
      
      {error && (
        <div style={{ padding: '10px', backgroundColor: '#ffebee', color: '#c62828', borderRadius: '4px', marginBottom: '20px' }}>
          <strong>Error:</strong> {error}
        </div>
      )}
      
      <div style={{ marginBottom: '30px' }}>
        <h3>Models Endpoint Test</h3>
        <button 
          onClick={testModelsEndpoint} 
          disabled={loading}
          style={{ padding: '8px 16px', marginRight: '10px' }}
        >
          Test /v1/models
        </button>
        
        {models.length > 0 && (
          <div style={{ marginTop: '10px' }}>
            <h4>Available Models:</h4>
            <ul>
              {models.map(model => (
                <li key={model.id}>
                  <strong>{model.id}</strong> - Owned by: {model.owned_by}
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>
      
      <div style={{ marginBottom: '30px' }}>
        <h3>Chat Completions Endpoint Test</h3>
        <button 
          onClick={testChatEndpoint} 
          disabled={loading}
          style={{ padding: '8px 16px', marginRight: '10px' }}
        >
          Test /v1/chat/completions
        </button>
        
        {chatResponse && (
          <div style={{ marginTop: '10px' }}>
            <h4>Chat Response:</h4>
            <div style={{ padding: '10px', backgroundColor: '#f5f5f5', borderRadius: '4px' }}>
              <pre style={{ whiteSpace: 'pre-wrap', margin: 0 }}>{chatResponse}</pre>
            </div>
          </div>
        )}
      </div>
      
      <div style={{ marginBottom: '30px' }}>
        <h3>Memories Endpoint Test</h3>
        <button 
          onClick={testMemoriesEndpoint} 
          disabled={loading}
          style={{ padding: '8px 16px', marginRight: '10px' }}
        >
          Test /v1/memories
        </button>
        
        {testMemory && (
          <button 
            onClick={deleteTestMemory} 
            disabled={loading}
            style={{ padding: '8px 16px', backgroundColor: '#ffcdd2' }}
          >
            Delete Test Memory
          </button>
        )}
        
        {memories.length > 0 && (
          <div style={{ marginTop: '10px' }}>
            <h4>Memories:</h4>
            <ul>
              {memories.map(memory => (
                <li key={memory.id}>
                  <strong>{memory.title}</strong> - Tags: {memory.tags.join(', ')}
                  <p>{memory.content.substring(0, 100)}...</p>
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>
      
      {loading && (
        <div style={{ textAlign: 'center', padding: '20px' }}>
          Loading...
        </div>
      )}
    </div>
  );
};

export default ApiTest;
