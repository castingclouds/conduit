// Simple test script for the OpenAI-compatible API
import fetch from 'node-fetch';

// Try different base URLs
const BASE_URLS = [
  'http://localhost:3000/v1',
  'http://127.0.0.1:3000/v1',
  'http://localhost:1420/v1',
  'http://127.0.0.1:1420/v1'
];

let BASE_URL = BASE_URLS[0]; // Default to the first one

async function testListModels() {
  console.log('Testing /v1/models endpoint...');
  try {
    const response = await fetch(`${BASE_URL}/models`);
    const data = await response.json();
    console.log('Models:', JSON.stringify(data, null, 2));
    return data;
  } catch (error) {
    console.error('Error testing models endpoint:', error);
    return null;
  }
}

async function testChatCompletions() {
  console.log('Testing /v1/chat/completions endpoint...');
  try {
    const response = await fetch(`${BASE_URL}/chat/completions`, {
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
    console.log('Chat Completion:', JSON.stringify(data, null, 2));
    return data;
  } catch (error) {
    console.error('Error testing chat completions endpoint:', error);
    return null;
  }
}

async function testMemoryOperations() {
  console.log('Testing memory operations...');
  
  // Create a memory
  try {
    console.log('Creating a memory...');
    const createResponse = await fetch(`${BASE_URL}/memories`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        title: 'Test Memory',
        content: 'This is a test memory created by the API test script.',
        tags: ['test', 'api']
      }),
    });
    const memory = await createResponse.json();
    console.log('Created Memory:', JSON.stringify(memory, null, 2));
    
    if (memory && memory.id) {
      // List memories
      console.log('Listing all memories...');
      const listResponse = await fetch(`${BASE_URL}/memories`);
      const memories = await listResponse.json();
      console.log('All Memories:', JSON.stringify(memories, null, 2));
      
      // Get specific memory
      console.log(`Getting memory with ID ${memory.id}...`);
      const getResponse = await fetch(`${BASE_URL}/memories/${memory.id}`);
      const retrievedMemory = await getResponse.json();
      console.log('Retrieved Memory:', JSON.stringify(retrievedMemory, null, 2));
      
      // Delete memory
      console.log(`Deleting memory with ID ${memory.id}...`);
      const deleteResponse = await fetch(`${BASE_URL}/memories/${memory.id}`, {
        method: 'DELETE',
      });
      const deleteResult = await deleteResponse.json();
      console.log('Delete Result:', JSON.stringify(deleteResult, null, 2));
      
      // Verify deletion by listing memories again
      console.log('Listing memories after deletion...');
      const finalListResponse = await fetch(`${BASE_URL}/memories`);
      const finalMemories = await finalListResponse.json();
      console.log('Final Memories List:', JSON.stringify(finalMemories, null, 2));
    }
    
    return memory;
  } catch (error) {
    console.error('Error testing memory operations:', error);
    return null;
  }
}

async function tryAllBaseUrls() {
  for (const url of BASE_URLS) {
    console.log(`\nTrying base URL: ${url}\n`);
    BASE_URL = url;
    
    try {
      // Just try the models endpoint as a test
      const response = await fetch(`${BASE_URL}/models`);
      const data = await response.json();
      console.log('Success! Found working API endpoint.');
      console.log('Models:', JSON.stringify(data, null, 2));
      return true; // Found a working URL
    } catch (error) {
      console.log(`Failed with ${url}: ${error.message || error}`);
    }
  }
  
  console.log('\nCould not connect to any of the API endpoints.\n');
  return false;
}

async function runTests() {
  const foundWorkingUrl = await tryAllBaseUrls();
  
  if (foundWorkingUrl) {
    console.log(`\nUsing base URL: ${BASE_URL}\n`);
    console.log('\n----------------------------\n');
    
    await testChatCompletions();
    console.log('\n----------------------------\n');
    
    await testMemoryOperations();
  } else {
    console.log('Cannot proceed with tests as no working API endpoint was found.');
  }
}

runTests().catch(console.error);
