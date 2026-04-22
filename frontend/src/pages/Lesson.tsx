import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';

// Types
export type ExerciseType = 'vocabulary' | 'grammar' | 'listening' | 'speaking';

export interface Exercise {
  id: string;
  type: ExerciseType;
  question: string;
  options?: string[]; // for vocab, grammar, listening
  correctAnswer?: string; // for vocab, grammar, listening
  audioText?: string; // for listening
}

// Vocabulary Component
const VocabularyExercise: React.FC<{ exercise: Exercise; onAnswer: (isCorrect: boolean) => void }> = ({ exercise, onAnswer }) => {
  const [selected, setSelected] = useState<string | null>(null);
  
  const handleSelect = (option: string) => {
    if (selected !== null) return;
    setSelected(option);
    setTimeout(() => {
      onAnswer(option === exercise.correctAnswer);
    }, 1000);
  };

  return (
    <div className="flex flex-col items-center gap-6 w-full">
      <div className="text-2xl font-bold text-gray-800">{exercise.question}</div>
      <div className="grid grid-cols-2 gap-4 w-full max-w-md">
        {exercise.options?.map((option, idx) => {
          let bgColor = 'bg-white hover:bg-blue-50';
          if (selected === option) {
            bgColor = option === exercise.correctAnswer ? 'bg-green-200' : 'bg-red-200';
          } else if (selected !== null && option === exercise.correctAnswer) {
            bgColor = 'bg-green-200';
          }
          return (
            <button
              key={idx}
              onClick={() => handleSelect(option)}
              disabled={selected !== null}
              className={`p-4 border border-gray-200 rounded-lg shadow-sm transition-colors text-lg font-medium text-gray-700 disabled:cursor-not-allowed ${bgColor}`}
            >
              {option}
            </button>
          );
        })}
      </div>
    </div>
  );
};

// Grammar Component
const GrammarExercise: React.FC<{ exercise: Exercise; onAnswer: (isCorrect: boolean) => void }> = ({ exercise, onAnswer }) => {
  const [selected, setSelected] = useState<string | null>(null);

  const handleSelect = (option: string) => {
    if (selected !== null) return;
    setSelected(option);
    setTimeout(() => {
      onAnswer(option === exercise.correctAnswer);
    }, 1000);
  };

  return (
    <div className="flex flex-col items-center gap-6 w-full">
      <div className="text-xl font-medium text-center text-gray-800">{exercise.question}</div>
      <div className="flex flex-col gap-3 w-full max-w-md">
        {exercise.options?.map((option, idx) => {
          let bgColor = 'bg-white hover:bg-blue-50';
          if (selected === option) {
            bgColor = option === exercise.correctAnswer ? 'bg-green-200' : 'bg-red-200';
          } else if (selected !== null && option === exercise.correctAnswer) {
            bgColor = 'bg-green-200';
          }
          return (
            <button
              key={idx}
              onClick={() => handleSelect(option)}
              disabled={selected !== null}
              className={`p-3 border border-gray-200 rounded-lg shadow-sm text-left transition-colors text-gray-700 font-medium disabled:cursor-not-allowed ${bgColor}`}
            >
              {option}
            </button>
          );
        })}
      </div>
    </div>
  );
};

// Listening Component
const ListeningExercise: React.FC<{ exercise: Exercise; onAnswer: (isCorrect: boolean) => void }> = ({ exercise, onAnswer }) => {
  const [selected, setSelected] = useState<string | null>(null);

  const playAudio = () => {
    const textToSpeak = exercise.audioText || exercise.question;
    const utterance = new SpeechSynthesisUtterance(textToSpeak);
    // utterance.lang = 'en-US'; // optional based on language
    window.speechSynthesis.speak(utterance);
  };

  const handleSelect = (option: string) => {
    if (selected !== null) return;
    setSelected(option);
    setTimeout(() => {
      onAnswer(option === exercise.correctAnswer);
    }, 1000);
  };

  return (
    <div className="flex flex-col items-center gap-6 w-full">
      <div className="text-xl font-medium mb-2 text-gray-700">听音选择正确的选项</div>
      <button 
        onClick={playAudio}
        className="p-4 bg-blue-100 text-blue-700 rounded-full hover:bg-blue-200 transition-colors shadow-sm"
        title="播放音频"
      >
        <svg xmlns="http://www.w3.org/2000/svg" className="h-10 w-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 12.728M5 10v4a2 2 0 002 2h3l5 5V3l-5 5H7a2 2 0 00-2 2z" />
        </svg>
      </button>

      <div className="flex flex-col gap-3 w-full max-w-md mt-4">
        {exercise.options?.map((option, idx) => {
          let bgColor = 'bg-white hover:bg-blue-50';
          if (selected === option) {
            bgColor = option === exercise.correctAnswer ? 'bg-green-200' : 'bg-red-200';
          } else if (selected !== null && option === exercise.correctAnswer) {
            bgColor = 'bg-green-200';
          }
          return (
            <button
              key={idx}
              onClick={() => handleSelect(option)}
              disabled={selected !== null}
              className={`p-3 border border-gray-200 rounded-lg shadow-sm text-left transition-colors font-medium text-gray-700 disabled:cursor-not-allowed ${bgColor}`}
            >
              {option}
            </button>
          );
        })}
      </div>
    </div>
  );
};

// Speaking Component
const SpeakingExercise: React.FC<{ exercise: Exercise; onAnswer: (isCorrect: boolean) => void }> = ({ exercise, onAnswer }) => {
  const [recording, setRecording] = useState(false);
  const [mediaRecorder, setMediaRecorder] = useState<MediaRecorder | null>(null);
  const [score, setScore] = useState<number | null>(null);
  const [hasAnswered, setHasAnswered] = useState(false);

  const startRecording = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      const recorder = new MediaRecorder(stream);
      recorder.start();
      setMediaRecorder(recorder);
      setRecording(true);
      setScore(null);
    } catch (err) {
      console.error("Microphone access denied or error:", err);
      // Fallback for demo without real microphone
      alert("无法访问麦克风。将进行模拟录音。");
      setRecording(true);
      setScore(null);
    }
  };

  const stopRecording = () => {
    if (mediaRecorder && mediaRecorder.state !== 'inactive') {
      mediaRecorder.stop();
      mediaRecorder.stream.getTracks().forEach(track => track.stop());
    }
    
    setRecording(false);
    
    // Mock scoring
    if (!hasAnswered) {
      setHasAnswered(true);
      setTimeout(() => {
        const fakeScore = Math.floor(Math.random() * 41) + 60; // 60 to 100
        setScore(fakeScore);
        setTimeout(() => {
          onAnswer(fakeScore >= 60);
        }, 2500);
      }, 1000);
    }
  };

  return (
    <div className="flex flex-col items-center gap-6 w-full">
      <div className="text-xl font-medium text-gray-700">请朗读以下句子：</div>
      <div className="text-2xl font-bold text-blue-600 my-4 text-center">{exercise.question}</div>
      
      {!recording ? (
        <button 
          onClick={startRecording}
          disabled={hasAnswered}
          className="px-8 py-4 bg-red-500 text-white font-semibold rounded-full shadow hover:bg-red-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
          </svg>
          开始录音
        </button>
      ) : (
        <button 
          onClick={stopRecording}
          className="px-8 py-4 bg-gray-500 text-white font-semibold rounded-full shadow hover:bg-gray-600 transition-colors animate-pulse flex items-center gap-2"
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 10a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1v-4z" />
          </svg>
          停止录音
        </button>
      )}

      {score !== null && (
        <div className="mt-6 text-center animate-fade-in">
          <div className="text-lg text-gray-600">得分：<span className="font-bold text-3xl ml-2 text-gray-800">{score}</span></div>
          <div className={`mt-2 text-xl font-medium ${score >= 80 ? "text-green-500" : "text-yellow-600"}`}>
            {score >= 80 ? "发音很棒！🎉" : "再接再厉！💪"}
          </div>
        </div>
      )}
    </div>
  );
};

// Main Lesson Component
export default function Lesson() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [exercises, setExercises] = useState<Exercise[]>([]);
  const [currentIndex, setCurrentIndex] = useState(0);
  const [score, setScore] = useState(0);
  const [finished, setFinished] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchExercises = async () => {
      try {
        const response = await fetch(`/api/lessons/${id}/exercises`);
        if (!response.ok) {
          throw new Error('获取题目失败');
        }
        const data = await response.json();
        
        if (data && Array.isArray(data) && data.length > 0) {
          setExercises(data);
        } else {
          throw new Error('Empty data');
        }
      } catch (err: unknown) {
        console.warn("API request failed, falling back to mock data:", err);
        // Fallback to mock data if API is not ready or returns empty/error
        setExercises([
          {
            id: '1',
            type: 'vocabulary',
            question: 'Apple',
            options: ['苹果', '香蕉', '橘子', '葡萄'],
            correctAnswer: '苹果'
          },
          {
            id: '2',
            type: 'grammar',
            question: 'He ____ a book now.',
            options: ['is reading', 'read', 'reading', 'reads'],
            correctAnswer: 'is reading'
          },
          {
            id: '3',
            type: 'listening',
            question: 'What time is it?',
            audioText: 'What time is it?',
            options: ['How are you?', 'What time is it?', 'Who is he?', 'Where are you?'],
            correctAnswer: 'What time is it?'
          },
          {
            id: '4',
            type: 'speaking',
            question: 'I love learning new languages.',
          }
        ]);
        // setError(err instanceof Error ? err.message : 'Error fetching exercises');
      } finally {
        setLoading(false);
      }
    };

    fetchExercises();
  }, [id]);

  if (loading) return (
    <div className="flex justify-center items-center min-h-[50vh]">
      <div className="text-xl text-gray-500 font-medium animate-pulse">加载题目中...</div>
    </div>
  );
  
  if (!exercises || exercises.length === 0) return <div className="p-8 text-center text-gray-500">没有找到题目。</div>;

  if (finished) {
    return (
      <div className="max-w-2xl mx-auto mt-12 p-10 bg-white rounded-2xl shadow-xl text-center border border-gray-100">
        <h2 className="text-4xl font-bold mb-8 text-gray-800">学习完成！🎉</h2>
        <div className="text-2xl mb-10 text-gray-600">
          你的得分：<span className="text-blue-600 font-extrabold text-5xl mx-2">{score}</span> / {exercises.length}
        </div>
        <div className="w-full bg-gray-200 rounded-full h-4 mb-10 overflow-hidden">
          <div 
            className="bg-green-500 h-4 rounded-full transition-all duration-1000 ease-out" 
            style={{ width: `${(score / exercises.length) * 100}%` }}
          ></div>
        </div>
        <button 
          onClick={() => navigate('/course')}
          className="px-8 py-3 bg-blue-600 text-white font-semibold text-lg rounded-xl shadow-md hover:bg-blue-700 hover:shadow-lg transition-all"
        >
          返回课程列表
        </button>
      </div>
    );
  }

  const currentExercise = exercises[currentIndex];
  // Calculate progress ensuring we account for current exercise being active
  const progress = ((currentIndex) / exercises.length) * 100;

  const handleAnswer = (isCorrect: boolean) => {
    let newScore = score;
    if (isCorrect) {
      newScore += 1;
      setScore(newScore);
    }
    
    if (currentIndex < exercises.length - 1) {
      setCurrentIndex(prev => prev + 1);
    } else {
      setFinished(true);
      
      // 保存学习进度
      fetch('/api/progress', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('token')}` // 假设使用 token
        },
        body: JSON.stringify({
          lessonId: id,
          score: newScore,
          total: exercises.length,
          percentage: exercises.length > 0 ? Math.round((newScore / exercises.length) * 100) : 0
        })
      }).catch(err => console.error("保存进度失败:", err));
    }
  };

  const renderExercise = () => {
    switch (currentExercise.type) {
      case 'vocabulary':
        return <VocabularyExercise key={currentExercise.id} exercise={currentExercise} onAnswer={handleAnswer} />;
      case 'grammar':
        return <GrammarExercise key={currentExercise.id} exercise={currentExercise} onAnswer={handleAnswer} />;
      case 'listening':
        return <ListeningExercise key={currentExercise.id} exercise={currentExercise} onAnswer={handleAnswer} />;
      case 'speaking':
        return <SpeakingExercise key={currentExercise.id} exercise={currentExercise} onAnswer={handleAnswer} />;
      default:
        return <div className="text-red-500">未知的题目类型: {currentExercise.type}</div>;
    }
  };

  const typeLabels: Record<string, string> = {
    vocabulary: '词汇',
    grammar: '语法',
    listening: '听力',
    speaking: '口语'
  };

  return (
    <div className="max-w-3xl mx-auto mt-10 p-8 bg-white rounded-2xl shadow-lg border border-gray-100">
      {/* Progress Bar */}
      <div className="w-full bg-gray-100 rounded-full h-3 mb-6 overflow-hidden">
        <div className="bg-blue-500 h-3 rounded-full transition-all duration-500 ease-in-out" style={{ width: `${progress}%` }}></div>
      </div>
      
      <div className="flex justify-between items-center mb-10 text-gray-500">
        <span className="font-medium text-lg">题目 {currentIndex + 1} <span className="text-gray-400">/ {exercises.length}</span></span>
        <span className="text-sm font-bold tracking-wider bg-blue-50 text-blue-600 px-4 py-1.5 rounded-full border border-blue-100">
          {typeLabels[currentExercise.type] || currentExercise.type.toUpperCase()}
        </span>
      </div>

      <div className="min-h-[350px] flex items-center justify-center">
        {renderExercise()}
      </div>
    </div>
  );
}
