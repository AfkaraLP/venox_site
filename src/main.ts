import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'

// Suppress specific console warnings and errors
const originalWarn = console.warn;
console.warn = function (...args) {
  const msg = args.join(' ');
  if (
    msg.includes('mozPressure') ||
    msg.includes('mozInputSource') ||
    msg.includes('unreachable code after return statement')
  ) {
    return; // Suppress these warnings
  }
  originalWarn.apply(console, args);
};

const originalError = console.error;
console.error = function (...args) {
  const msg = args.join(' ');
  if (
    msg.includes('mozPressure') ||
    msg.includes('mozInputSource') ||
    msg.includes('unreachable code after return statement')
  ) {
    return; // Suppress these errors
  }
  originalError.apply(console, args);
};

createApp(App).mount('#app')
