'use client';

import './globals.css';
import ApiTest from './components/ApiTest';

export default function Home() {
  return (
    <main className="min-h-screen w-full bg-background">
      <div className="container">
        <header className="mb-8 text-center pt-8">
          <h1 className="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl mb-3">Conduit API Test</h1>
          <p className="text-xl text-muted-foreground">Test the OpenAI-compatible API endpoints and memory management</p>
        </header>
        <ApiTest />
      </div>
    </main>
  );
}
