const axios = require('axios');

const API_URL = 'http://localhost:3000/api';

async function runTests() {
  let token = '';
  let userId = '';

  console.log('--- Testing Auth ---');
  try {
    const regRes = await axios.post(`${API_URL}/auth/register`, {
      email: 'testuser22@example.com',
      username: 'testuser2',
      password: 'password123'
    });
    console.log('Register:', regRes.status, regRes.data);
  } catch (e) {
    console.log('Register error (expected if already registered):', e.response?.status, e.response?.data);
  }

  try {
    const loginRes = await axios.post(`${API_URL}/auth/login`, {
      email: 'testuser22@example.com',
      password: 'password123'
    });
    console.log('Login:', loginRes.status);
    token = loginRes.data.token;
    userId = loginRes.data.user.id;
  } catch (e) {
    console.error('Login failed!', e.response?.status, e.response?.data);
    return;
  }

  console.log('\n--- Testing Courses ---');
  try {
    const coursesRes = await axios.get(`${API_URL}/courses`);
    console.log('Get courses:', coursesRes.status, `Found ${coursesRes.data.length} courses`);
    if (coursesRes.data.length > 0) {
      const courseId = coursesRes.data[0].id;
      const courseRes = await axios.get(`${API_URL}/courses/${courseId}`);
      console.log('Get single course:', courseRes.status, courseRes.data.title);
    }
  } catch (e) {
    console.error('Courses failed!', e.response?.status, e.response?.data);
  }

  console.log('\n--- Testing Progress (Happy Path) ---');
  try {
    const progRes = await axios.post(`${API_URL}/progress`, {
      lessonId: 13,
      score: 85
    }, { headers: { Authorization: `Bearer ${token}` } });
    console.log('Submit progress:', progRes.status, progRes.data);
  } catch (e) {
    console.error('Submit progress failed!', e.response?.status, e.response?.data);
  }

  console.log('\n--- Adversarial Probes ---');
  
  // Probe 1: Duplicate Registration
  try {
    const dupRes = await axios.post(`${API_URL}/auth/register`, {
      email: 'testuser22@example.com',
      username: 'testuser22',
      password: 'password123'
    });
    console.log('PROBE 1 FAIL: Duplicate registration allowed!', dupRes.status);
  } catch (e) {
    console.log('PROBE 1 PASS: Duplicate registration rejected with', e.response?.status, e.response?.data);
  }

  // Probe 2: Invalid Progress Score (negative)
  try {
    const badProgRes = await axios.post(`${API_URL}/progress`, {
      lessonId: 13,
      score: 150
    }, { headers: { Authorization: `Bearer ${token}` } });
    console.log('PROBE 2 FAIL: Negative score allowed!', badProgRes.status, badProgRes.data);
  } catch (e) {
    console.log('PROBE 2 PASS: Negative score rejected with', e.response?.status, e.response?.data);
  }

  // Probe 3: Invalid Course ID
  try {
    const badCourseRes = await axios.get(`${API_URL}/courses/not-a-number`);
    console.log('PROBE 3 FAIL: Invalid course ID allowed!', badCourseRes.status, badCourseRes.data);
  } catch (e) {
    console.log('PROBE 3 PASS: Invalid course ID rejected with', e.response?.status, e.response?.data);
  }
}

runTests();
