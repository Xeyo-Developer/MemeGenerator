import React, { useState, useEffect } from 'react';
import { RefreshCw, Zap, Download, Heart, Github } from 'lucide-react';
import { SiRust, SiReact, SiTailwindcss } from 'react-icons/si';
import icon from './icon.ico';

const API_BASE = 'http://localhost:8080';

const MemeGenerator = () => {
  const [isServerOnline, setIsServerOnline] = useState(false);
  const [isCheckingServer, setIsCheckingServer] = useState(true);
  const [currentMeme, setCurrentMeme] = useState(null);
  const [isGenerating, setIsGenerating] = useState(false);
  const [error, setError] = useState('');
  const [isZoomed, setIsZoomed] = useState(false);

  const checkServerHealth = async () => {
    try {
      const response = await fetch(`${API_BASE}/health`);
      const data = await response.json();

      if (response.ok && data.status === 'OK') {
        setIsServerOnline(true);
        setError('');
      } else {
        throw new Error('Server returned non-OK status');
      }
    } catch (error) {
      setIsServerOnline(false);
      setError('Cannot connect to server. Make sure the backend is running on localhost:8080');
    } finally {
      setIsCheckingServer(false);
    }
  };

  const generateRandomMeme = async () => {
    if (!isServerOnline) {
      setError('Server is offline');
      return;
    }

    setIsGenerating(true);
    setError('');

    try {
      const response = await fetch(`${API_BASE}/generate`);
      const data = await response.json();

      if (response.ok) {
        setCurrentMeme(data);
        setIsZoomed(false);
      } else {
        throw new Error(data.error || 'Error generating meme');
      }
    } catch (error) {
      setError(`Error: ${error.message}`);
    } finally {
      setIsGenerating(false);
    }
  };

  const downloadMeme = () => {
    if (!currentMeme) return;

    const link = document.createElement('a');
    link.href = currentMeme.image_url;
    link.download = currentMeme.template_name;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  const toggleZoom = () => {
    setIsZoomed(!isZoomed);
  };

  useEffect(() => {
    checkServerHealth();
    const interval = setInterval(checkServerHealth, 30000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-600 via-blue-600 to-indigo-800 flex items-center justify-center p-4">
      <div className="bg-white/95 backdrop-blur-xl rounded-3xl shadow-2xl p-8 max-w-4xl w-full">
        <div className="text-center mb-8">
          <div className="flex items-center justify-center space-x-2">
            <img
              src={icon}
              className="w-20 h-20"
              alt="Meme Generator Logo"
              onError={(e) => {
                e.target.style.display = 'none';
              }}
            />
            <h1 className="text-5xl font-bold bg-gradient-to-r from-purple-600 to-blue-600 bg-clip-text text-transparent">
              MemeGenerator
            </h1>
          </div>
          <p className="text-gray-600 text-lg mt-4">Generate random memes from template collection</p>
        </div>

        <div className="flex justify-center mb-8">
          {isCheckingServer ? (
            <div className="flex items-center gap-2 px-4 py-2 bg-yellow-100 text-yellow-800 rounded-full border border-yellow-300">
              <RefreshCw className="w-4 h-4 animate-spin" />
              Checking connection...
            </div>
          ) : isServerOnline ? (
            <div className="flex items-center gap-2 px-4 py-2 bg-green-100 text-green-800 rounded-full border border-green-300">
              <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
              Server online
            </div>
          ) : (
            <div className="flex items-center gap-2 px-4 py-2 bg-red-100 text-red-800 rounded-full border border-red-300">
              <div className="w-2 h-2 bg-red-500 rounded-full"></div>
              Server offline
            </div>
          )}
        </div>

        <div className="flex flex-wrap gap-4 justify-center mb-8">
          <button
            onClick={generateRandomMeme}
            disabled={!isServerOnline || isGenerating}
            className="flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-purple-600 to-blue-600 text-white rounded-xl font-semibold hover:from-purple-700 hover:to-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transform hover:scale-105 transition-all duration-200 shadow-lg"
          >
            {isGenerating ? (
              <RefreshCw className="w-5 h-5 animate-spin" />
            ) : (
              <Zap className="w-5 h-5" />
            )}
            {isGenerating ? 'Generating...' : 'Generate random meme'}
          </button>

          {currentMeme && (
            <button
              onClick={downloadMeme}
              className="flex items-center gap-2 px-6 py-3 bg-green-500 text-white rounded-xl font-semibold hover:bg-green-600 transform hover:scale-105 transition-all duration-200 shadow-lg"
            >
              <Download className="w-5 h-5" />
              Download
            </button>
          )}
        </div>

        {error && (
          <div className="mb-6 p-4 bg-red-100 border border-red-300 text-red-700 rounded-xl">
            {error}
          </div>
        )}

        <div className={`rounded-2xl p-8 mb-6 min-h-[400px] flex items-center justify-center transition-all duration-300 ${
          currentMeme 
            ? 'bg-white border-2 border-purple-200 shadow-inner' 
            : 'bg-gray-50 border-2 border-dashed border-gray-300'
        }`}>
          {currentMeme ? (
            <div className="text-center">
              <div
                className={`${isZoomed ? 'fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/90 cursor-zoom-out' : 'relative cursor-zoom-in'}`}
                onClick={toggleZoom}
              >
                <img
                  src={currentMeme.image_url}
                  alt={currentMeme.template_name}
                  className={`rounded-xl shadow-lg transition-transform duration-300 ${
                    isZoomed ? 'max-h-screen max-w-screen object-contain' : 'max-w-full max-h-96'
                  }`}
                />
              </div>
              {!isZoomed && (
                <div className="mt-4 p-4 bg-blue-50 rounded-xl border border-blue-200">
                  <h3 className="text-xl font-bold text-blue-800 mb-2">
                    🎭 {currentMeme.template_name}
                  </h3>
                  <p className="text-blue-600">
                    <strong>Type:</strong> {currentMeme.content_type}
                  </p>
                  <p className="text-gray-500 text-sm mt-2">Click on the meme to zoom</p>
                </div>
              )}
            </div>
          ) : (
            <div className="text-center text-gray-500">
              <div className="text-6xl mb-4 opacity-30">🖼️</div>
              <div className="text-xl">Click "Generate random meme" to start!</div>
            </div>
          )}
        </div>

        <div className="text-center mt-8 pt-6 border-t border-gray-200">
          <div className="flex flex-col sm:flex-row justify-center items-center gap-4 mb-4">
            <a
              href="https://github.com/Xeyo-Developer/MemeGenerator"
              className="flex items-center gap-2 text-purple-600 hover:text-purple-700 transition-colors"
              target="_blank"
              rel="noopener noreferrer"
            >
              <Github className="w-5 h-5" />
              GitHub Repository
            </a>
            <span className="hidden sm:block">•</span>
            <p className="flex items-center gap-1 text-gray-600">
              Made with <Heart className="w-4 h-4 text-red-500" /> by Xeyo
            </p>
          </div>
          <div className="flex flex-wrap justify-center items-center gap-2 text-sm text-gray-500">
            <span>Built with:</span>
            <span className="flex items-center gap-1 px-1 py-1 bg-gray-100 rounded-full">
              <SiRust className="text-orange-600" /> Rust
            </span>
            <span className="flex items-center gap-1 px-1 py-1 bg-gray-100 rounded-full">
              <SiReact className="text-blue-500" /> React
            </span>
            <span className="flex items-center gap-1 px-1 py-1 bg-gray-100 rounded-full">
              <SiTailwindcss className="text-cyan-400" /> Tailwind CSS
            </span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default MemeGenerator;