'use client';

import { useState } from 'react';
import { Button } from '../../components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../../components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../../components/ui/tabs';

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
      const response = await fetch('http://localhost:3000/v1/models', {
        method: 'GET',
      });
      const data = await response.json() as ModelList;
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
      const memory = await createResponse.json() as Memory;
      setTestMemory(memory);
      console.log('Created memory:', memory);

      // List memories
      const listResponse = await fetch('http://localhost:3000/v1/memories', {
        method: 'GET',
      });
      const memories = await listResponse.json() as Memory[];
      setMemories(memories);
      console.log('All memories:', memories);

      // Get specific memory
      if (memory && memory.id) {
        const getResponse = await fetch(`http://localhost:3000/v1/memories/${memory.id}`, {
          method: 'GET',
        });
        const retrievedMemory = await getResponse.json() as Memory;
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
        const listResponse = await fetch('http://localhost:3000/v1/memories', {
          method: 'GET',
        });
        const memories = await listResponse.json() as Memory[];
        setMemories(memories);
      } else {
        // Handle error response
        console.error('Error deleting memory:', deleteResponse.status);
        setError(`Error deleting memory: ${deleteResponse.status}`);
      }
    } catch (err) {
      console.error('Error deleting memory:', err);
      setError(`Error deleting memory: ${err instanceof Error ? err.message : String(err)}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Tabs defaultValue="models" className="w-full h-[calc(100vh-12rem)] flex flex-col space-y-2">
      <TabsList className="grid w-full grid-cols-3">
        <TabsTrigger value="models">Models</TabsTrigger>
        <TabsTrigger value="chat">Chat</TabsTrigger>
        <TabsTrigger value="memories">Memories</TabsTrigger>
      </TabsList>
      <TabsContent value="models" className="flex-1">
        <Card className="h-full flex flex-col">
          <CardHeader>
            <CardTitle>Models</CardTitle>
            <CardDescription>Test the OpenAI-compatible models endpoint</CardDescription>
          </CardHeader>
          <CardContent className="flex-1 flex flex-col gap-2 pt-0">
            <Button className="w-full" onClick={testModelsEndpoint} disabled={loading}>Test Models</Button>
            {models.length > 0 && (
              <div className="rounded-lg border p-4 flex-1 overflow-y-auto">
                {models.map(model => (
                  <div key={model.id} className="py-3 text-sm">{model.id}</div>
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      </TabsContent>
      <TabsContent value="chat" className="flex-1">
        <Card className="h-full flex flex-col">
          <CardHeader>
            <CardTitle>Chat Completions</CardTitle>
            <CardDescription>Test the OpenAI-compatible chat completions endpoint</CardDescription>
          </CardHeader>
          <CardContent className="flex-1 flex flex-col gap-2 pt-0">
            <Button className="w-full" onClick={testChatEndpoint} disabled={loading}>Test Chat</Button>
            {chatResponse && (
              <div className="rounded-lg border p-4 flex-1 overflow-y-auto">
                <p className="py-3 text-sm">{chatResponse}</p>
              </div>
            )}
          </CardContent>
        </Card>
      </TabsContent>
      <TabsContent value="memories" className="flex-1">
        <Card className="h-full flex flex-col">
          <CardHeader>
            <CardTitle>Memories</CardTitle>
            <CardDescription>Test the OpenAI-compatible memories endpoint</CardDescription>
          </CardHeader>
          <CardContent className="flex-1 flex flex-col gap-2 pt-0">
            <div className="flex w-full gap-2">
              <Button className="flex-1" onClick={testMemoriesEndpoint} disabled={loading}>Test Memories</Button>
              {testMemory && (
                <Button variant="destructive" onClick={deleteTestMemory} disabled={loading}>Delete</Button>
              )}
            </div>
            {memories.length > 0 && (
              <div className="rounded-lg border p-4 flex-1 overflow-y-auto">
                {memories.map(memory => (
                  <div key={memory.id} className="py-3 text-sm">{memory.title}</div>
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      </TabsContent>
      {loading && (
        <div className="mt-2 text-center text-sm text-muted-foreground">
          Loading...
        </div>
      )}
    </Tabs>
  );
};

export default ApiTest;
