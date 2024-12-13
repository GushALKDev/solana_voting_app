import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Home from './components/Home';
import CreatePoll from './components/CreatePoll';
import Poll from './components/Poll';

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/create-poll" element={<CreatePoll />} />
        <Route path="/poll/:pollPDAAddress" element={<Poll />} />
      </Routes>
    </Router>
  );
}

export default App;