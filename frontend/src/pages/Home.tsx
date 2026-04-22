import { Link } from 'react-router-dom';
import { useEffect, useState } from 'react';
import axios from 'axios';
import { useAuth } from '../context/AuthContext';

interface Recommendation {
  id: number;
  title: string;
  description: string;
}

export default function Home() {
  const { isReady } = useAuth();
  const [recommendations, setRecommendations] = useState<Recommendation[]>([]);

  useEffect(() => {
    if (isReady) {
      axios.get('/api/recommendations')
        .then(res => {
          if (Array.isArray(res.data)) {
            setRecommendations(res.data);
          } else if (res.data.lesson) {
            setRecommendations([res.data.lesson]);
          } else if (res.data.title) {
            setRecommendations([res.data]);
          } else {
            setRecommendations([]);
          }
        })
        .catch(err => console.error(err));
    }
  }, [isReady]);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100 py-12">
      <h1 className="text-4xl font-bold text-blue-600 mb-6">主页 (Home)</h1>
      <div className="space-x-4 mb-10">
        <Link to="/login" className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">登录</Link>
        <Link to="/course" className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600">课程</Link>
        <Link to="/community" className="px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600">社区</Link>
      </div>

      <div className="w-full max-w-2xl px-4">
        <h2 className="text-2xl font-bold mb-4 text-gray-800">为您推荐</h2>
        <div className="space-y-4">
          {recommendations.map(rec => (
            <div key={rec.id} className="bg-white p-4 rounded shadow">
              <h3 className="text-xl font-semibold">{rec.title}</h3>
              <p className="text-gray-600 mt-2">{rec.description}</p>
            </div>
          ))}
          {recommendations.length === 0 && (
            <p className="text-gray-500 text-center py-4">暂无推荐内容</p>
          )}
        </div>
      </div>
    </div>
  );
}