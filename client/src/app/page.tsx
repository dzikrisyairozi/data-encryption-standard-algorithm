"use client";
import { useState } from "react";

export default function Encrypt() {
  const [formData, setFormData] = useState({
    plaintext: "",
    encrypted_text: "",
    key: "",
  });

  const [encryptedResult, setEncryptedResult] = useState("");
  const [decryptedResult, setDecryptedResult] = useState("");

  const handleChange = (e: any) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  const handleEncrypt = async () => {
    const response = await fetch("http://127.0.0.1:8000/encrypt", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        plaintext: formData.plaintext,
        key: formData.key,
      }),
    });

    const data = await response.json();
    setEncryptedResult(data.encrypted_text || "Error");
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
    setDecryptedResult(data.plaintext || 'Error');
  };


  return (
    <main className="flex min-h-screen flex-col items-center p-12 sm:p-24">
      <h1 className="text-2xl ">Data Encryption Standard</h1>
      <div className="flex flex-col sm:flex-row gap-8">
        <div className="mt-4 p-4 border-2 border-red-600 rounded flex flex-col items-center">
          <h1 className="text-lg">Encryption</h1>
          <div className="form mt-2">
            <div className="flex flex-col">
              <label>Plaintext</label>
              <input
                type="text"
                name="plaintext"
                value={formData.plaintext}
                onChange={handleChange}
              />
            </div>
            <div className="flex flex-col">
              <label>Key</label>
              <input
                type="text"
                name="key"
                value={formData.key}
                onChange={handleChange}
              />
            </div>
              <button className='bg-red-700 text-white p-2 rounded mt-2' onClick={handleEncrypt}>Encrypt</button>
            <div className="mt-2">
              <label>Result:</label>
              <p>{encryptedResult}</p>
            </div>
          </div>
        </div>
        <div className="mt-4 p-4 border-2 border-green-600 rounded flex flex-col items-center">
        <h1 className="text-lg">Decryption</h1>
          <div className="form mt-2">
          <div className="flex flex-col">
            <label>Encrypted Text</label>
            <input
              type="text"
              name="encrypted_text"
              value={formData.encrypted_text}
              onChange={handleChange}
            />
          </div>
            <div className="flex flex-col">
              <label>Key</label>
              <input
                type="text"
                name="key"
                value={formData.key}
                onChange={handleChange}
              />
            </div>
              <button className='bg-green-700 text-white p-2 rounded mt-2' onClick={handleDecrypt}>Decrypt</button>
            <div className="mt-2">
              <label>Result:</label>
              <p>{decryptedResult}</p>
            </div>
          </div>
        </div>
      </div>
     
    </main>
  );
}
