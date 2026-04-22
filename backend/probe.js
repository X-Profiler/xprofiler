const axios = require('axios');
const token = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOjEsInVzZXJuYW1lIjoidGVzdHVzZXIiLCJpYXQiOjE3NzY4NDUxNTUsImV4cCI6MTc3Njg0ODc1NX0.F1G01rUc-F4g0Os5uofA5wOpUI9VlXoKACGsL72erpE';

async function test() {
  try {
    const res = await axios.post('http://localhost:3000/api/community', {
      title: 'A'.repeat(500),
      content: 'B'.repeat(5000000)
    }, {
      headers: { Authorization: `Bearer ${token}` }
    });
    console.log(res.status);
  } catch (e) {
    console.log(e.response?.status, e.response?.data);
  }
}
test();
