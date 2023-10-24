'use client'
import { useState } from 'react';

export default function Home() {
  const [formData, setFormData] = useState({
    plaintext: '',
    encrypted_text: '',
    key: ''
  });

  const [result, setResult] = useState('');

  const handleChange = (e:any) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value
    });
  };

  const handleEncrypt = async () => {
    const response = await fetch('http://127.0.0.1:8000/encrypt', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        plaintext: formData.plaintext,
        key: formData.key
      })
    });

    const data = await response.json();
    setResult(data.encrypted_text || 'Error');
  };

  const handleDecrypt = async () => {
    const response = await fetch('http://127.0.0.1:8000/decrypt', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        encrypted_text: formData.encrypted_text,
        key: formData.key
      })
    });

    const data = await response.json();
    setResult(data.plaintext || 'Error');
  };

  return (
    <main className="flex min-h-screen flex-col items-center p-24">
      <h1>Data Encryption Standard</h1>
      <div className="form">
        <div>
          <label>Plaintext</label>
          <input
            type="text"
            name="plaintext"
            value={formData.plaintext}
            onChange={handleChange}
          />
        </div>
        <div>
          <label>Encrypted Text</label>
          <input
            type="text"
            name="encrypted_text"
            value={formData.encrypted_text}
            onChange={handleChange}
          />
        </div>
        <div>
          <label>Key</label>
          <input
            type="text"
            name="key"
            value={formData.key}
            onChange={handleChange}
          />
        </div>
        <div>
          <button onClick={handleEncrypt}>Encrypt</button>
          <button onClick={handleDecrypt}>Decrypt</button>
        </div>
        <div>
          <label>Result:</label>
          <p>{result}</p>
        </div>
      </div>
    </main>
  );
}
