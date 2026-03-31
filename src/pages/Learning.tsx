import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { useParams, useNavigate } from 'react-router-dom';
import { Mic, Play, CheckCircle, XCircle, ArrowRight, X, Volume2, HelpCircle } from 'lucide-react';
import { useStore } from '../store';

const EXERCISES = [
  { type: 'vocab', question: 'Apple', answer: '苹果', options: ['苹果', '香蕉', '橘子', '梨'] },
  { type: 'listening', question: '请听录音并选择正确的单词', audio: true, answer: 'Morning', options: ['Morning', 'Evening', 'Afternoon', 'Night'] },
  { type: 'speaking', question: '请朗读以下句子', text: 'How are you doing today?' },
  { type: 'grammar', question: 'I ___ to the store yesterday.', answer: 'went', options: ['go', 'goes', 'went', 'going'] },
];

const Learning = () => {
  const { lessonId } = useParams();
  const navigate = useNavigate();
  const updateExp = useStore(state => state.updateExp);
  
  const [currentStep, setCurrentStep] = useState(0);
  const [selectedOption, setSelectedOption] = useState<string | null>(null);
  const [isCorrect, setIsCorrect] = useState<boolean | null>(null);
  const [isRecording, setIsRecording] = useState(false);

  const exercise = EXERCISES[currentStep];
  const isFinished = currentStep >= EXERCISES.length;

  const handleSelect = (option: string) => {
    if (selectedOption) return; // Prevent multiple selections
    setSelectedOption(option);
    const correct = option === exercise.answer;
    setIsCorrect(correct);
    if (correct) updateExp(10);
  };

  const handleNext = () => {
    setSelectedOption(null);
    setIsCorrect(null);
    setCurrentStep(prev => prev + 1);
  };

  const simulateRecording = () => {
    setIsRecording(true);
    setTimeout(() => {
      setIsRecording(false);
      setIsCorrect(true);
      updateExp(15);
    }, 2000);
  };

  if (isFinished) {
    return (
      <div className="h-full flex flex-col items-center justify-center p-8 bg-gradient-to-b from-blue-50 to-white">
        <motion.div 
          initial={{ scale: 0.8, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          className="bg-white p-10 rounded-3xl shadow-xl text-center max-w-md w-full"
        >
          <div className="w-24 h-24 bg-green-100 text-green-500 rounded-full flex items-center justify-center mx-auto mb-6">
            <CheckCircle className="w-12 h-12" />
          </div>
          <h2 className="text-3xl font-bold text-slate-800 mb-2">太棒了！</h2>
          <p className="text-slate-500 mb-8">你已完成本次学习任务，获得了 <span className="font-bold text-amber-500">45 EXP</span></p>
          <button 
            onClick={() => navigate('/courses')}
            className="w-full btn-primary text-lg py-4"
          >
            返回课程中心
          </button>
        </motion.div>
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col bg-slate-50 relative">
      {/* Header Bar */}
      <div className="h-16 px-6 flex items-center justify-between bg-white border-b border-slate-200">
        <button onClick={() => navigate('/courses')} className="p-2 hover:bg-slate-100 rounded-full text-slate-400 hover:text-slate-600 transition">
          <X className="w-6 h-6" />
        </button>
        
        {/* Progress Bar */}
        <div className="flex-1 max-w-xl mx-8">
          <div className="h-2.5 bg-slate-100 rounded-full overflow-hidden">
            <motion.div 
              className="h-full bg-blue-500 rounded-full"
              initial={{ width: `${(currentStep / EXERCISES.length) * 100}%` }}
              animate={{ width: `${(currentStep / EXERCISES.length) * 100}%` }}
              transition={{ duration: 0.5 }}
            />
          </div>
        </div>
        
        <button className="p-2 hover:bg-slate-100 rounded-full text-slate-400 hover:text-slate-600 transition">
          <HelpCircle className="w-6 h-6" />
        </button>
      </div>

      {/* Main Exercise Area */}
      <div className="flex-1 overflow-auto flex flex-col items-center p-6 md:p-12">
        <div className="w-full max-w-3xl flex-1 flex flex-col">
          
          <AnimatePresence mode="wait">
            <motion.div
              key={currentStep}
              initial={{ x: 20, opacity: 0 }}
              animate={{ x: 0, opacity: 1 }}
              exit={{ x: -20, opacity: 0 }}
              transition={{ duration: 0.3 }}
              className="flex-1 flex flex-col justify-center"
            >
              <h2 className="text-2xl md:text-3xl font-bold text-slate-800 mb-10 text-center flex items-center justify-center gap-3">
                {exercise.audio && (
                  <button className="w-12 h-12 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center hover:bg-blue-200 transition shadow-sm">
                    <Volume2 className="w-6 h-6" />
                  </button>
                )}
                {exercise.question}
              </h2>

              {exercise.type === 'speaking' ? (
                <div className="flex flex-col items-center gap-8">
                  <div className="text-4xl font-medium text-slate-700 bg-white px-8 py-6 rounded-2xl shadow-sm border border-slate-100">
                    "{exercise.text}"
                  </div>
                  
                  <div className="h-32 flex items-center justify-center">
                    {isRecording ? (
                      <div className="flex items-center gap-2 h-12">
                        {[...Array(5)].map((_, i) => (
                          <motion.div
                            key={i}
                            animate={{ height: ['20%', '100%', '20%'] }}
                            transition={{ repeat: Infinity, duration: 0.8, delay: i * 0.1 }}
                            className="w-3 bg-blue-500 rounded-full"
                          />
                        ))}
                      </div>
                    ) : (
                      <button 
                        onClick={simulateRecording}
                        className={`w-24 h-24 rounded-full flex items-center justify-center transition-all ${
                          isCorrect 
                            ? 'bg-green-100 text-green-500' 
                            : 'bg-blue-600 text-white hover:bg-blue-700 hover:shadow-lg hover:scale-105'
                        }`}
                      >
                        {isCorrect ? <CheckCircle className="w-10 h-10" /> : <Mic className="w-10 h-10" />}
                      </button>
                    )}
                  </div>
                </div>
              ) : (
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  {exercise.options?.map((option, idx) => {
                    let btnClass = "bg-white border-2 border-slate-200 text-slate-700 hover:border-blue-400 hover:bg-blue-50";
                    if (selectedOption === option) {
                      btnClass = isCorrect 
                        ? "bg-green-50 border-2 border-green-500 text-green-700" 
                        : "bg-red-50 border-2 border-red-500 text-red-700";
                    } else if (selectedOption && option === exercise.answer) {
                      btnClass = "bg-green-50 border-2 border-green-500 text-green-700"; // Show correct answer if failed
                    }

                    return (
                      <button
                        key={idx}
                        onClick={() => handleSelect(option)}
                        disabled={!!selectedOption}
                        className={`p-6 text-xl font-medium rounded-2xl transition-all duration-200 ${btnClass} flex justify-between items-center`}
                      >
                        {option}
                        {selectedOption === option && isCorrect && <CheckCircle className="w-6 h-6 text-green-500" />}
                        {selectedOption === option && !isCorrect && <XCircle className="w-6 h-6 text-red-500" />}
                      </button>
                    );
                  })}
                </div>
              )}
            </motion.div>
          </AnimatePresence>

        </div>
      </div>

      {/* Footer Actions */}
      <div className={`border-t p-6 md:px-12 flex items-center justify-between transition-colors duration-300 ${
        selectedOption || (exercise.type === 'speaking' && isCorrect) 
          ? isCorrect ? 'bg-green-50 border-green-100' : 'bg-red-50 border-red-100' 
          : 'bg-white border-slate-200'
      }`}>
        <div className="flex-1">
          {selectedOption && (
            <div className="flex items-center gap-3">
              <div className={`w-12 h-12 rounded-full flex items-center justify-center ${isCorrect ? 'bg-green-100 text-green-600' : 'bg-red-100 text-red-600'}`}>
                {isCorrect ? <CheckCircle className="w-6 h-6" /> : <XCircle className="w-6 h-6" />}
              </div>
              <div>
                <div className={`text-xl font-bold ${isCorrect ? 'text-green-700' : 'text-red-700'}`}>
                  {isCorrect ? '回答正确！' : '正确的答案是：'}
                </div>
                {!isCorrect && <div className="text-red-600 font-medium">{exercise.answer}</div>}
              </div>
            </div>
          )}
        </div>
        
        <button
          onClick={handleNext}
          disabled={!selectedOption && !(exercise.type === 'speaking' && isCorrect)}
          className={`px-8 py-3.5 rounded-xl font-bold text-lg flex items-center gap-2 transition-all ${
            selectedOption || (exercise.type === 'speaking' && isCorrect)
              ? isCorrect 
                ? 'bg-green-500 hover:bg-green-600 text-white' 
                : 'bg-red-500 hover:bg-red-600 text-white'
              : 'bg-slate-200 text-slate-400 cursor-not-allowed'
          }`}
        >
          继续
          <ArrowRight className="w-5 h-5" />
        </button>
      </div>
    </div>
  );
};

export default Learning;
