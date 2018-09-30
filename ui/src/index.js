// theirs
import React, { render } from 'preact';
import { BrowserRouter as Router } from 'react-router-dom';

// mine
import App from './components/App';
require('./favicon.png')

render((
  <Router>
    <App />
  </Router>
), document.getElementById('root'))

