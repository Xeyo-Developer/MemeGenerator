import React, { useState, useEffect } from 'react';
import { RefreshCw, Zap, Download, Heart, Github, Star, List, BarChart2, Search, X, Image as ImageIcon, Home, ChevronLeft } from 'lucide-react';
import { SiRust, SiReact, SiTailwindcss } from 'react-icons/si';

const API_BASE = 'http://localhost:8080';

const MemeGenerator = () => {
  const [isServerOnline, setIsServerOnline] = useState(false);
  const [isCheckingServer, setIsCheckingServer] = useState(true);
  const [currentMeme, setCurrentMeme] = useState(null);
  const [isGenerating, setIsGenerating] = useState(false);
  const [error, setError] = useState('');
  const [isZoomed, setIsZoomed] = useState(false);
  const [view, setView] = useState('home');
  const [memeList, setMemeList] = useState([]);
  const [stats, setStats] = useState(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [searchResults, setSearchResults] = useState([]);
  const [favorites, setFavorites] = useState([]);
  const [isFavorite, setIsFavorite] = useState(false);
  const [loading, setLoading] = useState(false);

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
        checkIfFavorite(data.template_name);
      } else {
        throw new Error(data.error || 'Error generating meme');
      }
    } catch (error) {
      setError(`Error: ${error.message}`);
    } finally {
      setIsGenerating(false);
    }
  };

  const fetchMemeList = async () => {
    setLoading(true);
    try {
      const response = await fetch(`${API_BASE}/list`);
      const data = await response.json();
      if (response.ok) {
        setMemeList(data.templates);
      } else {
        throw new Error(data.error || 'Error fetching meme list');
      }
    } catch (error) {
      setError(`Error: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  const fetchStats = async () => {
    setLoading(true);
    try {
      const response = await fetch(`${API_BASE}/stats`);
      const data = await response.json();
      if (response.ok) {
        setStats(data);
      } else {
        throw new Error(data.error || 'Error fetching stats');
      }
    } catch (error) {
      setError(`Error: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  const searchMemes = async () => {
    if (!searchTerm.trim()) return;

    setLoading(true);
    try {
      const response = await fetch(`${API_BASE}/search?q=${encodeURIComponent(searchTerm)}`);
      const data = await response.json();
      if (response.ok) {
        setSearchResults(data.results);
      } else {
        throw new Error(data.error || 'Error searching memes');
      }
    } catch (error) {
      setError(`Error: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  const fetchFavorites = async () => {
    setLoading(true);
    try {
      const response = await fetch(`${API_BASE}/favorites`);
      const data = await response.json();
      if (response.ok) {
        setFavorites(data.favorites);
      } else {
        throw new Error(data.error || 'Error fetching favorites');
      }
    } catch (error) {
      setError(`Error: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  const toggleFavorite = async () => {
    if (!currentMeme) return;

    try {
      const response = await fetch(`${API_BASE}/favorite`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ meme_name: currentMeme.template_name }),
      });
      const data = await response.json();
      if (response.ok) {
        setIsFavorite(data.is_favorite);
        fetchFavorites();
      } else {
        throw new Error(data.error || 'Error toggling favorite');
      }
    } catch (error) {
      setError(`Error: ${error.message}`);
    }
  };

  const checkIfFavorite = (memeName) => {
    setIsFavorite(favorites.includes(memeName));
  };

  const getSpecificMeme = async (memeName) => {
    setLoading(true);
    try {
      const response = await fetch(`${API_BASE}/meme/${memeName}`);
      const data = await response.json();
      if (response.ok) {
        setCurrentMeme(data);
        setIsZoomed(false);
        checkIfFavorite(data.template_name);
        setView('meme');
      } else {
        throw new Error(data.error || 'Error fetching meme');
      }
    } catch (error) {
      setError(`Error: ${error.message}`);
    } finally {
      setLoading(false);
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

  useEffect(() => {
    checkServerHealth();
    fetchFavorites();
    const interval = setInterval(checkServerHealth, 30000);
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    if (view === 'gallery' && memeList.length === 0) {
      fetchMemeList();
    } else if (view === 'stats' && !stats) {
      fetchStats();
    }
  }, [view, memeList.length, stats]);

  return (
    <div className="min-h-screen bg-gradient-to-br from-indigo-900 via-purple-900 to-pink-800 p-4">
      {isZoomed && (
        <div className="fixed inset-0 z-50 bg-black/90 flex items-center justify-center p-4">
          <button
            onClick={() => setIsZoomed(false)}
            className="absolute top-4 right-4 p-2 bg-white/10 hover:bg-white/20 rounded-full text-white transition-colors"
          >
            <X className="w-6 h-6" />
          </button>
          <img
            src={currentMeme?.image_url}
            alt={currentMeme?.template_name}
            className="max-h-screen max-w-screen object-contain"
          />
        </div>
      )}

      <div className="max-w-6xl mx-auto">
        <header className="bg-white/10 backdrop-blur-lg rounded-2xl p-6 mb-6 shadow-lg border border-white/20">
          <div className="flex flex-col md:flex-row justify-between items-center">
            <div className="flex items-center mb-4 md:mb-0">
              <ImageIcon className="w-8 h-8 text-white mr-3" />
              <h1 className="text-3xl font-bold text-white">MemeGenerator</h1>
            </div>

            <div className="flex flex-wrap gap-2">
              <button
                onClick={() => setView('home')}
                className={`flex items-center gap-2 px-4 py-2 rounded-xl ${view === 'home' ? 'bg-white text-purple-800' : 'bg-white/10 text-white hover:bg-white/20'}`}
              >
                <Home className="w-5 h-5" />
                Home
              </button>
              <button
                onClick={() => setView('gallery')}
                className={`flex items-center gap-2 px-4 py-2 rounded-xl ${view === 'gallery' ? 'bg-white text-purple-800' : 'bg-white/10 text-white hover:bg-white/20'}`}
              >
                <List className="w-5 h-5" />
                Gallery
              </button>
              <button
                onClick={() => setView('stats')}
                className={`flex items-center gap-2 px-4 py-2 rounded-xl ${view === 'stats' ? 'bg-white text-purple-800' : 'bg-white/10 text-white hover:bg-white/20'}`}
              >
                <BarChart2 className="w-5 h-5" />
                Stats
              </button>
            </div>
          </div>

          <div className="flex justify-center mt-4">
            {isCheckingServer ? (
              <div className="flex items-center gap-2 px-4 py-2 bg-yellow-500/20 text-yellow-100 rounded-full border border-yellow-300/50">
                <RefreshCw className="w-4 h-4 animate-spin" />
                Checking connection...
              </div>
            ) : isServerOnline ? (
              <div className="flex items-center gap-2 px-4 py-2 bg-green-500/20 text-green-100 rounded-full border border-green-300/50">
                <div className="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
                Server online
              </div>
            ) : (
              <div className="flex items-center gap-2 px-4 py-2 bg-red-500/20 text-red-100 rounded-full border border-red-300/50">
                <div className="w-2 h-2 bg-red-400 rounded-full"></div>
                Server offline
              </div>
            )}
          </div>
        </header>

        {error && (
          <div className="bg-red-500/20 text-red-100 p-4 rounded-xl mb-6 border border-red-300/50">
            {error}
          </div>
        )}

        <main className="bg-white/10 backdrop-blur-lg rounded-2xl p-6 shadow-lg border border-white/20">
          {view === 'home' && (
            <div className="space-y-8">
              <div className="text-center">
                <h2 className="text-2xl font-bold text-white mb-2">Meme Generator</h2>
                <p className="text-white/80">Create random memes from our collection</p>
              </div>

              <div className="flex justify-center">
                <button
                  onClick={generateRandomMeme}
                  disabled={!isServerOnline || isGenerating}
                  className="flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-pink-500 to-purple-600 text-white rounded-xl font-semibold hover:from-pink-600 hover:to-purple-700 disabled:opacity-50 disabled:cursor-not-allowed transform hover:scale-105 transition-all duration-200 shadow-lg"
                >
                  {isGenerating ? (
                    <RefreshCw className="w-5 h-5 animate-spin" />
                  ) : (
                    <Zap className="w-5 h-5" />
                  )}
                  {isGenerating ? 'Generating...' : 'Generate Meme'}
                </button>
              </div>

              {currentMeme && view === 'home' && (
                <div className="bg-white/20 rounded-xl p-6 border border-white/30">
                  <div className="flex justify-between items-start mb-4">
                    <h3 className="text-xl font-bold text-white">{currentMeme.template_name}</h3>
                    <div className="flex gap-2">
                      <button
                        onClick={toggleFavorite}
                        className={`p-2 rounded-lg ${isFavorite ? 'bg-yellow-500/20 text-yellow-300' : 'bg-white/10 text-white hover:bg-white/20'}`}
                        title={isFavorite ? 'Remove from favorites' : 'Add to favorites'}
                      >
                        <Star className="w-5 h-5" fill={isFavorite ? 'currentColor' : 'none'} />
                      </button>
                      <button
                        onClick={downloadMeme}
                        className="p-2 bg-white/10 text-white rounded-lg hover:bg-white/20"
                        title="Download"
                      >
                        <Download className="w-5 h-5" />
                      </button>
                      <button
                        onClick={() => setIsZoomed(true)}
                        className="p-2 bg-white/10 text-white rounded-lg hover:bg-white/20"
                        title="Zoom"
                      >
                        <Search className="w-5 h-5" />
                      </button>
                    </div>
                  </div>

                  <div className="flex justify-center mb-4">
                    <img
                      src={currentMeme.image_url}
                      alt={currentMeme.template_name}
                      className="max-h-96 rounded-lg cursor-zoom-in shadow-lg"
                      onClick={() => setIsZoomed(true)}
                    />
                  </div>

                  <div className="grid grid-cols-2 md:grid-cols-3 gap-4 text-sm text-white/80">
                    <div className="bg-white/10 p-3 rounded-lg">
                      <p className="font-medium">Type</p>
                      <p>{currentMeme.content_type}</p>
                    </div>
                    <div className="bg-white/10 p-3 rounded-lg">
                      <p className="font-medium">Size</p>
                      <p>{(currentMeme.size_bytes / 1024).toFixed(2)} KB</p>
                    </div>
                    <div className="bg-white/10 p-3 rounded-lg">
                      <p className="font-medium">Generated</p>
                      <p>{new Date(currentMeme.generated_at).toLocaleTimeString()}</p>
                    </div>
                  </div>
                </div>
              )}

              {!currentMeme && (
                <div className="text-center py-12">
                  <div className="text-6xl mb-4 opacity-20 text-white">🖼️</div>
                  <p className="text-white/70">No meme generated yet</p>
                </div>
              )}
            </div>
          )}

          {view === 'gallery' && (
            <div className="space-y-6">
              <div className="relative">
                <input
                  type="text"
                  placeholder="Search memes..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  onKeyPress={(e) => e.key === 'Enter' && searchMemes()}
                  className="w-full p-4 pl-12 rounded-xl bg-white/10 border border-white/20 text-white placeholder-white/50 focus:outline-none focus:ring-2 focus:ring-purple-500"
                />
                <Search className="absolute left-4 top-1/2 transform -translate-y-1/2 text-white/50" />
                <button
                  onClick={searchMemes}
                  disabled={!searchTerm.trim()}
                  className="absolute right-4 top-1/2 transform -translate-y-1/2 bg-purple-600 text-white px-4 py-2 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-purple-700"
                >
                  Search
                </button>
              </div>

              {loading ? (
                <div className="flex justify-center items-center min-h-[300px]">
                  <RefreshCw className="w-8 h-8 animate-spin text-purple-400" />
                </div>
              ) : searchResults.length > 0 ? (
                <div>
                  <h2 className="text-xl font-bold mb-4 text-white">Search Results ({searchResults.length})</h2>
                  <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
                    {searchResults.map((meme) => (
                      <div
                        key={meme.name}
                        className="bg-white/10 rounded-xl overflow-hidden border border-white/20 hover:border-purple-400 transition-colors cursor-pointer group"
                        onClick={() => getSpecificMeme(meme.name)}
                      >
                        <div className="p-4">
                          <div className="flex justify-between items-start">
                            <h3 className="font-medium text-white truncate">{meme.name}</h3>
                            {favorites.includes(meme.name) && (
                              <Star className="w-4 h-4 text-yellow-400 flex-shrink-0" fill="currentColor" />
                            )}
                          </div>
                          <p className="text-xs text-white/60 mt-1">
                            {meme.file_type.toUpperCase()} • {(meme.size_bytes / 1024).toFixed(2)} KB
                          </p>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              ) : (
                <div>
                  <h2 className="text-xl font-bold mb-4 text-white">Meme Gallery ({memeList.length})</h2>
                  <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
                    {memeList.map((meme) => (
                      <div
                        key={meme.name}
                        className="bg-white/10 rounded-xl overflow-hidden border border-white/20 hover:border-purple-400 transition-colors cursor-pointer group"
                        onClick={() => getSpecificMeme(meme.name)}
                      >
                        <div className="p-4">
                          <div className="flex justify-between items-start">
                            <h3 className="font-medium text-white truncate">{meme.name}</h3>
                            {favorites.includes(meme.name) && (
                              <Star className="w-4 h-4 text-yellow-400 flex-shrink-0" fill="currentColor" />
                            )}
                          </div>
                          <p className="text-xs text-white/60 mt-1">
                            {meme.file_type.toUpperCase()} • {(meme.size_bytes / 1024).toFixed(2)} KB
                          </p>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          )}

          {view === 'meme' && currentMeme && (
            <div className="space-y-6">
              <button
                onClick={() => setView('gallery')}
                className="flex items-center gap-2 text-white hover:text-purple-300"
              >
                <ChevronLeft className="w-5 h-5" />
                Back to gallery
              </button>

              <div className="bg-white/20 rounded-xl p-6 border border-white/30">
                <div className="flex justify-between items-start mb-4">
                  <h3 className="text-xl font-bold text-white">{currentMeme.template_name}</h3>
                  <div className="flex gap-2">
                    <button
                      onClick={toggleFavorite}
                      className={`p-2 rounded-lg ${isFavorite ? 'bg-yellow-500/20 text-yellow-300' : 'bg-white/10 text-white hover:bg-white/20'}`}
                      title={isFavorite ? 'Remove from favorites' : 'Add to favorites'}
                    >
                      <Star className="w-5 h-5" fill={isFavorite ? 'currentColor' : 'none'} />
                    </button>
                    <button
                      onClick={downloadMeme}
                      className="p-2 bg-white/10 text-white rounded-lg hover:bg-white/20"
                      title="Download"
                    >
                      <Download className="w-5 h-5" />
                    </button>
                    <button
                      onClick={() => setIsZoomed(true)}
                      className="p-2 bg-white/10 text-white rounded-lg hover:bg-white/20"
                      title="Zoom"
                    >
                      <Search className="w-5 h-5" />
                    </button>
                  </div>
                </div>

                <div className="flex justify-center mb-4">
                  <img
                    src={currentMeme.image_url}
                    alt={currentMeme.template_name}
                    className="max-h-96 rounded-lg cursor-zoom-in shadow-lg"
                    onClick={() => setIsZoomed(true)}
                  />
                </div>

                <div className="grid grid-cols-2 md:grid-cols-2 gap-4 text-sm text-white/80">
                  <div className="bg-white/10 p-3 rounded-lg">
                    <p className="font-medium">Type</p>
                    <p>{currentMeme.content_type}</p>
                  </div>
                  <div className="bg-white/10 p-3 rounded-lg">
                    <p className="font-medium">Size</p>
                    <p>{(currentMeme.size_bytes / 1024).toFixed(2)} KB</p>
                  </div>
                </div>
              </div>
            </div>
          )}

          {view === 'stats' && (
            <div className="space-y-6">
              {loading ? (
                <div className="flex justify-center items-center min-h-[300px]">
                  <RefreshCw className="w-8 h-8 animate-spin text-purple-400" />
                </div>
              ) : stats ? (
                <div className="text-white">
                  <h2 className="text-2xl font-bold mb-6 text-center">Collection Statistics</h2>

                  <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                    <div className="bg-white/10 p-6 rounded-xl border border-white/20">
                      <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
                        <BarChart2 className="w-5 h-5" />
                        General Stats
                      </h3>
                      <div className="space-y-3">
                        <div>
                          <p className="text-sm text-white/70">Total Memes</p>
                          <p className="text-2xl font-bold">{stats.total_memes}</p>
                        </div>
                        <div>
                          <p className="text-sm text-white/70">Total Size</p>
                          <p className="text-2xl font-bold">{(stats.total_size_bytes / (1024 * 1024)).toFixed(2)} MB</p>
                        </div>
                        <div>
                          <p className="text-sm text-white/70">Average Size</p>
                          <p className="text-2xl font-bold">{(stats.average_file_size / 1024).toFixed(2)} KB</p>
                        </div>
                      </div>
                    </div>

                    <div className="bg-white/10 p-6 rounded-xl border border-white/20">
                      <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
                        <Star className="w-5 h-5" />
                        Size Extremes
                      </h3>
                      <div className="space-y-4">
                        <div>
                          <p className="text-sm text-white/70">Largest File</p>
                          <p className="font-medium">{stats.largest_file_name}</p>
                          <p className="text-lg">{(stats.largest_file_size / (1024 * 1024)).toFixed(2)} MB</p>
                        </div>
                        <div>
                          <p className="text-sm text-white/70">Smallest File</p>
                          <p className="font-medium">{stats.smallest_file_name}</p>
                          <p className="text-lg">{(stats.smallest_file_size / 1024).toFixed(2)} KB</p>
                        </div>
                      </div>
                    </div>
                  </div>

                  <div className="bg-white/10 p-6 rounded-xl border border-white/20">
                    <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
                      <List className="w-5 h-5" />
                      File Types
                    </h3>
                    <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
                      {Object.entries(stats.file_types).map(([type, count]) => (
                        <div key={type} className="bg-white/20 p-4 rounded-lg">
                          <p className="text-sm text-white/70">{type.toUpperCase()}</p>
                          <p className="text-2xl font-bold">{count}</p>
                        </div>
                      ))}
                    </div>
                  </div>
                </div>
              ) : (
                <div className="text-center py-12">
                  <div className="text-6xl mb-4 opacity-20 text-white">📊</div>
                  <p className="text-white/70">No statistics available</p>
                </div>
              )}
            </div>
          )}
        </main>

        <footer className="mt-6 text-center text-white/60 text-sm">
          <div className="flex flex-col sm:flex-row justify-center items-center gap-4 mb-4">
            <a
              href="https://github.com/Xeyo-Developer/MemeGenerator"
              className="flex items-center gap-2 text-white hover:text-purple-300 transition-colors"
              target="_blank"
              rel="noopener noreferrer"
            >
              <Github className="w-5 h-5" />
              GitHub Repository
            </a>
            <span className="hidden sm:block">•</span>
            <p className="flex items-center gap-1">
              Made with <Heart className="w-4 h-4 text-pink-400" /> by Xeyo
            </p>
          </div>
          <div className="flex flex-wrap justify-center items-center gap-3">
            <span>Built with:</span>
            <span className="flex items-center gap-1 px-2 py-1 bg-white/10 rounded-full">
              <SiRust className="text-orange-500" /> Rust
            </span>
            <span className="flex items-center gap-1 px-2 py-1 bg-white/10 rounded-full">
              <SiReact className="text-blue-400" /> React
            </span>
            <span className="flex items-center gap-1 px-2 py-1 bg-white/10 rounded-full">
              <SiTailwindcss className="text-cyan-400" /> Tailwind
            </span>
          </div>
        </footer>
      </div>
    </div>
  );
};

export default MemeGenerator;