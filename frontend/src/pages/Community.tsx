import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useAuth } from '../context/AuthContext';

interface Post {
  id: number;
  title: string;
  content: string;
  author: {
    username: string;
  };
  createdAt: string;
}

export default function Community() {
  const { isReady } = useAuth();
  const [posts, setPosts] = useState<Post[]>([]);
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');
  const [submitting, setSubmitting] = useState(false);

  const fetchPosts = () => {
    axios.get('/api/community')
      .then(res => setPosts(res.data))
      .catch(err => console.error(err));
  };

  useEffect(() => {
    if (isReady) {
      fetchPosts();
    }
  }, [isReady]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!title.trim() || !content.trim()) return;

    try {
      setSubmitting(true);
      await axios.post('/api/community', { title, content });
      setTitle('');
      setContent('');
      fetchPosts();
    } catch (error) {
      console.error('发帖失败:', error);
      alert('发帖失败，请重试');
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-3xl mx-auto space-y-8">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 text-center">学习社区</h1>
          <p className="mt-2 text-center text-gray-600">在这里与大家交流学习心得</p>
        </div>

        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-4">发布新动态</h2>
          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <label htmlFor="title" className="block text-sm font-medium text-gray-700">标题</label>
              <input
                type="text"
                id="title"
                value={title}
                onChange={e => setTitle(e.target.value)}
                className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 border p-2"
                placeholder="输入标题"
                required
              />
            </div>
            <div>
              <label htmlFor="content" className="block text-sm font-medium text-gray-700">内容</label>
              <textarea
                id="content"
                rows={4}
                value={content}
                onChange={e => setContent(e.target.value)}
                className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 border p-2"
                placeholder="分享你的想法..."
                required
              />
            </div>
            <div className="flex justify-end">
              <button
                type="submit"
                disabled={submitting}
                className={`px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 ${submitting ? 'opacity-50 cursor-not-allowed' : ''}`}
              >
                {submitting ? '发布中...' : '发布'}
              </button>
            </div>
          </form>
        </div>

        <div className="space-y-6">
          <h2 className="text-xl font-semibold">最新动态</h2>
          {posts.length > 0 ? (
            posts.map(post => (
              <div key={post.id} className="bg-white p-6 rounded-lg shadow">
                <h3 className="text-lg font-bold text-gray-900">{post.title}</h3>
                <p className="mt-2 text-gray-700 whitespace-pre-wrap">{post.content}</p>
                <div className="mt-4 flex items-center justify-between text-sm text-gray-500">
                  <span>作者: {post.author?.username || '匿名'}</span>
                  <span>{new Date(post.createdAt).toLocaleString()}</span>
                </div>
              </div>
            ))
          ) : (
            <div className="text-center py-8 text-gray-500 bg-white rounded-lg shadow">
              暂无动态，快来发布第一条吧！
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
