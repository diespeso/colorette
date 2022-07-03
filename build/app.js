import Clock from './clock.js';
import LikeButton from './like_button.js';
import Stopper from './stopper.js';

var container = document.getElementById('like_button_container');
var root = ReactDOM.createRoot(container);
root.render(React.createElement(Stopper, null));